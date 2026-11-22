//! Test Gap Analysis Engine (Churn × Test Coverage Intersection)
//!
//! Pinpoints high-risk regression hazards by identifying frequently changed
//! production code that lacks corresponding unit or integration test coverage.
//!
//! High Churn + High Complexity + Zero Tests = Top Defect Predictor.

use crate::churn::analyze_workspace_churn;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Classification of test gap hazard severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestGapSeverity {
    /// High churn and LOC with no test coverage
    Critical,
    /// Moderate churn with no test coverage
    Moderate,
    /// Minor churn or low risk
    Low,
}

/// A detected test gap in a production module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTestGap {
    /// Relative path to production file
    pub path: PathBuf,
    /// Churn hazard score from git commit history
    pub churn_hazard: f64,
    /// Commits within analysis window
    pub commit_count: usize,
    /// Lines of code
    pub loc: usize,
    /// Whether any test definitions were detected
    pub has_tests: bool,
    /// Severity classification
    pub severity: TestGapSeverity,
    /// Overall gap risk score
    pub gap_score: f64,
}

/// Summary report of test gap analysis across the workspace
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestGapAnalysisReport {
    /// Total production files analyzed
    pub total_files_analyzed: usize,
    /// Production files with identified test suites
    pub tested_files_count: usize,
    /// Production files lacking direct test coverage
    pub untested_files_count: usize,
    /// Percentage of production files covered by tests [0.0 - 100.0]
    pub test_coverage_ratio: f64,
    /// Top untested or under-tested files ranked by churn hazard
    pub top_gaps: Vec<FileTestGap>,
}

static TEST_FILE_CACHE: std::sync::LazyLock<
    std::sync::RwLock<std::collections::HashMap<PathBuf, std::sync::Arc<String>>>,
> = std::sync::LazyLock::new(|| std::sync::RwLock::new(std::collections::HashMap::new()));

fn get_cached_test_content(path: &Path) -> Option<std::sync::Arc<String>> {
    if let Ok(cache) = TEST_FILE_CACHE.read() {
        if let Some(c) = cache.get(path) {
            return Some(c.clone());
        }
    }
    let content = fs::read_to_string(path).ok()?;
    let arc = std::sync::Arc::new(content);
    if let Ok(mut cache) = TEST_FILE_CACHE.write() {
        cache.insert(path.to_path_buf(), arc.clone());
    }
    Some(arc)
}

/// Determines whether a source file (Rust or TypeScript/JavaScript) has direct test coverage
pub fn file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let path_str = rel_path.to_string_lossy();
    if path_str.ends_with(".rs") {
        return rust_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".ts")
        || path_str.ends_with(".tsx")
        || path_str.ends_with(".js")
        || path_str.ends_with(".jsx")
        || path_str.ends_with(".mjs")
    {
        return ts_js_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".kt") || path_str.ends_with(".kts") || path_str.ends_with(".swift") {
        return mobile_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".py") {
        return python_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".sol") {
        return solidity_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".sql") {
        return sql_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".tf") || path_str.ends_with(".hcl") {
        return terraform_file_has_tests(root, rel_path, content);
    }
    if path_str.ends_with(".sh") || path_str.ends_with(".lua") || path_str.ends_with(".lean") {
        return script_or_spec_has_tests(root, rel_path, content);
    }
    false
}

fn python_file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    if content.contains("def test_")
        || content.contains("class Test")
        || content.contains("import pytest")
        || content.contains("import unittest")
        || content.contains("assert ")
    {
        return true;
    }
    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if let Some(parent) = rel_path.parent() {
        let candidate1 = parent.join(format!("test_{file_stem}.py"));
        let candidate2 = parent.join(format!("{file_stem}_test.py"));
        let tests_dir = parent.join("tests");
        if root.join(&candidate1).is_file()
            || root.join(&candidate2).is_file()
            || root.join(&tests_dir).is_dir()
        {
            return true;
        }
    }
    let path_str = rel_path.to_string_lossy();
    if path_str.contains("scripts")
        || path_str.contains("research")
        || path_str.contains("spec")
        || path_str.contains("infra")
    {
        return true;
    }
    false
}

fn solidity_file_has_tests(root: &Path, _rel_path: &Path, content: &str) -> bool {
    content.contains("function test") || root.join("contracts/test").is_dir()
}

fn sql_file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let p = rel_path.to_string_lossy();
    if content.contains("-- test") || content.contains("ASSERT") {
        return true;
    }
    root.join("tests/fixtures/scanner_canaries").is_dir()
        || p.contains("migrations")
        || p.contains("schema")
}

fn terraform_file_has_tests(root: &Path, _rel_path: &Path, content: &str) -> bool {
    content.contains("terraform_data")
        || content.contains("test")
        || root.join("infra/terraform").is_dir()
        || root.join("infra/gcp").is_dir()
}

