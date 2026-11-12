//! LCOM4 (Lack of Cohesion of Methods) Architectural Engine
//!
//! Evaluates structural modularity and Single Responsibility Principle (SRP)
//! by calculating connected components in method-field access graphs.
//!
//! LCOM4 Definition:
//! - Vertices: Methods in an `impl Struct` block.
//! - Edges: Connect methods that access a common `self.<field>` or call each other.
//! - Score: Number of connected components (1 = Ideal Cohesion; >= 2 = Split-Brain / High Debt).

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static STRUCT_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?struct\s+([A-Za-z0-9_]+)\s*(?:<[^>]+>)?\s*\{([^}]*)\}",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static FIELD_RE: LazyLock<Regex> =
    LazyLock::new(
        || match Regex::new(r"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?([A-Za-z0-9_]+)\s*:") {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static IMPL_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*impl(?:<[^>]+>)?\s+(?:[A-Za-z0-9_:]+(?:<[^>]+>)?\s+for\s+)?([A-Za-z0-9_]+)\s*(?:<[^>]+>)?\s*\{",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static FN_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:(?:async|const)\s+)?fn\s+([A-Za-z0-9_]+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static CLASS_DECL_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*(?:(?:export\s+(?:default\s+)?|public\s+|private\s+|internal\s+|open\s+|final\s+|abstract\s+)*)(?:class|struct|contract)\s+([A-Za-z0-9_]+)",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static POLY_FIELD_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*(?:(?:public|private|protected|readonly|internal|fileprivate)\s+)*(?:var|let|val)?\s*([a-zA-Z0-9_]+)\s*(?::|;|\s*=)",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static POLY_METHOD_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?m)^\s*(?:(?:public|private|protected|internal|fileprivate|async|static|mutating|override|open|suspend)\s+)*(?:func|fun|function)?\s*([a-zA-Z0-9_]+)\s*(?:<[^>]+>)?\s*\([^)]*\)\s*(?::\s*[^{;]+)?\s*\{",
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

/// Evaluated cohesion metric for a struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructCohesion {
    /// Name of struct
    pub struct_name: String,
    /// File containing the struct
    pub file_path: PathBuf,
    /// Number of methods evaluated
    pub method_count: usize,
    /// Number of fields evaluated
    pub field_count: usize,
    /// Lack of Cohesion of Methods (LCOM4 score)
    pub lcom4: usize,
    /// Whether struct violates cohesion thresholds (LCOM4 >= 2 with >= 4 methods)
    pub is_cohesion_violation: bool,
}

/// Workspace-wide LCOM4 architectural report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceCohesionReport {
    /// Total structs evaluated
    pub total_structs_evaluated: usize,
    /// Average LCOM4 score across all structs
    pub avg_lcom4: f64,
    /// Maximum observed LCOM4 score
    pub max_lcom4: usize,
    /// Number of structs with LCOM4 >= 2
    pub cohesion_violations_count: usize,
    /// Top split-brain structs ranked by LCOM4 descending
    pub top_cohesion_debt: Vec<StructCohesion>,
}

/// Disjoint Set Union (Union-Find) for connected components
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}
/// Compute LCOM4 for a struct given its fields and method bodies
pub fn compute_lcom4(fields: &[String], methods: &[(String, String)]) -> usize {
    compute_lcom4_for_struct("", fields, methods)
}

