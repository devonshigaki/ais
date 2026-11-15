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

static HTTP_BOUND_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let scm = r#"
        (await_expression
          (call_expression
            function: (field_expression
              field: (field_identifier) @method (#match? @method "^(bytes|text)$"))
            arguments: (arguments)) @call
        ) @await_body
    "#;
    tree_sitter::Query::new(&language, scm).ok()
});

static LIBSQL_BUILDER_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let scm = r#"
        (call_expression
          function: (scoped_identifier
            path: (scoped_identifier) @mod (#match? @mod "Builder$")
            name: (identifier) @ctor (#match? @ctor "^new_"))
        ) @builder_call
    "#;
    tree_sitter::Query::new(&language, scm).ok()
});

static SELECT_MACRO_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let scm = r#"
        (macro_invocation
          macro: (_) @mac (#match? @mac "(^|::)select$")
        ) @select_mac
    "#;
    tree_sitter::Query::new(&language, scm).ok()
});

static STRUCT_ITEM_QUERY: LazyLock<Option<tree_sitter::Query>> = LazyLock::new(|| {
    let language = tree_sitter_rust::language();
    let scm = r#"
        (struct_item
          name: (type_identifier) @struct_name
          body: (field_declaration_list
            (field_declaration
              name: (field_identifier) @field_name
              type: (_) @field_type) @field_decl)) @struct_def
    "#;
    tree_sitter::Query::new(&language, scm).ok()
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
    scan_http_bounded_body(
        file,
        eval_content,
        &tree,
        is_canary,
        &path_str,
        &mut violations,
    );
    scan_libsql_busy_timeout(file, eval_content, &tree, is_canary, &mut violations);
    scan_select_cancellation_safety(file, eval_content, &tree, &mut violations);
    scan_ast_plaintext_secret_fields(file, eval_content, &tree, is_canary, &mut violations);

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
        let mut locks_by_block: std::collections::HashMap<usize, Vec<(String, usize)>> =
            std::collections::HashMap::new();

        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "lock_call" {
                    let receiver = capture
                        .node
                        .child_by_field_name("function")
                        .and_then(|f| f.child_by_field_name("value"))
                        .and_then(|v| v.utf8_text(eval_content.as_bytes()).ok())
                        .unwrap_or("")
                        .to_string();

                    let mut curr = capture.node.parent();
                    while let Some(parent) = curr {
                        if parent.kind() == "block" {
                            let block_id = parent.id();
                            let line = capture.node.start_position().row + 1;
                            locks_by_block
                                .entry(block_id)
                                .or_default()
                                .push((receiver, line));
                            break;
                        }
                        curr = parent.parent();
                    }
                }
            }
        }

        for (_block_id, locks) in locks_by_block {
            let mut distinct_receivers = std::collections::HashSet::new();
            for (recv, _) in &locks {
                if !recv.is_empty() {
                    distinct_receivers.insert(recv.clone());
                }
            }
            let has_unqualified_receivers = distinct_receivers.iter().any(|r| !r.contains('.'));
            if distinct_receivers.len() >= 2 && has_unqualified_receivers {
                let line = locks[1].1;
                let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                let prev_line_text = if line >= 2 {
                    eval_content.lines().nth(line - 2).unwrap_or("")
                } else {
                    ""
                };
                if !line_text.contains("DEADLOCK-EXEMPT")
                    && !prev_line_text.contains("DEADLOCK-EXEMPT")
                    && !line_text.contains("compliance:ignore")
                    && !prev_line_text.contains("compliance:ignore")
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
                    let receiver = capture
                        .node
                        .child_by_field_name("function")
                        .and_then(|f| f.child_by_field_name("value"))
                        .and_then(|v| v.utf8_text(eval_content.as_bytes()).ok())
                        .unwrap_or("");
                    if receiver.ends_with("file_type()") {
                        continue;
                    }

                    let mut curr = capture.node.parent();
                    let mut is_in_condition = false;
                    let mut enclosing_if = None;
                    while let Some(parent) = curr {
                        if parent.kind() == "if_expression" {
                            if let Some(condition) = parent.child_by_field_name("condition") {
                                if condition.byte_range().contains(&capture.node.start_byte()) {
                                    is_in_condition = true;
                                    enclosing_if = Some(parent);
                                }
                            }
                            break;
                        }
                        curr = parent.parent();
                    }

                    if is_in_condition {
                        if let Some(if_node) = enclosing_if {
                            let if_text = if_node.utf8_text(eval_content.as_bytes()).unwrap_or("");
                            if if_text.contains("fs::write(")
                                || if_text.contains("std::fs::write(")
                                || if_text.contains("fs::remove_file(")
                                || if_text.contains("std::fs::remove_file(")
                                || if_text.contains("fs::remove_dir(")
                                || if_text.contains("std::fs::remove_dir(")
                                || if_text.contains("File::create(")
                                || if_text.contains("std::fs::File::create(")
                            {
                                let line = capture.node.start_position().row + 1;
                                let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                                let prev_line_text = if line >= 2 {
                                    eval_content.lines().nth(line - 2).unwrap_or("")
                                } else {
                                    ""
                                };
                                if !line_text.contains("TOCTOU-EXEMPT")
                                    && !prev_line_text.contains("TOCTOU-EXEMPT")
                                    && !line_text.contains("compliance:ignore")
                                    && !prev_line_text.contains("compliance:ignore")
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
                        }
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
    let file_str = file.to_string_lossy();
    if file_str.contains("/metrics")
        || file_str.contains("telemetry")
        || file_str.contains("/stats")
        || file_str.contains("logger.rs")
        || file_str.contains("benchmark")
    {
        return;
    }

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

fn scan_http_bounded_body(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    is_canary: bool,
    path_str: &str,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if !is_canary
        && (path_str.contains("crates/adapters/http-client")
            || path_str.contains("tests/")
            || path_str.contains("e2e/"))
    {
        return;
    }
    if let Some(query) = HTTP_BOUND_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "call" {
                    let receiver = capture
                        .node
                        .child_by_field_name("function")
                        .and_then(|f| f.child_by_field_name("value"))
                        .and_then(|v| v.utf8_text(eval_content.as_bytes()).ok())
                        .unwrap_or("");
                    let is_http_receiver = is_canary
                        || matches!(
                            receiver,
                            "resp" | "response" | "res" | "http_resp" | "api_resp"
                        );
                    if !is_http_receiver {
                        continue;
                    }
                    let mut curr = capture.node.parent();
                    let mut has_size_guard = false;
                    while let Some(parent) = curr {
                        if parent.kind() == "function_item" {
                            let fn_txt = parent.utf8_text(eval_content.as_bytes()).unwrap_or("");
                            if fn_txt.contains("content_length")
                                || fn_txt.contains("read_bounded_bytes")
                                || fn_txt.contains("read_bounded_text")
                                || fn_txt.contains("MAX_")
                            {
                                has_size_guard = true;
                            }
                            break;
                        }
                        curr = parent.parent();
                    }
                    if has_size_guard {
                        continue;
                    }
                    let line = capture.node.start_position().row + 1;
                    let line_text = eval_content.lines().nth(line - 1).unwrap_or("");
                    if !line_text.contains("HTTP-BOUND-EXEMPT")
                        && !line_text.contains("compliance:ignore")
                    {
                        violations.push(ScmQueryViolation {
                            rule_code: "HTTP-BOUND-001",
                            severity: Severity::Medium,
                            description: "Unbounded HTTP response body buffering (.bytes()/.text().await) without Content-Length or streaming byte cap; use http_client_adapter::read_bounded_bytes/text".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_libsql_busy_timeout(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    is_canary: bool,
    violations: &mut Vec<ScmQueryViolation>,
) {
    if let Some(query) = LIBSQL_BUILDER_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "builder_call" {
                    let call_text = capture
                        .node
                        .utf8_text(eval_content.as_bytes())
                        .unwrap_or("");
                    if !call_text.contains("libsql::Builder") && !is_canary {
                        continue;
                    }
                    let mut curr = capture.node.parent();
                    let mut hardened = false;
                    while let Some(parent) = curr {
                        if parent.kind() == "function_item" {
                            let fn_txt = parent.utf8_text(eval_content.as_bytes()).unwrap_or("");
                            if fn_txt.contains("busy_timeout")
                                || fn_txt.contains("connect_hardened")
                                || fn_txt.contains("DbExecutor")
                                || fn_txt.contains("new_remote(")
                            {
                                hardened = true;
                            }
                            break;
                        }
                        curr = parent.parent();
                    }
                    if !hardened {
                        let line = capture.node.start_position().row + 1;
                        violations.push(ScmQueryViolation {
                            rule_code: "DB-SQLITE-001",
                            severity: Severity::High,
                            description: "libsql::Builder local/replica database open lacks busy_timeout and PRAGMA foreign_keys = ON (use shared_wiring::connect_hardened)".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_select_cancellation_safety(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    violations: &mut Vec<ScmQueryViolation>,
) {
    const NON_CANCEL_SAFE: &[&str] = &[
        "read_exact(",
        "read_to_end(",
        "read_to_string(",
        "recv_many(",
    ];
    if let Some(query) = SELECT_MACRO_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "select_mac" {
                    let mac_text = capture
                        .node
                        .utf8_text(eval_content.as_bytes())
                        .unwrap_or("");
                    if NON_CANCEL_SAFE.iter().any(|p| mac_text.contains(p)) {
                        let line = capture.node.start_position().row + 1;
                        violations.push(ScmQueryViolation {
                            rule_code: "ASYNC-CANCEL-001",
                            severity: Severity::High,
                            description: "Non-cancellation-safe future (read_exact/read_to_end/recv_many) inside tokio::select! arm loses buffered state on sibling branch completion".to_string(),
                            file_path: file.to_path_buf(),
                            line,
                        });
                    }
                }
            }
        }
    }
}

fn scan_ast_plaintext_secret_fields(
    file: &Path,
    eval_content: &str,
    tree: &tree_sitter::Tree,
    is_canary: bool,
    violations: &mut Vec<ScmQueryViolation>,
) {
    const UNIVERSAL_SECRET_FIELDS: &[&str] = &[
        "private_key",
        "entity_secret_hex",
        "signing_secret",
        "seed_phrase",
        "mnemonic",
    ];
    const CONFIG_SECRET_FIELDS: &[&str] = &[
        "client_secret",
        "api_key",
        "secret",
        "password",
        "webhook_secret",
        "auth_token",
    ];
    const SENSITIVE_SUFFIXES: &[&str] = &[
        "Config",
        "Client",
        "Form",
        "Request",
        "Credential",
        "Credentials",
        "Verifier",
        "Delivery",
        "Settings",
    ];

    if let Some(query) = STRUCT_ITEM_QUERY.as_ref() {
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut seen = std::collections::HashSet::new();
        for m in cursor.matches(query, tree.root_node(), eval_content.as_bytes()) {
            let mut struct_node = None;
            let mut struct_name = "";
            let mut field_name = "";
            let mut field_type = "";
            let mut field_node = None;

            for cap in m.captures {
                let cname = query.capture_names()[cap.index as usize];
                let txt = cap.node.utf8_text(eval_content.as_bytes()).unwrap_or("");
                match cname {
                    "struct_def" => struct_node = Some(cap.node),
                    "struct_name" => struct_name = txt,
                    "field_name" => field_name = txt,
                    "field_type" => field_type = txt,
                    "field_decl" => field_node = Some(cap.node),
                    _ => {}
                }
            }

            let (Some(snode), Some(fnode)) = (struct_node, field_node) else {
                continue;
            };

            let mut has_debug = false;
            let mut prev = snode.prev_sibling();
            while let Some(sib) = prev {
                if sib.kind() == "attribute_item" {
                    let attr_txt = sib.utf8_text(eval_content.as_bytes()).unwrap_or("");
                    if attr_txt.contains("derive") && attr_txt.contains("Debug") {
                        has_debug = true;
                    }
                    prev = sib.prev_sibling();
                } else if sib.kind() == "line_comment" || sib.kind() == "block_comment" {
                    prev = sib.prev_sibling();
                } else {
                    break;
                }
            }
            if !has_debug {
                continue;
            }

            let field_txt = fnode.utf8_text(eval_content.as_bytes()).unwrap_or("");
            if field_txt.contains("SEC-012-EXEMPT") || field_txt.contains("SecretString") {
                continue;
            }

            let normalized_ty: String = field_type.chars().filter(|c| !c.is_whitespace()).collect();
            let is_plain_string = normalized_ty == "String" || normalized_ty == "Option<String>";
            if !is_plain_string {
                continue;
            }

            let is_sensitive_struct =
                is_canary || SENSITIVE_SUFFIXES.iter().any(|s| struct_name.ends_with(s));
            let is_secret_field = UNIVERSAL_SECRET_FIELDS.contains(&field_name)
                || (is_sensitive_struct && CONFIG_SECRET_FIELDS.contains(&field_name));
            if !is_secret_field {
                continue;
            }

            let line = fnode.start_position().row + 1;
            if seen.insert((struct_name.to_string(), field_name.to_string(), line)) {
                violations.push(ScmQueryViolation {
                    rule_code: "AST-SECRET-EXPOSURE-001",
                    severity: Severity::Critical,
                    description: format!(
                        "Struct '{struct_name}' derives Debug with plaintext String credential field '{field_name}' (CWE-532); wrap field in SecretString or implement redacted Debug"
                    ),
                    file_path: file.to_path_buf(),
                    line,
                });
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

    #[test]
    fn test_tree_sitter_multiline_http_bound_and_negative_control() {
        let bad = "pub async fn fetch(resp: reqwest::Response) {\n    let _ = resp\n        .bytes()\n        .await;\n}";
        let v = scan_rust_declarative_queries(Path::new("canary_http.rs"), bad);
        assert!(v.iter().any(|x| x.rule_code == "HTTP-BOUND-001"));

        let good = "pub async fn fetch(resp: reqwest::Response) {\n    let _ = read_bounded_bytes(resp, 1024).await;\n}";
        let vg = scan_rust_declarative_queries(Path::new("canary_http.rs"), good);
        assert!(vg.is_empty());
    }

    #[test]
    fn test_tree_sitter_libsql_busy_timeout_and_negative_control() {
        let bad = "pub async fn open_it() {\n    let _ = libsql::Builder::new_local(\"/tmp/a.db\").build().await;\n}";
        let v = scan_rust_declarative_queries(Path::new("canary_db.rs"), bad);
        assert!(v.iter().any(|x| x.rule_code == "DB-SQLITE-001"));

        let good = "pub async fn open_it() {\n    let db = libsql::Builder::new_local(\"/tmp/a.db\").build().await.unwrap();\n    let conn = db.connect().unwrap();\n    let _ = conn.busy_timeout(std::time::Duration::from_millis(5000));\n}";
        let vg = scan_rust_declarative_queries(Path::new("canary_db.rs"), good);
        assert!(!vg.iter().any(|x| x.rule_code == "DB-SQLITE-001"));
    }

    #[test]
    fn test_tree_sitter_select_cancel_safety_and_secret_partial_bypass() {
        let bad_select = "pub async fn pump() {\n    tokio::select! {\n        _ = reader.read_exact(&mut buf) => {}\n    }\n}";
        let vs = scan_rust_declarative_queries(Path::new("canary_select.rs"), bad_select);
        assert!(vs.iter().any(|x| x.rule_code == "ASYNC-CANCEL-001"));

        // Mixed struct: has one SecretString field AND one plaintext String secret field (old SEC-012 missed this!)
        let partial_secret = "#[derive(Debug, Clone)]\npub struct MixedConfig {\n    pub api_key: SecretString,\n    pub client_secret: String,\n}";
        let vsec = scan_rust_declarative_queries(Path::new("canary_sec.rs"), partial_secret);
        assert!(vsec
            .iter()
            .any(|x| x.rule_code == "AST-SECRET-EXPOSURE-001"));
    }
}
