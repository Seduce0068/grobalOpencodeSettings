//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! chrono = { version = "0.4", features = ["serde"] }
//! regex = "1.10"
//! clap = { version = "4.5", features = ["derive"] }
//! ```

use chrono::{DateTime, Local};
use clap::Parser;
use regex::Regex;
use serde_yaml::{Mapping, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "okf_convert", about = "Convert legacy frontmatter to OKF")]
struct Args {
    input: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long)]
    dry_run: bool,
}

fn parse_legacy_timestamp(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    let re_rfc3339_offset =
        Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$").unwrap();
    if re_rfc3339_offset.is_match(value) {
        return Some(value.to_string());
    }

    let re_rfc3339_z = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap();
    if re_rfc3339_z.is_match(value) {
        return Some(value.replace('Z', "+00:00"));
    }

    let re_date_only = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if re_date_only.is_match(value) {
        return Some(format!("{}T00:00:00+00:00", value));
    }

    let re_space = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();
    if re_space.is_match(value) {
        return Some(value.replace(' ', "T") + "+00:00");
    }

    let formats = [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y-%m-%d",
    ];

    for fmt in formats {
        if let Ok(dt) =
            DateTime::parse_from_str(&format!("{} +0000", value), &format!("{} %z", fmt))
        {
            return Some(dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false));
        }
        if let Ok(dt) = DateTime::parse_from_str(value, &format!("{} %z", fmt)) {
            return Some(dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false));
        }
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(value, fmt) {
            return Some(
                dt.and_utc()
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            );
        }
    }

    None
}

fn convert_tags(value: &Value) -> Vec<String> {
    let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    let tags: Vec<String> = match value {
        Value::String(s) => s.split(',').map(|t| t.trim().to_string()).collect(),
        Value::Sequence(seq) => seq
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => return vec![],
    };

    tags.into_iter()
        .filter(|t| !t.is_empty())
        .map(|t| {
            let s = t.to_lowercase();
            let s = Regex::new(r"[_\s]+").unwrap().replace_all(&s, "-");
            let s = Regex::new(r"[^a-z0-9-]").unwrap().replace_all(&s, "");
            let s = Regex::new(r"-+").unwrap().replace_all(&s, "-");
            s.trim_matches('-').to_string()
        })
        .filter(|t| re.is_match(t))
        .collect()
}

fn normalize_type(value: &str) -> String {
    let map = [
        ("reference", "Reference"),
        ("doc", "Reference"),
        ("documentation", "Reference"),
        ("playbook", "Playbook"),
        ("runbook", "Runbook"),
        ("adr", "ADR"),
        ("architecture-decision", "ADR"),
        ("spec", "Spec"),
        ("specification", "Spec"),
        ("report", "Report"),
        ("analysis", "Report"),
    ];
    map.iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(value))
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "Reference".to_string())
}

fn normalize_status(value: &str) -> String {
    let map = [
        ("wip", "draft"),
        ("in-progress", "draft"),
        ("pending", "review"),
        ("ready", "review"),
        ("accepted", "approved"),
        ("merged", "approved"),
        ("published", "approved"),
        ("obsolete", "deprecated"),
        ("deleted", "archived"),
    ];
    map.iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(value))
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "draft".to_string())
}

fn get_first_string(mapping: &Mapping, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(v) = mapping
            .get(&Value::String(key.to_string()))
            .and_then(|v| v.as_str())
        {
            return Some(v.to_string());
        }
    }
    None
}

