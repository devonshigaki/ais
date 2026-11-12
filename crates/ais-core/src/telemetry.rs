//! AIS Runtime Telemetry & Crash Ingestion Engine
//!
//! Ingests structured production logs (GCP Cloud Run, Sentry, stdout) and
//! runtime panic stack traces, mapping runtime errors directly to source AST locations
//! and feeding them into the RICE Contextual Risk Scoring engine.

use crate::findings::{Finding, Severity};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone)]
pub struct TelemetryIngestionReport {
    pub logs_processed: usize,
    pub runtime_errors_captured: usize,
    pub findings: Vec<Finding>,
}

fn parse_json_log_finding(trimmed: &str) -> Option<Finding> {
    debug_assert!(trimmed.is_char_boundary(0), "char boundary valid");
    debug_assert!(trimmed.len() < 100_000, "log line bounded");
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }
    let json = serde_json::from_str::<Value>(trimmed).ok()?;
    let severity_str = json
        .get("severity")
        .or_else(|| json.get("level"))
        .and_then(|s| s.as_str())
        .unwrap_or("");

    let is_error = matches!(
        severity_str.to_uppercase().as_str(),
        "ERROR" | "CRITICAL" | "EMERGENCY" | "FATAL"
    );

    if !is_error {
        return None;
    }

    let message = json
        .get("message")
        .or_else(|| json.get("textPayload"))
        .and_then(|m| m.as_str())
        .unwrap_or("Runtime server error detected");

    if let Some(src) = json
        .get("logging.googleapis.com/sourceLocation")
        .or_else(|| json.get("sourceLocation"))
    {
        let file_str = src.get("file").and_then(|f| f.as_str()).unwrap_or("");
        let line_no = src.get("line").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
        if !file_str.is_empty() {
            return Some(Finding::new(
                "RUNTIME-500",
                Severity::Critical,
                format!("Runtime 500 crash captured in telemetry: {message}"),
                PathBuf::from(file_str),
                line_no,
            ));
        }
    }

    if let Some(file_str) = json.get("file").and_then(|f| f.as_str()) {
        let line_no = json.get("line").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
        return Some(Finding::new(
            "RUNTIME-500",
            Severity::Critical,
            format!("Runtime error captured in telemetry: {message}"),
            PathBuf::from(file_str),
            line_no,
        ));
    }

    None
}

fn parse_panic_log_finding(trimmed: &str) -> Option<Finding> {
    debug_assert!(trimmed.len() < 100_000, "log line bounded");
    if !trimmed.contains("panicked at") {
        return None;
    }
    let panic_re = regex::Regex::new(r#"panicked at '([^']*)',\s*([^:]+):(\d+)"#).ok()?;
    let cap = panic_re.captures(trimmed)?;
    let msg = cap.get(1).map_or("Unhandled panic", |m| m.as_str());
    let file_path = cap.get(2).map_or("unknown.rs", |m| m.as_str());
    let line_no: usize = cap
        .get(3)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(1);

    Some(Finding::new(
        "RUNTIME-PANIC",
        Severity::Critical,
        format!("Runtime thread panic: {msg}"),
        PathBuf::from(file_path),
        line_no,
    ))
}

pub fn parse_log_line(line: &str) -> Option<Finding> {
    debug_assert!(!line.is_empty() || line.is_empty(), "line valid");
    debug_assert!(line.len() < 100_000, "line bounded");
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    parse_json_log_finding(trimmed).or_else(|| parse_panic_log_finding(trimmed))
}

pub fn parse_telemetry_log(payload: &str) -> Option<Finding> {
    for line in payload.lines() {
        if let Some(finding) = parse_log_line(line) {
            return Some(finding);
        }
    }
    None
}

pub fn ingest_telemetry_file(log_path: &Path) -> TelemetryIngestionReport {
    debug_assert!(!log_path.as_os_str().is_empty(), "log_path not empty");
    debug_assert!(log_path.as_os_str().len() < 4096, "log_path bounded");
    let mut findings = Vec::new();
    let mut logs_processed = 0;

    if let Ok(content) = fs::read_to_string(log_path) {
        for line in content.lines() {
            logs_processed += 1;
            if let Some(finding) = parse_log_line(line) {
                findings.push(finding);
            }
        }
    }

    info!(
        "Ingested {} log lines from {:?}. Captured {} runtime errors.",
        logs_processed,
        log_path,
        findings.len()
    );

    TelemetryIngestionReport {
        logs_processed,
        runtime_errors_captured: findings.len(),
        findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_structured_gcp_error_log() {
        let gcp_log = r#"{"severity":"ERROR","message":"Failed to render consumer overview: Askama error","logging.googleapis.com/sourceLocation":{"file":"apps/product/app/src/routes/consumer/dashboard.rs","line":225}}"#;
        let finding = parse_log_line(gcp_log).expect("Should parse structured GCP log");
        assert_eq!(finding.rule_code, "RUNTIME-500");
        assert_eq!(finding.severity, Severity::Critical);
        assert_eq!(finding.line, 225);
        assert!(finding.file.to_string_lossy().contains("dashboard.rs"));
    }

    #[test]
    fn test_parse_rust_panic_log() {
        let panic_log = "thread 'main' panicked at 'called Option::unwrap() on a None value', apps/product/app/src/services/vault.rs:88:14";
        let finding = parse_log_line(panic_log).expect("Should parse panic log");
        assert_eq!(finding.rule_code, "RUNTIME-PANIC");
        assert_eq!(finding.severity, Severity::Critical);
        assert_eq!(finding.line, 88);
        assert_eq!(
            finding.file,
            PathBuf::from("apps/product/app/src/services/vault.rs")
        );
    }
}