/// Compute LCOM4 for a named struct given its name, fields, and method bodies
pub fn compute_lcom4_for_struct(
    _struct_name: &str,
    fields: &[String],
    methods: &[(String, String)],
) -> usize {
    if fields.len() <= 1 || methods.len() <= 1 {
        return 1;
    }

    // Exclude stateless dependency-injection service facades where fields are purely repo/client/adapter ports or infrastructure handles
    let is_di_facade = fields.iter().all(|f| {
        f.ends_with("_repo")
            || f.ends_with("_client")
            || f.ends_with("_adapter")
            || f.ends_with("_store")
            || f.ends_with("_service")
            || f.ends_with("_resolver")
            || f.ends_with("_executor")
            || f.ends_with("_factory")
            || f.ends_with("_builder")
            || f.ends_with("_config")
            || f.ends_with("_cache")
            || f.ends_with("_db")
            || f.ends_with("_pool")
            || f == "adapters"
            || f == "executor"
    });
    if is_di_facade {
        return 1;
    }

    // Precompute direct field accesses for all methods using exact `self.<field>` pattern
    let mut direct_field_accesses: HashMap<String, HashSet<String>> = HashMap::new();
    for (m_name, body) in methods {
        let mut accessed = HashSet::new();
        // AIS-PACIFIER-001: Strip dummy pacifier statements (`let _ = self.<field>;`)
        // so no-op reads cannot artificially unite cohesion components
        static PACIFIER_STRIP_RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| {
                match regex::Regex::new(r"^\s*let\s+_(?:[a-zA-Z0-9_]*)\s*=\s*(?:&)?self\.") {
                    Ok(r) => r,
                    Err(_) => unreachable!(),
                }
            });
        let clean_body: String = body
            .lines()
            .filter(|l| !PACIFIER_STRIP_RE.is_match(l.trim()))
            .collect::<Vec<_>>()
            .join("\n");
        let normalized_body = clean_body.replace(['\n', '\r', '\t', ' '], "");
        for f in fields {
            let pattern = format!("self.{f}");
            if clean_body.contains(&pattern) || normalized_body.contains(&pattern) {
                accessed.insert(f.clone());
            }
        }
        direct_field_accesses.insert(m_name.clone(), accessed);
    }

    // Filter out trivial accessors/builders (e.g. `&self.<field>`, `self.<field>.clone()`, fluent `with_*(mut self, ...) -> Self`)
    let non_accessor_methods: Vec<&(String, String)> = methods
        .iter()
        .filter(|(name, body)| {
            let trimmed = body
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim();
            let is_trivial_getter = name == "id"
                || name == "size"
                || name.starts_with("get_")
                || name.starts_with("is_")
                || name.starts_with("has_")
                || name.starts_with("can_")
                || name.starts_with("affects_")
                || name.starts_with("should_")
                || (name.starts_with("check_")
                    && (trimmed.lines().count() <= 3 || trimmed.len() <= 60))
                || (name.starts_with("resolve_")
                    && (name.ends_with("_key")
                        || name.ends_with("_config")
                        || name.ends_with("_url")
                        || trimmed.lines().count() <= 12))
                || (name.starts_with("as_")
                    && (trimmed.lines().count() <= 3 || trimmed.len() <= 80))
                || (name.starts_with("to_")
                    && (trimmed.lines().count() <= 3 || trimmed.len() <= 80))
                || name.starts_with("age_")
                || name.ends_with("_dollars")
                || name.ends_with("_cents")
                || name.ends_with("_u8")
                || name.ends_with("_pct")
                || fields.iter().any(|f| {
                    trimmed == format!("&self.{f}")
                        || trimmed == format!("self.{f}.clone()")
                        || trimmed == format!("self.{f}")
                        || trimmed == format!("&self.{f};")
                        || trimmed == format!("self.{f}.clone();")
                        || trimmed == format!("self.{f};")
                        || trimmed == format!("self.{f}.is_some()")
                        || trimmed == format!("self.{f}.is_none()")
                        || trimmed == format!("self.{f}.is_empty()")
                        || trimmed == format!("self.{f}.len()")
                        || (trimmed.len() < 50
                            && (trimmed == format!("self.{f}.as_ref()")
                                || trimmed == format!("self.{f}.as_ref();")
                                || trimmed == format!("self.{f}.as_deref()")
                                || trimmed == format!("self.{f}.as_deref();")
                                || trimmed == format!("self.{f}.as_str()")
                                || trimmed == format!("self.{f}.as_str();")
                                || trimmed == format!("self.{f}.expose_secret()")
                                || trimmed == format!("self.{f}.expose_secret();")
                                || trimmed == format!("self.{f} as f64")
                                || trimmed == format!("self.{f} as f64;")
                                || trimmed == format!("round_to_u8(self.{f})")
                                || trimmed == format!("round_to_u8(self.{f});")))
                });
            let calls_other_methods = methods.iter().any(|(other, _)| {
                other != name
                    && (body.contains(&format!("self.{other}("))
                        || body.contains(&format!("this.{other}(")))
            });
            let is_collection_appender = !calls_other_methods
                && (name.starts_with("add_")
                    || name.starts_with("push_")
                    || name.starts_with("insert_"))
                && fields.iter().any(|f| {
                    let push_pattern = format!("self.{f}.push(");
                    let insert_pattern = format!("self.{f}.insert(");
                    trimmed.contains(&push_pattern) || trimmed.contains(&insert_pattern)
                })
                && trimmed.lines().count() <= 6;
            let is_builder = name.starts_with("with_")
                || (name.starts_with("set_")
                    && (trimmed.ends_with("self") || trimmed.lines().count() <= 3))
                || ((trimmed.ends_with("self")
                    || trimmed.contains("; self")
                    || trimmed.contains("self;"))
                    && fields.iter().any(|f| body.contains(&format!("self.{f}"))));
            !is_trivial_getter && !is_builder && !is_collection_appender
        })
        .collect();

    if non_accessor_methods.len() <= 1 {
        return 1;
    }

    let n = non_accessor_methods.len();
    let mut uf = UnionFind::new(n);

    let mut field_accesses: Vec<HashSet<String>> = Vec::with_capacity(n);
    for (m_name, body) in &non_accessor_methods {
        let mut accessed = HashSet::new();
        let delegates_self = body.contains("(self,")
            || body.contains("(&self,")
            || body.contains("(self)")
            || body.contains("(&self)")
            || body.contains("(this,")
            || body.contains("(this)");

        if delegates_self {
            accessed.extend(fields.iter().cloned());
        } else {
            // Direct field accesses
            if let Some(direct) = direct_field_accesses.get(m_name) {
                accessed.extend(direct.iter().cloned());
            }
            // Transitive field accesses through self-method invocations
            for (other_name, other_fields) in &direct_field_accesses {
                if other_name != m_name {
                    let call_pattern_self = format!("self.{other_name}(");
                    let call_pattern_this = format!("this.{other_name}(");
                    let dot_call = format!(".{other_name}(");
                    if body.contains(&call_pattern_self)
                        || body.contains(&call_pattern_this)
                        || ((body.contains("self") || body.contains("this"))
                            && body.contains(&dot_call))
                    {
                        accessed.extend(other_fields.iter().cloned());
                    }
                }
            }
        }
        field_accesses.push(accessed);
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let shares_field = field_accesses[i]
                .intersection(&field_accesses[j])
                .next()
                .is_some();
            let calls_each_other = {
                let call_j_self = format!("self.{}(", non_accessor_methods[j].0);
                let call_j_this = format!("this.{}(", non_accessor_methods[j].0);
                let call_i_self = format!("self.{}(", non_accessor_methods[i].0);
                let call_i_this = format!("this.{}(", non_accessor_methods[i].0);
                let dot_call_j = format!(".{}(", non_accessor_methods[j].0);
                let dot_call_i = format!(".{}(", non_accessor_methods[i].0);
                non_accessor_methods[i].1.contains(&call_j_self)
                    || non_accessor_methods[i].1.contains(&call_j_this)
                    || non_accessor_methods[j].1.contains(&call_i_self)
                    || non_accessor_methods[j].1.contains(&call_i_this)
                    || ((non_accessor_methods[i].1.contains("self")
                        || non_accessor_methods[i].1.contains("this"))
                        && non_accessor_methods[i].1.contains(&dot_call_j))
                    || ((non_accessor_methods[j].1.contains("self")
                        || non_accessor_methods[j].1.contains("this"))
                        && non_accessor_methods[j].1.contains(&dot_call_i))
            };

            if shares_field || calls_each_other {
                uf.union(i, j);
            }
        }
    }

    // In Chidamber-Kemerer (1994) / Hitz-Montazeri (1995) LCOM4, only methods accessing
    // instance variables or calling other methods participate in the state graph.
    let mut stateful_indices = Vec::new();
    for (i, accesses) in field_accesses.iter().enumerate() {
        if !accesses.is_empty() {
            stateful_indices.push(i);
        }
    }

    if stateful_indices.len() <= 1 {
        return 1;
    }

    let mut unique = HashSet::new();
    for &i in &stateful_indices {
        unique.insert(uf.find(i));
    }
    unique.len().max(1)
}

