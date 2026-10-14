//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_yaml = "0.9"
//! chrono = { version = "0.4", features = ["serde"] }
//! regex = "1.10"
//! which = "4.2"
//! ```

use chrono::{DateTime, FixedOffset};
use regex::Regex;
use serde_yaml::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn validate_rfc3339(timestamp: &str) -> Result<(), String> {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$").unwrap();
    if !re.is_match(timestamp) {
        return Err(format!(
            "Invalid format: {} (expected YYYY-MM-DDTHH:MM:SS±HH:MM)",
            timestamp
        ));
    }
    timestamp
        .parse::<DateTime<FixedOffset>>()
        .map(|_| ())
        .map_err(|e| format!("Invalid datetime: {}", e))
}

fn validate_tags(tags: &[Value]) -> Vec<String> {
    let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    let mut errors = Vec::new();
    for tag in tags {
        if let Some(s) = tag.as_str() {
            if !re.is_match(s) {
                errors.push(format!("Tag '{}' is not lowercase kebab-case", s));
            }
        }
    }
    errors
}

fn validate_markdownlint(filepath: &Path) -> Result<(), String> {
    let output = Command::new("markdownlint-cli2")
        .arg(filepath)
        .output()
        .map_err(|_| {
            "markdownlint-cli2 not installed. Install: npm install -g markdownlint-cli2".to_string()
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let msg = stdout.trim().to_string();
        let msg = if msg.is_empty() {
            stderr.trim().to_string()
        } else {
            msg
        };
        Err(if msg.is_empty() {
            "markdownlint failed".to_string()
        } else {
            msg
        })
    }
}

fn validate_heading_structure(content: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_frontmatter = false;
    let mut frontmatter_ended = false;

    for (i, line) in lines.iter().enumerate() {
        if i == 0 && line.trim() == "---" {
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter {
            if line.trim() == "---" {
                frontmatter_ended = true;
                in_frontmatter = false;
            }
            continue;
        }
        if frontmatter_ended {
            if line.starts_with('#') {
                if !line.starts_with("## ") {
                    errors.push(format!(
                        "Line {}: First body heading must be '## ' (got '{}')",
                        i + 1,
                        &line[..line.len().min(20)]
                    ));
                }
                if line.starts_with("# ") {
                    errors.push(format!(
                        "Line {}: H1 (# ) not allowed in body (use frontmatter title)",
                        i + 1
                    ));
                }
                break;
            }
        }
    }
    errors
}

fn get_mapping_value<'a>(mapping: &'a serde_yaml::Mapping, key: &str) -> Option<&'a Value> {
    mapping.get(&Value::String(key.to_string()))
}

fn validate_frontmatter(fm: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    let mapping = match fm.as_mapping() {
        Some(m) => m,
        None => return vec!["Frontmatter is not a mapping".to_string()],
    };

    let required = ["type", "title", "description", "tags", "generated"];
    for field in required {
        if get_mapping_value(mapping, field).is_none() {
            errors.push(format!("Missing required field: {}", field));
        }
    }

    if let Some(gen) = get_mapping_value(mapping, "generated").and_then(|v| v.as_mapping()) {
        if get_mapping_value(gen, "by").is_none() {
            errors.push("generated.by is required".to_string());
        }
        if let Some(at) = get_mapping_value(gen, "at").and_then(|v| v.as_str()) {
            if let Err(e) = validate_rfc3339(at) {
                errors.push(format!("generated.at: {}", e));
            }
            if at.ends_with('Z') {
                errors.push(
                    "generated.at: Use explicit offset (+00:00) instead of Z suffix".to_string(),
                );
            }
        } else {
            errors.push("generated.at is required".to_string());
        }
    } else {
        errors.push("generated must be a mapping".to_string());
    }

    if let Some(ver) = get_mapping_value(mapping, "verified").and_then(|v| v.as_mapping()) {
        if let Some(at) = get_mapping_value(ver, "at").and_then(|v| v.as_str()) {
            if let Err(e) = validate_rfc3339(at) {
                errors.push(format!("verified.at: {}", e));
            }
            if at.ends_with('Z') {
                errors.push(
                    "verified.at: Use explicit offset (+00:00) instead of Z suffix".to_string(),
                );
            }
        }
    }

    if let Some(stale) = get_mapping_value(mapping, "stale_after").and_then(|v| v.as_str()) {
        if let Err(e) = validate_rfc3339(stale) {
            errors.push(format!("stale_after: {}", e));
        }
        if stale.ends_with('Z') {
            errors
                .push("stale_after: Use explicit offset (+00:00) instead of Z suffix".to_string());
        }
    }

    let valid_types = ["Reference", "Playbook", "ADR", "Runbook", "Spec", "Report"];
    if let Some(t) = get_mapping_value(mapping, "type").and_then(|v| v.as_str()) {
        if !valid_types.contains(&t) {
            errors.push(format!(
                "Invalid type: {} (valid: {})",
                t,
                valid_types.join(", ")
            ));
        }
    }

    let valid_status = ["draft", "review", "approved", "deprecated", "archived"];
    if let Some(s) = get_mapping_value(mapping, "status").and_then(|v| v.as_str()) {
        if !valid_status.contains(&s) {
            errors.push(format!(
                "Invalid status: {} (valid: {})",
                s,
                valid_status.join(", ")
            ));
        }
    }

    if let Some(tags) = get_mapping_value(mapping, "tags").and_then(|v| v.as_sequence()) {
        errors.extend(validate_tags(tags));
    }

    errors
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: okf_validate <file.md>");
        std::process::exit(1);
    }

    let filepath = Path::new(&args[1]);
    if !filepath.exists() {
        eprintln!("❌ File not found: {}", filepath.display());
        std::process::exit(1);
    }

    println!("🔍 Validating: {}", filepath.display());
    println!();

    let content = fs::read_to_string(filepath).unwrap_or_else(|_| {
        eprintln!("❌ Failed to read file");
        std::process::exit(1);
    });

    // Extract frontmatter
    let fm = if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            serde_yaml::from_str(parts[1]).unwrap_or(Value::Mapping(serde_yaml::Mapping::new()))
        } else {
            eprintln!("❌ Frontmatter: Malformed (missing closing ---)");
            std::process::exit(1);
        }
    } else {
        eprintln!("❌ Frontmatter: Missing (must start with ---)");
        std::process::exit(1);
    };

    println!("📋 Frontmatter validation:");
    let fm_errors = validate_frontmatter(&fm);
    if fm_errors.is_empty() {
        println!("  ✅ Frontmatter: Valid");
    } else {
        for err in &fm_errors {
            println!("  ❌ {}", err);
        }
    }
    println!();

    println!("📝 Markdownlint validation:");
    match validate_markdownlint(filepath) {
        Ok(_) => println!("  ✅ Markdownlint: Passed"),
        Err(e) => println!("  ❌ Markdownlint: {}", e),
    }
    println!();

    println!("📐 Heading structure:");
    let hs_errors = validate_heading_structure(&content);
    if hs_errors.is_empty() {
        println!("  ✅ Heading structure: Valid");
    } else {
        for err in &hs_errors {
            println!("  ❌ {}", err);
        }
    }
    println!();

    println!("{}", "=".repeat(40));
    let all_ok =
        fm_errors.is_empty() && validate_markdownlint(filepath).is_ok() && hs_errors.is_empty();
    if all_ok {
        println!("✅ All validations passed");
        std::process::exit(0);
    } else {
        println!("❌ Validation failed");
        std::process::exit(1);
    }
}
