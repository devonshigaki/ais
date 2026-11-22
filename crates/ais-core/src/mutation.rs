//! Native AST Mutation Testing & Test Suite Efficacy Analyzer
//!
//! Evaluates whether test suites meaningfully assert program invariants by identifying
//! and scoring semantic mutation targets (comparison inversions, boolean flips, return swaps).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static BINARY_CMP_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(r#"\b([a-zA-Z0-9_]+)\s*(==|!=|<=|>=|<|>)\s*([a-zA-Z0-9_]+)\b"#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static BOOL_LITERAL_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| match regex::Regex::new(r#"\b(true|false)\b"#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    });

fn is_genuine_binary_comparison(line: &str, op: &str, left: &str, right: &str) -> bool {
    if op == "==" || op == "!=" || op == "<=" || op == ">=" {
        return true;
    }
    // For '<' and '>':
    // 1. Rust generic type definitions: struct Foo<T>, enum Bar<T>, impl<T>, trait Baz<T>, type X<T>
    if line.starts_with("struct ")
        || line.starts_with("pub struct ")
        || line.starts_with("pub(crate) struct ")
        || line.starts_with("enum ")
        || line.starts_with("pub enum ")
        || line.starts_with("pub(crate) enum ")
        || line.starts_with("impl")
        || line.starts_with("trait ")
        || line.starts_with("pub trait ")
        || line.starts_with("pub(crate) trait ")
        || line.starts_with("type ")
        || line.starts_with("pub type ")
        || line.starts_with("pub(crate) type ")
        || line.starts_with("use ")
        || line.starts_with("pub use ")
        || line.starts_with("pub(crate) use ")
    {
        return false;
    }
    // 2. Struct field or argument type annotation (e.g. "field: Vec<Type>," or "arg: Arc<T>) ->")
    if let Some((_, type_part)) = line.split_once(':') {
        if !type_part.contains('=')
            && (type_part.contains(&format!("<{right}")) || type_part.contains(&format!("{left}<")))
        {
            return false;
        }
    }
    // 3. Turbofish method call or generic type instantiations (e.g. "::<Type>", "Vec::<T>")
    if line.contains(&format!("::<{right}")) || line.contains(&format!("{left}::<")) {
        return false;
    }
    // 4. Rust PascalCase type names followed directly by '<' (e.g. "Vec<T>", "Option<T>")
    if left.chars().next().is_some_and(|c| c.is_ascii_uppercase())
        && line.contains(&format!("{left}<"))
    {
        return false;
    }
    // 5. Must have whitespace around '<' or '>' in idiomatic Rust comparisons
    line.contains(&format!(" {op} "))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationSite {
    pub file: PathBuf,
    pub line: usize,
    pub original_op: String,
    pub mutated_op: String,
    pub operator_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MutationAnalysisReport {
    pub files_analyzed: usize,
    pub total_mutation_sites: usize,
    pub comparison_mutations: usize,
    pub boolean_mutations: usize,
    pub sample_mutations: Vec<MutationSite>,
    pub mutation_efficacy_score: f64,
    pub empirical_outcomes_present: bool,
    pub is_heuristic_proxy: bool,
    pub estimation_method: String,
    pub mutants_caught: u64,
    pub mutants_missed: u64,
}

fn parse_cargo_mutants_outcomes(val: &serde_json::Value) -> Option<(u64, u64)> {
    if let (Some(c), Some(m)) = (
        val.get("caught").and_then(|v| v.as_u64()),
        val.get("missed").and_then(|v| v.as_u64()),
    ) {
        if c + m > 0 {
            return Some((c, m));
        }
    }
    if let Some(arr) = val.get("outcomes").and_then(|v| v.as_array()) {
        let mut caught = 0u64;
        let mut missed = 0u64;
        for item in arr {
            match item.get("summary").and_then(|s| s.as_str()) {
                Some("CaughtMutant") => caught += 1,
                Some("MissedMutant") => missed += 1,
                _ => {}
            }
        }
        if caught + missed > 0 {
            return Some((caught, missed));
        }
    }
    None
}

pub fn analyze_mutation_coverage(root: &Path) -> MutationAnalysisReport {
    // 1. Ingest true AST mutant outcomes from cargo-mutants if present
    for mutants_path in &[
        root.join("mutants.out/outcomes.json"),
        root.join("target/mutants/outcomes.json"),
    ] {
        if mutants_path.is_file() {
            if let Ok(content) = std::fs::read_to_string(mutants_path) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some((caught, missed)) = parse_cargo_mutants_outcomes(&val) {
                        let total = caught + missed;
                        let score = (caught as f64 / total as f64) * 100.0;
                        return MutationAnalysisReport {
                            mutation_efficacy_score: (score * 10.0).round() / 10.0,
                            empirical_outcomes_present: true,
                            is_heuristic_proxy: false,
                            estimation_method: "empirical_cargo_mutants".to_string(),
                            mutants_caught: caught,
                            mutants_missed: missed,
                            ..Default::default()
                        };
                    }
                }
            }
        }
    }

    let mut report = MutationAnalysisReport::default();
    let mut covered_mutation_sites = 0;

    let inv = crate::structure::WorkspaceInventory::discover(root);
    let scan_dirs = if !inv.source_roots.is_empty() {
        inv.source_roots
    } else {
        vec![root.to_path_buf()]
    };

    for dir in &scan_dirs {
        if !dir.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| {
                if e.file_type().is_dir()
                    && crate::structure::is_ignored_directory_name(&e.file_name().to_string_lossy())
                {
                    return false;
                }
                true
            })
            .flatten()
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().and_then(|s| s.to_str()) == Some("rs")
            })
        {
            let path_str = entry.path().to_string_lossy();
            if crate::is_test_or_mock_path(entry.path())
                || path_str.contains("/xtask/")
                || path_str.starts_with("xtask/")
                || path_str.starts_with("./xtask/")
                || path_str.contains("/scripts/")
                || path_str.starts_with("scripts/")
                || path_str.starts_with("./scripts/")
                || path_str.contains("/e2e/")
                || path_str.starts_with("e2e/")
                || path_str.starts_with("./e2e/")
                || path_str.contains("/src/bin/")
                || path_str.contains("/bin/")
                || path_str.contains("/tests/")
                || path_str.contains("/__tests__/")
                || path_str.ends_with("_test.rs")
                || path_str.ends_with("_tests.rs")
                || path_str.ends_with("/tests.rs")
                || path_str.contains("target")
                || path_str.contains("fixtures")
                || path_str.contains("/examples/")
                || path_str.contains("/benches/")
            {
                continue;
            }

            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };

            report.files_analyzed += 1;
            let rel_path = entry
                .path()
                .strip_prefix(root)
                .unwrap_or(entry.path())
                .to_path_buf();

            let has_tests = crate::test_gaps::file_has_tests(root, &rel_path, &content);
            let prod_content = crate::ast::strip_cfg_test_blocks(&content);
            let mut file_sites = 0;

            for (idx, line) in prod_content.lines().enumerate() {
                let line_num = idx + 1;
                let trimmed = line.trim();
                if trimmed.starts_with("//") {
                    continue;
                }

                if let Some(cap) = BINARY_CMP_RE.captures(trimmed) {
                    let left = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    let right = cap.get(3).map(|m| m.as_str()).unwrap_or("");
                    if let Some(op_match) = cap.get(2) {
                        let op = op_match.as_str();
                        if is_genuine_binary_comparison(trimmed, op, left, right) {
                            let mutated = match op {
                                "==" => "!=",
                                "!=" => "==",
                                "<" => ">=",
                                ">" => "<=",
                                "<=" => ">",
                                ">=" => "<",
                                _ => "!=",
                            };
                            report.total_mutation_sites += 1;
                            report.comparison_mutations += 1;
                            file_sites += 1;
                            if !has_tests && report.sample_mutations.len() < 5 {
                                report.sample_mutations.push(MutationSite {
                                    file: rel_path.clone(),
                                    line: line_num,
                                    original_op: op.to_string(),
                                    mutated_op: mutated.to_string(),
                                    operator_kind: "Comparison Inversion".to_string(),
                                });
                            }
                        }
                    }
                }

                if let Some(cap) = BOOL_LITERAL_RE.captures(trimmed) {
                    if let Some(b_match) = cap.get(1) {
                        let b = b_match.as_str();
                        let flipped = if b == "true" { "false" } else { "true" };
                        report.total_mutation_sites += 1;
                        report.boolean_mutations += 1;
                        file_sites += 1;
                        if !has_tests && report.sample_mutations.len() < 5 {
                            report.sample_mutations.push(MutationSite {
                                file: rel_path.clone(),
                                line: line_num,
                                original_op: b.to_string(),
                                mutated_op: flipped.to_string(),
                                operator_kind: "Boolean Inversion".to_string(),
                            });
                        }
                    }
                }
            }

            if has_tests {
                covered_mutation_sites += file_sites;
            }
        }
    }

    // Static heuristic site-coverage proxy (no cargo-mutants execution outcomes found)
    let branch_ratio = crate::test_gaps::compute_branch_coverage_ratio(root);
    let coverage_fraction = if report.total_mutation_sites > 0 {
        covered_mutation_sites as f64 / report.total_mutation_sites as f64
    } else {
        1.0
    };
    let score = (coverage_fraction * branch_ratio).clamp(0.0, 100.0);
    report.mutation_efficacy_score = (score * 10.0).round() / 10.0;
    report.is_heuristic_proxy = true;
    report.estimation_method = "heuristic_static_site_proxy".to_string();
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_analysis_smoke() {
        let root = Path::new(".");
        let report = analyze_mutation_coverage(root);
        assert!(report.files_analyzed > 0);
        assert!(report.total_mutation_sites > 0);
    }
    #[test]
    fn test_mutation_ignores_rust_generic_type_brackets() {
        assert!(!is_genuine_binary_comparison(
            "pub all_roles: Vec<RoleType>,",
            "<",
            "Vec",
            "RoleType"
        ));
        assert!(!is_genuine_binary_comparison(
            "pub struct RoleExtractor(pub Arc<AuthenticatedUserWithRoles>);",
            "<",
            "Arc",
            "AuthenticatedUserWithRoles"
        ));
        assert!(!is_genuine_binary_comparison(
            "impl<S> FromRequestParts<S> for RoleExtractor",
            "<",
            "impl",
            "S"
        ));
        assert!(!is_genuine_binary_comparison(
            "parts.extensions.get::<Arc<AuthenticatedUserWithRoles>>()",
            "<",
            "get",
            "Arc"
        ));
        assert!(is_genuine_binary_comparison(
            "if index < max_count {",
            "<",
            "index",
            "max_count"
        ));
        assert!(is_genuine_binary_comparison(
            "while count > 0 {",
            ">",
            "count",
            "0"
        ));
    }
}
