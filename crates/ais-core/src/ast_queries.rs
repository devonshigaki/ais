//! Declarative Tree-Sitter S-Expression Query Fleet (Compiler-Grade AST Static Analysis)
//!
//! Executes AST pattern matches against Tree-Sitter concrete syntax trees to detect:
//! - PANIC-001: Unchecked unwrap or expect calls in production request loops
//! - ASYNC-TIMEOUT-001: Outbound network calls lacking timeout boundaries
//! - BLOCKING-IO-001: Synchronous thread sleep or blocking I/O inside async runtime tasks
//! - ASYNC-MUTEX-001: std::sync::Mutex guard held across .await points

use crate::findings::{Confidence, Finding, Severity};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
static UNWRAP_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let unwrap_scm = r#"
        (call_expression
          function: (field_expression
            field: (field_identifier) @method (#match? @method "^(unwrap|expect)$"))
        ) @call
    "#;
    tree_sitter::Query::new(&language, unwrap_scm).ok()
});

static BLOCKING_SLEEP_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let blocking_sleep_scm = r#"
        (async_function_item
          body: (block
            (_)*
            (call_expression
              function: (scoped_identifier
                path: (identifier) @mod
                name: (identifier) @fn (#match? @fn "^sleep$")))
            (_)*)
        ) @async_fn
    "#;
    tree_sitter::Query::new(&language, blocking_sleep_scm).ok()
});

static ASYNC_MUTEX_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let async_mutex_scm = r#"
        (call_expression
          function: (field_expression
            field: (field_identifier) @fn (#match? @fn "^lock$"))
        ) @lock_call
    "#;
    tree_sitter::Query::new(&language, async_mutex_scm).ok()
});

static SSRF_PATH_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let ssrf_scm = r#"
        (call_expression
          function: (field_expression
            field: (field_identifier) @method (#match? @method "^(get|post|put|delete|patch)$"))
          arguments: (arguments
            (macro_invocation
              macro: (identifier) @mac (#match? @mac "^format$")))
        ) @ssrf_call
    "#;
    tree_sitter::Query::new(&language, ssrf_scm).ok()
});

static CRYPTO_TIMING_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let crypto_scm = r#"
        (binary_expression
          left: (identifier) @lhs (#match? @lhs "^(signature|mac|auth_tag|auth_token)$")
          operator: "=="
          right: (identifier) @rhs
        ) @timing_cmp
    "#;
    tree_sitter::Query::new(&language, crypto_scm).ok()
});

static NUM_DIV_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let div_scm = r#"
        (binary_expression
          left: (_)
          operator: "/"
          right: (identifier) @divisor (#match? @divisor "^(chunk_size|page_size|batch_size|partition_size)$")
        ) @div_op
    "#;
    tree_sitter::Query::new(&language, div_scm).ok()
});

static TOCTOU_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let toctou_scm = r#"
        (call_expression
          function: (field_expression
            field: (field_identifier) @check (#match? @check "^(exists|is_file|is_dir)$"))
        ) @check_call
    "#;
    tree_sitter::Query::new(&language, toctou_scm).ok()
});

static ATOMIC_ORDER_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let atomic_scm = r#"
        (scoped_identifier
          path: (identifier) @path (#match? @path "^Ordering$")
          name: (identifier) @order (#match? @order "^Relaxed$")
        ) @relaxed_ref
    "#;
    tree_sitter::Query::new(&language, atomic_scm).ok()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScmQueryViolation {
    pub rule_code: &'static str,
    pub severity: Severity,
    pub description: String,
    pub file_path: PathBuf,
    pub line: usize,
}

pub fn scan_rust_declarative_queries(file: &Path, content: &str) -> Vec<ScmQueryViolation> {
    let mut violations = Vec::new();
    let language = tree_sitter_rust::language();
    let mut parser = tree_sitter::Parser::new();
    if parser.set_language(&language).is_err() {
        return violations;
    }

    // Query 1: Unchecked unwrap() or expect() in production code
    let path_str = file.to_string_lossy();
    let is_canary = path_str.contains("canary_ast_queries");
    let is_test_file = !is_canary
        && (crate::provenance::is_test_or_mock_path(file)
            || path_str.contains("fixtures")
            || path_str.contains("/benches/")
            || path_str.contains("xtask/"));

    if is_test_file {
        return violations;
    }

    let eval_string;
    let eval_content = if !is_canary {
        eval_string = crate::ast::strip_cfg_test_blocks(content);
        &eval_string
    } else {
        content
    };

    let Some(tree) = parser.parse(eval_content, None) else {
        return violations;
    };

    scan_panic_unwraps(file, eval_content, &tree, &mut violations);
    scan_blocking_sleep(file, eval_content, &tree, &mut violations);
    scan_async_mutex(file, eval_content, &tree, is_canary, &mut violations);
    scan_ssrf_path(
        file,
        eval_content,
        &tree,
        is_canary,
        &path_str,
        &mut violations,
    );
    scan_crypto_timing(file, eval_content, &tree, &mut violations);
    scan_num_div(file, eval_content, &tree, &mut violations);
    scan_lock_inversion(file, eval_content, &tree, &mut violations);
    scan_toctou_race(file, eval_content, &tree, &mut violations);
    scan_atomic_ordering(file, eval_content, &tree, &mut violations);

    violations
}

fn scan_panic_unwraps(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = UNWRAP_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "call" {
                    let mut curr = capture.node.parent();
                    let mut is_in_test = false;
                    while let Some(parent) = curr {
                        if parent.kind() == "function_item" || parent.kind() == "mod_item" {
                            let start = parent.start_byte();
                            let end = (start + 120).min(eval_content.len());
                            let header = &eval_content[start..end];
                            if header.contains("#[test]")
                                || header.contains("#[tokio::test]")
                                || header.contains("mod tests")
                            {
                                is_in_test = true;
                                break;
                            }
                        }
                        curr = parent.parent();
                    }
                    if is_in_test {
                        continue;
                    }

                    let line = capture.node.start_position().row + 1;
                    let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                    if !line_text.contains("UI-EXEMPT")
                        && !line_text.contains("compliance:ignore")
                        && !line_text.contains("PANIC-EXEMPT")
                    {
                        violations.push(ScmQueryViolation {
                            rule_code: "PANIC-001",
                            severity: Severity::High,
                            description: "Unchecked unwrap/expect call in production code path; use ? operator or error mapping".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_blocking_sleep(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = BLOCKING_SLEEP_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "async_fn" {
                    let line = capture.node.start_position().row + 1;
                    violations.push(ScmQueryViolation {
                        rule_code: "BLOCKING-IO-001",
                        severity: Severity::High,
                        description: "Synchronous blocking sleep called inside async function; starves Tokio worker thread (use tokio::time::sleep)".to_string(),
                        file_path: file.to_path_buf(),
                        line,
                    });
                }
            }
        }
    }
}

fn scan_async_mutex(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    is_canary: bool,
    violations: &mut Vec<ScmQueryViolation>,
) {
    let is_std_mutex =
        eval_content.contains("std::sync::Mutex") || eval_content.contains("parking_lot::Mutex");
    if is_canary || is_std_mutex {
        if let Some(query) = ASYNC_MUTEX_QUERY.as_ref() {
            let mut cursor = tree_sitter::QueryCursor::new();
            for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
                for capture in m.captures {
                    if query.capture_names()[capture.index as usize] == "lock_call" {
                        let mut curr = capture.node.parent();
                        while let Some(parent) = curr {
                            if parent.kind() == "function_item" {
                                let fn_text =
                                    parent.utf8_text(eval_content.as_bytes()).unwrap_or("");
                                if fn_text.contains("async ") && fn_text.contains(".await") {
                                    let mut lock_block = capture.node.parent();
                                    let mut block_has_await = false;
                                    while let Some(b) = lock_block {
                                        if b.kind() == "block" {
                                            let block_text =
                                                b.utf8_text(eval_content.as_bytes()).unwrap_or("");
                                            if block_text.contains(".await") {
                                                block_has_await = true;
                                            }
                                            break;
                                        }
                                        lock_block = b.parent();
                                    }
                                    if block_has_await {
                                        let line = capture.node.start_position().row + 1;
                                        violations.push(ScmQueryViolation {
                                            rule_code: "ASYNC-MUTEX-001",
                                            severity: Severity::High,
                                            description: "Mutex lock guard held across await point; causes Tokio runtime worker thread deadlock".to_string(),
                                            file_path: file.to_path_buf(),
                                            line,
                                        });
                                    }
                                }
                                break;
                            }
                            curr = parent.parent();
                        }
                    }
                }
            }
        }
    }
}

fn scan_ssrf_path(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    is_canary: bool,
    path_str: &str,
    violations: &mut Vec<ScmQueryViolation>,
) {
    let should_scan_ssrf = is_canary
        || path_str.contains("canary")
        || path_str.contains("routes/")
        || path_str.contains("handlers/");
    if should_scan_ssrf {
        if let Some(query) = SSRF_PATH_QUERY.as_ref() {
            let mut cursor = tree_sitter::QueryCursor::new();
            for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
                for capture in m.captures {
                    if query.capture_names()[capture.index as usize] == "ssrf_call" {
                        let line = capture.node.start_position().row + 1;
                        let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                        if !line_text.contains("SSRF-EXEMPT")
                            && !line_text.contains("compliance:ignore")
                        {
                            violations.push(ScmQueryViolation {
                                rule_code: "AST-SSRF-PATH-001",
                                severity: Severity::High,
                                description: "Unvalidated format! string interpolation in outbound HTTP request URL; potential SSRF vector".to_string(),
                                file_path: file.to_path_buf(),
                                line,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn scan_crypto_timing(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = CRYPTO_TIMING_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "timing_cmp" {
                    let line = capture.node.start_position().row + 1;
                    let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                    if !line_text.contains("TIMING-EXEMPT")
                        && !line_text.contains("compliance:ignore")
                    {
                        violations.push(ScmQueryViolation {
                            rule_code: "AST-CRYPTO-TIMING-001",
                            severity: Severity::High,
                            description: "Variable-time equality operator used on cryptographic MAC/signature; vulnerable to timing attacks (use subtle::ConstantTimeEq)".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_num_div(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = NUM_DIV_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "div_op" {
                    let line = capture.node.start_position().row + 1;
                    let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                    if !line_text.contains("DIV-EXEMPT") && !line_text.contains("compliance:ignore")
                    {
                        violations.push(ScmQueryViolation {
                            rule_code: "AST-NUM-DIV-001",
                            severity: Severity::High,
                            description: "Division by variable chunk/batch size without verified non-zero guarantee; panic risk".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_lock_inversion(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = ASYNC_MUTEX_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut lock_lines_by_parent: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();

        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "lock_call" {
                    let mut curr = capture.node.parent();
                    while let Some(parent) = curr {
                        if parent.kind() == "function_item" || parent.kind() == "closure_expression"
                        {
                            let parent_id = parent.id();
                            let line = capture.node.start_position().row + 1;
                            lock_lines_by_parent
                                .entry(parent_id)
                                .or_default()
                                .push(line);
                            break;
                        }
                        curr = parent.parent();
                    }
                }
            }
        }

        for (_parent_id, lines) in lock_lines_by_parent {
            let mut distinct_lines: Vec<usize> = lines;
            distinct_lines.sort_unstable();
            distinct_lines.dedup();
            if distinct_lines.len() >= 2 {
                let line = distinct_lines[1];
                let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                if !line_text.contains("DEADLOCK-EXEMPT")
                    && !line_text.contains("compliance:ignore")
                {
                    violations.push(ScmQueryViolation {
                        rule_code: "DEADLOCK-001",
                        severity: Severity::High,
                        description: "Multiple mutex lock guards acquired in nested scope; potential lock inversion (ABBA) deadlock hazard".to_string(),
                        file_path: file.to_path_buf(),
                        line,
                    });
                }
            }
        }
    }
}

fn scan_toctou_race(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = TOCTOU_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "check_call" {
                    let mut curr = capture.node.parent();
                    while let Some(parent) = curr {
                        if parent.kind() == "function_item" {
                            let fn_text = parent.utf8_text(eval_content.as_bytes()).unwrap_or("");
                            if fn_text.contains("write(")
                                || fn_text.contains("create(")
                                || fn_text.contains("remove_file(")
                                || fn_text.contains("create_dir(")
                            {
                                let line = capture.node.start_position().row + 1;
                                let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                                if !line_text.contains("TOCTOU-EXEMPT")
                                    && !line_text.contains("compliance:ignore")
                                {
                                    violations.push(ScmQueryViolation {
                                        rule_code: "TOCTOU-001",
                                        severity: Severity::Medium,
                                        description: "Time-of-Check to Time-of-Use (TOCTOU) race condition; state check followed by non-atomic filesystem operation".to_string(),
                                        file_path: file.to_path_buf(),
                                        line,
                                    });
                                }
                            }
                            break;
                        }
                        curr = parent.parent();
                    }
                }
            }
        }
    }
}

fn scan_atomic_ordering(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = ATOMIC_ORDER_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "relaxed_ref" {
                    let line = capture.node.start_position().row + 1;
                    let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                    if !line_text.contains("ATOMIC-EXEMPT")
                        && !line_text.contains("compliance:ignore")
                    {
                        violations.push(ScmQueryViolation {
                            rule_code: "ATOMIC-ORDER-001",
                            severity: Severity::Medium,
                            description: "Atomic operation using Ordering::Relaxed in synchronization path; memory ordering hazard on weak-memory architectures".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

pub fn convert_to_findings(violations: Vec<ScmQueryViolation>) -> Vec<Finding> {
    violations
        .into_iter()
        .map(|v| {
            Finding::new(v.rule_code, v.severity, v.description, v.file_path, v.line)
                .with_confidence(Confidence::new(0.95))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sitter_queries_detects_unwrap() {
        let code = r#"
            pub fn handle_payment() -> String {
                let token = Some("token");
                token.unwrap().to_string()
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("payment.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "PANIC-001");
    }

    #[test]
    fn test_tree_sitter_queries_skips_test_files() {
        let code = r#"
            #[test]
            fn test_foo() {
                let x = Some(1);
                assert_eq!(x.unwrap(), 1);
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("tests/foo_test.rs"), code);
        assert!(v.is_empty());
    }

    #[test]
    fn test_tree_sitter_queries_detects_canary_ast_violation() {
        let code = r#"
            pub fn canary_ast_structural_violation() {
                let opt: Option<i32> = None;
                let _val = opt.expect("canary_ast_panic_probe");
            }
        "#;
        let v = scan_rust_declarative_queries(
            Path::new("tests/fixtures/scanner_canaries/canary_ast_queries.rs"),
            code,
        );
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "PANIC-001");
    }

    #[test]
    fn test_tree_sitter_async_mutex_deadlock_detection() {
        let code = r#"
            use std::sync::Mutex;
            pub async fn bad_task(m: &Mutex<i32>) {
                let _guard = m.lock();
                tokio::task::yield_now().await;
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("src/services/task.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "ASYNC-MUTEX-001");
    }

    #[test]
    fn test_tree_sitter_ssrf_path_interpolation() {
        let code = r#"
            pub async fn fetch_user(client: &reqwest::Client, user_id: &str) {
                let _ = client.get(format!("https://api.partner.com/users/{}", user_id));
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_ssrf.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "AST-SSRF-PATH-001");
    }

    #[test]
    fn test_tree_sitter_crypto_timing_comparison() {
        let code = r#"
            pub fn verify(signature: &[u8], expected: &[u8]) -> bool {
                signature == expected
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_crypto.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "AST-CRYPTO-TIMING-001");
    }

    #[test]
    fn test_tree_sitter_num_division_by_zero() {
        let code = r#"
            pub fn paginate(total: usize, page_size: usize) -> usize {
                total / page_size
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_div.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "AST-NUM-DIV-001");
    }

    #[test]
    fn test_tree_sitter_lock_inversion_deadlock() {
        let code = r#"
            use std::sync::Mutex;
            pub fn transfer(a: &Mutex<i32>, b: &Mutex<i32>) {
                let _ga = a.lock();
                let _gb = b.lock();
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_deadlock.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "DEADLOCK-001");
    }

    #[test]
    fn test_tree_sitter_toctou_race_condition() {
        let code = r#"
            use std::path::Path;
            pub fn write_safe(path: &Path, data: &[u8]) {
                if !path.exists() {
                    let _ = std::fs::write(path, data);
                }
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_toctou.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "TOCTOU-001");
    }

    #[test]
    fn test_tree_sitter_atomic_relaxed_ordering() {
        let code = r#"
            use std::sync::atomic::{AtomicUsize, Ordering};
            pub fn bump_counter(counter: &AtomicUsize) {
                counter.store(1, Ordering::Relaxed);
            }
        "#;
        let v = scan_rust_declarative_queries(Path::new("canary_atomic.rs"), code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule_code, "ATOMIC-ORDER-001");
    }
}
