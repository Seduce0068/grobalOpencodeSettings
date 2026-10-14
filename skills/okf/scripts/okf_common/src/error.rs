//! Error types and exit codes for OKF scripts.

use std::fmt;
use thiserror::Error;

/// Exit codes for OKF scripts - enables automation to distinguish failure types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    Success = 0,
    /// Frontmatter validation failed (missing fields, invalid format, etc.)
    FrontmatterInvalid = 2,
    /// Markdownlint validation failed
    MarkdownlintFailed = 3,
    /// Heading structure invalid (first heading not ##, etc.)
    HeadingStructureInvalid = 4,
    /// I/O error (file not found, permission denied, etc.)
    IoError = 5,
    /// Path traversal or security violation
    SecurityViolation = 6,
    /// Internal error (bug, unexpected state)
    InternalError = 7,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> i32 {
        code as i32
    }
}

impl ExitCode {
    pub fn exit(self) -> ! {
        std::process::exit(self as i32);
    }
}

/// Main error type for OKF operations.
#[derive(Error, Debug)]
pub enum OkfError {
    #[error("Frontmatter validation failed: {0}")]
    FrontmatterInvalid(String),

    #[error("Markdownlint validation failed: {0}")]
    MarkdownlintFailed(String),

    #[error("Heading structure invalid: {0}")]
    HeadingStructureInvalid(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Path traversal detected: {0}")]
    PathTraversal(String),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Timestamp parse error: {0}")]
    TimestampParse(String),

    #[error("Invalid tag format: {0}")]
    InvalidTag(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type OkfResult<T> = Result<T, OkfError>;

impl OkfError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            OkfError::FrontmatterInvalid(_) => ExitCode::FrontmatterInvalid,
            OkfError::MarkdownlintFailed(_) => ExitCode::MarkdownlintFailed,
            OkfError::HeadingStructureInvalid(_) => ExitCode::HeadingStructureInvalid,
            OkfError::IoError(_) => ExitCode::IoError,
            OkfError::PathTraversal(_) | OkfError::SecurityViolation(_) => {
                ExitCode::SecurityViolation
            }
            _ => ExitCode::InternalError,
        }
    }
}

impl From<anyhow::Error> for OkfError {
    fn from(err: anyhow::Error) -> Self {
        OkfError::Internal(err.to_string())
    }
}
