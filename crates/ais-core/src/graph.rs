//! Inter-Procedural Semantic Callgraph & AST Taint-Flow Engine (Native Rust)
//!
//! Maps function calls across architectural layers and traces untrusted data
//! from ingress sources (HTTP Path, Query, Json, Form) to sensitive sinks
//! (Database Execute, Subprocess Command, Log Output, File Storage).

use crate::findings::{Confidence, Finding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static PATH_PARAM_RE: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r#"Path\s*\(\s*([a-zA-Z0-9_]+)\s*\)"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static QUERY_PARAM_RE: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r#"Query\s*\(\s*([a-zA-Z0-9_]+)\s*\)"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static JSON_PARAM_RE: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r#"Json\s*\(\s*([a-zA-Z0-9_]+)\s*\)"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static FORM_PARAM_RE: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r#"Form\s*\(\s*([a-zA-Z0-9_]+)\s*\)"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static CALL_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| match regex::Regex::new(r#"([a-zA-Z0-9_]+)\s*\("#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    });

static SQL_FORMAT_SINK: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r#"(?:query|execute)\s*\(\s*&?format!\s*\(\s*"(?:SELECT|INSERT|UPDATE|DELETE)[^"]*\{[a-zA-Z0-9_:?]*\}"#,
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static CMD_SINK: LazyLock<regex::Regex> =
    LazyLock::new(
        || match regex::Regex::new(r#"Command::new\s*\(\s*([a-zA-Z0-9_]+)\s*\)"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static HTML_SCRIPT_FORMAT_SINK: LazyLock<regex::Regex> =
    LazyLock::new(|| {
        match regex::Regex::new(r#"Html\s*\(\s*format!\s*\(.*<script[^>]*>.*\{[a-zA-Z0-9_]+\}"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        }
    });

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaintSourceKind {
    HttpPath,
    HttpQuery,
    HttpJsonBody,
    HttpForm,
    ClientWebStorage,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaintSinkKind {
    DatabaseQuery,
    CommandExecution,
    PlaintextLog,
    UnsanitizedTemplateRender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintFlow {
    pub source_kind: TaintSourceKind,
    pub source_file: PathBuf,
    pub source_line: usize,
    pub sink_kind: TaintSinkKind,
    pub sink_file: PathBuf,
    pub sink_line: usize,
    pub variable_name: String,
    pub call_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallGraphNode {
    pub name: String,
    pub file: PathBuf,
    pub line: usize,
    pub callers: HashSet<String>,
    pub callees: HashSet<String>,
    pub is_ingress: bool,
    pub ingress_sources: Vec<(TaintSourceKind, String, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterProceduralGraph {
    pub nodes: HashMap<String, CallGraphNode>,
    pub total_functions_indexed: usize,
    pub detected_taint_flows: Vec<TaintFlow>,
}

impl InterProceduralGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_function(&mut self, name: &str, file: PathBuf, line: usize) {
        if let Some(node) = self.nodes.get_mut(name) {
            node.file = file.clone();
            node.line = line;
        } else {
            self.total_functions_indexed += 1;
            self.nodes.insert(
                name.to_string(),
                CallGraphNode {
                    name: name.to_string(),
                    file: file.clone(),
                    line,
                    callers: HashSet::new(),
                    callees: HashSet::new(),
                    is_ingress: false,
                    ingress_sources: Vec::new(),
                },
            );
        }

        // Also index qualified name for disambiguation without double-counting functions
        let qualified = format!("{}:{}", file.display(), name);
        self.nodes
            .entry(qualified)
            .or_insert_with(|| CallGraphNode {
                name: name.to_string(),
                file,
                line,
                callers: HashSet::new(),
                callees: HashSet::new(),
                is_ingress: false,
                ingress_sources: Vec::new(),
            });
    }

    pub fn add_edge(&mut self, caller: &str, callee: &str) {
        let caller_node = self.nodes.entry(caller.to_string()).or_insert_with(|| {
            self.total_functions_indexed += 1;
            CallGraphNode {
                name: caller.to_string(),
                file: PathBuf::new(),
                line: 0,
                callers: HashSet::new(),
                callees: HashSet::new(),
                is_ingress: false,
                ingress_sources: Vec::new(),
            }
        });
        caller_node.callees.insert(callee.to_string());

        let callee_node = self.nodes.entry(callee.to_string()).or_insert_with(|| {
            self.total_functions_indexed += 1;
            CallGraphNode {
                name: callee.to_string(),
                file: PathBuf::new(),
                line: 0,
                callers: HashSet::new(),
                callees: HashSet::new(),
                is_ingress: false,
                ingress_sources: Vec::new(),
            }
        });
        callee_node.callers.insert(caller.to_string());
    }
}

fn parse_ingress_params(params: &str, line_no: usize) -> Vec<(TaintSourceKind, String, usize)> {
    let mut sources = Vec::new();
    let patterns: [(&LazyLock<regex::Regex>, TaintSourceKind); 4] = [
        (&PATH_PARAM_RE, TaintSourceKind::HttpPath),
        (&QUERY_PARAM_RE, TaintSourceKind::HttpQuery),
        (&JSON_PARAM_RE, TaintSourceKind::HttpJsonBody),
        (&FORM_PARAM_RE, TaintSourceKind::HttpForm),
    ];
    for (re, kind) in patterns {
        if let Some(m) = re.captures(params) {
            if let Some(cap1) = m.get(1) {
                sources.push((kind, cap1.as_str().to_string(), line_no));
            }
        }
    }
    sources
}

fn extract_fn_and_params(header: &str) -> Option<(String, String)> {
    let fn_idx = header.find("fn ")?;
    let rest = header[fn_idx + 3..].trim_start();
    let paren_idx = rest.find('(')?;
    let mut name = rest[..paren_idx].trim();
    if let Some(angle_idx) = name.find('<') {
        name = name[..angle_idx].trim();
    }
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let after_paren = &rest[paren_idx..];
    let mut depth = 0;
    let mut end_idx = None;
    for (i, ch) in after_paren.char_indices() {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                end_idx = Some(i);
                break;
            }
        }
    }
    let end = end_idx?;
    let params = &after_paren[1..end];
    Some((name.to_string(), params.to_string()))
}

#[derive(Debug, Clone)]
struct RecordedSink {
    caller: String,
    sink_kind: TaintSinkKind,
    file: PathBuf,
    line: usize,
    variable_name: String,
}

#[allow(clippy::needless_range_loop)]
pub fn analyze_workspace_callgraph(root: &Path) -> InterProceduralGraph {
    let mut graph = InterProceduralGraph::new();
    let mut recorded_sinks = Vec::new();
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
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let eval_content = crate::ast::strip_cfg_test_blocks(&content);
            let rel_path = entry
                .path()
                .strip_prefix(root)
                .unwrap_or(entry.path())
                .to_path_buf();
            let mut current_fn: Option<String> = None;

            let lines: Vec<&str> = eval_content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                let line_no = idx + 1;

                let mut header_line = (*line).to_string();
                if line.contains("fn ") && !line.contains(')') {
                    for next_idx in idx + 1..(idx + 8).min(lines.len()) {
                        header_line.push(' ');
                        header_line.push_str(lines[next_idx].trim());
                        if lines[next_idx].contains(')') {
                            break;
                        }
                    }
                }

                if let Some((fn_name, params)) = extract_fn_and_params(&header_line) {
                    graph.add_function(&fn_name, rel_path.clone(), line_no);
                    let sources = parse_ingress_params(&params, line_no);
                    if !sources.is_empty() {
                        if let Some(node) = graph.nodes.get_mut(&fn_name) {
                            node.is_ingress = true;
                            node.ingress_sources.extend(sources);
                        }
                    }
                    current_fn = Some(fn_name);
                    continue;
                }

                if let Some(caller) = &current_fn {
                    for cap in CALL_RE.captures_iter(line) {
                        if let Some(callee_match) = cap.get(1) {
                            let callee = callee_match.as_str();
                            if callee != caller && !callee.starts_with("test_") {
                                graph.add_edge(caller, callee);
                            }
                        }
                    }

                    let mut check_line = (*line).to_string();
                    if line.contains("format!") {
                        for next_idx in idx + 1..(idx + 4).min(lines.len()) {
                            check_line.push(' ');
                            check_line.push_str(lines[next_idx].trim());
                        }
                    }

                    if SQL_FORMAT_SINK.is_match(&check_line) {
                        recorded_sinks.push(RecordedSink {
                            caller: caller.clone(),
                            sink_kind: TaintSinkKind::DatabaseQuery,
                            file: rel_path.clone(),
                            line: line_no,
                            variable_name: String::new(),
                        });
                    }

                    if let Some(cap) = CMD_SINK.captures(line) {
                        if let Some(var_match) = cap.get(1) {
                            let var_name = var_match.as_str();
                            recorded_sinks.push(RecordedSink {
                                caller: caller.clone(),
                                sink_kind: TaintSinkKind::CommandExecution,
                                file: rel_path.clone(),
                                line: line_no,
                                variable_name: var_name.to_string(),
                            });
                        }
                    }

                    if HTML_SCRIPT_FORMAT_SINK.is_match(&check_line) {
                        recorded_sinks.push(RecordedSink {
                            caller: caller.clone(),
                            sink_kind: TaintSinkKind::UnsanitizedTemplateRender,
                            file: rel_path.clone(),
                            line: line_no,
                            variable_name: String::new(),
                        });
                    }
                }
            }
        }
    }

    // Pass 2: Trace reachability from ingress handlers to sinks over the complete graph
    for sink in recorded_sinks {
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((sink.caller.clone(), 0usize));
        visited.insert(sink.caller.clone());

        while let Some((curr, depth)) = queue.pop_front() {
            if let Some(curr_node) = graph.nodes.get(&curr) {
                if curr_node.is_ingress {
                    if let Some((src, var, sline)) = curr_node.ingress_sources.first() {
                        let final_var = if !sink.variable_name.is_empty() {
                            sink.variable_name.clone()
                        } else {
                            var.clone()
                        };
                        graph.detected_taint_flows.push(TaintFlow {
                            source_kind: src.clone(),
                            source_file: curr_node.file.clone(),
                            source_line: *sline,
                            sink_kind: sink.sink_kind.clone(),
                            sink_file: sink.file.clone(),
                            sink_line: sink.line,
                            variable_name: final_var,
                            call_depth: depth,
                        });
                        break;
                    }
                }
                if depth < 3 {
                    for parent in &curr_node.callers {
                        if !visited.contains(parent) {
                            visited.insert(parent.clone());
                            queue.push_back((parent.clone(), depth + 1));
                        }
                    }
                }
            }
        }
    }

    graph
}

pub fn scan_taint_violations(root: &Path) -> (InterProceduralGraph, Vec<Finding>) {
    let graph = analyze_workspace_callgraph(root);
    let mut findings = Vec::new();

    for flow in &graph.detected_taint_flows {
        let (code, msg) = match flow.sink_kind {
            TaintSinkKind::DatabaseQuery => (
                "TAINT-001",
                format!(
                    "Unsafe SQL format string interpolation with untrusted input parameter '{}' originating from {:?} at {}:{}",
                    flow.variable_name, flow.source_kind, flow.source_file.display(), flow.source_line
                ),
            ),
            TaintSinkKind::CommandExecution => (
                "TAINT-002",
                format!(
                    "Subprocess Command execution instantiated with untrusted input parameter '{}' originating from {:?} at {}:{}",
                    flow.variable_name, flow.source_kind, flow.source_file.display(), flow.source_line
                ),
            ),
            TaintSinkKind::UnsanitizedTemplateRender => (
                "TAINT-003",
                format!(
                    "Unsanitized inline script/HTML format string interpolation with untrusted input parameter '{}' originating from {:?} at {}:{}",
                    flow.variable_name, flow.source_kind, flow.source_file.display(), flow.source_line
                ),
            ),
            _ => ("TAINT-004", format!("Unsanitized taint flow: variable '{}'", flow.variable_name)),
        };

        findings.push(Finding {
            rule_code: code.to_string(),
            severity: Severity::Critical,
            message: msg,
            file: flow.sink_file.clone(),
            line: flow.sink_line,
            confidence: Confidence(0.95),
            suppressed: None,
            hint: Some("Use parameterized queries (? or $1) or whitelist allowed commands instead of dynamic formatting.".to_string()),
        });
    }

    (graph, findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_fn_and_params_multi_extractor() {
        let sig = "pub async fn handle_route(State(state): State<Arc<AppState>>, Path(user_id): Path<String>, Json(body): Json<CreateReq>) -> Response";
        let (name, params) = extract_fn_and_params(sig).unwrap();
        assert_eq!(name, "handle_route");
        assert!(params.contains("Path(user_id)"));
        assert!(params.contains("Json(body)"));
        let sources = parse_ingress_params(&params, 1);
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].0, TaintSourceKind::HttpPath);
        assert_eq!(sources[0].1, "user_id");
        assert_eq!(sources[1].0, TaintSourceKind::HttpJsonBody);
        assert_eq!(sources[1].1, "body");
    }

    #[test]
    fn test_callgraph_construction() {
        let mut graph = InterProceduralGraph::new();
        graph.add_function("handle_login", PathBuf::from("routes/auth.rs"), 10);
        graph.add_function("verify_password", PathBuf::from("services/auth.rs"), 45);
        graph.add_edge("handle_login", "verify_password");

        assert_eq!(graph.total_functions_indexed, 2);
        assert!(graph
            .nodes
            .get("handle_login")
            .unwrap()
            .callees
            .contains("verify_password"));
        assert!(graph
            .nodes
            .get("verify_password")
            .unwrap()
            .callers
            .contains("handle_login"));
    }

    #[test]
    fn test_forward_reference_callgraph_edge_preservation() {
        let mut graph = InterProceduralGraph::new();
        // Forward reference: handle_checkout calls process_payment before process_payment is declared
        graph.add_edge("handle_checkout", "process_payment");
        graph.add_function("process_payment", PathBuf::from("services/pay.rs"), 50);
        graph.add_function("handle_checkout", PathBuf::from("routes/checkout.rs"), 20);

        assert_eq!(graph.total_functions_indexed, 2);
        let caller_node = graph.nodes.get("handle_checkout").unwrap();
        let callee_node = graph.nodes.get("process_payment").unwrap();
        assert!(caller_node.callees.contains("process_payment"));
        assert!(callee_node.callers.contains("handle_checkout"));
        assert_eq!(callee_node.file, PathBuf::from("services/pay.rs"));
        assert_eq!(callee_node.line, 50);
    }

    #[test]
    fn test_taint_flow_serialization() {
        let flow = TaintFlow {
            source_kind: TaintSourceKind::HttpQuery,
            source_file: PathBuf::from("routes/search.rs"),
            source_line: 14,
            sink_kind: TaintSinkKind::DatabaseQuery,
            sink_file: PathBuf::from("routes/search.rs"),
            sink_line: 22,
            variable_name: "query_param".to_string(),
            call_depth: 0,
        };

        let json = serde_json::to_string(&flow).unwrap();
        assert!(json.contains("HttpQuery"));
        assert!(json.contains("DatabaseQuery"));
        assert!(json.contains("query_param"));
    }
}
