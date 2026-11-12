//! Allure 2 Formal Verification & Metrology Report Emitter
//!
//! Emits standardized Allure 2 JSON test artifacts into `audit/allure-results/`
//! for every Compliance Rule, Dip-Test Canary, Opportunity Signal, and Discovery Limitation.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllureStatus {
    Passed,
    Failed,
    Broken,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllureLabel {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllureParameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllureTestResult {
    pub uuid: String,
    pub history_id: String,
    pub full_name: String,
    pub name: String,
    pub status: AllureStatus,
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub start: u64,
    pub stop: u64,
    pub labels: Vec<AllureLabel>,
    pub parameters: Vec<AllureParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AllureSuiteSummary {
    pub total_test_results: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub output_directory: PathBuf,
}

impl AllureTestResult {
    pub fn new(
        name: impl Into<String>,
        full_name: impl Into<String>,
        status: AllureStatus,
    ) -> Self {
        let name_str = name.into();
        let full_name_str = full_name.into();
        let hash = crate::meta::compute_sha_hash(&format!("{full_name_str}-{name_str}"));
        let uuid = format!("ais-{hash}");
        let history_id = uuid.clone();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(1771977600000u64);

        Self {
            uuid,
            history_id,
            full_name: full_name_str,
            name: name_str,
            status,
            stage: "finished".to_string(),
            description: None,
            start: now,
            stop: now + 50,
            labels: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn with_label(mut self, name: &str, value: &str) -> Self {
        self.labels.push(AllureLabel {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.parameters.push(AllureParameter {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}

/// Emits an individual Allure 2 JSON result file
pub fn emit_allure_result(out_dir: &Path, result: &AllureTestResult) -> std::io::Result<PathBuf> {
    fs::create_dir_all(out_dir)?;
    let file_path = out_dir.join(format!("{}-result.json", result.uuid));
    let json = serde_json::to_string_pretty(result)?;
    fs::write(&file_path, json)?;
    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allure_serialization() {
        let result =
            AllureTestResult::new("Test Probe", "ais.canaries.TestProbe", AllureStatus::Passed)
                .with_label("epic", "AIS Metrology")
                .with_parameter("rice_score", "120.0");
        assert_eq!(result.status, AllureStatus::Passed);
        assert_eq!(result.labels.len(), 1);
        let json = serde_json::to_string(&result).expect("json");
        assert!(json.contains("ais.canaries.TestProbe"));
    }
}
