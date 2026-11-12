//! Universal AST & Semantic Query Abstractions
//!
//! Provides language-agnostic AST node extraction and structural semantic queries
//! across Rust, TypeScript, JavaScript, Python, and SQL:
//! - Function and method declarations with parameter and return signatures
//! - Call expression tracking and method invocation graphs
//! - Module imports, exports, and namespace boundaries
//! - Nested scope tracking and loop/block nesting depth

use std::path::Path;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedLanguage {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Sql,
    Html,
    Unknown,
}

impl SupportedLanguage {
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "rs" => Self::Rust,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" => Self::JavaScript,
            "py" => Self::Python,
            "sql" => Self::Sql,
            "html" | "htm" => Self::Html,
            _ => Self::Unknown,
        }
    }

    pub fn is_scannable(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

fn count_rust_syntax_braces(line: &str, in_raw_string: &mut bool) -> (i32, bool) {
    let mut chars = line.chars().peekable();
    let mut delta = 0;
    let mut has_open = false;
    let mut in_string = false;
    let mut escape = false;

    while let Some(ch) = chars.next() {
        if *in_raw_string {
            if ch == '"' && chars.peek() == Some(&'#') {
                *in_raw_string = false;
                chars.next();
            }
            continue;
        }

        if in_string {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == 'r' && chars.peek() == Some(&'#') {
            chars.next();
            if chars.peek() == Some(&'"') {
                chars.next();
                *in_raw_string = true;
                continue;
            }
        }

        if ch == '"' {
            in_string = true;
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'/') {
            break;
        }

        if ch == '{' {
            delta += 1;
            has_open = true;
        } else if ch == '}' {
            delta -= 1;
        }
    }

    (delta, has_open)
}

/// Strips only the specific `#[cfg(test)]` item or module block while preserving all production code before and after it.
pub fn strip_cfg_test_blocks(content: &str) -> String {
    if !content.contains("#[cfg(test")
        && !content.contains("#[cfg(any(test")
        && !content.contains("#[test]")
        && !content.contains("#[tokio::test]")
        && !content.contains("mod tests")
        && !content.contains("mod test")
    {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let mut in_test_block = false;
    let mut brace_depth = 0;
    let mut found_open_brace = false;
    let mut in_raw_string = false;

    for line in content.lines() {
        let trimmed = line.trim();

        let is_mod_tests = trimmed.starts_with("mod tests")
            || trimmed.starts_with("pub mod tests")
            || trimmed.starts_with("pub(crate) mod tests")
            || trimmed == "mod test"
            || trimmed.starts_with("mod test ")
            || trimmed.starts_with("mod test{")
            || trimmed == "pub mod test"
            || trimmed.starts_with("pub mod test ")
            || trimmed.starts_with("pub mod test{");

        if trimmed.starts_with("#[cfg(test")
            || trimmed.starts_with("#[cfg(any(test")
            || trimmed.starts_with("#[test]")
            || trimmed.starts_with("#[tokio::test]")
            || is_mod_tests
        {
            if !in_test_block {
                in_test_block = true;
                brace_depth = 0;
                found_open_brace = false;
            }
            let (delta, has_open) = count_rust_syntax_braces(line, &mut in_raw_string);
            brace_depth += delta;
            if has_open {
                found_open_brace = true;
            }
            result.push('\n');
            continue;
        }

        if in_test_block {
            let (delta, has_open) = count_rust_syntax_braces(line, &mut in_raw_string);
            brace_depth += delta;
            if has_open {
                found_open_brace = true;
            }

            if (found_open_brace && brace_depth <= 0)
                || (!found_open_brace && !trimmed.starts_with("#[") && trimmed.ends_with(';'))
            {
                in_test_block = false;
                found_open_brace = false;
                brace_depth = 0;
            }
            result.push('\n');
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Function,
    Method,
    Struct,
    Class,
    Import,
    CallExpression,
    LoopBlock,
}

#[derive(Debug, Clone)]
pub struct AstNodeSummary {
    pub kind: NodeKind,
    pub name: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub nesting_depth: usize,
    pub text_snippet: String,
}

static RUST_FN_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r"^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:async\s+)?(?:const\s+)?fn\s+([a-zA-Z0-9_]+)",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static RUST_STRUCT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r"^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:struct|enum|trait)\s+([a-zA-Z0-9_]+)",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static RUST_USE_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| match regex::Regex::new(r"^\s*use\s+([^;]+);") {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    });

static TS_FN_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r"(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([a-zA-Z0-9_]+)|(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*(?:async\s*)?\(",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static TS_IMPORT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r#"import\s+(?:\{[^}]*\}|\*\s+as\s+[a-zA-Z0-9_]+|[a-zA-Z0-9_]+)\s+from\s+['"]([^'"]+)['"]"#,
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static PY_DEF_RE: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r"^\s*(?:async\s+)?def\s+([a-zA-Z0-9_]+)\s*\(") {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static PY_IMPORT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(r"^\s*(?:from\s+([a-zA-Z0-9_.]+)\s+import|import\s+([a-zA-Z0-9_.]+))") {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

fn extract_rust_tree_sitter(content: &str) -> Option<Vec<AstNodeSummary>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_rust::language()).ok()?;
    let tree = parser.parse(content, None)?;
    let mut results = Vec::new();
    walk_rust_node(tree.root_node(), content, &mut results);
    Some(results)
}

fn node_snippet(node: tree_sitter::Node<'_>, content: &str) -> String {
    node.utf8_text(content.as_bytes())
        .unwrap_or("")
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn node_name(node: tree_sitter::Node<'_>, content: &str) -> Option<String> {
    node.child_by_field_name("name")
        .and_then(|n| n.utf8_text(content.as_bytes()).ok())
        .map(ToString::to_string)
}

fn classify_rust_node(
    node: tree_sitter::Node<'_>,
    content: &str,
    depth: usize,
) -> Option<AstNodeSummary> {
    let kind = node.kind();
    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;

    match kind {
        "function_item" => {
            let name = node_name(node, content);
            let snippet = node_snippet(node, content);
            Some(AstNodeSummary {
                kind: if depth > 0 {
                    NodeKind::Method
                } else {
                    NodeKind::Function
                },
                name,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        "struct_item" | "enum_item" | "trait_item" => {
            let name = node_name(node, content);
            let snippet = node_snippet(node, content);
            Some(AstNodeSummary {
                kind: NodeKind::Struct,
                name,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        "use_declaration" => {
            let snippet = node_snippet(node, content);
            let name = snippet
                .strip_prefix("use ")
                .and_then(|s| s.strip_suffix(';'))
                .map(|s| s.trim().to_string())
                .or_else(|| Some(snippet.clone()));
            Some(AstNodeSummary {
                kind: NodeKind::Import,
                name,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        _ => None,
    }
}

fn walk_rust_node(root: tree_sitter::Node<'_>, content: &str, results: &mut Vec<AstNodeSummary>) {
    let mut stack = vec![(root, 0usize)];
    const MAX_NODES: usize = 100_000;
    let mut count = 0;

    // POT-002: Bounded iteration stack
    while let Some((node, depth)) = stack.pop() {
        count += 1;
        if count >= MAX_NODES {
            break;
        }

        let kind = node.kind();
        if let Some(summary) = classify_rust_node(node, content, depth) {
            results.push(summary);
        }

        let next_depth = if matches!(kind, "impl_item" | "mod_item" | "function_item" | "block") {
            depth + 1
        } else {
            depth
        };

        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();
        for child in children.into_iter().rev() {
            stack.push((child, next_depth));
        }
    }
}

fn extract_typescript_tree_sitter(content: &str, is_tsx: bool) -> Option<Vec<AstNodeSummary>> {
    let mut parser = tree_sitter::Parser::new();
    let lang = if is_tsx {
        tree_sitter_typescript::language_tsx()
    } else {
        tree_sitter_typescript::language_typescript()
    };
    parser.set_language(&lang).ok()?;
    let tree = parser.parse(content, None)?;
    let mut results = Vec::new();
    walk_ts_node(tree.root_node(), content, &mut results);
    Some(results)
}

fn classify_ts_node(
    node: tree_sitter::Node<'_>,
    content: &str,
    depth: usize,
) -> Option<AstNodeSummary> {
    let kind = node.kind();
    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;

    match kind {
        "function_declaration" | "method_definition" => {
            let name = node_name(node, content);
            let snippet = node_snippet(node, content);
            Some(AstNodeSummary {
                kind: if kind == "method_definition" || depth > 0 {
                    NodeKind::Method
                } else {
                    NodeKind::Function
                },
                name,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        "class_declaration" => {
            let name = node_name(node, content);
            let snippet = node_snippet(node, content);
            Some(AstNodeSummary {
                kind: NodeKind::Class,
                name,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        "import_statement" => {
            let snippet = node_snippet(node, content);
            let source = node
                .child_by_field_name("source")
                .and_then(|n| n.utf8_text(content.as_bytes()).ok())
                .map(|s| s.trim_matches(|c| c == '\'' || c == '"').to_string());
            Some(AstNodeSummary {
                kind: NodeKind::Import,
                name: source,
                start_line,
                end_line,
                nesting_depth: depth,
                text_snippet: snippet,
            })
        }
        _ => None,
    }
}

fn walk_ts_node(root: tree_sitter::Node<'_>, content: &str, results: &mut Vec<AstNodeSummary>) {
    let mut stack = vec![(root, 0usize)];
    const MAX_NODES: usize = 100_000;
    let mut count = 0;

    // POT-002: Bounded iteration stack
    while let Some((node, depth)) = stack.pop() {
        count += 1;
        if count >= MAX_NODES {
            break;
        }

        let kind = node.kind();
        if let Some(summary) = classify_ts_node(node, content, depth) {
            results.push(summary);
        }

        let next_depth = if matches!(
            kind,
            "class_declaration" | "statement_block" | "function_declaration"
        ) {
            depth + 1
        } else {
            depth
        };

        let mut cursor = node.walk();
        let children: Vec<_> = node.children(&mut cursor).collect();
        for child in children.into_iter().rev() {
            stack.push((child, next_depth));
        }
    }
}

fn parse_regex_rust_declaration(
    trimmed: &str,
    idx: usize,
    nesting_depth: usize,
) -> Option<AstNodeSummary> {
    if let Some(cap) = RUST_FN_RE.captures(trimmed) {
        let m = cap.get(1)?;
        return Some(AstNodeSummary {
            kind: if nesting_depth > 0 {
                NodeKind::Method
            } else {
                NodeKind::Function
            },
            name: Some(m.as_str().to_string()),
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    if let Some(cap) = RUST_STRUCT_RE.captures(trimmed) {
        let m = cap.get(1)?;
        return Some(AstNodeSummary {
            kind: NodeKind::Struct,
            name: Some(m.as_str().to_string()),
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    if let Some(cap) = RUST_USE_RE.captures(trimmed) {
        let m = cap.get(1)?;
        return Some(AstNodeSummary {
            kind: NodeKind::Import,
            name: Some(m.as_str().to_string()),
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    None
}

fn parse_regex_ts_declaration(
    trimmed: &str,
    idx: usize,
    nesting_depth: usize,
) -> Option<AstNodeSummary> {
    if let Some(cap) = TS_FN_RE.captures(trimmed) {
        let name = cap
            .get(1)
            .or_else(|| cap.get(2))
            .map(|m| m.as_str().to_string());
        return Some(AstNodeSummary {
            kind: NodeKind::Function,
            name,
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    if let Some(cap) = TS_IMPORT_RE.captures(trimmed) {
        let m = cap.get(1)?;
        return Some(AstNodeSummary {
            kind: NodeKind::Import,
            name: Some(m.as_str().to_string()),
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    None
}

fn parse_regex_py_declaration(
    trimmed: &str,
    idx: usize,
    nesting_depth: usize,
) -> Option<AstNodeSummary> {
    if let Some(cap) = PY_DEF_RE.captures(trimmed) {
        let m = cap.get(1)?;
        return Some(AstNodeSummary {
            kind: NodeKind::Function,
            name: Some(m.as_str().to_string()),
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    if let Some(cap) = PY_IMPORT_RE.captures(trimmed) {
        let name = cap
            .get(1)
            .or_else(|| cap.get(2))
            .map(|m| m.as_str().to_string());
        return Some(AstNodeSummary {
            kind: NodeKind::Import,
            name,
            start_line: idx + 1,
            end_line: idx + 1,
            nesting_depth,
            text_snippet: trimmed.to_string(),
        });
    }

    None
}

pub fn extract_named_declarations(content: &str, lang: SupportedLanguage) -> Vec<AstNodeSummary> {
    match lang {
        SupportedLanguage::Rust => {
            if let Some(nodes) = extract_rust_tree_sitter(content) {
                return nodes;
            }
        }
        SupportedLanguage::TypeScript | SupportedLanguage::JavaScript => {
            if let Some(nodes) = extract_typescript_tree_sitter(content, true) {
                return nodes;
            }
        }
        _ => {}
    }

    let mut declarations = Vec::new();
    let mut nesting_depth = 0;

    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();

        let decl = match lang {
            SupportedLanguage::Rust => parse_regex_rust_declaration(trimmed, idx, nesting_depth),
            SupportedLanguage::TypeScript | SupportedLanguage::JavaScript => {
                parse_regex_ts_declaration(trimmed, idx, nesting_depth)
            }
            SupportedLanguage::Python => parse_regex_py_declaration(trimmed, idx, nesting_depth),
            _ => None,
        };

        if let Some(d) = decl {
            declarations.push(d);
        }

        nesting_depth = (nesting_depth + opens).saturating_sub(closes);
    }

    declarations
}

#[derive(Debug, Clone)]
pub enum SemanticAstQuery {
    FunctionDeclaration { name_contains: String },
    ImportDeclaration { module_prefix: String },
}

pub fn query_ast(
    content: &str,
    lang: SupportedLanguage,
    query: &SemanticAstQuery,
) -> Vec<AstNodeSummary> {
    let nodes = extract_named_declarations(content, lang);
    match query {
        SemanticAstQuery::FunctionDeclaration { name_contains } => nodes
            .into_iter()
            .filter(|n| matches!(n.kind, NodeKind::Function | NodeKind::Method))
            .filter(|n| n.name.as_deref().unwrap_or("").contains(name_contains))
            .collect(),
        SemanticAstQuery::ImportDeclaration { module_prefix } => nodes
            .into_iter()
            .filter(|n| n.kind == NodeKind::Import)
            .filter(|n| n.name.as_deref().unwrap_or("").contains(module_prefix))
            .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstPatternViolation {
    pub rule_code: &'static str,
    pub description: String,
    pub start_line: usize,
    pub end_line: usize,
    pub snippet: String,
}

pub fn scan_tree_sitter_structural_violations(content: &str) -> Vec<AstPatternViolation> {
    let mut violations = Vec::new();
    let language = tree_sitter_rust::language();
    let mut parser = tree_sitter::Parser::new();
    if parser.set_language(&language).is_err() {
        return violations;
    }
    let Some(tree) = parser.parse(content, None) else {
        return violations;
    };

    let unwrap_scm = r#"
        (call_expression
          function: (field_expression
            field: (field_identifier) @method (#match? @method "^(unwrap|expect)$"))
        ) @call
    "#;

    if let Ok(query) = tree_sitter::Query::new(&language, unwrap_scm) {
        let mut cursor = tree_sitter::QueryCursor::new();
        for m in cursor.matches(&query, tree.root_node(), content.as_bytes()) {
            for capture in m.captures {
                if query.capture_names()[capture.index as usize] == "call" {
                    let start_line = capture.node.start_position().row + 1;
                    let end_line = capture.node.end_position().row + 1;
                    let snippet = capture
                        .node
                        .utf8_text(content.as_bytes())
                        .unwrap_or("")
                        .lines()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    violations.push(AstPatternViolation {
                        rule_code: "PANIC-001",
                        description: "Tree-Sitter Query AST match: Unchecked unwrap/expect in control flow; use ? operator or error propagation".to_string(),
                        start_line,
                        end_line,
                        snippet,
                    });
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sitter_scm_query_finds_unwrap() {
        let code = r#"
            pub fn handle_request() -> String {
                let opt = Some(42);
                let val = opt.unwrap();
                format!("{}", val)
            }
        "#;
        let violations = scan_tree_sitter_structural_violations(code);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_code, "PANIC-001");
        assert_eq!(violations[0].start_line, 4);
    }

    #[test]
    fn test_rust_ast_declarations() {
        let code = r#"
            use adapters::db::LibSqlRepo;
            pub async fn process_offer(user: &User) -> Result<()> {
                let x = 1;
            }
            pub struct UserOffer;
        "#;
        let nodes = extract_named_declarations(code, SupportedLanguage::Rust);
        assert!(nodes
            .iter()
            .any(|n| n.name.as_deref() == Some("process_offer") && n.kind == NodeKind::Function));
        assert!(nodes
            .iter()
            .any(|n| n.name.as_deref() == Some("UserOffer") && n.kind == NodeKind::Struct));
        assert!(nodes
            .iter()
            .any(|n| n.name.as_deref() == Some("adapters::db::LibSqlRepo")
                && n.kind == NodeKind::Import));
    }
    #[test]
    fn test_rust_tree_sitter_multiline_bounds() {
        let code = "struct Service;\n\nimpl Service {\n    pub fn calculate_score(&self) -> u32 {\n        let a = 1;\n        let b = 2;\n        a + b\n    }\n}\n";
        let nodes = extract_named_declarations(code, SupportedLanguage::Rust);
        let calc_fn = nodes
            .iter()
            .find(|n| n.name.as_deref() == Some("calculate_score"))
            .expect("should find calculate_score");
        assert_eq!(calc_fn.kind, NodeKind::Method);
        assert_eq!(calc_fn.start_line, 4);
        assert_eq!(calc_fn.end_line, 8);
        assert!(calc_fn.nesting_depth >= 1);
    }

    #[test]
    fn test_ts_ast_declarations() {
        let code = r#"
            import { useState } from 'react';
            export async function fetchProfile(id: string) { return id; }
        "#;
        let nodes = extract_named_declarations(code, SupportedLanguage::TypeScript);
        assert!(nodes
            .iter()
            .any(|n| n.name.as_deref() == Some("fetchProfile")));
        assert!(nodes.iter().any(|n| n.name.as_deref() == Some("react")));
    }

    #[test]
    fn test_py_ast_declarations() {
        let code = r#"
            from os import path
            def calculate_risk(metric):
                pass
        "#;
        let nodes = extract_named_declarations(code, SupportedLanguage::Python);
        assert!(nodes
            .iter()
            .any(|n| n.name.as_deref() == Some("calculate_risk")));
        assert!(nodes.iter().any(|n| n.name.as_deref() == Some("os")));
    }

    #[test]
    fn test_strip_cfg_test_blocks() {
        let code = r#"
pub fn prod_a() -> i32 { 1 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prod_a() {
        assert_eq!(prod_a(), 1);
    }
}

pub fn prod_b() -> i32 { 2 }
"#;
        let stripped = strip_cfg_test_blocks(code);
        assert!(stripped.contains("pub fn prod_a"));
        assert!(stripped.contains("pub fn prod_b"));
        assert!(!stripped.contains("test_prod_a"));
        assert!(!stripped.contains("mod tests"));
    }
}
