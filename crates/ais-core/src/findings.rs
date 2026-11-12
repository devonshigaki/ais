//! Finding data structures for Autonomous Iteration System (AIS)

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Low => write!(f, "LOW"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::High => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(pub f32);

impl Default for Confidence {
    fn default() -> Self {
        Self(1.0)
    }
}

// POT-005: Confidence implementation
impl Confidence {
    pub fn new(val: f32) -> Self {
        Self(val.clamp(0.0, 1.0))
    }

    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }

    pub fn is_medium(&self) -> bool {
        self.0 >= 0.5 && self.0 < 0.8
    }

    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suppression {
    pub reason: String,
    pub suppressed_by: String,
    pub ticket: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub rule_code: String,
    pub severity: Severity,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub confidence: Confidence,
    pub suppressed: Option<Suppression>,
    pub hint: Option<String>,
}

// POT-005: Finding implementation
impl Finding {
    pub fn new(
        rule_code: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        file: PathBuf,
        line: usize,
    ) -> Self {
        Self {
            rule_code: rule_code.into(),
            severity,
            message: message.into(),
            file,
            line,
            confidence: Confidence::default(),
            suppressed: None,
            hint: None,
        }
    }

    pub fn with_confidence(mut self, confidence: Confidence) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn is_active(&self) -> bool {
        self.suppressed.is_none()
    }
}
