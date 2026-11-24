//! opencode-notify.rs - Termux通知フック (rust-script)
//!
//! 使用方法:
//!   rust-script notify.rs [--dry-run] <type> <message>
//!
//! 設定 (すべて任意、未設定なら既定値で動作する):
//!   ~/.config/opencode/notify.toml で通知内容をタイプ別に上書きできる。
//!   セクション: complete / error / wait / start / other
//!   キー: title / priority / sound / vibrate / ongoing /
//!         button1_label / button1_action
//!   ボタン: 既定ボタンのある wait は片方のみの上書きが可能。
//!   ボタンを持たないタイプへ追加する場合は両方の指定が必要
//!   (片方のみの場合は警告のうえ無視)。
//!   未知キー・形式不正があると設定全体が無視され、既定値で動作する
//!   (警告を stderr に出力)。
//!
//!   例:
//!     [error]
//!     priority = "high"
//!     vibrate = "100,200,100"
//!
//!     [wait]
//!     button1_action = "am start -n com.termux/.app.TermuxActivity"
//!
//! 環境変数:
//!   OPENCODE_NOTIFY_CONFIG        設定ファイルパス (既定を上書き)
//!   OPENCODE_NOTIFY_FOCUS_ACTION  wait通知のボタンアクション (最優先)
//!   OPENCODE_NOTIFY_INCLUDE_TITLE セッションタイトル相当の接尾辞を保持する場合の明示的 opt-in
//!                                 (既定では固定ラベル以降を除去して通知へ渡さない)
//!
//! 信頼境界:
//!   notify.toml と環境変数は実行ユーザー自身が管理する信頼済み入力として扱う。
//!   設定値はサニタイズせずそのまま termux-notification に渡す。
//!   OPENCODE_NOTIFY_CONFIG はアプリ専用領域 (Termuxホーム) 内のパスを指定すること。
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! toml = "0.8"
//! ```

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum NotificationType {
    Complete,
    Error,
    Wait,
    Start,
    Other(String),
}

impl FromStr for NotificationType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "complete" => NotificationType::Complete,
            "error" => NotificationType::Error,
            "wait" => NotificationType::Wait,
            "start" => NotificationType::Start,
            other => NotificationType::Other(other.to_string()),
        })
    }
}

const CONFIG_PATH_ENV: &str = "OPENCODE_NOTIFY_CONFIG";
const FOCUS_ACTION_ENV: &str = "OPENCODE_NOTIFY_FOCUS_ACTION";
const INCLUDE_TITLE_ENV: &str = "OPENCODE_NOTIFY_INCLUDE_TITLE";
const DEFAULT_FOCUS_ACTION: &str = "am start -n com.termux/.app.TermuxActivity";

/// 通知タイプごとの上書き設定 (notify.toml のセクション)。
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct TypeOverride {
    title: Option<String>,
    priority: Option<String>,
    sound: Option<bool>,
    vibrate: Option<String>,
    ongoing: Option<bool>,
    button1_label: Option<String>,
    button1_action: Option<String>,
}

/// notify.toml 全体の設定。未指定セクション・キーは既定値のまま。
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct NotifyOptions {
    complete: TypeOverride,
    error: TypeOverride,
    wait: TypeOverride,
    start: TypeOverride,
    other: TypeOverride,
}

impl NotifyOptions {
    fn for_type(&self, typ: &NotificationType) -> &TypeOverride {
        match typ {
            NotificationType::Complete => &self.complete,
            NotificationType::Error => &self.error,
            NotificationType::Wait => &self.wait,
            NotificationType::Start => &self.start,
            NotificationType::Other(_) => &self.other,
        }
    }
}

impl TypeOverride {
    fn apply_to(&self, config: &mut NotificationConfig, type_name: &str) {
        if let Some(title) = &self.title {
            config.title = title.clone();
        }
        if let Some(priority) = &self.priority {
            config.priority = Some(priority.clone());
        }
        if let Some(sound) = self.sound {
            config.sound = sound;
        }
        if let Some(vibrate) = &self.vibrate {
            config.vibrate = Some(vibrate.clone());
        }
        if let Some(ongoing) = self.ongoing {
            config.ongoing = ongoing;
        }

        let label = self
            .button1_label
            .clone()
            .or_else(|| config.button1.as_ref().map(|button| button.0.clone()));
        let action = self
            .button1_action
            .clone()
            .or_else(|| config.button1.as_ref().map(|button| button.1.clone()));

        match (label, action) {
            (Some(label), Some(action)) => config.button1 = Some((label, action)),
            (Some(_), None) | (None, Some(_)) => {
                eprintln!(
                    "[opencode-notify] ignoring partial button1 override for \
                     '{type_name}': button1_label and button1_action must be set together"
                );
            }
            (None, None) => {}
        }
    }
}

fn config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var(CONFIG_PATH_ENV) {
        return Some(PathBuf::from(path));
    }
    env::var_os("HOME").map(|home| PathBuf::from(home).join(".config/opencode/notify.toml"))
}

/// 設定ファイルを読み込む。未作成・読取不能・形式不正は既定値で継続する (fail-open)。
fn load_options() -> NotifyOptions {
    let path = match config_path() {
        Some(path) => path,
        None => return NotifyOptions::default(),
    };

    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return NotifyOptions::default();
        }
        Err(err) => {
            eprintln!(
                "[opencode-notify] cannot read config {}: {err}",
                path.display()
            );
            return NotifyOptions::default();
        }
    };

    match toml::from_str(&text) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("[opencode-notify] invalid config {}: {err}", path.display());
            NotifyOptions::default()
        }
    }
}