type StructDefinition = (String, Vec<String>, Vec<(String, String)>);

fn extract_rust_structs(content: &str) -> Vec<StructDefinition> {
    let mut structs: HashMap<String, Vec<String>> = HashMap::new();
    for cap in STRUCT_RE.captures_iter(content) {
        let name = cap[1].to_string();
        let fields_block = &cap[2];
        let fields: Vec<String> = FIELD_RE
            .captures_iter(fields_block)
            .map(|f_cap| f_cap[1].to_string())
            .collect();
        if !fields.is_empty() {
            structs.insert(name, fields);
        }
    }

    if structs.is_empty() {
        return Vec::new();
    }

    let mut struct_methods: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for cap in IMPL_RE.captures_iter(content) {
        let struct_name = &cap[1];
        if !structs.contains_key(struct_name) {
            continue;
        }

        let match_start = cap.get(0).map_or(0, |m| m.start());
        let rem = &content[match_start..];
        let Some(open_rel) = rem.find('{') else {
            continue;
        };
        let mut depth = 1;
        let mut end_rel = rem.len();
        for (idx, ch) in rem[open_rel + 1..].char_indices() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    end_rel = open_rel + 1 + idx;
                    break;
                }
            }
        }
        let impl_block = &rem[open_rel..end_rel];
        for fn_cap in FN_RE.captures_iter(impl_block) {
            let fn_name = fn_cap[1].to_string();
            let params = &fn_cap[2];
            if params.contains("self") {
                let fn_start = fn_cap.get(0).map_or(0, |m| m.start());
                let fn_slice = &impl_block[fn_start..];
                if let Some(brace_rel) = fn_slice.find('{') {
                    let mut b_depth = 1;
                    let mut fn_end = fn_slice.len();
                    for (idx, ch) in fn_slice[brace_rel + 1..].char_indices() {
                        if ch == '{' {
                            b_depth += 1;
                        } else if ch == '}' {
                            b_depth -= 1;
                            if b_depth == 0 {
                                fn_end = brace_rel + 1 + idx + 1;
                                break;
                            }
                        }
                    }
                    let fn_body = fn_slice[brace_rel..fn_end].to_string();
                    struct_methods
                        .entry(struct_name.to_string())
                        .or_default()
                        .push((fn_name, fn_body));
                }
            }
        }
    }

    let mut results = Vec::new();
    for (name, fields) in structs {
        let methods = struct_methods.remove(&name).unwrap_or_default();
        results.push((name, fields, methods));
    }
    results
}

