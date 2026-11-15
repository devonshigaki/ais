//! Complexity-Based Model Router & Provenance Attribution

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelTier {
    FastLowCost,
    BalancedPrecision,
    DeepReasoning,
}

pub fn route_model(rule: &str, ccn: usize, crs: f32) -> (&'static str, ModelTier) {
    if rule.contains("GOF")
        || rule.contains("DDD")
        || rule.contains("POT")
        || ccn > 15
        || crs >= 80.0
    {
        ("gemini-2.5-pro", ModelTier::DeepReasoning)
    } else if rule.contains("DDIA")
        || rule.contains("WIRE")
        || rule.contains("SEC")
        || ccn > 5
        || crs >= 60.0
    {
        ("gemini-2.5-flash", ModelTier::BalancedPrecision)
    } else {
        ("gemini-2.5-flash", ModelTier::FastLowCost)
    }
}

pub fn generate_attribution_watermark(rule: &str, model: &str) -> String {
    format!("// [AIS-AUTONOMOUS-HEAL: {rule} | Model: {model}]")
}

pub fn generate_attribution_watermark_for_file(rule: &str, model: &str, path: &Path) -> String {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "html" | "htm" => format!("<!-- [AIS-AUTONOMOUS-HEAL: {rule} | Model: {model}] -->"),
        "sql" => format!("-- [AIS-AUTONOMOUS-HEAL: {rule} | Model: {model}]"),
        "py" | "yaml" | "yml" | "sh" | "toml" => {
            format!("# [AIS-AUTONOMOUS-HEAL: {rule} | Model: {model}]")
        }
        _ => format!("// [AIS-AUTONOMOUS-HEAL: {rule} | Model: {model}]"),
    }
}

pub fn generate_commit_author() -> (String, String) {
    let name = std::env::var("AIS_COMMIT_AUTHOR_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "AIS Autonomous Agent".to_string());
    let email = std::env::var("AIS_COMMIT_AUTHOR_EMAIL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "ais-bot@users.noreply.github.com".to_string());
    (name, email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_aware_watermark() {
        let rs_path = Path::new("crates/core/src/lib.rs");
        let html_path = Path::new("templates/index.html");
        let sql_path = Path::new("migrations/001.sql");
        let yaml_path = Path::new("ais.yaml");

        assert_eq!(
            generate_attribution_watermark_for_file("A11Y-002", "gemini-2.5-flash", html_path),
            "<!-- [AIS-AUTONOMOUS-HEAL: A11Y-002 | Model: gemini-2.5-flash] -->"
        );
        assert_eq!(
            generate_attribution_watermark_for_file("ADAPT-003", "gemini-2.5-pro", rs_path),
            "// [AIS-AUTONOMOUS-HEAL: ADAPT-003 | Model: gemini-2.5-pro]"
        );
        assert_eq!(
            generate_attribution_watermark_for_file("MIGRATE-001", "gemini-2.5-flash", sql_path),
            "-- [AIS-AUTONOMOUS-HEAL: MIGRATE-001 | Model: gemini-2.5-flash]"
        );
        assert_eq!(
            generate_attribution_watermark_for_file("ARCH-001", "gemini-2.5-flash", yaml_path),
            "# [AIS-AUTONOMOUS-HEAL: ARCH-001 | Model: gemini-2.5-flash]"
        );
    }
}
