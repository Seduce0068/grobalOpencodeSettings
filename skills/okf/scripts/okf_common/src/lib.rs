//! OKF Common Library
//!
//! Shared types, error handling, and utilities for OKF scripts.

pub mod error;
pub mod frontmatter;
pub mod regex_util;
pub mod timestamp;
pub mod validation;

pub use error::{OkfError, OkfResult, ExitCode};
pub use frontmatter::{DocType, Status, Frontmatter, Generated, Verified};
pub use regex_util::{KEBAB_CASE_RE, RFC3339_OFFSET_RE, RFC3339_Z_RE, DATE_ONLY_RE, SPACE_TIME_RE};
pub use timestamp::{rfc3339_now, parse_legacy_timestamp};
pub use validation::{validate_rfc3339, validate_tags, validate_markdownlint, validate_heading_structure};