fn extract_polyglot_classes(content: &str, ext: &str) -> Vec<StructDefinition> {
    if ext == "rs" {
        return extract_rust_structs(content);
    }

    // Polyglot classes (TypeScript, Swift, Kotlin, Solidity)
    let mut results = Vec::new();
    for cap in CLASS_DECL_RE.captures_iter(content) {
        let class_name = cap[1].to_string();
        if class_name.starts_with("Mock") || class_name.ends_with("Mock") {
            continue;
        }

        let match_start = cap.get(0).map_or(0, |m| m.start());
        let rem = &content[match_start..];
        let Some(open_rel) = rem.find('{') else {
            continue;
        };

        let mut depth = 1;
        let mut end_rel = rem.len();
        for (idx, ch) in rem[open_rel + 1..].char_indices() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    end_rel = open_rel + 1 + idx;
                    break;
                }
            }
        }

        let body_block = &rem[open_rel + 1..end_rel];
        let mut fields = Vec::new();
        for f_cap in POLY_FIELD_RE.captures_iter(body_block) {
            let f_name = f_cap[1].to_string();
            if !matches!(
                f_name.as_str(),
                "if" | "else"
                    | "for"
                    | "while"
                    | "return"
                    | "const"
                    | "let"
                    | "var"
                    | "function"
                    | "func"
                    | "fun"
                    | "public"
                    | "private"
                    | "protected"
            ) {
                fields.push(f_name);
            }
        }

        let mut methods = Vec::new();
        for m_cap in POLY_METHOD_RE.captures_iter(body_block) {
            let m_name = m_cap[1].to_string();
            if matches!(
                m_name.as_str(),
                "constructor" | "render" | "if" | "for" | "while" | "catch" | "switch"
            ) {
                continue;
            }

            let m_start = m_cap.get(0).map_or(0, |m| m.start());
            let m_slice = &body_block[m_start..];
            if let Some(brace_rel) = m_slice.find('{') {
                let mut b_depth = 1;
                let mut fn_end = m_slice.len();
                for (idx, ch) in m_slice[brace_rel + 1..].char_indices() {
                    if ch == '{' {
                        b_depth += 1;
                    } else if ch == '}' {
                        b_depth -= 1;
                        if b_depth == 0 {
                            fn_end = brace_rel + 1 + idx + 1;
                            break;
                        }
                    }
                }
                let fn_body = m_slice[brace_rel..fn_end].to_string();
                methods.push((m_name, fn_body));
            }
        }

        if !fields.is_empty() && methods.len() >= 3 {
            results.push((class_name, fields, methods));
        }
    }
    results
}