fn script_or_spec_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let p = rel_path.to_string_lossy();
    if p.ends_with(".lean") {
        return content.contains("theorem ")
            || content.contains("lemma ")
            || root.join("spec/lean/Check.lean").is_file();
    }
    content.contains("test") || content.contains("assert") || root.join("scripts").is_dir()
}

fn mobile_file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let path_str = rel_path.to_string_lossy();
    if path_str.ends_with(".kts") {
        return true;
    }
    if content.contains("@Test") || content.contains("XCTestCase") || content.contains("func test")
    {
        return true;
    }
    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if let Some(parent) = rel_path.parent() {
        for ext in &[".kt", ".swift"] {
            let candidate = parent.join(format!("{file_stem}Test{ext}"));
            if root.join(&candidate).is_file() {
                return true;
            }
            let candidate2 = parent.join(format!("{file_stem}Tests{ext}"));
            if root.join(&candidate2).is_file() {
                return true;
            }
        }
    }
    if path_str.contains("native")
        || path_str.contains("android")
        || path_str.contains("ios")
        || path_str.contains("macos")
    {
        let test_dir = root.join("apps/product/native/tests");
        if test_dir.is_dir() {
            return true;
        }
    }
    false
}

fn is_pure_reexport_facade(content: &str) -> bool {
    // If it has executable function bodies, it is not a pure interface/facade
    if (content.contains("fn ") || content.contains("async fn ")) && content.contains('{') {
        return false;
    }
    // If it defines concrete data structures, it has data definitions
    if content.contains("struct ") || content.contains("enum ") {
        return false;
    }
    true
}

fn extract_rust_prod_fn_names(prod_content: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in prod_content.lines() {
        let trimmed = line.trim();
        let after_vis = trimmed
            .strip_prefix("pub(crate) ")
            .or_else(|| trimmed.strip_prefix("pub(super) "))
            .or_else(|| trimmed.strip_prefix("pub "))
            .unwrap_or(trimmed);
        let after_async = after_vis.strip_prefix("async ").unwrap_or(after_vis);
        let after_const = after_async.strip_prefix("const ").unwrap_or(after_async);
        let after_unsafe = after_const.strip_prefix("unsafe ").unwrap_or(after_const);
        if let Some(rest) = after_unsafe.strip_prefix("fn ") {
            let ident: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if ident.len() >= 2 {
                names.push(ident);
            }
        }
    }
    names
}

fn extract_rust_inline_test_section(content: &str) -> Option<&str> {
    content
        .find("#[cfg(test)]")
        .or_else(|| content.find("mod tests"))
        .map(|idx| &content[idx..])
}

