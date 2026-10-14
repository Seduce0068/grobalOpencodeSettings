//! Strongly-typed frontmatter structures.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocType {
    Reference,
    Playbook,
    ADR,
    Runbook,
    Spec,
    Report,
}

impl DocType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocType::Reference => "Reference",
            DocType::Playbook => "Playbook",
            DocType::ADR => "ADR",
            DocType::Runbook => "Runbook",
            DocType::Spec => "Spec",
            DocType::Report => "Report",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "reference" => Some(DocType::Reference),
            "playbook" => Some(DocType::Playbook),
            "adr" => Some(DocType::ADR),
            "runbook" => Some(DocType::Runbook),
            "spec" => Some(DocType::Spec),
            "report" => Some(DocType::Report),
            _ => None,
        }
    }
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

use std::fmt;

/// Document status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Review,
    Approved,
    Deprecated,
    Archived,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Review => "review",
            Status::Approved => "approved",
            Status::Deprecated => "deprecated",
            Status::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "draft" => Some(Status::Draft),
            "review" => Some(Status::Review),
            "approved" => Some(Status::Approved),
            "deprecated" => Some(Status::Deprecated),
            "archived" => Some(Status::Archived),
            _ => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Generated metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Generated {
    pub by: String,
    pub at: DateTime<FixedOffset>,
}

/// Verified metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verified {
    pub by: String,
    pub at: DateTime<FixedOffset>,
}

/// Complete OKF frontmatter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frontmatter {
    #[serde(rename = "type")]
    pub doc_type: DocType,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    pub generated: Generated,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<Verified>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stale_after")]
    pub stale_after: Option<DateTime<FixedOffset>>,
}

impl Frontmatter {
    /// Create a new frontmatter with required fields.
    pub fn new(
        doc_type: DocType,
        title: String,
        description: String,
        tags: Vec<String>,
        generated_by: String,
    ) -> Self {
        Self {
            doc_type,
            title,
            description,
            tags,
            resource: None,
            generated: Generated {
                by: generated_by,
                at: crate::timestamp::rfc3339_now(false),
            },
            verified: None,
            status: None,
            stale_after: None,
        }
    }

    /// Serialize to YAML string with proper formatting.
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let mut s = serde_yaml::to_string(self)?;
        // Ensure trailing newline
        if !s.ends_with('\n') {
            s.push('\n');
        }
        Ok(s)
    }
}

/// Legacy frontmatter fields for conversion.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LegacyFrontmatter {
    #[serde(alias = "kind")]
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<serde_yaml::Value>,
    pub keywords: Option<serde_yaml::Value>,
    pub labels: Option<serde_yaml::Value>,
    pub resource: Option<String>,
    pub url: Option<String>,
    pub link: Option<String>,
    pub date: Option<String>,
    pub created: Option<String>,
    pub created_at: Option<String>,
    pub timestamp: Option<String>,
    pub generated_at: Option<String>,
    pub author: Option<String>,
    pub generated_by: Option<String>,
    pub verified: Option<String>,
    pub verified_at: Option<String>,
    pub reviewed: Option<String>,
    pub reviewed_at: Option<String>,
    pub updated: Option<String>,
    pub updated_at: Option<String>,
    pub verified_by: Option<String>,
    pub reviewer: Option<String>,
    pub status: Option<String>,
    pub state: Option<String>,
    pub stale_after: Option<String>,
    pub expires: Option<String>,
    pub expires_at: Option<String>,
    pub valid_until: Option<String>,
}