#[derive(Debug, Clone)]
struct NotificationConfig {
    title: String,
    content: String,
    id: String,
    priority: Option<String>,
    sound: bool,
    vibrate: Option<String>,
    ongoing: bool,
    button1: Option<(String, String)>,
}

impl NotificationType {
    fn to_config(&self, message: &str, options: &NotifyOptions) -> NotificationConfig {
        let mut config = self.base_config(&notification_content(message, self.as_str()));

        options.for_type(self).apply_to(&mut config, self.as_str());

        if let NotificationType::Wait = self {
            if let Ok(action) = env::var(FOCUS_ACTION_ENV) {
                if let Some(button) = config.button1.as_mut() {
                    button.1 = action;
                }
            }
        }

        config
    }

    fn base_config(&self, message: &str) -> NotificationConfig {
        let id = format!("opencode-{}", self.as_str());
        match self {
            NotificationType::Complete => NotificationConfig {
                title: "✅ opencode".into(),
                content: message.into(),
                id,
                priority: None,
                sound: true,
                vibrate: None,
                ongoing: false,
                button1: None,
            },
            NotificationType::Error => NotificationConfig {
                title: "❌ opencode".into(),
                content: message.into(),
                id,
                priority: Some("high".into()),
                sound: false,
                vibrate: Some("100,200,100".into()),
                ongoing: false,
                button1: None,
            },
            NotificationType::Wait => NotificationConfig {
                title: "⏳ opencode".into(),
                content: message.into(),
                id,
                priority: None,
                sound: false,
                vibrate: None,
                ongoing: true,
                button1: Some(("フォーカス".into(), DEFAULT_FOCUS_ACTION.into())),
            },
            NotificationType::Start => NotificationConfig {
                title: "🚀 opencode".into(),
                content: message.into(),
                id,
                priority: None,
                sound: true,
                vibrate: None,
                ongoing: false,
                button1: None,
            },
            NotificationType::Other(_) => NotificationConfig {
                title: "opencode".into(),
                content: message.into(),
                id,
                priority: None,
                sound: false,
                vibrate: None,
                ongoing: false,
                button1: None,
            },
        }
    }

    fn as_str(&self) -> &str {
        match self {
            NotificationType::Complete => "complete",
            NotificationType::Error => "error",
            NotificationType::Wait => "wait",
            NotificationType::Start => "start",
            NotificationType::Other(s) => s,
        }
    }
}

fn send_notification(
    config: &NotificationConfig,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("termux-notification");
    cmd.arg("-t")
        .arg(&config.title)
        .arg("-c")
        .arg(&config.content)
        .arg("--id")
        .arg(&config.id);

    if let Some(priority) = &config.priority {
        cmd.arg("--priority").arg(priority);
    }
    if config.sound {
        cmd.arg("--sound");
    }
    if let Some(vibrate) = &config.vibrate {
        cmd.arg("--vibrate").arg(vibrate);
    }
    if config.ongoing {
        cmd.arg("--ongoing");
    }
    if let Some((label, action)) = &config.button1 {
        cmd.arg("--button1")
            .arg(label)
            .arg("--button1-action")
            .arg(action);
    }

    if dry_run {
        let mut preview = String::from("termux-notification");
        for arg in cmd.get_args() {
            preview.push(' ');
            preview.push_str(&format!("{:?}", arg.to_string_lossy()));
        }
        println!("[opencode-notify] dry-run: {preview}");
        return Ok(());
    }

    cmd.stderr(Stdio::inherit())
        .stdout(Stdio::null())
        .status()?;

    Ok(())
}

fn print_usage(bin: &str) {
    eprintln!("Usage: {bin} [--dry-run] <type> <message>");
    eprintln!("Types: complete, error, wait, start");
    eprintln!("Options:");
    eprintln!("  --dry-run  Print the termux-notification command without executing it");
    eprintln!("Config: ~/.config/opencode/notify.toml (optional, see source header)");
}

/// 呼び出し側が「固定ラベル: セッションタイトル」を連結した場合でも、
/// 既定ではタイトル相当の接尾辞を通知へ渡さない。明示的な opt-in 時のみ元文字列を使う。
fn notification_content(message: &str, type_name: &str) -> String {
    if env::var_os(INCLUDE_TITLE_ENV).is_some() {
        return message.to_string();
    }

    let labels: &[&str] = match type_name {
        "complete" => &["タスク完了"],
        "error" => &["エラー発生"],
        "wait" => &["入力待ち"],
        _ => &[],
    };
    for label in labels {
        if let Some((head, _title)) = message.split_once(": ") {
            if head.starts_with(label) {
                return head.to_string();
            }
        }
    }
    message.to_string()
}

/// Truncate to `max_chars` characters (not bytes) with an ellipsis suffix.
fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }

    let mut truncated: String = text.chars().take(max_chars.saturating_sub(3)).collect();
    truncated.push_str("...");
    truncated
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut dry_run = false;
    let mut positional: Vec<&str> = Vec::new();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            "--help" | "-h" => {
                print_usage(&args[0]);
                return Ok(());
            }
            value => positional.push(value),
        }
    }

    if positional.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let typ = positional[0].parse::<NotificationType>()?;
    let message = truncate_chars(positional[1], 500);
    let options = load_options();

    let config = typ.to_config(&message, &options);

    if let Err(e) = send_notification(&config, dry_run) {
        eprintln!("[opencode-notify] notification failed: {e}");
    }

    Ok(())
}
