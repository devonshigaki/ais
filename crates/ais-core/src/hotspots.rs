//! Defect Hotspot Triangulation Engine (Churn + MI + CCN)
//!
//! Pinpoints high-risk defect hotspots by cross-referencing:
//! 1. Git churn volatility (Nagappan-Ball Pareto defect distribution)
//! 2. Low Maintainability Index (MI < 65)
//! 3. High Cyclomatic Complexity (CCN > 15)
//!
//! Formula: Hotspot Index = Hazard * (max(CCN - 10, 1) / 10) * (max(100 - MI, 1) / 10)

use crate::churn::analyze_workspace_churn;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Analyzed file hotspot risk record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHotspotRisk {
    /// Relative file path
    pub path: PathBuf,
    /// Empirical churn hazard score
    pub churn_hazard: f64,
    /// Commits within analysis window
    pub commit_count: usize,
    /// Unique authors modifying file
    pub unique_authors: usize,
    /// Cyclomatic complexity number
    pub ccn: usize,
    /// Maintainability index [0.0 - 100.0]
    pub mi: f64,
    /// Combined hotspot risk score
    pub hotspot_score: f64,
    /// Whether file is classified as a Critical Technical Debt Hotspot
    pub is_critical_hotspot: bool,
}

/// Workspace-wide hotspot analysis report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceHotspotReport {
    /// Total files evaluated
    pub total_files_evaluated: usize,
    /// Count of critical hotspots identified
    pub critical_hotspots_count: usize,
    /// Top highest risk files ranked by hotspot score
    pub top_hotspots: Vec<FileHotspotRisk>,
}

/// Analyze workspace defect hotspots across churn, MI, and CCN
pub fn analyze_workspace_hotspots(root: &Path, days: usize) -> WorkspaceHotspotReport {
    let churn_report = analyze_workspace_churn(root, days);
    let mut hotspots = Vec::new();
    let mut critical_count = 0;

    for metric in churn_report.top_hazard_files {
        let path_str = metric.file_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(Path::new(&metric.file_path))
            || path_str.starts_with("xtask/")
            || path_str.starts_with("scripts/")
            || path_str.starts_with("crates/ais-core/")
            || path_str.starts_with("crates/ais-cli/")
            || path_str.contains("ais-")
            || path_str.contains("fixtures")
            || path_str.contains(".spec.")
            || path_str.contains("/vendor/")
            || path_str.contains("static/js/vendor")
            || path_str.contains("smoldot-bundle")
            || path_str.contains("nomt-wasm")
            || path_str.contains(".min.")
            || path_str.contains(".bundle.")
        {
            continue;
        }
        let ext = Path::new(&metric.file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let is_source_code = matches!(
            ext,
            "rs" | "ts"
                | "tsx"
                | "js"
                | "mjs"
                | "jsx"
                | "py"
                | "swift"
                | "kt"
                | "kts"
                | "sol"
                | "lua"
                | "sh"
                | "tf"
                | "hcl"
                | "lean"
        );
        let path_str = metric.file_path.to_string_lossy();
        if !is_source_code
            || path_str.starts_with("xtask/")
            || path_str.starts_with("scripts/")
            || path_str.starts_with("tests/")
        {
            continue;
        }

        let abs_path = root.join(&metric.file_path);
        let Ok(content) = fs::read_to_string(&abs_path) else {
            continue;
        };

        let loc = content.lines().filter(|l| !l.trim().is_empty()).count();
        if loc == 0 {
            continue;
        }

        let (ccn, mi, _) = match crate::metrics::analyze_file_metrics(&abs_path) {
            Some(m) => m,
            None => continue,
        };

        // Hotspot Triangulation Formula (Nagappan-Ball defect distribution coupled with complexity debt)
        // Debt factors: Scaled by cyclomatic complexity excess above clean baseline (CCN <= 5.0)
        // and SEI maintainability deficiency (< 80.0). Code with low complexity and high maintainability
        // exhibits natural velocity without defect risk accumulation.
        let ccn_factor = if ccn <= 5 {
            0.20 * (ccn.max(1) as f64) / 5.0
        } else {
            0.20 + 0.80 * ((ccn - 5) as f64) / 5.0
        };
        let mi_debt_factor = if mi >= 80.0 {
            (1.0 - (mi - 80.0) / 40.0).max(0.20)
        } else {
            1.0 + (80.0 - mi) / 20.0
        };
        let hotspot_score = metric.hazard_score * ccn_factor * mi_debt_factor;
        let is_critical = metric.hazard_score > 80.0 && mi < 60.0 && ccn > 10;

        if is_critical {
            critical_count += 1;
        }

        hotspots.push(FileHotspotRisk {
            path: metric.file_path,
            churn_hazard: metric.hazard_score,
            commit_count: metric.commit_count,
            unique_authors: metric.unique_authors,
            ccn,
            mi,
            hotspot_score: (hotspot_score * 10.0).round() / 10.0,
            is_critical_hotspot: is_critical,
        });
    }

    hotspots.sort_by(|a, b| {
        b.hotspot_score
            .partial_cmp(&a.hotspot_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    WorkspaceHotspotReport {
        total_files_evaluated: hotspots.len(),
        critical_hotspots_count: critical_count,
        top_hotspots: hotspots.into_iter().take(15).collect(),
    }
}
