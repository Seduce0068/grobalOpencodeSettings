//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! chrono = { version = "0.4", features = ["serde"] }
//! clap = { version = "4.5", features = ["derive"] }
//! regex = "1.10"
//! ```

use chrono::{DateTime, FixedOffset, Local, Utc};
use clap::Parser;
use serde_yaml::Mapping;
use std::collections::BTreeMap;

#[derive(Parser)]
#[command(name = "okf_generate", about = "Generate OKF-compliant frontmatter")]
struct Args {
    #[arg(short = 't', long, value_enum, required = true)]
    r#type: DocType,

    #[arg(long, required = true)]
    title: String,

    #[arg(short = 'd', long, required = true)]
    description: String,

    #[arg(long, required = true, value_delimiter = ',')]
    tags: Vec<String>,

    #[arg(long)]
    resource: Option<String>,

    #[arg(long, default_value = "okf-generator")]
    by: String,

    #[arg(long)]
    utc: bool,

    #[arg(long, value_enum)]
    status: Option<Status>,

    #[arg(long)]
    verified_by: Option<String>,

    #[arg(long)]
    stale_after_days: Option<i64>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
enum DocType {
    Reference,
    Playbook,
    ADR,
    Runbook,
    Spec,
    Report,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
enum Status {
    Draft,
    Review,
    Approved,
    Deprecated,
    Archived,
}

fn kebab_case(s: &str) -> String {
    let s = s.to_lowercase();
    let s = regex::Regex::new(r"[_\s]+").unwrap().replace_all(&s, "-");
    let s = regex::Regex::new(r"[^a-z0-9-]")
        .unwrap()
        .replace_all(&s, "");
    let s = regex::Regex::new(r"-+").unwrap().replace_all(&s, "-");
    s.trim_matches('-').to_string()
}

fn rfc3339_now(utc: bool) -> String {
    let dt: DateTime<FixedOffset> = if utc {
        Utc::now().fixed_offset()
    } else {
        Local::now().fixed_offset()
    };
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
}

fn main() {
    let args = Args::parse();

    let mut fm = BTreeMap::new();
    fm.insert(
        "type",
        serde_yaml::Value::String(
            match args.r#type {
                DocType::Reference => "Reference",
                DocType::Playbook => "Playbook",
                DocType::ADR => "ADR",
                DocType::Runbook => "Runbook",
                DocType::Spec => "Spec",
                DocType::Report => "Report",
            }
            .to_string(),
        ),
    );
    fm.insert("title", serde_yaml::Value::String(args.title));
    fm.insert("description", serde_yaml::Value::String(args.description));
    fm.insert(
        "tags",
        serde_yaml::Value::Sequence(
            args.tags
                .iter()
                .map(|t| serde_yaml::Value::String(kebab_case(t)))
                .collect(),
        ),
    );

    let mut generated = Mapping::new();
    generated.insert(
        serde_yaml::Value::String("by".into()),
        serde_yaml::Value::String(args.by),
    );
    generated.insert(
        serde_yaml::Value::String("at".into()),
        serde_yaml::Value::String(rfc3339_now(args.utc)),
    );
    fm.insert("generated", serde_yaml::Value::Mapping(generated));

    if let Some(resource) = args.resource {
        fm.insert("resource", serde_yaml::Value::String(resource));
    }

    if let Some(status) = args.status {
        let status_str = match status {
            Status::Draft => "draft",
            Status::Review => "review",
            Status::Approved => "approved",
            Status::Deprecated => "deprecated",
            Status::Archived => "archived",
        };
        fm.insert("status", serde_yaml::Value::String(status_str.to_string()));
    }

    if let Some(verified_by) = args.verified_by {
        let mut verified = Mapping::new();
        verified.insert(
            serde_yaml::Value::String("by".into()),
            serde_yaml::Value::String(verified_by),
        );
        verified.insert(
            serde_yaml::Value::String("at".into()),
            serde_yaml::Value::String(rfc3339_now(args.utc)),
        );
        fm.insert("verified", serde_yaml::Value::Mapping(verified));
    }

    if let Some(days) = args.stale_after_days {
        let dt: DateTime<FixedOffset> = if args.utc {
            (Utc::now() + chrono::Duration::days(days)).fixed_offset()
        } else {
            (Local::now() + chrono::Duration::days(days)).fixed_offset()
        };
        fm.insert(
            "stale_after",
            serde_yaml::Value::String(dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)),
        );
    }

    println!("---");
    println!("{}", serde_yaml::to_string(&fm).unwrap().trim_end());
    println!("---");
}