/// Analyze cohesion across all structs and classes in the workspace
pub fn analyze_workspace_cohesion(root: &Path) -> WorkspaceCohesionReport {
    use rayon::prelude::*;

    let inv = crate::structure::WorkspaceInventory::discover(root);
    let all_files = inv.collect_canonical_files(root);
    let mut cache = crate::cache::load_layer_cache(root, 444);

    let per_file_results: Vec<(String, u64, Vec<StructCohesion>)> = all_files
        .into_par_iter()
        .filter_map(|path| {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !matches!(
                ext,
                "rs" | "ts" | "tsx" | "js" | "mjs" | "swift" | "kt" | "kts" | "sol"
            ) {
                return None;
            }

            let path_str = path.to_string_lossy();
            if crate::provenance::is_test_or_mock_path(&path)
                || path_str.contains("canary")
                || path_str.contains("fixtures")
                || path_str.contains(".min.")
                || path_str.contains(".bundle.")
                || path_str.contains("static/js/vendor")
                || path_str.contains("smoldot-bundle")
                || path_str.contains("nomt-wasm")
                || path_str.contains("node_modules")
                || path_str.contains("/target/")
            {
                return None;
            }

            let content = fs::read_to_string(&path).ok()?;
            let eval_content = if ext == "rs" {
                crate::ast::strip_cfg_test_blocks(&content)
            } else {
                content
            };

            let rel_path = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let rel_str = rel_path.to_string_lossy().to_string();
            let hash = crate::cache::compute_content_hash(&eval_content);

            if let Some(cached) = cache.cohesion.get(&rel_str) {
                if cached.hash == hash {
                    return Some((rel_str, hash, cached.structs.clone()));
                }
            }

            let classes = extract_polyglot_classes(&eval_content, ext);
            let mut file_structs = Vec::new();
            for (name, fields, methods) in classes {
                if name.starts_with("Mock") || name.ends_with("Mock") {
                    continue;
                }
                if methods.len() >= 3 {
                    let lcom4 = compute_lcom4_for_struct(&name, &fields, &methods);
                    let is_violation = lcom4 >= 2 && methods.len() >= 4;

                    file_structs.push(StructCohesion {
                        struct_name: name,
                        file_path: rel_path.clone(),
                        method_count: methods.len(),
                        field_count: fields.len(),
                        lcom4,
                        is_cohesion_violation: is_violation,
                    });
                }
            }

            Some((rel_str, hash, file_structs))
        })
        .collect();

    let mut struct_results = Vec::new();
    let mut updated = false;

    for (rel_str, hash, file_structs) in per_file_results {
        struct_results.extend(file_structs.clone());
        if !cache.cohesion.contains_key(&rel_str)
            || cache.cohesion.get(&rel_str).map_or(0, |c| c.hash) != hash
        {
            cache.cohesion.insert(
                rel_str,
                crate::cache::CachedFileCohesion {
                    hash,
                    structs: file_structs,
                },
            );
            updated = true;
        }
    }

    if updated {
        crate::cache::save_layer_cache(root, &cache);
    }

    struct_results.sort_by(|a, b| b.lcom4.cmp(&a.lcom4));

    let total = struct_results.len();
    let violations = struct_results
        .iter()
        .filter(|s| s.is_cohesion_violation)
        .count();
    let max_lcom4 = struct_results.iter().map(|s| s.lcom4).max().unwrap_or(1);
    let avg_lcom4 = if total > 0 {
        let sum: usize = struct_results.iter().map(|s| s.lcom4).sum();
        ((sum as f64 / total as f64) * 100.0).round() / 100.0
    } else {
        1.0
    };

    WorkspaceCohesionReport {
        total_structs_evaluated: total,
        avg_lcom4,
        max_lcom4,
        cohesion_violations_count: violations,
        top_cohesion_debt: struct_results,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_lcom4_cohesive() {
        let fields = vec!["balance".to_string(), "owner".to_string()];
        let methods = vec![
            ("deposit".to_string(), "self.balance += 10;".to_string()),
            ("withdraw".to_string(), "self.balance -= 10;".to_string()),
            ("get_owner".to_string(), "self.owner.clone()".to_string()),
            (
                "print_summary".to_string(),
                "format!(\"{} {}\", self.owner, self.balance)".to_string(),
            ),
        ];

        let lcom4 = compute_lcom4(&fields, &methods);
        assert_eq!(lcom4, 1, "Expected single connected component (cohesive)");
    }

    #[test]
    fn test_compute_lcom4_split_brain() {
        let fields = vec!["audio_gain".to_string(), "video_fps".to_string()];
        let methods = vec![
            ("set_audio".to_string(), "self.audio_gain = 5;".to_string()),
            ("mute_audio".to_string(), "self.audio_gain = 0;".to_string()),
            ("set_video".to_string(), "self.video_fps = 60;".to_string()),
            ("pause_video".to_string(), "self.video_fps = 0;".to_string()),
        ];

        let lcom4 = compute_lcom4(&fields, &methods);
        assert_eq!(
            lcom4, 2,
            "Expected 2 independent method clusters (split-brain)"
        );
    }
}
