//! opencode-notify.rs - Termux通知フック (rust-script)
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    fn to_config(&self, message: &str) -> NotificationConfig {
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
                button1: Some((
                    "フォーカス".into(),
                    "am start -n com.termux/.app.TermuxActivity".into(),
                )),
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

    let config = typ.to_config(&message);

    if let Err(e) = send_notification(&config, dry_run) {
        eprintln!("[opencode-notify] notification failed: {e}");
    }

    Ok(())
}