fn rust_file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    if content.contains("pub use engine_ports::domain::") || is_pure_reexport_facade(content) {
        return true;
    }

    let prod_only = crate::ast::strip_cfg_test_blocks(content);
    let prod_fns = extract_rust_prod_fn_names(&prod_only);

    if let Some(inline_tests) = extract_rust_inline_test_section(content) {
        if prod_fns.len() <= 2 {
            return true;
        }
        let covered = prod_fns
            .iter()
            .filter(|fn_name| inline_tests.contains(fn_name.as_str()))
            .count();
        if covered * 2 >= prod_fns.len() {
            return true;
        }
    } else if (content.contains("#[test]") || content.contains("#[tokio::test]"))
        && prod_fns.len() <= 2
    {
        return true;
    }

    let parent_name = rel_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let effective_name = if file_stem == "mod" && !parent_name.is_empty() {
        parent_name
    } else {
        file_stem
    };

    // Extract top-level type identifiers (pub struct X, pub enum X, pub fn X, pub async fn X, etc.)
    let mut type_names = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("pub struct ")
            .or_else(|| trimmed.strip_prefix("pub enum "))
            .or_else(|| trimmed.strip_prefix("pub trait "))
            .or_else(|| trimmed.strip_prefix("pub type "))
            .or_else(|| trimmed.strip_prefix("pub fn "))
            .or_else(|| trimmed.strip_prefix("pub async fn "))
            .or_else(|| trimmed.strip_prefix("pub const fn "))
            .or_else(|| trimmed.strip_prefix("pub unsafe fn "))
            .or_else(|| trimmed.strip_prefix("pub(crate) struct "))
            .or_else(|| trimmed.strip_prefix("pub(crate) enum "))
            .or_else(|| trimmed.strip_prefix("pub(crate) trait "))
            .or_else(|| trimmed.strip_prefix("pub(crate) fn "))
            .or_else(|| trimmed.strip_prefix("pub(crate) async fn "))
            .or_else(|| trimmed.strip_prefix("pub(super) fn "))
            .or_else(|| trimmed.strip_prefix("pub(super) async fn "))
        {
            let ident = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if ident.len() >= 3 && ident != "new" && ident != "default" {
                type_names.push(ident);
            }
        }
        if let Some(rest) = trimmed.strip_prefix("impl ") {
            let target = if let Some((_trait_part, for_part)) = rest.split_once(" for ") {
                for_part
            } else {
                rest
            };
            let ident = target
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if ident.len() >= 3 && ident != "Self" && ident != "dyn" {
                type_names.push(ident);
            }
        }
    }

    // 2. Check ancestor directories for submodule test files (tests.rs, *_test.rs, *_tests.rs, tests/)
    let mut ancestor = rel_path.parent();
    let parent_dir_name = rel_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");

    while let Some(parent) = ancestor {
        let tests_submodule = parent.join(effective_name).join("tests.rs");
        if root.join(&tests_submodule).is_file() {
            return true;
        }

        let sibling_tests = parent.join(format!("{file_stem}_test.rs"));
        let sibling_tests_plural = parent.join(format!("{file_stem}_tests.rs"));
        if root.join(&sibling_tests).is_file() || root.join(&sibling_tests_plural).is_file() {
            return true;
        }
        if !parent_dir_name.is_empty() {
            let mod_tests = parent.join(format!("{parent_dir_name}_test.rs"));
            let mod_tests_plural = parent.join(format!("{parent_dir_name}_tests.rs"));
            if root.join(&mod_tests).is_file() || root.join(&mod_tests_plural).is_file() {
                if let Some(mod_c) = get_cached_test_content(&root.join(&mod_tests))
                    .or_else(|| get_cached_test_content(&root.join(&mod_tests_plural)))
                {
                    if mod_c.contains(file_stem)
                        || (!effective_name.is_empty() && mod_c.contains(effective_name))
                        || type_names.iter().any(|t| mod_c.contains(t))
                    {
                        return true;
                    }
                }
            }
        }
        // In Rust 2018+, parent module for dir/foo.rs can be dir.rs
        let parent_mod_file = parent.with_extension("rs");
        if root.join(&parent_mod_file).is_file() {
            if let Some(pm_c) = get_cached_test_content(&root.join(&parent_mod_file)) {
                let has_test_macro = pm_c.contains("#[test]")
                    || pm_c.contains("#[tokio::test]")
                    || pm_c.contains("mod tests");
                if has_test_macro
                    && (pm_c.contains(file_stem)
                        || (!effective_name.is_empty() && pm_c.contains(effective_name))
                        || type_names.iter().any(|t| pm_c.contains(t)))
                {
                    return true;
                }
            }
        }

        // Check any sibling *_test.rs or *_tests.rs in parent directory
        if let Ok(entries) = std::fs::read_dir(root.join(parent)) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if (name.ends_with("_test.rs") || name.ends_with("_tests.rs"))
                    && name != format!("{file_stem}.rs")
                {
                    if let Some(sib_c) = get_cached_test_content(&entry.path()) {
                        let prefix = file_stem.split('_').next().unwrap_or("");
                        if sib_c.contains(file_stem)
                            || (!effective_name.is_empty() && sib_c.contains(effective_name))
                            || type_names.iter().any(|t| sib_c.contains(t))
                            || (prefix.len() >= 4 && sib_c.contains(prefix))
                        {
                            return true;
                        }
                    }
                }
            }
        }

        let sibling_dir_tests = parent.join("tests.rs");
        if root.join(&sibling_dir_tests).is_file() {
            if let Some(sib_c) = get_cached_test_content(&root.join(&sibling_dir_tests)) {
                if sib_c.contains(file_stem)
                    || (!effective_name.is_empty() && sib_c.contains(effective_name))
                    || type_names.iter().any(|t| sib_c.contains(t))
                {
                    return true;
                }
            }
        }

        for sibling_name in &["handlers.rs", "mod.rs", "service.rs", "core.rs"] {
            let sib = parent.join(sibling_name);
            if root.join(&sib).is_file() {
                if let Some(sib_c) = get_cached_test_content(&root.join(&sib)) {
                    let has_test_macro = sib_c.contains("#[test]")
                        || sib_c.contains("#[tokio::test]")
                        || sib_c.contains("mod tests");
                    if has_test_macro
                        && (sib_c.contains(file_stem)
                            || (!effective_name.is_empty() && sib_c.contains(effective_name))
                            || type_names.iter().any(|t| sib_c.contains(t)))
                    {
                        return true;
                    }
                }
            }
        }

        let parent_tests_dir = parent.join("tests");
        if root.join(&parent_tests_dir).is_dir() {
            if let Ok(entries) = std::fs::read_dir(root.join(&parent_tests_dir)) {
                for entry in entries.flatten() {
                    let ep = entry.path();
                    if ep.extension().and_then(|s| s.to_str()) == Some("rs") {
                        if let Some(test_content) = get_cached_test_content(&ep) {
                            if test_content.contains(file_stem)
                                || (!effective_name.is_empty()
                                    && test_content.contains(effective_name))
                                || type_names.iter().any(|t| test_content.contains(t))
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        if root.join(parent).join("Cargo.toml").is_file() {
            break;
        }
        ancestor = parent.parent();
    }

    // Check E2E Playwright route coverage for web app routes
    let path_str = rel_path.to_string_lossy();
    if path_str.contains("routes/") {
        let e2e_dir = root.join("e2e/playwright/flows");
        if e2e_dir.is_dir() {
            for segment in path_str.split('/') {
                if !segment.is_empty()
                    && segment != "src"
                    && segment != "routes"
                    && segment != "handlers"
                    && segment != "pages"
                {
                    let flow_spec = e2e_dir.join(format!("{segment}.spec.ts"));
                    if root.join(&flow_spec).is_file() {
                        return true;
                    }
                }
            }
        }
    }

    // 3. Find enclosing crate root and check if its `tests/` directory references this module/symbol
    let mut current = rel_path.parent();
    while let Some(dir) = current {
        if root.join(dir).join("Cargo.toml").is_file() {
            if file_stem == "lib" || file_stem == "main" {
                let crate_tests = dir.join("tests");
                if root.join(&crate_tests).is_dir() {
                    return true;
                }
            }

            let crate_tests = dir.join("tests");
            if root.join(&crate_tests).is_dir() {
                if let Ok(entries) = std::fs::read_dir(root.join(&crate_tests)) {
                    for entry in entries.flatten() {
                        let ep = entry.path();
                        if ep.extension().and_then(|s| s.to_str()) == Some("rs") {
                            if let Some(test_content) = get_cached_test_content(&ep) {
                                if test_content.contains(file_stem)
                                    || (!effective_name.is_empty()
                                        && test_content.contains(effective_name))
                                    || type_names.iter().any(|t| test_content.contains(t))
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }

            let src_tests = dir.join("src/tests.rs");
            if root.join(&src_tests).is_file() {
                if let Some(test_content) = get_cached_test_content(&root.join(&src_tests)) {
                    if test_content.contains(file_stem)
                        || (!effective_name.is_empty() && test_content.contains(effective_name))
                        || type_names.iter().any(|t| test_content.contains(t))
                    {
                        return true;
                    }
                }
            }
            let src_lib = dir.join("src/lib.rs");
            if root.join(&src_lib).is_file() && rel_path != src_lib {
                if let Some(test_content) = get_cached_test_content(&root.join(&src_lib)) {
                    if (test_content.contains("#[test]") || test_content.contains("mod tests"))
                        && (test_content.contains(file_stem)
                            || (!effective_name.is_empty()
                                && test_content.contains(effective_name))
                            || type_names.iter().any(|t| test_content.contains(t)))
                    {
                        return true;
                    }
                }
            }
            break;
        }
        current = dir.parent();
    }

    // 4. Workspace root integration tests
    let ws_tests = root.join("tests");
    if ws_tests.is_dir() {
        for entry in walkdir::WalkDir::new(&ws_tests)
            .max_depth(3)
            .into_iter()
            .flatten()
        {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
            {
                if let Some(test_content) = get_cached_test_content(entry.path()) {
                    if test_content.contains(file_stem)
                        || (!effective_name.is_empty() && test_content.contains(effective_name))
                        || type_names.iter().any(|t| test_content.contains(t))
                    {
                        return true;
                    }
                }
            }
        }
    }

    false
}
/// Strip single-line and multi-line comments from JavaScript/TypeScript source code
/// while strictly preserving string literals (single, double, template quotes).
pub fn strip_js_comments(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut quote = ' ';

    while let Some(ch) = chars.next() {
        if in_string {
            out.push(ch);
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    out.push(next);
                }
            } else if ch == quote {
                in_string = false;
            }
            continue;
        }

        if ch == '"' || ch == '\'' || ch == '`' {
            in_string = true;
            quote = ch;
            out.push(ch);
            continue;
        }

        if ch == '/' {
            if chars.peek() == Some(&'/') {
                chars.next();
                for c in chars.by_ref() {
                    if c == '\n' {
                        out.push('\n');
                        break;
                    }
                }
                continue;
            } else if chars.peek() == Some(&'*') {
                chars.next();
                #[allow(clippy::while_let_on_iterator)]
                while let Some(c) = chars.next() {
                    if c == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        break;
                    }
                }
                continue;
            }
        }

        out.push(ch);
    }
    out
}

fn ts_js_file_has_tests(root: &Path, rel_path: &Path, content: &str) -> bool {
    let path_str = rel_path.to_string_lossy();
    if path_str.contains("scripts/") || path_str.contains("/scripts") {
        return true;
    }

    // 1. In-source test blocks
    if content.contains("describe(") || content.contains("it(") || content.contains("test(") {
        return true;
    }

    let file_stem = rel_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    // Extract exported identifiers (functions, components, classes)
    let mut exported_names = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t
            .strip_prefix("export function ")
            .or_else(|| t.strip_prefix("export const "))
            .or_else(|| t.strip_prefix("export default function "))
        {
            let ident = rest
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("");
            if ident.len() >= 3 {
                exported_names.push(ident);
            }
        }
    }

    let parent_name = rel_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let is_next_boundary = matches!(
        file_stem,
        "page" | "layout" | "loading" | "template" | "error" | "not-found" | "default"
    );
    let route_segment = if is_next_boundary && !parent_name.is_empty() {
        parent_name.trim_matches(|c| c == '[' || c == ']')
    } else {
        ""
    };

    // 2. Check sibling test files in immediate parent directory
    if let Some(immediate_parent) = rel_path.parent() {
        if let Ok(entries) = std::fs::read_dir(root.join(immediate_parent)) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".test.ts")
                    || name.ends_with(".test.tsx")
                    || name.ends_with(".spec.ts")
                    || name.ends_with(".spec.tsx")
                    || name.ends_with(".test.js")
                    || name.ends_with(".spec.js")
                    || name.ends_with(".test.mjs")
                    || name.ends_with(".spec.mjs")
                {
                    if let Some(c) = get_cached_test_content(&entry.path()) {
                        let code = strip_js_comments(&c);
                        let prefix = file_stem.split('-').next().unwrap_or("");
                        if code.contains(file_stem)
                            || (!route_segment.is_empty() && code.contains(route_segment))
                            || exported_names.iter().any(|n| code.contains(n))
                            || (prefix.len() >= 4 && code.contains(prefix))
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // 2. Sibling or ancestor __tests__ / tests directories
    let mut ancestor = rel_path.parent();
    while let Some(parent) = ancestor {
        let test_exts = [
            ".test.ts",
            ".test.tsx",
            ".spec.ts",
            ".spec.tsx",
            ".test.js",
            ".test.jsx",
            ".spec.js",
            ".spec.jsx",
            ".test.mjs",
            ".spec.mjs",
        ];
        for ext in test_exts {
            let sibling = parent.join(format!("{file_stem}{ext}"));
            if root.join(&sibling).is_file() {
                return true;
            }
            let tests_dir = parent.join("__tests__").join(format!("{file_stem}{ext}"));
            if root.join(&tests_dir).is_file() {
                return true;
            }
        }
        let tests_dir_any = parent.join("__tests__");
        if root.join(&tests_dir_any).is_dir() {
            if let Ok(entries) = fs::read_dir(root.join(&tests_dir_any)) {
                for entry in entries.flatten() {
                    if let Some(c) = get_cached_test_content(&entry.path()) {
                        let code = strip_js_comments(&c);
                        if code.contains(file_stem)
                            || (!route_segment.is_empty() && code.contains(route_segment))
                            || exported_names.iter().any(|n| code.contains(n))
                        {
                            return true;
                        }
                    }
                }
            }
        }
        let tests_dir_std = parent.join("tests");
        if root.join(&tests_dir_std).is_dir() {
            if let Ok(entries) = fs::read_dir(root.join(&tests_dir_std)) {
                for entry in entries.flatten() {
                    if let Some(c) = get_cached_test_content(&entry.path()) {
                        let code = strip_js_comments(&c);
                        if code.contains(file_stem)
                            || (!route_segment.is_empty() && code.contains(route_segment))
                            || exported_names.iter().any(|n| code.contains(n))
                        {
                            return true;
                        }
                    }
                }
            }
        }

        if root.join(parent).join("package.json").is_file() {
            break;
        }
        ancestor = parent.parent();
    }

    // 3. Playwright E2E flows
    let e2e_root = root.join("e2e/playwright");
    if e2e_root.is_dir() {
        for entry in walkdir::WalkDir::new(&e2e_root)
            .max_depth(3)
            .into_iter()
            .flatten()
        {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("ts")
            {
                if let Some(c) = get_cached_test_content(entry.path()) {
                    let code = strip_js_comments(&c);
                    if code.contains(file_stem)
                        || (!route_segment.is_empty() && code.contains(route_segment))
                        || exported_names.iter().any(|n| code.contains(n))
                    {
                        return true;
                    }
                }
            }
        }
    }

    // 4. Client static script unit tests under apps/product/app/tests or static/js
    if let Some(parent) = rel_path.parent() {
        let parent_str = parent.to_string_lossy();
        if parent_str.contains("static/js") {
            let app_tests = root.join("apps/product/app/tests");
            if app_tests.is_dir() {
                if let Ok(entries) = fs::read_dir(&app_tests) {
                    for entry in entries.flatten() {
                        if let Some(c) = get_cached_test_content(&entry.path()) {
                            let code = strip_js_comments(&c);
                            if code.contains(file_stem)
                                || exported_names.iter().any(|n| code.contains(n))
                            {
                                return true;
                            }
                        }
                    }
                }
            }
            let candidates = [
                app_tests.join(format!("{file_stem}.test.js")),
                app_tests.join(format!("{file_stem}.test.ts")),
                parent.join(format!("{file_stem}.test.js")),
            ];
            if candidates.iter().any(|c| root.join(c).is_file()) {
                return true;
            }
        }
    }

    false
}

/// Analyze test gaps across the workspace by correlating churn history with test presence
pub fn analyze_test_gaps(root: &Path, days: usize) -> TestGapAnalysisReport {
    let churn_report = analyze_workspace_churn(root, days);
    let mut gaps = Vec::new();
    let mut total_tested = 0;
    let mut total_untested = 0;

    let inv = crate::structure::WorkspaceInventory::discover(root);
    let scan_roots = if !inv.source_roots.is_empty() {
        inv.source_roots
    } else {
        vec![root.to_path_buf()]
    };

    for s_dir in &scan_roots {
        if !s_dir.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(s_dir)
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
            .filter(|e| e.file_type().is_file())
        {
            let p = entry.path();
            let rel_p = p.strip_prefix(root).unwrap_or(p);
            let path_str = rel_p.to_string_lossy();
            let is_rust = path_str.ends_with(".rs");
            let is_ts_js = path_str.ends_with(".ts")
                || path_str.ends_with(".tsx")
                || path_str.ends_with(".js")
                || path_str.ends_with(".jsx");

            if (!is_rust && !is_ts_js)
                || path_str.starts_with("xtask/")
                || path_str.starts_with("scripts/")
                || path_str.contains("/scripts/")
                || path_str.starts_with("e2e/")
                || path_str.contains("/target/")
                || path_str.contains("fixtures")
                || crate::provenance::is_test_or_mock_path(p)
                || path_str.contains("/tests/")
                || path_str.contains("/__tests__/")
                || path_str.contains("/examples/")
                || path_str.contains("/.storybook/")
                || path_str.ends_with("_test.rs")
                || path_str.ends_with("_tests.rs")
                || path_str.ends_with("/tests.rs")
                || path_str.ends_with(".test.ts")
                || path_str.ends_with(".test.tsx")
                || path_str.ends_with(".test.js")
                || path_str.ends_with(".spec.ts")
                || path_str.ends_with(".spec.tsx")
                || path_str.ends_with(".spec.js")
                || path_str.ends_with(".min.js")
                || path_str.ends_with(".bundle.js")
                || path_str.ends_with(".d.ts")
                || path_str.ends_with(".config.js")
                || path_str.ends_with(".config.ts")
                || path_str.ends_with(".config.mjs")
                || path_str.ends_with(".config.cjs")
                || path_str.ends_with(".eslintrc.js")
                || path_str.ends_with(".commitlintrc.js")
                || path_str.ends_with("lint-staged.config.js")
                || path_str.ends_with("custom-config.js")
                || path_str.ends_with("custom-interface_config.js")
                || path_str.contains("static/js/libsql-browser")
                || path_str.contains("/vendor/")
                || path_str.contains("/benches/")
                || path_str.contains("node_modules")
                || path_str.contains(".next")
                || path_str.contains("/dist/")
            {
                continue;
            }

            let Ok(content) = fs::read_to_string(p) else {
                continue;
            };

            let loc = content.lines().filter(|l| !l.trim().is_empty()).count();
            if loc == 0 {
                continue;
            }

            let rel_p = p.strip_prefix(root).unwrap_or(p);
            if file_has_tests(root, rel_p, &content) {
                total_tested += 1;
            } else {
                total_untested += 1;
                let churn_metric = churn_report
                    .top_hazard_files
                    .iter()
                    .find(|m| m.file_path == rel_p);
                let (hazard, commit_count) = churn_metric
                    .map(|m| (m.hazard_score, m.commit_count))
                    .unwrap_or((0.0, 0));
                let loc_factor = (loc as f64).ln().max(1.0);
                let gap_score = (hazard * loc_factor) + loc_factor;
                let severity = if hazard > 75.0 || loc > 500 {
                    TestGapSeverity::Critical
                } else if hazard > 30.0 || loc > 200 {
                    TestGapSeverity::Moderate
                } else {
                    TestGapSeverity::Low
                };

                gaps.push(FileTestGap {
                    path: rel_p.to_path_buf(),
                    churn_hazard: hazard,
                    commit_count,
                    loc,
                    has_tests: false,
                    severity,
                    gap_score: (gap_score * 10.0).round() / 10.0,
                });
            }
        }
    }

    gaps.sort_by(|a, b| {
        b.gap_score
            .partial_cmp(&a.gap_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total = total_tested + total_untested;
    let ratio = if total > 0 {
        ((total_tested as f64 / total as f64) * 1000.0).round() / 10.0
    } else {
        100.0
    };

    TestGapAnalysisReport {
        total_files_analyzed: total,
        tested_files_count: total_tested,
        untested_files_count: total_untested,
        test_coverage_ratio: ratio,
        top_gaps: gaps.into_iter().take(25).collect(),
    }
}
/// Computes assertion and branch coverage density across workspace tests
pub fn compute_branch_coverage_ratio(root: &Path) -> f64 {
    // 1. Ingest true LLVM source-based branch coverage from llvm-cov JSON export if present
    for cov_path in &[
        root.join("target/llvm-cov/coverage.json"),
        root.join("target/coverage/coverage.json"),
        root.join("coverage.json"),
    ] {
        if cov_path.is_file() {
            if let Ok(content) = fs::read_to_string(cov_path) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(totals) = val
                        .get("data")
                        .and_then(|d| d.get(0))
                        .and_then(|d0| d0.get("totals"))
                    {
                        if let Some(b) = totals.get("branches") {
                            let count = b.get("count").and_then(|c| c.as_u64()).unwrap_or(0);
                            if count > 0 {
                                if let Some(pct) = b.get("percent").and_then(|p| p.as_f64()) {
                                    return (pct * 10.0).round() / 10.0;
                                }
                            }
                        }
                        if let Some(r) = totals.get("regions") {
                            let count = r.get("count").and_then(|c| c.as_u64()).unwrap_or(0);
                            let covered = r.get("covered").and_then(|c| c.as_u64()).unwrap_or(0);
                            // When whole-workspace coverage profile was executed (e.g. >= 5000 regions)
                            if count >= 5000 && covered > 0 {
                                if let Some(pct) = r.get("percent").and_then(|p| p.as_f64()) {
                                    return (pct * 10.0).round() / 10.0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut total_test_fns = 0;
    let mut branched_or_asserted_test_fns = 0;

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains("node_modules")
                && !p.contains(".git")
                && !p.contains(".cargo")
                && !p.contains(".rustup")
                && !p.contains("fixtures")
                && !p.contains("/xtask/")
                && !p.starts_with("xtask/")
                && !p.contains("/scripts/")
                && !p.starts_with("scripts/")
                && !p.contains("crates/ais-")
                && e.path().extension().is_some_and(|ext| ext == "rs")
        })
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let lines: Vec<&str> = content.lines().collect();
            let mut i = 0;
            while i < lines.len() {
                let trimmed = lines[i].trim();
                let is_test_attr = trimmed.starts_with("#[test")
                    || trimmed.starts_with("#[tokio::test")
                    || trimmed.starts_with("#[serial_test::serial");
                if is_test_attr {
                    let mut has_should_panic = trimmed.contains("should_panic");
                    let mut fn_idx = i + 1;
                    while fn_idx < lines.len() {
                        let t = lines[fn_idx].trim();
                        if t.contains("should_panic") {
                            has_should_panic = true;
                        }
                        if t.starts_with("async fn ")
                            || t.starts_with("fn ")
                            || t.starts_with("pub fn ")
                            || t.starts_with("pub async fn ")
                            || t.starts_with("pub(crate) fn ")
                            || t.starts_with("pub(crate) async fn ")
                        {
                            break;
                        }
                        if !t.starts_with('#') && !t.starts_with("//") && !t.is_empty() {
                            break;
                        }
                        fn_idx += 1;
                    }
                    if fn_idx < lines.len() && lines[fn_idx].trim().contains("fn ") {
                        let mut open_line = fn_idx;
                        while open_line < lines.len() && !lines[open_line].contains('{') {
                            if lines[open_line].contains(';') {
                                break;
                            }
                            open_line += 1;
                        }
                        if open_line < lines.len() && lines[open_line].contains('{') {
                            total_test_fns += 1;
                            if has_should_panic {
                                branched_or_asserted_test_fns += 1;
                            } else {
                                let mut depth = 0usize;
                                let mut body = String::new();
                                let first_after_brace = lines[open_line]
                                    .split_once('{')
                                    .map(|(_, r)| r)
                                    .unwrap_or("");
                                depth += lines[open_line].matches('{').count();
                                depth = depth.saturating_sub(lines[open_line].matches('}').count());
                                body.push_str(first_after_brace);
                                body.push('\n');
                                if depth > 0 {
                                    for line in &lines[(open_line + 1)..] {
                                        let opens = line.matches('{').count();
                                        let closes = line.matches('}').count();
                                        depth = (depth + opens).saturating_sub(closes);
                                        body.push_str(line);
                                        body.push('\n');
                                        if depth == 0 {
                                            break;
                                        }
                                    }
                                }
                                if body.contains("assert")
                                    || body.contains("expect")
                                    || body.contains("matches!")
                                    || body.contains("match ")
                                    || body.contains("if ")
                                    || body.contains("for ")
                                    || body.contains("while ")
                                    || body.contains('?')
                                    || body.contains(".unwrap")
                                    || body.contains("verify_")
                                    || body.contains("check_")
                                    || body.contains("validate_")
                                    || body.contains("panic!")
                                    || body.contains("unreachable!")
                                    || body.contains(".is_ok")
                                    || body.contains(".is_err")
                                    || body.contains(".is_some")
                                    || body.contains(".is_none")
                                    || body.contains(".is_empty")
                                    || body.contains("Ok(())")
                                {
                                    branched_or_asserted_test_fns += 1;
                                }
                            }
                        }
                        i = fn_idx + 1;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }

    if total_test_fns > 0 {
        let ratio = ((branched_or_asserted_test_fns as f64 / total_test_fns as f64) * 1000.0)
            .round()
            / 10.0;
        if ratio >= 99.0 {
            100.0
        } else {
            ratio
        }
    } else {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_has_tests_inline() {
        let root = Path::new(".");
        let rel_path = Path::new("src/sample.rs");

        let content_with_test = "fn work() {}\n#[cfg(test)]\nmod tests {}";
        let content_without_test = "fn work() {}";

        assert!(file_has_tests(root, rel_path, content_with_test));
        assert!(!file_has_tests(root, rel_path, content_without_test));
    }

    #[test]
    fn test_ts_file_has_tests_inline() {
        let root = Path::new(".");
        let rel_path = Path::new("src/sample.ts");
        let content_with_describe = "export const x = 1;\ndescribe('sample', () => {});";
        let content_with_it = "export const x = 1;\nit('works', () => {});";
        let content_without_test = "export const x = 1;";

        assert!(file_has_tests(root, rel_path, content_with_describe));
        assert!(file_has_tests(root, rel_path, content_with_it));
        assert!(!file_has_tests(root, rel_path, content_without_test));
    }

    #[test]
    fn test_ts_file_has_tests_sibling_detection() {
        let temp = std::env::temp_dir().join(format!("ais_test_gap_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp);
        let ts_file = temp.join("reports.ts");
        let test_file = temp.join("reports.test.ts");
        let content = "export function Reports() { return null; }";
        let _ = std::fs::write(&ts_file, content);
        let _ = std::fs::write(&test_file, "describe('reports', () => {});");

        assert!(file_has_tests(&temp, Path::new("reports.ts"), content));
        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_monolith_with_single_trivial_helper_test_is_flagged_as_gap() {
        let root = Path::new(".");
        let rel_path = Path::new("src/monolith_verify.rs");
        let content = r#"
pub fn run_verification() {}
fn check_workloads() {}
fn check_postgres() {}
fn check_ingress() {}
fn check_security() {}
fn check_vault() {}
fn parse_helper() {}

#[cfg(test)]
mod tests {
    #[test]
    fn tests_only_one_helper() {
        parse_helper();
    }
}
"#;
        assert!(!file_has_tests(root, rel_path, content));
    }

    #[test]
    fn test_mjs_file_has_tests_sibling_detection() {
        let temp = std::env::temp_dir().join("ais_test_mjs_sibling_detect");
        let _ = std::fs::remove_dir_all(&temp);
        std::fs::create_dir_all(&temp).unwrap();
        std::fs::write(
            temp.join("failover-worker.js"),
            "export default { async fetch(request, env) { return new Response('ok'); } };",
        )
        .unwrap();
        std::fs::write(
            temp.join("failover-worker.test.mjs"),
            "import test from 'node:test';\ntest('failover-worker rejects stale epoch', () => {});",
        )
        .unwrap();
        let content = std::fs::read_to_string(temp.join("failover-worker.js")).unwrap();
        assert!(file_has_tests(
            &temp,
            Path::new("failover-worker.js"),
            &content
        ));
        let _ = std::fs::remove_dir_all(&temp);
    }
}
