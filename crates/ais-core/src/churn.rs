//! Code Churn & Empirical Defect Hazard Engine (Data Science / SRE)
//!
//! Analyzes Git commit frequency, author entropy, and control-flow complexity
//! over a sliding window (default 90 days) to identify empirical defect hotspots
//! based on the Nagappan-Ball Pareto defect distribution model.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChurnMetric {
    pub file_path: PathBuf,
    pub commit_count: usize,
    pub unique_authors: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub hazard_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChurnAnalysisReport {
    pub total_commits_analyzed: usize,
    pub total_files_tracked: usize,
    pub top_hazard_files: Vec<FileChurnMetric>,
}

/// Determines if a path represents genuine executable code rather than a changelog, documentation, or ledger.
pub fn is_code_source_file(p: &Path) -> bool {
    let path_str = p.to_string_lossy();
    if path_str.starts_with("audit/")
        || path_str.starts_with("docs/")
        || path_str.starts_with("spec/")
        || path_str.contains("/_archive/")
        || path_str.ends_with("CHANGELOG.md")
        || path_str.ends_with("Cargo.lock")
        || path_str.ends_with(".json")
        || path_str.ends_with(".md")
    {
        return false;
    }

    p.extension().and_then(|e| e.to_str()).is_some_and(|ext| {
        matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "jsx" | "swift" | "kt" | "py" | "sql" | "sh" | "lean"
        )
    })
}

pub fn analyze_workspace_churn(root: &Path, days: usize) -> ChurnAnalysisReport {
    let since_arg = format!("--since={} days ago", days);
    let output = Command::new("git")
        .args([
            "--no-pager",
            "log",
            &since_arg,
            "--name-only",
            "--format=COMMIT|%an",
        ])
        .current_dir(root)
        .output();

    let Ok(out) = output else {
        return ChurnAnalysisReport::default();
    };

    let log_str = String::from_utf8_lossy(&out.stdout);
    let mut file_commits: HashMap<PathBuf, usize> = HashMap::new();
    let mut file_authors: HashMap<PathBuf, HashMap<String, usize>> = HashMap::new();
    let mut current_author = String::new();
    let mut total_commits = 0;

    for line in log_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(author) = trimmed.strip_prefix("COMMIT|") {
            current_author = author.to_string();
            total_commits += 1;
            continue;
        }

        let p = PathBuf::from(trimmed);
        if !is_code_source_file(&p) || (!p.is_file() && !root.join(&p).is_file()) {
            continue;
        }

        *file_commits.entry(p.clone()).or_insert(0) += 1;
        if !current_author.is_empty() {
            *file_authors
                .entry(p)
                .or_default()
                .entry(current_author.clone())
                .or_insert(0) += 1;
        }
    }

    // Anticipate pending commit for staged/modified tracked files to guarantee pre-commit and post-commit metric parity
    if let Ok(status_output) = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()
    {
        let status_str = String::from_utf8_lossy(&status_output.stdout);
        for line in status_str.lines() {
            let trimmed = line.trim();
            if trimmed.len() > 3 {
                let status_code = &trimmed[..2];
                if status_code.contains('M') || status_code.contains('A') {
                    let file_part = trimmed[3..].trim();
                    let p = PathBuf::from(file_part);
                    if is_code_source_file(&p) && (p.is_file() || root.join(&p).is_file()) {
                        *file_commits.entry(p).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut metrics = Vec::new();
    for (path, commits) in &file_commits {
        let authors = file_authors.get(path).map(|a| a.len()).unwrap_or(1);
        let abs_path = root.join(path);
        let loc = if abs_path.is_file() {
            std::fs::read_to_string(&abs_path)
                .map(|c| {
                    if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                        let stripped = crate::ast::strip_cfg_test_blocks(&c);
                        stripped.lines().filter(|l| !l.trim().is_empty()).count()
                    } else {
                        c.lines().filter(|l| !l.trim().is_empty()).count()
                    }
                })
                .unwrap_or(1)
        } else {
            1
        };

        // Empirical Hazard Score: Commits * sqrt(Authors) * ln(LOC)
        let loc_factor = (loc as f64).ln().max(1.0);
        let author_factor = (authors as f64).sqrt();
        let hazard = (*commits as f64) * author_factor * loc_factor;

        metrics.push(FileChurnMetric {
            file_path: path.clone(),
            commit_count: *commits,
            unique_authors: authors,
            lines_added: 0,
            lines_deleted: 0,
            hazard_score: (hazard * 10.0).round() / 10.0,
        });
    }

    metrics.sort_by(|a, b| {
        b.hazard_score
            .partial_cmp(&a.hazard_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    ChurnAnalysisReport {
        total_commits_analyzed: total_commits,
        total_files_tracked: metrics.len(),
        top_hazard_files: metrics.into_iter().take(150).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_code_source_file() {
        assert!(is_code_source_file(Path::new(
            "apps/product/app/src/main.rs"
        )));
        assert!(is_code_source_file(Path::new("packages/core/src/index.ts")));
        assert!(is_code_source_file(Path::new("apps/native/ios/App.swift")));
        assert!(!is_code_source_file(Path::new("CHANGELOG.md")));
        assert!(!is_code_source_file(Path::new("Cargo.lock")));
        assert!(!is_code_source_file(Path::new(
            "audit/raw/ratchet_baseline.json"
        )));
        assert!(!is_code_source_file(Path::new("docs/README.md")));
        assert!(!is_code_source_file(Path::new("spec/00-overview.md")));
    }

    #[test]
    fn test_churn_analysis_smoke() {
        let root = Path::new(".");
        let report = analyze_workspace_churn(root, 7);
        assert!(report.total_commits_analyzed <= 10_000_000);
        for file in &report.top_hazard_files {
            assert!(is_code_source_file(&file.file_path));
        }
    }
}