fn convert_frontmatter(legacy: &Value) -> Value {
    let mapping = legacy.as_mapping().cloned().unwrap_or_default();
    let mut okf = HashMap::new();

    // type
    if let Some(t) = get_first_string(&mapping, &["type", "kind"]) {
        okf.insert("type".to_string(), normalize_type(&t).into());
    } else {
        okf.insert("type".to_string(), "Reference".into());
    }

    // title
    if let Some(t) = get_first_string(&mapping, &["title", "name"]) {
        okf.insert("title".to_string(), t.into());
    } else {
        okf.insert("title".to_string(), "Untitled".into());
    }

    // description
    if let Some(d) = get_first_string(&mapping, &["description", "summary"]) {
        okf.insert("description".to_string(), d.into());
    } else {
        okf.insert("description".to_string(), "No description".into());
    }

    // tags
    let tags_val = mapping
        .get(&Value::String("tags".to_string()))
        .or_else(|| mapping.get(&Value::String("keywords".to_string())))
        .or_else(|| mapping.get(&Value::String("labels".to_string())));
    if let Some(tv) = tags_val {
        okf.insert(
            "tags".to_string(),
            Value::Sequence(convert_tags(tv).into_iter().map(|s| s.into()).collect()),
        );
    } else {
        okf.insert("tags".to_string(), Value::Sequence(vec![]));
    }

    // resource
    if let Some(r) = get_first_string(&mapping, &["resource", "url", "link"]) {
        okf.insert("resource".to_string(), r.into());
    }

    // generated
    let generated_at = get_first_string(
        &mapping,
        &["date", "created", "created_at", "timestamp", "generated_at"],
    )
    .and_then(|v| parse_legacy_timestamp(&v))
    .unwrap_or_else(|| {
        Local::now()
            .fixed_offset()
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    });

    let generated_by = get_first_string(&mapping, &["author", "generated_by"])
        .unwrap_or_else(|| "okf-converter".to_string());

    let mut gen = Mapping::new();
    gen.insert("by".into(), generated_by.into());
    gen.insert("at".into(), generated_at.into());
    okf.insert("generated".to_string(), Value::Mapping(gen));

    // verified
    if let Some(verified_at) = get_first_string(
        &mapping,
        &[
            "verified",
            "verified_at",
            "reviewed",
            "reviewed_at",
            "updated",
            "updated_at",
        ],
    )
    .and_then(|v| parse_legacy_timestamp(&v))
    {
        let verified_by = get_first_string(&mapping, &["verified_by", "reviewer"])
            .unwrap_or_else(|| "migrated".to_string());
        let mut ver = Mapping::new();
        ver.insert("by".into(), verified_by.into());
        ver.insert("at".into(), verified_at.into());
        okf.insert("verified".to_string(), Value::Mapping(ver));
    }

    // status
    if let Some(s) = get_first_string(&mapping, &["status", "state"]) {
        okf.insert("status".to_string(), normalize_status(&s).into());
    }

    // stale_after
    if let Some(stale) = get_first_string(
        &mapping,
        &["stale_after", "expires", "expires_at", "valid_until"],
    )
    .and_then(|v| parse_legacy_timestamp(&v))
    {
        okf.insert("stale_after".to_string(), stale.into());
    }

    // Convert to serde_yaml::Mapping preserving order
    let mut ordered = Mapping::new();
    let order = [
        "type",
        "title",
        "description",
        "tags",
        "resource",
        "generated",
        "verified",
        "status",
        "stale_after",
    ];
    for key in order {
        if let Some(val) = okf.remove(key) {
            ordered.insert(Value::String(key.to_string()), val);
        }
    }
    // Add any remaining keys
    for (k, v) in okf {
        ordered.insert(Value::String(k), v);
    }
    Value::Mapping(ordered)
}

fn main() {
    let args = Args::parse();
    let input_path = Path::new(&args.input);

    if !input_path.exists() {
        eprintln!("❌ File not found: {}", input_path.display());
        std::process::exit(1);
    }

    let content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("❌ Failed to read file");
        std::process::exit(1);
    });

    let (fm, body) = if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let fm = serde_yaml::from_str(parts[1]).unwrap_or(Value::Mapping(Mapping::new()));
            (fm, parts[2])
        } else {
            (Value::Mapping(Mapping::new()), content.as_str())
        }
    } else {
        (Value::Mapping(Mapping::new()), content.as_str())
    };

    let okf_fm = convert_frontmatter(&fm);
    let mut yaml_str = serde_yaml::to_string(&okf_fm).unwrap();
    if !yaml_str.ends_with('\n') {
        yaml_str.push('\n');
    }
    let output = format!("---\n{}---\n\n{}", yaml_str, body.trim_start());

    if args.dry_run {
        println!("=== Converted frontmatter ===");
        println!("{}", serde_yaml::to_string(&okf_fm).unwrap());
        println!("=== Body (first 200 chars) ===");
        let preview = &body[..body.len().min(200)];
        println!("{}{}", preview, if body.len() > 200 { "..." } else { "" });
    } else if let Some(output_path) = &args.output {
        fs::write(output_path, output).unwrap();
        println!("✅ Written to: {}", output_path);
    } else {
        print!("{}", output);
    }
}
