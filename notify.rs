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

fn send_notification(config: NotificationConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("termux-notification");
    cmd.arg("-t")
        .arg(&config.title)
        .arg("-c")
        .arg(&config.content)
        .arg("--id")
        .arg(&config.id);

    if let Some(priority) = config.priority {
        cmd.arg("--priority").arg(priority);
    }
    if config.sound {
        cmd.arg("--sound");
    }
    if let Some(vibrate) = config.vibrate {
        cmd.arg("--vibrate").arg(vibrate);
    }
    if config.ongoing {
        cmd.arg("--ongoing");
    }
    if let Some((label, action)) = config.button1 {
        cmd.arg("--button1")
            .arg(label)
            .arg("--button1-action")
            .arg(action);
    }

    cmd.stderr(Stdio::inherit())
        .stdout(Stdio::null())
        .status()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <type> <message>", args[0]);
        eprintln!("Types: complete, error, wait, start");
        std::process::exit(1);
    }

    let typ = args[1].parse::<NotificationType>()?;
    let message = &args[2];

    let message = if message.len() > 500 {
        format!("{}...", &message[..497])
    } else {
        message.clone()
    };

    let config = typ.to_config(&message);

    if let Err(e) = send_notification(config) {
        eprintln!("[opencode-notify] notification failed: {e}");
    }

    Ok(())
}
