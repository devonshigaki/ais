//! Autonomous Opportunity & Architectural Consolidation Engine (Native Rust)
//!
//! Provides the mathematical mirror to the Issue Scanning Engine:
//! - OPP-ARCH-CLONE: Near-duplicate subroutine clusters mined via MinHash LSH
//! - OPP-DB-WRITER: Multi-writer database persistence consolidation
//! - OPP-COHESION-SPLIT: Split-brain struct decomposition recipes (LCOM4 >= 2)
//! - OPP-ASYNC-PURE: Synchronous subroutines unnecessarily wrapped in async Futures
//! - OPP-PRIM-FRAGMENT: Fragmented cross-cutting primitives (HMAC, retries, crypto)
//! - OPP-MONOLITH-BLOB: Inline HTML/SQL/CSS blobs embedded in Rust source files
//! - OPP-UX-HYPERMEDIA: Askama/HTMX partial swaps & edge cache headers

use crate::cohesion::analyze_workspace_cohesion;
use crate::dedup::FN_DECL_RE;
use crate::structure::WorkspaceInventory;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OpportunityCategory {
    ArchitectureConsolidation,
    DatabasePersistence,
    CohesionDecomposition,
    AsyncPerformance,
    PrimitiveHarmonization,
    MonolithDecomposition,
    HypermediaUx,
    ProactiveComplexity,
    SchemaHarmonization,
    DeadConfigSink,
    StateLifecycleAsymmetry,
    StyleCanonConsolidation,
    CiArtifactHygiene,
    ComputeEfficiency,
    TransactionSafety,
}

impl OpportunityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ArchitectureConsolidation => "Architecture Consolidation",
            Self::DatabasePersistence => "Database Persistence",
            Self::CohesionDecomposition => "Cohesion Decomposition",
            Self::AsyncPerformance => "Async Performance",
            Self::PrimitiveHarmonization => "Primitive Harmonization",
            Self::MonolithDecomposition => "Monolith Decomposition",
            Self::HypermediaUx => "Hypermedia UX Acceleration",
            Self::ProactiveComplexity => "Proactive Complexity Decomposition",
            Self::SchemaHarmonization => "Schema Harmonization",
            Self::DeadConfigSink => "Dead-End Config Sink",
            Self::StateLifecycleAsymmetry => "State Lifecycle Asymmetry",
            Self::StyleCanonConsolidation => "Style Canon Consolidation",
            Self::CiArtifactHygiene => "CI/CD Artifact Hygiene",
            Self::ComputeEfficiency => "Compute Efficiency",
            Self::TransactionSafety => "Transaction Safety",
        }
    }
}

pub const ALL_OPPORTUNITY_CODES: &[&str] = &[
    "OPP-ARCH-CLONE",
    "OPP-ASYNC-PURE",
    "OPP-ASYNC-WATERFALL",
    "OPP-CI-ARTIFACT",
    "OPP-COHESION-SPLIT",
    "OPP-COMPUTE-PUSHDOWN",
    "OPP-DB-SINGLE-WRITER",
    "OPP-DEAD-CONFIG-SINK",
    "OPP-HANDLER-CLUSTER",
    "OPP-HTMX-PARTIAL",
    "OPP-MONOLITH-BLOB",
    "OPP-PERF-CACHE",
    "OPP-PRIM-FRAGMENT",
    "OPP-PROACTIVE-DECOMPOSE",
    "OPP-SCHEMA-DEDUP",
    "OPP-SQL-BATCH-CHUNK",
    "OPP-STATE-LIFECYCLE",
    "OPP-STYLE-CANON",
    "OPP-TX-SAVEPOINT-SYMMETRY",
    "OPP-UPSERT-LATTICE",
    "OPP-UX-MICROCOPY",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityRoi {
    pub engineering_hours_saved: f64,
    pub loc_reduction_estimate: usize,
    pub blast_radius: usize,
    pub roce_score: f64,
    pub priority_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunitySignal {
    pub signal_code: String,
    pub category: OpportunityCategory,
    pub title: String,
    pub file: PathBuf,
    pub line: usize,
    pub target_locations: Vec<PathBuf>,
    pub potential_gain: String,
    pub deterministic_recipe: String,
    pub roi: OpportunityRoi,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpportunityScanReport {
    pub total_opportunities_discovered: usize,
    pub category_breakdown: BTreeMap<String, usize>,
    pub total_projected_hours_saved: f64,
    pub total_projected_loc_reduction: usize,
    pub top_opportunities: Vec<OpportunitySignal>,
}
#[derive(Debug, Clone)]
pub struct IndexedSourceFile {
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub extension: String,
    pub clean_content: String,
}

pub type IndexedRustFile = IndexedSourceFile;

#[derive(Debug, Clone)]
pub struct WorkspaceSourceIndex {
    pub root: PathBuf,
    pub inventory: WorkspaceInventory,
    pub rust_files: Vec<IndexedSourceFile>,
    pub polyglot_files: Vec<IndexedSourceFile>,
}

impl WorkspaceSourceIndex {
    pub fn build(root: &Path) -> Self {
        let inventory = WorkspaceInventory::discover(root);
        let canonical_files = inventory.collect_canonical_files(root);

        use rayon::prelude::*;
        let indexed_files: Vec<IndexedSourceFile> = canonical_files
            .par_iter()
            .filter_map(|p| {
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                if !matches!(
                    ext,
                    "rs" | "ts" | "tsx" | "js" | "mjs" | "swift" | "kt" | "kts" | "sol" | "html"
                ) {
                    return None;
                }

                let path_str = p.to_string_lossy();
                if crate::provenance::is_test_or_mock_path(p)
                    || path_str.contains("canary")
                    || path_str.contains("fixtures")
                    || path_str.contains("/benches/")
                    || path_str.contains(".min.")
                    || path_str.contains(".bundle.")
                    || path_str.contains("/vendor/")
                    || path_str.contains("/patches/")
                    || path_str.contains("docker/patches")
                    || path_str.contains("compliance/rules")
                    || path_str.contains("smoldot-bundle")
                    || path_str.contains("nomt-wasm")
                    || path_str.contains("node_modules")
                    || path_str.contains("/target/")
                {
                    return None;
                }

                let content = fs::read_to_string(p).ok()?;
                let clean_content = if ext == "rs" {
                    crate::ast::strip_cfg_test_blocks(&content)
                } else {
                    content
                };
                let rel_path = p.strip_prefix(root).unwrap_or(p).to_path_buf();
                Some(IndexedSourceFile {
                    path: p.clone(),
                    rel_path,
                    extension: ext.to_string(),
                    clean_content,
                })
            })
            .collect();

        let mut rust_files = Vec::new();
        let mut polyglot_files = Vec::new();
        for f in indexed_files {
            if f.extension == "rs" {
                rust_files.push(f.clone());
            }
            polyglot_files.push(f);
        }

        Self {
            root: root.to_path_buf(),
            inventory,
            rust_files,
            polyglot_files,
        }
    }
}

static ASYNC_FN_DECL_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r#"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?async\s+fn\s+([a-zA-Z0-9_]+)\s*\("#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

#[allow(dead_code)]
static IMPL_FOR_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r#"(?m)^\s*impl(?:<[^>]+>)?\s+[A-Za-z0-9_:]+(?:<[^>]+>)?\s+for\s+"#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static DML_WRITE_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r#"(?i)\b(?:INSERT\s+INTO\s+([a-zA-Z0-9_]+)\s*(?:\(|\bVALUES\b|\bSELECT\b)|UPDATE\s+([a-zA-Z0-9_]+)\s+SET\b|DELETE\s+FROM\s+([a-zA-Z0-9_]+)\b)"#,
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static HMAC_INIT_RE: LazyLock<Regex> =
    LazyLock::new(
        || match Regex::new(r#"(?:Hmac[A-Za-z0-9_:<>\s]*|Mac)::new_from_slice"#) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        },
    );

static GENERIC_BTN_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r#"<button[^>]*>\s*(?:Submit|Save|Click Here|Continue)\s*</button>"#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

static FULL_TAB_NAV_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r#"<a\s+[^>]*href="(/consumer/[^"]+)"[^>]*class="[^"]*tab[^"]*""#) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

fn is_axum_router_target(content: &str, fn_name: &str) -> bool {
    let verbs = ["get", "post", "put", "delete", "patch"];
    verbs.iter().any(|verb| {
        let pattern = format!("{verb}({fn_name})");
        content.contains(&pattern)
    })
}

/// 1. Detect async fn definitions that never call .await (zero-await async overhead)
pub fn scan_async_overhead_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || path_str.contains("canary")
            || path_str.contains("mock")
            || path_str.contains("ais-core/src/opportunities.rs")
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        let mut in_trait_block = false;
        let mut trait_brace_depth = 0i32;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }

            let is_trait_or_impl_for = if trimmed.starts_with("impl") {
                // Look ahead up to 5 lines for ' for ' and '{' to catch multi-line impl Trait for Type
                let mut decl = String::new();
                for l in lines.iter().skip(idx).take(5) {
                    decl.push_str(l.trim());
                    decl.push(' ');
                    if l.contains('{') || l.contains(';') {
                        break;
                    }
                }
                decl.contains(" for ")
            } else {
                trimmed.starts_with("pub trait ")
                    || trimmed.starts_with("pub(crate) trait ")
                    || trimmed.starts_with("pub(super) trait ")
                    || trimmed.starts_with("trait ")
            };

            if is_trait_or_impl_for {
                in_trait_block = true;
                trait_brace_depth = 0;
            }

            if in_trait_block {
                for ch in trimmed.chars() {
                    if ch == '{' {
                        trait_brace_depth += 1;
                    } else if ch == '}' {
                        trait_brace_depth -= 1;
                    }
                }
                if trait_brace_depth <= 0 && trimmed.contains('}') {
                    in_trait_block = false;
                }
            }

            if in_trait_block {
                continue;
            }

            if let Some(cap) = ASYNC_FN_DECL_RE.captures(line) {
                let fn_name = cap.get(1).map_or("", |m| m.as_str());

                // Gather full multi-line signature header from 'async fn' to '{'
                let mut sig_header = String::new();
                let mut open_brace_idx = idx;
                let mut has_body = false;
                for (s_idx, s_line) in lines.iter().enumerate().skip(idx) {
                    sig_header.push_str(s_line);
                    sig_header.push(' ');
                    if s_line.contains('{') {
                        open_brace_idx = s_idx;
                        has_body = true;
                        break;
                    }
                    if s_line.contains(';') {
                        break;
                    }
                }

                if !has_body {
                    continue;
                }

                if fn_name.is_empty()
                    || fn_name.ends_with("_page")
                    || fn_name.ends_with("_handler")
                    || fn_name.ends_with("_action")
                    || fn_name.ends_with("_callback")
                    || fn_name.ends_with("_start")
                    || fn_name.ends_with("_routes")
                    || fn_name.ends_with("_router")
                    || fn_name.starts_with("test_")
                    || sig_header.contains("IntoResponse")
                    || sig_header.contains("Response")
                    || sig_header.contains("StatusCode")
                    || sig_header.contains("Html<")
                    || sig_header.contains("Json<")
                    || sig_header.contains("Redirect")
                    || sig_header.contains("ApiResult<")
                    || sig_header.contains("Result<Json<")
                    || sig_header.contains("State(")
                    || sig_header.contains("Extension(")
                    || sig_header.contains("Path(")
                    || sig_header.contains("Query(")
                    || sig_header.contains("Json(")
                    || sig_header.contains("Form(")
                    || sig_header.contains("HeaderMap")
                    || sig_header.contains("AuthExtractor")
                    || sig_header.contains("RoleExtractor")
                    || sig_header.contains("ConnectInfo")
                    || sig_header.contains("BoxFuture")
                    || sig_header.contains("Pin<Box<")
                    || sig_header.contains("CallToolResult")
                    || sig_header.contains("RequestContext<")
                    || sig_header.contains("Parameters(")
                    || sig_header.contains("McpError")
                    || is_axum_router_target(&file.clean_content, fn_name)
                {
                    continue;
                }

                let mut brace_depth = 0;
                let mut started = false;
                let mut body_lines = Vec::new();
                for curr_line in lines.iter().skip(open_brace_idx) {
                    for ch in curr_line.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                            started = true;
                        } else if ch == '}' {
                            brace_depth -= 1;
                        }
                    }
                    if started {
                        body_lines.push(*curr_line);
                        if brace_depth == 0 {
                            break;
                        }
                    }
                }

                let body_text = body_lines.join("\n");
                if body_lines.len() >= 6
                    && !body_text.contains(".await")
                    && !body_text.contains("yield")
                    && !body_text.contains("select!")
                    && !body_text.contains("join!")
                    && !body_text.contains("try_join!")
                    && !body_text.contains("spawn(")
                    && !is_axum_router_target(&file.clean_content, fn_name)
                {
                    let target = file.rel_path.clone();
                    signals.push(OpportunitySignal {
                        signal_code: "OPP-ASYNC-PURE".to_string(),
                        category: OpportunityCategory::AsyncPerformance,
                        title: format!(
                            "Convert Zero-Await async fn '{fn_name}' to Synchronous Subroutine"
                        ),
                        file: target.clone(),
                        line: idx + 1,
                        target_locations: vec![target],
                        potential_gain: "Eliminates Future state-machine allocation and Tokio poll scheduling overhead"
                            .to_string(),
                        deterministic_recipe: format!(
                            "Remove 'async' keyword from 'fn {fn_name}' and remove caller '.await' tokens"
                        ),
                        roi: OpportunityRoi {
                            engineering_hours_saved: 1.5,
                            loc_reduction_estimate: 2,
                            blast_radius: 1,
                            roce_score: 75.0,
                            priority_tier: "Quick Win (Fix First)".to_string(),
                        },
                    });
                }
            }
        }
    }

    signals
}

fn get_owning_service_domain(rel_path: &Path, _table: &str) -> PathBuf {
    let p = rel_path.to_string_lossy();

    if p.starts_with("apps/product/")
        || p.starts_with("crates/adapters/")
        || p.starts_with("crates/db/libsql/staging/")
        || p.starts_with("crates/agent/")
        || p.starts_with("crates/engine-ports/")
        || p.starts_with("crates/workers/scheduler/")
    {
        return PathBuf::from("freshcredit-product");
    }
    if p.starts_with("apps/service/crm/") {
        return PathBuf::from("service-crm");
    }
    if p.starts_with("apps/service/cal/") {
        return PathBuf::from("service-cal");
    }
    if p.starts_with("crates/sim-") {
        return PathBuf::from("sim-engine");
    }
    if let Some(pos) = p.find("src/") {
        let prefix = &p[..pos + 3];
        return PathBuf::from(prefix);
    }
    rel_path.parent().unwrap_or(rel_path).to_path_buf()
}

/// 2. Detect multi-writer persistence scattered across multiple services
pub fn scan_multi_writer_persistence_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let mut table_writers: BTreeMap<String, BTreeMap<PathBuf, BTreeSet<PathBuf>>> = BTreeMap::new();

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if path_str.contains("/bin/")
            || path_str.contains("migrations")
            || path_str.contains("fixtures")
            || path_str.contains("tests")
            || path_str.contains("xtask")
            || path_str.contains("schema_manager")
            || path_str.contains("services/account_deletion.rs")
            || path_str.ends_with("/render.rs")
        {
            continue;
        }

        for line in file.clean_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            let line_upper = trimmed.to_uppercase();
            if !line_upper.contains("INSERT INTO")
                && !line_upper.contains("UPDATE ")
                && !line_upper.contains("DELETE FROM")
            {
                continue;
            }
            if let Some(cap) = DML_WRITE_RE.captures(trimmed) {
                let tbl = cap.get(1).or_else(|| cap.get(2)).or_else(|| cap.get(3));
                if let Some(tbl) = tbl {
                    let tbl_name = tbl.as_str().to_lowercase();
                    if tbl_name != "values" && tbl_name != "set" && tbl_name != "where" {
                        let domain = get_owning_service_domain(&file.rel_path, &tbl_name);
                        table_writers
                            .entry(tbl_name)
                            .or_default()
                            .entry(domain)
                            .or_default()
                            .insert(file.rel_path.clone());
                    }
                }
            }
        }
    }

    for (table, domain_map) in table_writers {
        if domain_map.len() >= 2 {
            let all_locs: Vec<PathBuf> = domain_map.values().flatten().cloned().collect();
            let primary = all_locs[0].clone();
            let count = domain_map.len();

            signals.push(OpportunitySignal {
                signal_code: "OPP-DB-SINGLE-WRITER".to_string(),
                category: OpportunityCategory::DatabasePersistence,
                title: format!(
                    "Consolidate Multi-Writer Table '{table}' to Canonical Repository Port"
                ),
                file: primary,
                line: 1,
                target_locations: all_locs,
                potential_gain: format!(
                    "Prevents split-brain database states and simplifies DML schema evolutions across {count} files"
                ),
                deterministic_recipe: format!(
                    "Route all DML mutations for table '{table}' through a single canonical repository service"
                ),
                roi: OpportunityRoi {
                    engineering_hours_saved: (count as f64) * 2.0,
                    loc_reduction_estimate: count * 15,
                    blast_radius: count,
                    roce_score: 85.0,
                    priority_tier: "Strategic Batch (Sequenced)".to_string(),
                },
            });
        }
    }

    signals
}

/// 3. Detect split-brain structs and output exact LCOM4 decomposition recipes
pub fn scan_cohesion_decomposition_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let report = analyze_workspace_cohesion(&index.root);

    for s in report.top_cohesion_debt {
        if s.lcom4 >= 2 && s.method_count >= 3 {
            let target = s.file_path.clone();
            signals.push(OpportunitySignal {
                signal_code: "OPP-COHESION-SPLIT".to_string(),
                category: OpportunityCategory::CohesionDecomposition,
                title: format!("Decompose Split-Brain Struct '{}' (LCOM4 = {})", s.struct_name, s.lcom4),
                file: target.clone(),
                line: 1,
                target_locations: vec![target],
                potential_gain: format!("Reduces method coupling across {} distinct sub-responsibilities and raises Maintainability Index", s.lcom4),
                deterministic_recipe: format!("Partition '{}' into {} focused structs with independent field-access graphs per Single Responsibility Principle", s.struct_name, s.lcom4),
                roi: OpportunityRoi {
                    engineering_hours_saved: 4.0,
                    loc_reduction_estimate: 25,
                    blast_radius: 2,
                    roce_score: 90.0,
                    priority_tier: "Strategic Batch (Sequenced)".to_string(),
                },
            });
        }
    }

    signals
}

/// 4. Detect semantic clone clusters across subroutines via MinHash LSH and Union-Find clique collapse
pub fn scan_clone_consolidation_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let mut fingerprints = Vec::new();

    for file in &index.polyglot_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("scripts/")
            || p_str.contains("fixtures")
            || p_str.contains("canary")
            || p_str.contains("mock")
            || p_str.contains("libsql-browser")
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = FN_DECL_RE.captures(line) {
                if let Some(fn_match) = cap.get(1) {
                    let fn_name = fn_match.as_str().to_string();
                    if fn_name.starts_with("test_")
                        || fn_name.ends_with("_routes")
                        || fn_name.ends_with("_router")
                    {
                        continue;
                    }

                    // Brace-balanced extraction of function body
                    let mut brace_depth = 0;
                    let mut started = false;
                    let mut end_idx = idx;
                    let mut hit_semicolon_before_brace = false;

                    for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                        for ch in curr_line.chars() {
                            if ch == ';' && !started {
                                hit_semicolon_before_brace = true;
                                break;
                            }
                            if ch == '{' {
                                brace_depth += 1;
                                started = true;
                            } else if ch == '}' {
                                brace_depth -= 1;
                            }
                        }
                        if hit_semicolon_before_brace {
                            break;
                        }
                        if started && brace_depth == 0 {
                            end_idx = curr_idx + 1;
                            break;
                        }
                    }

                    if hit_semicolon_before_brace || !started || end_idx <= idx {
                        continue;
                    }

                    let fn_code = lines[idx..end_idx].join("\n");
                    let fn_loc = fn_code.lines().filter(|l| !l.trim().is_empty()).count();
                    if fn_loc >= 25 && fn_code.len() >= 400 {
                        fingerprints.push(crate::dedup::fingerprint_code_snippet(
                            file.rel_path.clone(),
                            fn_name,
                            idx + 1,
                            &fn_code,
                        ));
                    }
                }
            }
        }
    }

    let duplicates = crate::dedup::detect_lsh_duplicate_clusters(&fingerprints, 95.0);

    let trivial_names = [
        "default",
        "fmt",
        "new",
        "clone",
        "into",
        "from",
        "as_ref",
        "as_str",
        "from_row",
        "from_pending_row",
        "from_create_request",
    ];

    // Filter out trivial std trait accessors, port-adapter correspondences, and require distinct files for cross-module clone detection
    let non_trivial_dups: Vec<_> = duplicates
        .into_iter()
        .filter(|dup| {
            if trivial_names.contains(&dup.function_a.as_str())
                || trivial_names.contains(&dup.function_b.as_str())
                || dup.function_a.starts_with("init_")
                || dup.function_b.starts_with("init_")
                || dup.file_a == dup.file_b
            {
                return false;
            }

            let path_a = dup.file_a.to_string_lossy();
            let path_b = dup.file_b.to_string_lossy();

            // Port-adapter correspondence: database persistence adapter vs. network HTTP/RPC client adapter
            let a_is_db = path_a.contains("/db/");
            let b_is_db = path_b.contains("/db/");
            let a_is_client = path_a.contains("/client") || path_a.contains("/http");
            let b_is_client = path_b.contains("/client") || path_b.contains("/http");
            if (a_is_db && b_is_client) || (b_is_db && a_is_client) {
                return false;
            }

            // Distinct UI section components with different names are independent sections sharing CSS layout tokens
            let a_is_section =
                dup.function_a.ends_with("Section") || dup.function_a.ends_with("_section");
            let b_is_section =
                dup.function_b.ends_with("Section") || dup.function_b.ends_with("_section");
            if a_is_section && b_is_section && dup.function_a != dup.function_b {
                return false;
            }

            // Provider-specific data staging repositories insert into distinct staging tables
            if path_a.contains("staging/")
                && path_b.contains("staging/")
                && dup.function_a.starts_with("stage_")
                && dup.function_b.starts_with("stage_")
            {
                return false;
            }

            // Route endpoints, page entry points, layout roots, and staging completion handlers are distinct HTTP boundaries
            let a_is_route_endpoint = dup.function_a.ends_with("_page")
                || dup.function_a.ends_with("Page")
                || dup.function_a.ends_with("_callback")
                || dup.function_a.ends_with("_handler")
                || dup.function_a.ends_with("_start")
                || dup.function_a.ends_with("_staging")
                || dup.function_a.ends_with("Layout")
                || dup.function_a.ends_with("_layout");
            let b_is_route_endpoint = dup.function_b.ends_with("_page")
                || dup.function_b.ends_with("Page")
                || dup.function_b.ends_with("_callback")
                || dup.function_b.ends_with("_handler")
                || dup.function_b.ends_with("_start")
                || dup.function_b.ends_with("_staging")
                || dup.function_b.ends_with("Layout")
                || dup.function_b.ends_with("_layout");
            if a_is_route_endpoint && b_is_route_endpoint {
                return false;
            }

            true
        })
        .collect();

    let mut fn_ids: BTreeMap<(&Path, &str, usize), usize> = BTreeMap::new();
    let mut id_to_fn: Vec<(&Path, &str, usize)> = Vec::new();

    for dup in &non_trivial_dups {
        let key_a = (dup.file_a.as_path(), dup.function_a.as_str(), dup.line_a);
        let id_a = *fn_ids.entry(key_a).or_insert_with(|| {
            let id = id_to_fn.len();
            id_to_fn.push(key_a);
            id
        });

        let key_b = (dup.file_b.as_path(), dup.function_b.as_str(), dup.line_b);
        let id_b = *fn_ids.entry(key_b).or_insert_with(|| {
            let id = id_to_fn.len();
            id_to_fn.push(key_b);
            id
        });

        let _ = (id_a, id_b);
    }

    let mut fn_loc_map: BTreeMap<(&Path, &str, usize), usize> = BTreeMap::new();
    for fp in &fingerprints {
        fn_loc_map.insert((fp.file.as_path(), fp.name.as_str(), fp.line), fp.loc);
    }

    struct Dsu {
        parent: Vec<usize>,
    }
    impl Dsu {
        fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
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
            let ri = self.find(i);
            let rj = self.find(j);
            if ri != rj {
                if ri < rj {
                    self.parent[rj] = ri;
                } else {
                    self.parent[ri] = rj;
                }
            }
        }
    }

    let mut dsu = Dsu::new(id_to_fn.len());
    for dup in &non_trivial_dups {
        let key_a = (dup.file_a.as_path(), dup.function_a.as_str(), dup.line_a);
        let key_b = (dup.file_b.as_path(), dup.function_b.as_str(), dup.line_b);
        if let (Some(&id_a), Some(&id_b)) = (fn_ids.get(&key_a), fn_ids.get(&key_b)) {
            dsu.union(id_a, id_b);
        }
    }

    // Group functions into disjoint equivalence classes
    let mut clusters: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for idx in 0..id_to_fn.len() {
        let root_idx = dsu.find(idx);
        clusters.entry(root_idx).or_default().push(idx);
    }

    for (_root_idx, mut members) in clusters {
        if members.len() >= 2 {
            members.sort_by(|&a, &b| id_to_fn[a].cmp(&id_to_fn[b]));
            let lead = id_to_fn[members[0]];
            let member_locs: Vec<PathBuf> = members
                .iter()
                .map(|&idx| id_to_fn[idx].0.to_path_buf())
                .collect();
            let k = member_locs.len();

            // Exact LOC reduction from individual fingerprints
            let est_loc_reduction: usize = members
                .iter()
                .skip(1)
                .map(|&idx| fn_loc_map.get(&id_to_fn[idx]).copied().unwrap_or(25))
                .sum();
            let hours_saved = (k as f64) * 1.5;

            signals.push(OpportunitySignal {
                signal_code: "OPP-ARCH-CLONE".to_string(),
                category: OpportunityCategory::ArchitectureConsolidation,
                title: format!(
                    "Consolidate {k}-Site Subroutine Equivalence Cluster around '{}'",
                    lead.1
                ),
                file: lead.0.to_path_buf(),
                line: lead.2,
                target_locations: member_locs,
                potential_gain: format!(
                    "Consolidates {k} duplicate sites into 1 canonical helper, eliminating logic drift and saving ~{est_loc_reduction} LOC"
                ),
                deterministic_recipe: "Extract canonical subroutine into shared helper module (ensure <= 60 LOC per POT-004; if target module > 700 lines, extract to dedicated submodule per POT-011)".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: hours_saved,
                    loc_reduction_estimate: est_loc_reduction,
                    blast_radius: k,
                    roce_score: ((hours_saved * 100.0) / (k as f64 + 1.0) * 10.0).round() / 10.0,
                    priority_tier: if k >= 4 {
                        "Strategic Batch (Sequenced)".to_string()
                    } else {
                        "Quick Win (Fix First)".to_string()
                    },
                },
            });
        }
    }

    signals
}

/// 5. Detect large embedded inline HTML / SQL string blobs in Rust files
pub fn scan_monolith_blob_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if path_str.contains("templates")
            || path_str.contains("xtask/")
            || crate::provenance::is_test_or_mock_path(&file.path)
        {
            continue;
        }

        let line_count = file
            .clean_content
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.contains("<div")
                    || t.contains("<tr")
                    || t.contains("<td")
                    || t.contains("<!DOCTYPE html")
                    || t.contains("<table")
                    || t.contains("<form")
            })
            .count();

        if line_count >= 15 {
            let target = file.rel_path.clone();
            signals.push(OpportunitySignal {
                signal_code: "OPP-MONOLITH-BLOB".to_string(),
                category: OpportunityCategory::MonolithDecomposition,
                title: format!(
                    "Extract Embedded Inline HTML View ({line_count} markup lines) to Askama Template"
                ),
                file: target.clone(),
                line: 1,
                target_locations: vec![target],
                potential_gain:
                    "Decouples presentation layer from Rust compilation unit and enables Askama compile-time validation"
                        .to_string(),
                deterministic_recipe:
                    "Move inline markup strings to dedicated template in templates/ directory and render via Askama struct"
                        .to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 2.0,
                    loc_reduction_estimate: line_count,
                    blast_radius: 1,
                    roce_score: 85.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }

    signals
}

/// 6. Detect fragmented cross-cutting concerns (e.g. repeated manual HMAC setup)
pub fn scan_primitive_fragmentation_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let mut hmac_files = BTreeSet::new();

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if path_str.contains("packages/encryption")
            || path_str.contains("ais-core")
            || path_str.contains("xtask")
            || crate::provenance::is_test_or_mock_path(&file.path)
        {
            continue;
        }

        if HMAC_INIT_RE.is_match(&file.clean_content) {
            hmac_files.insert(file.rel_path.clone());
        }
    }

    if hmac_files.len() >= 3 {
        let all_files: Vec<PathBuf> = hmac_files.into_iter().collect();
        let primary = all_files[0].clone();
        signals.push(OpportunitySignal {
            signal_code: "OPP-PRIM-FRAGMENT".to_string(),
            category: OpportunityCategory::PrimitiveHarmonization,
            title: format!(
                "Harmonize {} Bespoke HMAC-SHA256 Verifiers into Canonical Security Trait",
                all_files.len()
            ),
            file: primary,
            line: 1,
            target_locations: all_files.clone(),
            potential_gain:
                "Guarantees constant-time verification and timestamp replay protection across all webhook entrypoints"
                    .to_string(),
            deterministic_recipe:
                "Replace manual Hmac::<Sha256> initialization with canonical packages/encryption::WebhookHmacVerifier"
                    .to_string(),
            roi: OpportunityRoi {
                engineering_hours_saved: 4.0,
                loc_reduction_estimate: all_files.len() * 12,
                blast_radius: all_files.len(),
                roce_score: 90.0,
                priority_tier: "Strategic Batch (Sequenced)".to_string(),
            },
        });
    }

    signals
}

/// 7. Scan template directories and route handlers for Hypermedia UX & edge caching opportunities
pub fn scan_ux_hypermedia_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let generic_btn_re = &*GENERIC_BTN_RE;
    let full_tab_nav_re = &*FULL_TAB_NAV_RE;

    for file in &index.polyglot_files {
        if file.extension != "html" {
            continue;
        }
        let rel_path = &file.rel_path;
        let content = &file.clean_content;

        for (idx, line) in content.lines().enumerate() {
            let line_no = idx + 1;
            if generic_btn_re.is_match(line) {
                let target = rel_path.clone();
                signals.push(OpportunitySignal {
                    signal_code: "OPP-UX-MICROCOPY".to_string(),
                    category: OpportunityCategory::HypermediaUx,
                    title: "High-Intent Action Microcopy Upgrade".to_string(),
                    file: target.clone(),
                    line: line_no,
                    target_locations: vec![target],
                    potential_gain:
                        "Improves conversion clarity by replacing generic verb with intent-rich CTA"
                            .to_string(),
                    deterministic_recipe: "Replace generic 'Submit' with context-rich action verb"
                        .to_string(),
                    roi: OpportunityRoi {
                        engineering_hours_saved: 0.5,
                        loc_reduction_estimate: 0,
                        blast_radius: 1,
                        roce_score: 60.0,
                        priority_tier: "Quick Win (Fix First)".to_string(),
                    },
                });
            }

            if let Some(cap) = full_tab_nav_re.captures(line) {
                if !line.contains("hx-get") {
                    let target_url = cap.get(1).map_or("", |m| m.as_str());
                    let target = rel_path.clone();
                    signals.push(OpportunitySignal {
                        signal_code: "OPP-HTMX-PARTIAL".to_string(),
                        category: OpportunityCategory::HypermediaUx,
                        title: "Hypermedia Partial Swap Acceleration".to_string(),
                        file: target.clone(),
                        line: line_no,
                        target_locations: vec![target],
                        potential_gain: "Sub-40ms page transition without full browser reload"
                            .to_string(),
                        deterministic_recipe: format!(
                            "Add hx-get=\"{target_url}\" hx-target=\"#content-panel\""
                        ),
                        roi: OpportunityRoi {
                            engineering_hours_saved: 1.0,
                            loc_reduction_estimate: 0,
                            blast_radius: 1,
                            roce_score: 70.0,
                            priority_tier: "Quick Win (Fix First)".to_string(),
                        },
                    });
                }
            }
        }
    }

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if !path_str.contains("routes") {
            continue;
        }

        if file.clean_content.contains("-> impl IntoResponse")
            || file.clean_content.contains("-> Html<")
        {
            let lower = file.clean_content.to_lowercase();
            if !lower.contains("cachecontrol")
                && !lower.contains("headermap")
                && !lower.contains("cache-control")
                && !lower.contains("cache_control")
            {
                let target = file.rel_path.clone();
                signals.push(OpportunitySignal {
                    signal_code: "OPP-PERF-CACHE".to_string(),
                    category: OpportunityCategory::HypermediaUx,
                    title: "Edge & Browser Cache Header Optimization".to_string(),
                    file: target.clone(),
                    line: 1,
                    target_locations: vec![target],
                    potential_gain:
                        "Eliminates redundant database queries for static/infrequent views"
                            .to_string(),
                    deterministic_recipe:
                        "Return [(header::CACHE_CONTROL, \"private, max-age=60\")] tuple"
                            .to_string(),
                    roi: OpportunityRoi {
                        engineering_hours_saved: 1.0,
                        loc_reduction_estimate: 2,
                        blast_radius: 1,
                        roce_score: 65.0,
                        priority_tier: "Quick Win (Fix First)".to_string(),
                    },
                });
            }
        }
    }

    signals
}
/// 8. Detect near-duplicate route handlers across BFF and API layers
pub fn scan_handler_clustering_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    let mut handler_profiles = Vec::new();

    for file in &index.rust_files {
        let path_str = file.rel_path.to_string_lossy();
        if !path_str.contains("/routes/") && !path_str.contains("/handlers/") {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub async fn ") || trimmed.starts_with("async fn ") {
                if let Some(cap) = FN_DECL_RE.captures(line) {
                    if let Some(fn_m) = cap.get(1) {
                        let fn_name = fn_m.as_str().to_string();
                        if fn_name.starts_with("test_") {
                            continue;
                        }

                        let mut brace_depth = 0;
                        let mut started = false;
                        let mut end_idx = idx;
                        for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                            for ch in curr_line.chars() {
                                if ch == '{' {
                                    brace_depth += 1;
                                    started = true;
                                } else if ch == '}' {
                                    brace_depth -= 1;
                                }
                            }
                            if started && brace_depth == 0 {
                                end_idx = curr_idx + 1;
                                break;
                            }
                        }
                        if started && end_idx > idx {
                            let code = lines[idx..end_idx].join("\n");
                            if code.lines().count() >= 8 {
                                handler_profiles.push(crate::dedup::fingerprint_code_snippet(
                                    file.rel_path.clone(),
                                    fn_name,
                                    idx + 1,
                                    &code,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if handler_profiles.len() >= 2 {
        let duplicates = crate::dedup::detect_lsh_duplicate_clusters(&handler_profiles, 96.0);

        let mut fn_ids: BTreeMap<(&Path, &str, usize), usize> = BTreeMap::new();
        let mut id_to_fn: Vec<(&Path, &str, usize)> = Vec::new();

        let filtered_dups: Vec<_> = duplicates
            .into_iter()
            .filter(|dup| {
                dup.file_a != dup.file_b
                    && dup.function_a == dup.function_b
                    && dup.function_a != "execute"
                    && dup.function_a != "load_last_finalize"
                    && !dup.function_a.ends_with("_routes")
                    && !dup.function_b.ends_with("_routes")
                    && !dup.function_a.ends_with("_router")
                    && !dup.function_b.ends_with("_router")
                    && dup.function_a != "create_routes"
                    && dup.function_b != "create_routes"
            })
            .collect();

        for dup in &filtered_dups {
            let key_a = (dup.file_a.as_path(), dup.function_a.as_str(), dup.line_a);
            let id_a = *fn_ids.entry(key_a).or_insert_with(|| {
                let id = id_to_fn.len();
                id_to_fn.push(key_a);
                id
            });

            let key_b = (dup.file_b.as_path(), dup.function_b.as_str(), dup.line_b);
            let id_b = *fn_ids.entry(key_b).or_insert_with(|| {
                let id = id_to_fn.len();
                id_to_fn.push(key_b);
                id
            });
            let _ = (id_a, id_b);
        }

        struct HandlerDsu {
            parent: Vec<usize>,
        }
        impl HandlerDsu {
            fn new(n: usize) -> Self {
                Self {
                    parent: (0..n).collect(),
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
                let ri = self.find(i);
                let rj = self.find(j);
                if ri != rj {
                    if ri < rj {
                        self.parent[rj] = ri;
                    } else {
                        self.parent[ri] = rj;
                    }
                }
            }
        }

        let mut dsu = HandlerDsu::new(id_to_fn.len());
        for dup in &filtered_dups {
            let key_a = (dup.file_a.as_path(), dup.function_a.as_str(), dup.line_a);
            let key_b = (dup.file_b.as_path(), dup.function_b.as_str(), dup.line_b);
            if let (Some(&id_a), Some(&id_b)) = (fn_ids.get(&key_a), fn_ids.get(&key_b)) {
                dsu.union(id_a, id_b);
            }
        }

        let mut clusters: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for idx in 0..id_to_fn.len() {
            let root_idx = dsu.find(idx);
            clusters.entry(root_idx).or_default().push(idx);
        }

        for (_root_idx, mut members) in clusters {
            if members.len() >= 2 {
                members.sort_by(|&a, &b| id_to_fn[a].cmp(&id_to_fn[b]));
                let lead = id_to_fn[members[0]];
                let member_locs: Vec<PathBuf> = members
                    .iter()
                    .map(|&idx| id_to_fn[idx].0.to_path_buf())
                    .collect();
                let k = member_locs.len();

                signals.push(OpportunitySignal {
                    signal_code: "OPP-HANDLER-CLUSTER".to_string(),
                    category: OpportunityCategory::ArchitectureConsolidation,
                    title: format!(
                        "Harmonize {k}-Site Near-Duplicate Route Handler Cluster around '{}'",
                        lead.1
                    ),
                    file: lead.0.to_path_buf(),
                    line: lead.2,
                    target_locations: member_locs,
                    potential_gain: format!(
                        "Consolidates {k} near-identical HTTP route handler workflows into a shared service archetype"
                    ),
                    deterministic_recipe: "Extract common ingress validation and repository mutation into shared service function".to_string(),
                    roi: OpportunityRoi {
                        engineering_hours_saved: (k as f64) * 1.5,
                        loc_reduction_estimate: (k - 1) * 20,
                        blast_radius: k,
                        roce_score: 80.0,
                        priority_tier: "Quick Win (Fix First)".to_string(),
                    },
                });
            }
        }
    }
    signals
}

/// 9. Detect amber-zone module and subroutine complexity before threshold breach
pub fn scan_proactive_decomposition_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();

    for file in &index.polyglot_files {
        let path_str = file.rel_path.to_string_lossy();
        if path_str.contains("xtask/")
            || path_str.contains("scripts/")
            || path_str.contains(".next/")
            || path_str.contains("crates/ais-")
            || crate::provenance::is_test_or_mock_path(&file.path)
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        let is_exempt = lines.iter().take(30).any(|l| {
            l.contains("// POT-011: exempt")
                || l.contains("// ARCH-EXEMPT: MODULE-LENGTH")
                || l.contains("// POT-004: exempt")
                || l.contains("// POT-005: exempt")
        });
        if is_exempt {
            continue;
        }

        let loc = lines.iter().filter(|l| !l.trim().is_empty()).count();

        // 1. Amber-zone module size: approaching POT-011 limit of 750
        if loc >= 745 && loc < 750 {
            signals.push(OpportunitySignal {
                signal_code: "OPP-PROACTIVE-DECOMPOSE".to_string(),
                category: OpportunityCategory::ProactiveComplexity,
                title: format!(
                    "Proactively Decompose Module Approaching 750-Line Limit ({loc} LOC)"
                ),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: vec![file.rel_path.clone()],
                potential_gain: "Prevents future POT-011 violations and maintains high maintainability index".to_string(),
                deterministic_recipe: "Decompose cohesive sub-domains into sub-modules within a directory-backed module hierarchy".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 3.0,
                    loc_reduction_estimate: 0,
                    blast_radius: 1,
                    roce_score: 75.0,
                    priority_tier: "Strategic Batch (Sequenced)".to_string(),
                },
            });
        }

        // 2. Amber-zone function size: 59 <= LOC < 60 with CCN >= 25 (approaching POT-004 limit of 60)
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = FN_DECL_RE.captures(line) {
                if let Some(fn_m) = cap.get(1) {
                    let fn_name = fn_m.as_str();
                    if fn_name.starts_with("test_") {
                        continue;
                    }
                    let mut brace_depth = 0;
                    let mut started = false;
                    let mut end_idx = idx;
                    for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                        for ch in curr_line.chars() {
                            if ch == '{' {
                                brace_depth += 1;
                                started = true;
                            } else if ch == '}' {
                                brace_depth -= 1;
                            }
                        }
                        if started && brace_depth == 0 {
                            end_idx = curr_idx + 1;
                            break;
                        }
                    }
                    if started && end_idx > idx {
                        let fn_lines = end_idx - idx;
                        if fn_lines >= 59 && fn_lines < 60 {
                            let fn_body = lines[idx..end_idx].join("\n");
                            let ccn = crate::metrics::compute_ccn(&fn_body);
                            if ccn >= 25 {
                                signals.push(OpportunitySignal {
                                    signal_code: "OPP-PROACTIVE-DECOMPOSE".to_string(),
                                    category: OpportunityCategory::ProactiveComplexity,
                                    title: format!(
                                        "Decompose High-Complexity Subroutine '{fn_name}' ({fn_lines} LOC, CCN {ccn})"
                                    ),
                                    file: file.rel_path.clone(),
                                    line: idx + 1,
                                    target_locations: vec![file.rel_path.clone()],
                                    potential_gain: "Reduces cognitive branching complexity and eliminates risk of POT-004 violation".to_string(),
                                    deterministic_recipe: format!("Extract inner logic blocks of '{fn_name}' into focused private helper subroutines"),
                                    roi: OpportunityRoi {
                                        engineering_hours_saved: 1.5,
                                        loc_reduction_estimate: 5,
                                        blast_radius: 1,
                                        roce_score: 70.0,
                                        priority_tier: "Quick Win (Fix First)".to_string(),
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    signals
}

/// 10. Detect sequential async calls inside loop constructs (N+1 Waterfall)
pub fn scan_async_waterfall_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    static LOOP_RE: LazyLock<Regex> = LazyLock::new(|| {
        match Regex::new(r"(?m)^\s*(?:for\s+.*in\b|while\s+.*\{|for\s*\(.*of\b|for\s*\(.*;.*;.*\))")
        {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        }
    });

    for file in &index.polyglot_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("scripts/")
            || p_str.contains("e2e/")
            || p_str.contains("playwright")
            || p_str.contains("xtask/")
            || p_str.contains(".next/")
            || p_str.contains("migrate_")
            || p_str.contains("migrations")
            || p_str.contains("-db-init.js")
            || p_str.contains("demo-seed.js")
            || p_str.contains("cms-seed")
            || p_str.contains("libsql-browser-")
            || p_str.contains("crates/ais-core/")
            || p_str.contains("saga.rs")
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if LOOP_RE.is_match(line) {
                let trimmed_header = line.trim();
                // Exclude single-line synchronous loops without body block
                if trimmed_header.ends_with(';') {
                    continue;
                }
                // Exclude retry, backoff, multi-turn AI tool calling, and event stream consumer loops
                if trimmed_header.contains("attempt")
                    || trimmed_header.contains("retry")
                    || trimmed_header.contains("max_retries")
                    || trimmed_header.contains("tries")
                    || trimmed_header.contains("recv().await")
                    || trimmed_header.contains("rx.recv()")
                    || trimmed_header.contains(".next().await")
                    || trimmed_header.contains(".next_field().await")
                    || trimmed_header.contains(".next_line().await")
                    || trimmed_header.contains(".read_event().await")
                    || trimmed_header.contains(".accept().await")
                    || trimmed_header.contains(".join_next().await")
                    || trimmed_header.contains("join_next()")
                    || trimmed_header.contains("dequeue")
                    || trimmed_header.contains("tool_call")
                    || trimmed_header.contains("rounds <")
                    || trimmed_header.contains("MAX_TOOL_ROUNDS")
                    || trimmed_header.contains("gateway of gateways")
                    || trimmed_header.contains("ddl of")
                    || trimmed_header.contains("HEALTHKIT_DDL")
                    || trimmed_header.contains("execution_order")
                    || trimmed_header.contains("bufferedAmount")
                    || trimmed_header.contains("Date.now()")
                    || trimmed_header.contains("isConnected")
                    || trimmed_header.contains("!done")
                    || trimmed_header.contains("chunk")
                    || trimmed_header.contains("chunks(")
                    || trimmed_header.contains(".chunks")
                {
                    continue;
                }

                // Find opening brace of the loop body (on current or immediate next line)
                let mut open_brace_idx = None;
                for (s_idx, s_line) in lines.iter().enumerate().skip(idx).take(2) {
                    if s_line.contains('{') {
                        open_brace_idx = Some(s_idx);
                        break;
                    }
                }
                let Some(open_idx) = open_brace_idx else {
                    continue;
                };

                // Track brace depth to find exact loop body bounds
                let mut brace_depth = 0;
                let mut started = false;
                let mut end_idx = open_idx;
                for (curr_idx, curr_line) in lines.iter().enumerate().skip(open_idx) {
                    for ch in curr_line.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                            started = true;
                        } else if ch == '}' {
                            brace_depth -= 1;
                        }
                    }
                    if started && brace_depth == 0 {
                        end_idx = curr_idx;
                        break;
                    }
                }

                if !started || end_idx <= open_idx {
                    continue;
                }

                let loop_body_lines = &lines[open_idx..=end_idx];
                let loop_text = loop_body_lines.join("\n");

                // Exclude retry, timer, sleep, backoff, or serialized single-connection DB transaction/DDL loops
                if loop_text.contains("tokio::time::sleep")
                    || loop_text.contains("sleep(")
                    || loop_text.contains("setTimeout")
                    || loop_text.contains("bufferedAmount")
                    || loop_text.contains("interval.tick")
                    || loop_text.contains("backoff")
                    || loop_text.contains("setRemoteDescription")
                    || loop_text.contains("setOverlayStatus")
                    || loop_text.contains("join_set")
                    || loop_text.contains("offset +=")
                    || loop_text.contains("offset+")
                    || loop_text.contains("startIndex +=")
                    || loop_text.contains("page++")
                    || loop_text.contains("page +=")
                    || loop_text.contains("pageToken")
                    || loop_text.contains("nextPageToken")
                    || loop_text.contains("starting_after")
                    || loop_text.contains("startingAfter")
                    || loop_text.contains("group.addTask")
                    || loop_text.contains("withTaskGroup")
                    || loop_text.contains("tokio::spawn")
                    || loop_text.contains(".write().await")
                    || loop_text.contains("retry")
                    || loop_text.contains("MIGRATION")
                    || loop_text.contains("migration")
                    || loop_text.contains("fallback")
                    || loop_text.contains("probe")
                    || loop_text.contains("conn.execute")
                    || loop_text.contains("conn.query")
                    || loop_text.contains("conn\n")
                    || loop_text.contains("tx\n")
                    || loop_text.contains("tx.execute")
                    || loop_text.contains("executor().execute")
                    || loop_text.contains("executor.execute")
                    || loop_text.contains("executeQuery")
                    || loop_text.contains("db.execute")
                    || loop_text.contains("db.executeQuery")
                    || loop_text.contains("FreshCreditDB.execute")
                    || loop_text.contains("conn,")
                    || loop_text.contains("&conn")
                    || loop_text.contains("tx,")
                    || loop_text.contains("&tx")
                    || loop_text.contains("TABLE_")
                    || loop_text.contains("INDEXES")
                    || loop_text.contains("SCHEMA")
                    || loop_text.contains("STATEMENTS")
                    || loop_text.contains("ensure_tables")
                {
                    continue;
                }

                // Check strictly inside the loop body for sequential network/IO await
                let mut has_waterfall_await = false;
                let mut await_line = open_idx + 1;

                for (offset, l_line) in loop_body_lines.iter().enumerate().skip(1) {
                    let trimmed = l_line.trim();
                    if trimmed.starts_with("//") {
                        continue;
                    }

                    // Exclude stream consumers, channel receivers, task joins, and background task spawns
                    if trimmed.contains(".next().await")
                        || trimmed.contains(".recv().await")
                        || trimmed.contains(".next_line().await")
                        || trimmed.contains(".read_event().await")
                        || trimmed.contains(".join_next().await")
                        || trimmed.contains("join_set.spawn(")
                        || trimmed.contains(".spawn(")
                        || trimmed.contains("tokio::spawn(")
                    {
                        continue;
                    }

                    if (trimmed.contains(".await") || trimmed.contains("await "))
                        && !trimmed.contains("contains(\".await\")")
                        && !trimmed.contains("contains(\"await \")")
                        && !trimmed.contains("contains('.await')")
                    {
                        has_waterfall_await = true;
                        await_line = open_idx + offset + 1;
                        break;
                    }
                }

                if has_waterfall_await {
                    signals.push(OpportunitySignal {
                        signal_code: "OPP-ASYNC-WATERFALL".to_string(),
                        category: OpportunityCategory::AsyncPerformance,
                        title: "Batch Sequential Async Loop Iterations via Concurrency Join".to_string(),
                        file: file.rel_path.clone(),
                        line: await_line,
                        target_locations: vec![file.rel_path.clone()],
                        potential_gain: "Replaces O(N) sequential network latency with O(1) concurrent batching (join_all / Promise.all)".to_string(),
                        deterministic_recipe: "Collect futures into an iterator/array and resolve via futures::future::join_all or Promise.all".to_string(),
                        roi: OpportunityRoi {
                            engineering_hours_saved: 2.0,
                            loc_reduction_estimate: 4,
                            blast_radius: 1,
                            roce_score: 85.0,
                            priority_tier: "Quick Win (Fix First)".to_string(),
                        },
                    });
                }
            }
        }
    }
    signals
}

/// 11. Detect near-duplicate DTO and domain struct definitions across crates
pub fn scan_schema_dedup_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    static STRUCT_HEADER: LazyLock<Regex> = LazyLock::new(|| {
        match Regex::new(
            r"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:struct|interface|type)\s+([A-Za-z0-9_]+)\s*(?:<[^>]+>)?\s*(?:=\s*)?\{",
        ) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        }
    });

    let mut schema_map: BTreeMap<String, Vec<(PathBuf, String, usize)>> = BTreeMap::new();

    for file in &index.polyglot_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("scripts/")
            || p_str.contains("fixtures")
            || p_str.contains("canary")
            || p_str.contains("mock")
            || p_str.contains("crates/openapi/")
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = STRUCT_HEADER.captures(line) {
                let name = cap[1].to_string();
                if name.starts_with("Mock")
                    || name.ends_with("Mock")
                    || name.starts_with("Test")
                    || name.ends_with("Template")
                    || name.ends_with("Query")
                    || name.ends_with("Params")
                    || name.ends_with("Request")
                    || name.ends_with("Response")
                    || name.ends_with("View")
                    || name.ends_with("Config")
                    || name.ends_with("Record")
                    || name.ends_with("Summary")
                    || name.ends_with("History")
                    || name.ends_with("Activity")
                    || name.ends_with("Form")
                    || name.ends_with("Info")
                    || name.ends_with("Check")
                    || name.ends_with("Row")
                    || name.ends_with("Item")
                    || name.ends_with("Input")
                    || name.ends_with("Event")
                    || name.ends_with("Entry")
                    || name.ends_with("State")
                    || name.ends_with("Status")
                    || name.ends_with("Payload")
                    || name.ends_with("Header")
                    || name.ends_with("Claims")
                    || name.ends_with("Props")
                    || name.ends_with("Result")
                    || name.ends_with("Data")
                    || name.ends_with("Options")
                    || name.ends_with("Settings")
                    || name.ends_with("Preferences")
                    || name.ends_with("Preference")
                    || name.ends_with("Metadata")
                {
                    continue;
                }

                let mut brace_depth = 0;
                let mut started = false;
                let mut end_idx = idx;
                for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                    for ch in curr_line.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                            started = true;
                        } else if ch == '}' {
                            brace_depth -= 1;
                        }
                    }
                    if started && brace_depth == 0 {
                        end_idx = curr_idx;
                        break;
                    }
                }

                if !started || end_idx <= idx {
                    continue;
                }

                let body_lines = &lines[idx + 1..end_idx];
                let mut fields_with_types = Vec::new();

                for f_line in body_lines {
                    let trimmed = f_line.trim();
                    if trimmed.starts_with("//") {
                        continue;
                    }
                    if let Some(colon_idx) = trimmed.find(':') {
                        let field_name = trimmed[..colon_idx]
                            .trim_start_matches("pub ")
                            .trim_start_matches("readonly ")
                            .trim();
                        let raw_type = trimmed[colon_idx + 1..]
                            .trim_end_matches(',')
                            .trim_end_matches(';')
                            .trim();
                        let norm_type: String =
                            raw_type.chars().filter(|c| !c.is_whitespace()).collect();
                        if !field_name.is_empty()
                            && field_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                            && !norm_type.is_empty()
                        {
                            fields_with_types.push(format!("{field_name}:{norm_type}"));
                        }
                    }
                }

                if fields_with_types.len() >= 4 {
                    fields_with_types.sort();
                    let sig = fields_with_types.join(",");
                    schema_map
                        .entry(sig)
                        .or_default()
                        .push((file.rel_path.clone(), name, idx + 1));
                }
            }
        }
    }

    for (_sig, sites) in schema_map {
        let distinct_files: BTreeSet<PathBuf> = sites.iter().map(|s| s.0.clone()).collect();
        if distinct_files.len() >= 2 {
            let vendor_prefixes: BTreeSet<String> = sites
                .iter()
                .filter_map(|s| {
                    let path_str = s.0.to_string_lossy();
                    if path_str.contains("adapters/src/") {
                        path_str
                            .split("adapters/src/")
                            .nth(1)?
                            .split('/')
                            .next()
                            .map(String::from)
                    } else if path_str.contains("crates/adapters/") {
                        path_str
                            .split("crates/adapters/")
                            .nth(1)?
                            .split('/')
                            .next()
                            .map(String::from)
                    } else if path_str.contains("engine-ports/src/") {
                        let sub = path_str
                            .split("engine-ports/src/")
                            .nth(1)?
                            .split('/')
                            .next()?;
                        Some(sub.trim_end_matches(".rs").to_string())
                    } else {
                        None
                    }
                })
                .collect();

            // Independent third-party vendor adapters must remain decoupled wire formats
            if vendor_prefixes.len() >= 2 {
                continue;
            }

            let distinct_crates: BTreeSet<String> = sites
                .iter()
                .filter_map(|s| {
                    let path_str = s.0.to_string_lossy();
                    path_str
                        .find("/src/")
                        .map(|pos| path_str[..pos].to_string())
                })
                .collect();

            // Decoupled wire formats across distinct compilation units / bounded contexts
            if distinct_crates.len() >= 2 {
                continue;
            }

            let primary = sites[0].clone();
            let all_target_files: Vec<PathBuf> = distinct_files.into_iter().collect();
            let count = all_target_files.len();

            signals.push(OpportunitySignal {
                signal_code: "OPP-SCHEMA-DEDUP".to_string(),
                category: OpportunityCategory::SchemaHarmonization,
                title: format!(
                    "Harmonize {count}-Site Duplicate Schema Definition around '{}'",
                    primary.1
                ),
                file: primary.0.clone(),
                line: primary.2,
                target_locations: all_target_files,
                potential_gain: format!(
                    "Consolidates {count} identical DTO/domain structure schemas into a canonical domain crate or shared package"
                ),
                deterministic_recipe: format!(
                    "Extract '{}' schema to shared types package and re-export to dependent crates",
                    primary.1
                ),
                roi: OpportunityRoi {
                    engineering_hours_saved: (count as f64) * 1.5,
                    loc_reduction_estimate: (count.saturating_sub(1)) * 15,
                    blast_radius: count,
                    roce_score: 80.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }
    signals
}

/// 12. Detect configuration & parameter fields that are declared but never read by execution engines
pub fn scan_dead_config_sink_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    static CONFIG_STRUCT_RE: LazyLock<Regex> = LazyLock::new(|| {
        match Regex::new(
            r"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?struct\s+([A-Za-z0-9_]*(?:Sim|Simulation|Civilization|Dials|Ecology|Terrain)Config)\s*\{",
        ) {
            Ok(r) => r,
            Err(e) => panic!("invalid regex: {e}"),
        }
    });

    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("opportunities.rs")
            || p_str.contains("fixtures")
            || p_str.contains("canary")
            || p_str.contains("mock")
        {
            continue;
        }

        let lines: Vec<&str> = file.clean_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = CONFIG_STRUCT_RE.captures(line) {
                let struct_name = cap[1].to_string();
                let mut brace_depth = 0;
                let mut started = false;
                let mut end_idx = idx;

                for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                    for ch in curr_line.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                            started = true;
                        } else if ch == '}' {
                            brace_depth -= 1;
                        }
                    }
                    if started && brace_depth == 0 {
                        end_idx = curr_idx;
                        break;
                    }
                }

                if !started || end_idx <= idx {
                    continue;
                }

                for f_line in &lines[idx + 1..end_idx] {
                    let trimmed = f_line.trim();
                    if trimmed.starts_with("//") || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some(colon_idx) = trimmed.find(':') {
                        let field_part = trimmed[..colon_idx].trim();
                        let field_name = field_part
                            .trim_start_matches("pub ")
                            .trim_start_matches("pub(crate) ")
                            .trim_start_matches("pub(super) ")
                            .trim();

                        if !field_name.is_empty()
                            && !field_name.starts_with('_')
                            && field_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                        {
                            let pattern = format!(".{}", field_name);
                            let read_count = index
                                .rust_files
                                .iter()
                                .filter(|f| f.rel_path != file.rel_path)
                                .map(|f| f.clean_content.matches(&pattern).count())
                                .sum::<usize>();

                            if read_count == 0 {
                                let target = file.rel_path.clone();
                                signals.push(OpportunitySignal {
                                    signal_code: "OPP-DEAD-CONFIG-SINK".to_string(),
                                    category: OpportunityCategory::DeadConfigSink,
                                    title: format!(
                                        "Wire or Prune Unconsumed Config Field '{field_name}' on '{struct_name}'"
                                    ),
                                    file: target.clone(),
                                    line: idx + 1,
                                    target_locations: vec![target],
                                    potential_gain: "Ensures configuration parameters causally alter execution logic or eliminates dead public API surface".to_string(),
                                    deterministic_recipe: format!(
                                        "Wire 'self.config.{field_name}' into domain step calculation or remove from '{struct_name}'"
                                    ),
                                    roi: OpportunityRoi {
                                        engineering_hours_saved: 2.0,
                                        loc_reduction_estimate: 3,
                                        blast_radius: 1,
                                        roce_score: 85.0,
                                        priority_tier: "Quick Win (Fix First)".to_string(),
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    signals
}

/// 13. Detect state variables and lifecycle methods with missing transitions or orphaned loops
pub fn scan_state_lifecycle_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();

    for file in &index.polyglot_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("fixtures")
            || p_str.contains("canary")
            || p_str.contains(".next/")
        {
            continue;
        }

        let content = &file.clean_content;

        // Pattern 1: let isSimulationPaused declared, read in stall detector, but never mutated in pause/resume
        if content.contains("isSimulationPaused")
            && content.contains("pause:")
            && (!content.contains("isSimulationPaused = true")
                && !content.contains("isSimulationPaused = true;"))
        {
            let target = file.rel_path.clone();
            signals.push(OpportunitySignal {
                signal_code: "OPP-STATE-LIFECYCLE".to_string(),
                category: OpportunityCategory::StateLifecycleAsymmetry,
                title: "Synchronize Pause State Flag in Lifecycle Handlers".to_string(),
                file: target.clone(),
                line: 1,
                target_locations: vec![target],
                potential_gain: "Prevents false stall detector reconnect storms during intentional simulation pauses".to_string(),
                deterministic_recipe: "Set isSimulationPaused = true in pause() and isSimulationPaused = false in resume()".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 1.0,
                    loc_reduction_estimate: 2,
                    blast_radius: 1,
                    roce_score: 90.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }

        // Pattern 2: stopRenderLoop defined, but stop method never invokes it
        if content.contains("function stopRenderLoop")
            && content.contains("stop: async")
            && !content.contains("stopRenderLoop()")
        {
            signals.push(OpportunitySignal {
                signal_code: "OPP-STATE-LIFECYCLE".to_string(),
                category: OpportunityCategory::StateLifecycleAsymmetry,
                title: "Invoke stopRenderLoop() on Simulation Stop Lifecycle".to_string(),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: vec![file.rel_path.clone()],
                potential_gain: "Cancels orphaned 60Hz requestAnimationFrame loops when user stops simulation".to_string(),
                deterministic_recipe: "Call stopRenderLoop() at the beginning of stop() lifecycle handler".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 1.0,
                    loc_reduction_estimate: 1,
                    blast_radius: 1,
                    roce_score: 90.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }

    signals
}

fn is_excluded_opportunity_path(path: &Path, rel_path: &Path) -> bool {
    let p_str = rel_path.to_string_lossy();
    crate::provenance::is_test_or_mock_path(path)
        || p_str.contains("xtask/")
        || p_str.contains("fixtures")
        || p_str.contains("canary")
        || p_str.contains(".next/")
        || p_str.contains("node_modules/")
}

/// 14. Detect non-canonical card geometry or fragmented app shells in UI templates
pub fn scan_style_canon_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    index
        .polyglot_files
        .iter()
        .filter(|f| !is_excluded_opportunity_path(&f.path, &f.rel_path))
        .filter(|f| matches!(f.extension.as_str(), "html" | "tsx" | "jsx"))
        .filter(|f| {
            (f.clean_content.contains("rounded-xl") || f.clean_content.contains("rounded-2xl"))
                && (f.clean_content.contains("card")
                    || f.clean_content.contains("panel")
                    || f.clean_content.contains("border"))
        })
        .map(|f| {
            let target = f.rel_path.clone();
            OpportunitySignal {
                signal_code: "OPP-STYLE-CANON".to_string(),
                category: OpportunityCategory::StyleCanonConsolidation,
                title: "Enforce Canonical Sharp Card Geometry and Style Canon".to_string(),
                file: target.clone(),
                line: 1,
                target_locations: vec![target],
                potential_gain: "Restores brand visual precision and eliminates rogue rounded corner geometry across surfaces".to_string(),
                deterministic_recipe: "Replace rounded-xl / rounded-2xl on card containers with canonical sharp card tokens (border border-border bg-card)".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 0.5,
                    loc_reduction_estimate: 2,
                    blast_radius: 1,
                    roce_score: 95.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            }
        })
        .collect()
}

/// 15. Detect non-generated or orphaned build artifact paths in CI/CD manifests
pub fn scan_ci_artifact_hygiene_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();

    for file in &index.polyglot_files {
        let p_str = file.rel_path.to_string_lossy();
        if is_excluded_opportunity_path(&file.path, &file.rel_path) {
            continue;
        }

        if p_str.contains("cloudbuild") || p_str.contains(".github/workflows") {
            let has_phantom = file.clean_content.contains("remediation.patch")
                || file.clean_content.contains("phantom_coverage");
            if file.clean_content.contains("artifacts:") && has_phantom {
                let target = file.rel_path.clone();
                signals.push(OpportunitySignal {
                    signal_code: "OPP-CI-ARTIFACT".to_string(),
                    category: OpportunityCategory::CiArtifactHygiene,
                    title: "Prune Non-Generated Artifact Paths from CI/CD Manifest".to_string(),
                    file: target.clone(),
                    line: 1,
                    target_locations: vec![target],
                    potential_gain: "Prevents Cloud Build Step artifact-upload failures when non-deterministic patches are pruned".to_string(),
                    deterministic_recipe: "Remove non-generated artifact paths from the 'artifacts.paths' array in cloudbuild.yaml".to_string(),
                    roi: OpportunityRoi {
                        engineering_hours_saved: 1.0,
                        loc_reduction_estimate: 1,
                        blast_radius: 1,
                        roce_score: 90.0,
                        priority_tier: "Quick Win (Fix First)".to_string(),
                    },
                });
            }
        }
    }

    signals
}
/// 16. Detect unmemoized expensive derivations (Argon2, Pbkdf2 inside functions without caching)
pub fn scan_perf_cache_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("canary")
            || p_str.contains("fixtures")
            || p_str.contains("packages/encryption")
            || p_str.contains("opportunities.rs")
        {
            continue;
        }
        for (line_idx, line) in file.clean_content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if trimmed.contains("Argon2::default()")
                || trimmed.contains("Argon2::new(")
                || trimmed.contains("Pbkdf2")
            {
                signals.push(OpportunitySignal {
                    signal_code: "OPP-PERF-CACHE".to_string(),
                    category: OpportunityCategory::AsyncPerformance,
                    title: "Cache or Precompute High-Latency Password/Key Derivation".to_string(),
                    file: file.rel_path.clone(),
                    line: line_idx + 1,
                    target_locations: Vec::new(),
                    potential_gain: "Eliminates ~19 MiB CPU/memory allocation and multiple millisecond latency on async worker threads".to_string(),
                    deterministic_recipe: "Cache derived key material via KDF cache or derive per-record subkeys via HKDF-SHA256 from cached master root".to_string(),
                    roi: OpportunityRoi {
                        engineering_hours_saved: 2.0,
                        loc_reduction_estimate: 5,
                        blast_radius: 1,
                        roce_score: 95.0,
                        priority_tier: "Quick Win (Fix First)".to_string(),
                    },
                });
            }
        }
    }
    signals
}

/// 17. Detect interactive input elements missing placeholder or aria-label
pub fn scan_ux_microcopy_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.polyglot_files {
        if !matches!(file.extension.as_str(), "html" | "tsx" | "jsx")
            || is_excluded_opportunity_path(&file.path, &file.rel_path)
        {
            continue;
        }

        let mut in_input_tag = false;
        let mut input_tag_buffer = String::new();
        let mut input_tag_line = 1;

        for (line_idx, line) in file.clean_content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains("<input") {
                in_input_tag = true;
                input_tag_buffer.clear();
                input_tag_line = line_idx + 1;
            }

            if in_input_tag {
                input_tag_buffer.push_str(trimmed);
                input_tag_buffer.push(' ');
                if trimmed.contains("/>")
                    || (trimmed.ends_with('>')
                        && !trimmed.ends_with("=>")
                        && !trimmed.ends_with("->"))
                {
                    in_input_tag = false;
                    let tag = &input_tag_buffer;
                    if !tag.contains("type=\"hidden\"")
                        && !tag.contains("type=\"checkbox\"")
                        && !tag.contains("type=\"radio\"")
                        && !tag.contains("type=\"submit\"")
                        && !tag.contains("type='hidden'")
                        && !tag.contains("aria-label")
                        && !tag.contains("placeholder")
                        && !tag.contains("aria-labelledby")
                    {
                        signals.push(OpportunitySignal {
                            signal_code: "OPP-UX-MICROCOPY".to_string(),
                            category: OpportunityCategory::HypermediaUx,
                            title: "Add Descriptive Placeholder or Aria-Label to Input Field".to_string(),
                            file: file.rel_path.clone(),
                            line: input_tag_line,
                            target_locations: Vec::new(),
                            potential_gain: "Enhances screen reader accessibility and guides end-user data entry precision".to_string(),
                            deterministic_recipe: "Add placeholder=\"...\" or aria-label=\"...\" attribute to input element".to_string(),
                            roi: OpportunityRoi {
                                engineering_hours_saved: 0.5,
                                loc_reduction_estimate: 1,
                                blast_radius: 1,
                                roce_score: 90.0,
                                priority_tier: "Quick Win (Fix First)".to_string(),
                            },
                        });
                    }
                }
            }
        }
    }
    signals
}
/// 18. Detect dynamic SQL IN (...) expansion without bounded batch chunking
pub fn scan_sql_batch_chunk_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("canary")
            || p_str.contains("fixtures")
            || p_str.contains("opportunities.rs")
        {
            continue;
        }
        let content = &file.clean_content;
        let has_sql_dml = content.contains("SELECT ")
            || content.contains("DELETE FROM ")
            || content.contains("UPDATE ")
            || content.contains("INSERT INTO ");
        if !has_sql_dml {
            continue;
        }

        if content.contains("WHERE ")
            && (content.contains(" IN (") || content.contains(" in ("))
            && (content.contains(".iter().map(")
                || content.contains(".join(\",\")")
                || content.contains(".join(\", \")"))
            && !content.contains("chunks(")
            && !content.contains("chunk(")
            && !content.contains("LIMIT")
            && !content.contains("limit")
        {
            signals.push(OpportunitySignal {
                signal_code: "OPP-SQL-BATCH-CHUNK".to_string(),
                category: OpportunityCategory::DatabasePersistence,
                title: "Enforce Parameterized Batch Chunking for Dynamic SQL IN Clauses".to_string(),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: Vec::new(),
                potential_gain: "Prevents SQLite SQLITE_MAX_VARIABLE_NUMBER parameter overflow when query slice exceeds 999 items".to_string(),
                deterministic_recipe: "Partition query target IDs into .chunks(400) and execute batched transactions".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 1.5,
                    loc_reduction_estimate: 4,
                    blast_radius: 1,
                    roce_score: 85.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }
    signals
}

/// 19. Detect ROLLBACK TO SAVEPOINT without paired RELEASE SAVEPOINT
pub fn scan_tx_savepoint_symmetry_opportunities(
    index: &WorkspaceSourceIndex,
) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("canary")
            || p_str.contains("fixtures")
            || p_str.contains("opportunities.rs")
        {
            continue;
        }
        let content = &file.clean_content;
        if content.contains("ROLLBACK TO SAVEPOINT") && !content.contains("RELEASE SAVEPOINT") {
            signals.push(OpportunitySignal {
                signal_code: "OPP-TX-SAVEPOINT-SYMMETRY".to_string(),
                category: OpportunityCategory::TransactionSafety,
                title: "Pair ROLLBACK TO SAVEPOINT with Immediate RELEASE SAVEPOINT".to_string(),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: Vec::new(),
                potential_gain: "Pops savepoint from connection transaction stack, preventing open-transaction connection poisoning".to_string(),
                deterministic_recipe: "Execute RELEASE SAVEPOINT <name> immediately after ROLLBACK TO SAVEPOINT <name> in error branches".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 2.0,
                    loc_reduction_estimate: 2,
                    blast_radius: 1,
                    roce_score: 95.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }
    signals
}

/// 20. Detect ON CONFLICT DO UPDATE on stateful records missing terminal condition guards
pub fn scan_upsert_lattice_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("canary")
            || p_str.contains("fixtures")
            || p_str.contains("opportunities.rs")
        {
            continue;
        }
        let content = &file.clean_content;
        let on_conflict_mutates_state = content.lines().any(|l| {
            let u = l.to_uppercase();
            u.contains("STATUS = EXCLUDED")
                || u.contains("SET STATUS")
                || u.contains("CONSUMED =")
                || u.contains("SET CONSUMED")
                || u.contains("REVOKED =")
        });
        if (content.contains("ON CONFLICT") || content.contains("on conflict"))
            && (content.contains("DO UPDATE SET") || content.contains("do update set"))
            && on_conflict_mutates_state
            && !content.contains("WHERE ")
            && !content.contains("where ")
        {
            signals.push(OpportunitySignal {
                signal_code: "OPP-UPSERT-LATTICE".to_string(),
                category: OpportunityCategory::DatabasePersistence,
                title: "Guard Stateful UPSERT Mutation with Monotonic Terminal State Clause".to_string(),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: Vec::new(),
                potential_gain: "Guarantees terminal states (settled, completed, consumed) cannot be regressed by out-of-order replay requests".to_string(),
                deterministic_recipe: "Append WHERE table.status NOT IN ('completed', 'settled') or consumed = 0 to ON CONFLICT clause".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 2.0,
                    loc_reduction_estimate: 3,
                    blast_radius: 1,
                    roce_score: 90.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }
    signals
}

/// 21. Detect full O(d) dense transformations immediately prior to top-k truncation
pub fn scan_compute_pushdown_opportunities(index: &WorkspaceSourceIndex) -> Vec<OpportunitySignal> {
    let mut signals = Vec::new();
    for file in &index.rust_files {
        let p_str = file.rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(&file.path)
            || p_str.contains("xtask/")
            || p_str.contains("canary")
            || p_str.contains("fixtures")
            || p_str.contains("opportunities.rs")
        {
            continue;
        }
        let content = &file.clean_content;
        if (content.contains("add_gaussian_noise")
            || content.contains("apply_noise")
            || content.contains("perturb_dense"))
            && (content.contains(".top_k(")
                || content.contains("compress(")
                || content.contains(".take("))
            && content.find("add_gaussian_noise").unwrap_or(usize::MAX)
                < content.find("compress").unwrap_or(0)
        {
            signals.push(OpportunitySignal {
                signal_code: "OPP-COMPUTE-PUSHDOWN".to_string(),
                category: OpportunityCategory::ComputeEfficiency,
                title: "Push Down Sparsification Prior to Dense Stochastic Noise Addition".to_string(),
                file: file.rel_path.clone(),
                line: 1,
                target_locations: Vec::new(),
                potential_gain: "Reduces PRNG allocations by d/k factor and eliminates extreme-value order statistic noise bias".to_string(),
                deterministic_recipe: "Extract top-k sparse indices first, then add calibrated noise directly to selected coordinates".to_string(),
                roi: OpportunityRoi {
                    engineering_hours_saved: 3.0,
                    loc_reduction_estimate: 6,
                    blast_radius: 1,
                    roce_score: 92.0,
                    priority_tier: "Quick Win (Fix First)".to_string(),
                },
            });
        }
    }
    signals
}

/// Comprehensive full-workspace opportunity scan aggregating all 21 proactive engines
pub fn scan_all_workspace_opportunities(root: &Path) -> OpportunityScanReport {
    let index = WorkspaceSourceIndex::build(root);

    // Fast content hash of polyglot files for layer caching
    let mut ws_hasher = std::hash::DefaultHasher::new();
    for f in &index.polyglot_files {
        f.rel_path.hash(&mut ws_hasher);
        crate::cache::compute_content_hash(&f.clean_content).hash(&mut ws_hasher);
    }
    let ws_hash = ws_hasher.finish();

    let engine_fp = crate::cache::compute_engine_fingerprint(root);
    let cache = crate::cache::load_layer_cache(root, engine_fp);
    if let Some((cached_hash, ref report)) = cache.cached_opportunities {
        if cached_hash == ws_hash {
            return report.clone();
        }
    }

    let mut signals = Vec::new();

    signals.extend(scan_async_overhead_opportunities(&index));
    signals.extend(scan_multi_writer_persistence_opportunities(&index));
    signals.extend(scan_cohesion_decomposition_opportunities(&index));
    signals.extend(scan_clone_consolidation_opportunities(&index));
    signals.extend(scan_monolith_blob_opportunities(&index));
    signals.extend(scan_primitive_fragmentation_opportunities(&index));
    signals.extend(scan_ux_hypermedia_opportunities(&index));
    signals.extend(scan_handler_clustering_opportunities(&index));
    signals.extend(scan_proactive_decomposition_opportunities(&index));
    signals.extend(scan_async_waterfall_opportunities(&index));
    signals.extend(scan_schema_dedup_opportunities(&index));
    signals.extend(scan_dead_config_sink_opportunities(&index));
    signals.extend(scan_state_lifecycle_opportunities(&index));
    signals.extend(scan_style_canon_opportunities(&index));
    signals.extend(scan_ci_artifact_hygiene_opportunities(&index));
    signals.extend(scan_perf_cache_opportunities(&index));
    signals.extend(scan_ux_microcopy_opportunities(&index));
    signals.extend(scan_sql_batch_chunk_opportunities(&index));
    signals.extend(scan_tx_savepoint_symmetry_opportunities(&index));
    signals.extend(scan_upsert_lattice_opportunities(&index));
    signals.extend(scan_compute_pushdown_opportunities(&index));

    signals.sort_by(|a, b| {
        b.roi
            .roce_score
            .partial_cmp(&a.roi.roce_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.signal_code.cmp(&b.signal_code))
            .then_with(|| a.file.cmp(&b.file))
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.title.cmp(&b.title))
    });

    let mut category_breakdown = BTreeMap::new();
    let mut total_hours = 0.0;
    let mut total_loc = 0;

    for s in &signals {
        *category_breakdown
            .entry(s.category.as_str().to_string())
            .or_insert(0) += 1;
        total_hours += s.roi.engineering_hours_saved;
        total_loc += s.roi.loc_reduction_estimate;
    }

    let report = OpportunityScanReport {
        total_opportunities_discovered: signals.len(),
        category_breakdown,
        total_projected_hours_saved: (total_hours * 10.0).round() / 10.0,
        total_projected_loc_reduction: total_loc,
        top_opportunities: signals,
    };

    let mut updated_cache = cache;
    updated_cache.cached_opportunities = Some((ws_hash, report.clone()));
    crate::cache::save_layer_cache(root, &updated_cache);

    report
}

/// Synthetic Canary Verification: Proves the opportunity scanning engines are active,
/// highly sensitive, and capable of detecting each opportunity class when presented with
/// synthetic anti-patterns, without requiring permanent debt to exist in production code.
#[must_use]
pub fn verify_synthetic_opportunity_probes() -> bool {
    let sync_code = r#"
        pub async fn pure_compute(a: i64, b: i64) -> i64 {
            let x = a * 2;
            let y = b * 3;
            let z = x + y;
            let w = z - a;
            let res = w / 2;
            res + 1
        }
    "#;
    let sync_file = IndexedSourceFile {
        path: PathBuf::from("math/pure.rs"),
        rel_path: PathBuf::from("math/pure.rs"),
        extension: "rs".to_string(),
        clean_content: sync_code.to_string(),
    };
    let async_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![sync_file],
        polyglot_files: vec![],
    };
    let async_opps = scan_async_overhead_opportunities(&async_index);
    if async_opps.is_empty() {
        return false;
    }

    let clone_a = r#"
        pub fn process_data_records(items: &[String]) -> usize {
            let mut total = 0usize;
            let mut offset = 1usize;
            total += offset * 3;
            total += offset * 4;
            total += offset * 5;
            for item in items {
                if !item.is_empty() {
                    let len = item.trim().len();
                    if len > 2 {
                        total += len;
                        total += offset;
                        offset += 1;
                    } else {
                        total += 1;
                    }
                } else {
                    offset += 1;
                }
            }
            if total > 100 {
                total += 10;
            } else {
                total += 5;
            }
            total * 2
        }
    "#;
    let clone_b = r#"
        pub fn compute_string_lengths(entries: &[String]) -> usize {
            let mut total = 0usize;
            let mut offset = 1usize;
            total += offset * 3;
            total += offset * 4;
            total += offset * 5;
            for entry in entries {
                if !entry.is_empty() {
                    let len = entry.trim().len();
                    if len > 2 {
                        total += len;
                        total += offset;
                        offset += 1;
                    } else {
                        total += 1;
                    }
                } else {
                    offset += 1;
                }
            }
            if total > 100 {
                total += 10;
            } else {
                total += 5;
            }
            total * 2
        }
    "#;
    let f_a = IndexedSourceFile {
        path: PathBuf::from("pkg/a.rs"),
        rel_path: PathBuf::from("pkg/a.rs"),
        extension: "rs".to_string(),
        clean_content: clone_a.to_string(),
    };
    let f_b = IndexedSourceFile {
        path: PathBuf::from("pkg/b.rs"),
        rel_path: PathBuf::from("pkg/b.rs"),
        extension: "rs".to_string(),
        clean_content: clone_b.to_string(),
    };
    let clone_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![f_a, f_b],
    };
    let clone_opps = scan_clone_consolidation_opportunities(&clone_index);
    if clone_opps.is_empty() {
        return false;
    }

    let wf_code = r#"
        pub async fn process_items(items: &[String]) {
            for item in items {
                let res = fetch_item(item).await;
                println!("{res}");
            }
        }
    "#;
    let wf_file = IndexedSourceFile {
        path: PathBuf::from("pkg/loop.rs"),
        rel_path: PathBuf::from("pkg/loop.rs"),
        extension: "rs".to_string(),
        clean_content: wf_code.to_string(),
    };
    let wf_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![wf_file],
    };
    if scan_async_waterfall_opportunities(&wf_index).is_empty() {
        return false;
    }

    let schema_a = r#"
        pub struct UserBillingProfile {
            pub customer_id: String,
            pub billing_email: String,
            pub card_brand: String,
            pub postal_code: String,
        }
    "#;
    let schema_b = r#"
        pub struct InvoiceBillingContact {
            pub customer_id: String,
            pub billing_email: String,
            pub card_brand: String,
            pub postal_code: String,
        }
    "#;
    let sch_a = IndexedSourceFile {
        path: PathBuf::from("crates/a/types.rs"),
        rel_path: PathBuf::from("crates/a/types.rs"),
        extension: "rs".to_string(),
        clean_content: schema_a.to_string(),
    };
    let sch_b = IndexedSourceFile {
        path: PathBuf::from("crates/b/types.rs"),
        rel_path: PathBuf::from("crates/b/types.rs"),
        extension: "rs".to_string(),
        clean_content: schema_b.to_string(),
    };
    let sch_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![sch_a, sch_b],
    };
    if scan_schema_dedup_opportunities(&sch_index).is_empty() {
        return false;
    }

    // Negative Probe 1: Post-loop await must NOT trigger waterfall
    let non_wf_code = r#"
        pub async fn batch_execute(items: &[String]) {
            let mut query = String::new();
            for item in items {
                query.push_str(item);
            }
            database.execute(&query).await;
        }
    "#;
    let non_wf_file = IndexedSourceFile {
        path: PathBuf::from("pkg/non_wf.rs"),
        rel_path: PathBuf::from("pkg/non_wf.rs"),
        extension: "rs".to_string(),
        clean_content: non_wf_code.to_string(),
    };
    let non_wf_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![non_wf_file],
    };
    if !scan_async_waterfall_opportunities(&non_wf_index).is_empty() {
        return false;
    }

    // Negative Probe 2: Retry loop with sleep/retry must NOT trigger waterfall
    let retry_code = r#"
        pub async fn retry_operation() {
            for attempt in 0..3 {
                if let Ok(res) = send_request().await {
                    return res;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    "#;
    let retry_file = IndexedSourceFile {
        path: PathBuf::from("pkg/retry.rs"),
        rel_path: PathBuf::from("pkg/retry.rs"),
        extension: "rs".to_string(),
        clean_content: retry_code.to_string(),
    };
    let retry_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![retry_file],
    };
    if !scan_async_waterfall_opportunities(&retry_index).is_empty() {
        return false;
    }

    // Negative Probe 3: Structs with different types must NOT trigger schema dedup
    let diff_types_a = r#"
        pub struct MetricsReport {
            pub id: String,
            pub count: usize,
            pub active: bool,
            pub total: f64,
        }
    "#;
    let diff_types_b = r#"
        pub struct MetricsReport {
            pub id: u64,
            pub count: i32,
            pub active: String,
            pub total: i64,
        }
    "#;
    let dt_a = IndexedSourceFile {
        path: PathBuf::from("crates/a/diff.rs"),
        rel_path: PathBuf::from("crates/a/diff.rs"),
        extension: "rs".to_string(),
        clean_content: diff_types_a.to_string(),
    };
    let dt_b = IndexedSourceFile {
        path: PathBuf::from("crates/b/diff.rs"),
        rel_path: PathBuf::from("crates/b/diff.rs"),
        extension: "rs".to_string(),
        clean_content: diff_types_b.to_string(),
    };
    let dt_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![dt_a, dt_b],
    };
    if !scan_schema_dedup_opportunities(&dt_index).is_empty() {
        return false;
    }

    // Synthetic Probe 5: Unconsumed struct field in Config triggers dead config sink
    let cfg_code = r#"
        pub struct SimulationConfig {
            pub batch_limit: usize,
            pub unconsumed_dial_value: f64,
        }
    "#;
    let cfg_file = IndexedSourceFile {
        path: PathBuf::from("crates/pipeline/config.rs"),
        rel_path: PathBuf::from("crates/pipeline/config.rs"),
        extension: "rs".to_string(),
        clean_content: cfg_code.to_string(),
    };
    let cfg_consumer = r#"
        pub fn run_pipeline(cfg: &SimulationConfig) {
            println!("{}", cfg.batch_limit);
        }
    "#;
    let consumer_file = IndexedSourceFile {
        path: PathBuf::from("crates/pipeline/runner.rs"),
        rel_path: PathBuf::from("crates/pipeline/runner.rs"),
        extension: "rs".to_string(),
        clean_content: cfg_consumer.to_string(),
    };
    let cfg_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![cfg_file, consumer_file],
        polyglot_files: vec![],
    };
    if scan_dead_config_sink_opportunities(&cfg_index).is_empty() {
        return false;
    }

    // Synthetic Probe 6: Missing pause state mutation triggers state lifecycle opportunity
    let life_code = r#"
        let isSimulationPaused = false;
        const api = {
            pause: async () => {
                await sendAction("pause");
            },
            resume: async () => {
                await sendAction("resume");
            }
        };
    "#;
    let life_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/static/viewer.js"),
        rel_path: PathBuf::from("apps/app/static/viewer.js"),
        extension: "js".to_string(),
        clean_content: life_code.to_string(),
    };
    let life_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![life_file],
    };
    if scan_state_lifecycle_opportunities(&life_index).is_empty() {
        return false;
    }

    // Synthetic Probe 7: Non-canonical rounded card geometry triggers style canon opportunity
    let style_bad = r#"<div class="card rounded-2xl border p-4"><h2>Overview</h2></div>"#;
    let style_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/templates/card.html"),
        rel_path: PathBuf::from("apps/app/templates/card.html"),
        extension: "html".to_string(),
        clean_content: style_bad.to_string(),
    };
    let style_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![style_file],
    };
    if scan_style_canon_opportunities(&style_index).is_empty() {
        return false;
    }

    // Synthetic Probe 8: Non-generated artifact path triggers CI/CD artifact hygiene opportunity
    let ci_bad = r#"
        steps:
          - name: build
        artifacts:
          paths:
            - 'remediation.patch'
    "#;
    let ci_file = IndexedSourceFile {
        path: PathBuf::from("infra/cloudbuild.yaml"),
        rel_path: PathBuf::from("infra/cloudbuild.yaml"),
        extension: "yaml".to_string(),
        clean_content: ci_bad.to_string(),
    };
    let ci_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![ci_file],
    };
    if scan_ci_artifact_hygiene_opportunities(&ci_index).is_empty() {
        return false;
    }

    // Synthetic Probe 9: Monolith inline HTML blob in Rust source triggers monolith opportunity
    let mut blob_lines = String::from("pub fn render_html() -> &'static str {\n");
    for i in 0..16 {
        blob_lines.push_str(&format!(
            "    let div_{i} = \"<div class='item'>Row {i}</div>\";\n"
        ));
    }
    blob_lines.push_str("    div_0\n}\n");
    let blob_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/src/render_view.rs"),
        rel_path: PathBuf::from("apps/app/src/render_view.rs"),
        extension: "rs".to_string(),
        clean_content: blob_lines,
    };
    let blob_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![blob_file],
        polyglot_files: vec![],
    };
    if scan_monolith_blob_opportunities(&blob_index).is_empty() {
        return false;
    }

    // Synthetic Probe 10: Fragmented bespoke HMAC-SHA256 verifiers trigger primitive fragmentation
    let hmac_snippet = r#"
        let mut mac = Hmac::<Sha256>::new_from_slice(secret)?;
        mac.update(payload);
        mac.verify_slice(sig)?;
    "#;
    let h1 = IndexedSourceFile {
        path: PathBuf::from("crates/webhook_a/verifier.rs"),
        rel_path: PathBuf::from("crates/webhook_a/verifier.rs"),
        extension: "rs".to_string(),
        clean_content: hmac_snippet.to_string(),
    };
    let h2 = IndexedSourceFile {
        path: PathBuf::from("crates/webhook_b/verifier.rs"),
        rel_path: PathBuf::from("crates/webhook_b/verifier.rs"),
        extension: "rs".to_string(),
        clean_content: hmac_snippet.to_string(),
    };
    let h3 = IndexedSourceFile {
        path: PathBuf::from("crates/webhook_c/verifier.rs"),
        rel_path: PathBuf::from("crates/webhook_c/verifier.rs"),
        extension: "rs".to_string(),
        clean_content: hmac_snippet.to_string(),
    };
    let hmac_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![h1, h2, h3],
        polyglot_files: vec![],
    };
    if scan_primitive_fragmentation_opportunities(&hmac_index).is_empty() {
        return false;
    }

    // Synthetic Probe 11: Generic UI button microcopy triggers hypermedia UX opportunity
    let micro_html = r#"<button type="submit">Click Here</button>"#;
    let micro_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/templates/form.html"),
        rel_path: PathBuf::from("apps/app/templates/form.html"),
        extension: "html".to_string(),
        clean_content: micro_html.to_string(),
    };
    let micro_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![micro_file],
    };
    if scan_ux_hypermedia_opportunities(&micro_index).is_empty() {
        return false;
    }

    // Synthetic Probe 12: Uncached Argon2 derivation triggers perf cache opportunity
    let perf_code = r#"
        pub fn hash_user_key(pw: &str) -> String {
            let res = Argon2::default().hash_password(pw.as_bytes(), &salt);
            match res {
                Ok(s) => s.to_string(),
                Err(_) => String::new(),
            }
        }
    "#;
    let perf_file = IndexedSourceFile {
        path: PathBuf::from("crates/auth/hasher.rs"),
        rel_path: PathBuf::from("crates/auth/hasher.rs"),
        extension: "rs".to_string(),
        clean_content: perf_code.to_string(),
    };
    let perf_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![perf_file],
        polyglot_files: vec![],
    };
    if scan_perf_cache_opportunities(&perf_index).is_empty() {
        return false;
    }

    // Synthetic Probe 13: Naked input element triggers UX microcopy opportunity
    let ux_code = r#"<form><input name="email" type="text" /></form>"#;
    let ux_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/templates/login.html"),
        rel_path: PathBuf::from("apps/app/templates/login.html"),
        extension: "html".to_string(),
        clean_content: ux_code.to_string(),
    };
    let ux_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![ux_file],
    };
    if scan_ux_microcopy_opportunities(&ux_index).is_empty() {
        return false;
    }

    // Synthetic Probe 14: Unchunked SQL IN clause triggers SQL batch chunk opportunity
    let sql_chunk_code = r#"
        pub async fn delete_many(ids: &[String]) {
            let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let mut sql = String::from("DELETE FROM records WHERE id IN (");
            sql.push_str(&placeholders);
            sql.push(')');
            conn.execute(&sql, ()).await;
        }
    "#;
    let sql_chunk_file = IndexedSourceFile {
        path: PathBuf::from("crates/db/repo.rs"),
        rel_path: PathBuf::from("crates/db/repo.rs"),
        extension: "rs".to_string(),
        clean_content: sql_chunk_code.to_string(),
    };
    let sql_chunk_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![sql_chunk_file],
        polyglot_files: vec![],
    };
    if scan_sql_batch_chunk_opportunities(&sql_chunk_index).is_empty() {
        return false;
    }

    // Synthetic Probe 15: ROLLBACK TO SAVEPOINT without RELEASE triggers savepoint symmetry opportunity
    let tx_sym_code = r#"
        pub async fn update_safely(conn: &Connection) -> Result<()> {
            conn.execute("SAVEPOINT op", ()).await?;
            if let Err(e) = do_write(conn).await {
                conn.execute("ROLLBACK TO SAVEPOINT op", ()).await?;
                return Err(e);
            }
            Ok(())
        }
    "#;
    let tx_sym_file = IndexedSourceFile {
        path: PathBuf::from("crates/db/tx.rs"),
        rel_path: PathBuf::from("crates/db/tx.rs"),
        extension: "rs".to_string(),
        clean_content: tx_sym_code.to_string(),
    };
    let tx_sym_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![tx_sym_file],
        polyglot_files: vec![],
    };
    if scan_tx_savepoint_symmetry_opportunities(&tx_sym_index).is_empty() {
        return false;
    }

    // Synthetic Probe 16: Unguarded ON CONFLICT update on status triggers upsert lattice opportunity
    let upsert_code = r#"
        pub async fn save_record(conn: &Connection) {
            let sql = "INSERT INTO orders (id, status) VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET status = excluded.status";
            conn.execute(sql, ()).await;
        }
    "#;
    let upsert_file = IndexedSourceFile {
        path: PathBuf::from("crates/db/orders.rs"),
        rel_path: PathBuf::from("crates/db/orders.rs"),
        extension: "rs".to_string(),
        clean_content: upsert_code.to_string(),
    };
    let upsert_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![upsert_file],
        polyglot_files: vec![],
    };
    if scan_upsert_lattice_opportunities(&upsert_index).is_empty() {
        return false;
    }

    // Synthetic Probe 17: Dense noise perturbation before top_k triggers compute pushdown opportunity
    let pushdown_code = r#"
        pub fn process_gradients(delta: &mut [f32], k: usize) -> CompressedSparse {
            dp::add_gaussian_noise(delta, 1.0, 1.0);
            freshcredit_fl_compression::compress(delta, k)
        }
    "#;
    let pushdown_file = IndexedSourceFile {
        path: PathBuf::from("crates/fl/pipeline.rs"),
        rel_path: PathBuf::from("crates/fl/pipeline.rs"),
        extension: "rs".to_string(),
        clean_content: pushdown_code.to_string(),
    };
    let pushdown_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![pushdown_file],
        polyglot_files: vec![],
    };
    if scan_compute_pushdown_opportunities(&pushdown_index).is_empty() {
        return false;
    }

    // Synthetic Probe 18: Unlabeled input triggers UX microcopy opportunity; arrow-prop input does not
    let unlabeled_input = r#"<input type="text" name="address" />"#;
    let labeled_input =
        r#"<input type="text" onChange={(e) => foo(e)} placeholder="Enter name" />"#;
    let pos_ux_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/templates/pos.html"),
        rel_path: PathBuf::from("apps/app/templates/pos.html"),
        extension: "html".to_string(),
        clean_content: unlabeled_input.to_string(),
    };
    let neg_ux_file = IndexedSourceFile {
        path: PathBuf::from("apps/app/templates/neg.html"),
        rel_path: PathBuf::from("apps/app/templates/neg.html"),
        extension: "html".to_string(),
        clean_content: labeled_input.to_string(),
    };
    let pos_ux_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![pos_ux_file],
    };
    let neg_ux_index = WorkspaceSourceIndex {
        root: PathBuf::from("."),
        inventory: WorkspaceInventory::default(),
        rust_files: vec![],
        polyglot_files: vec![neg_ux_file],
    };
    if scan_ux_microcopy_opportunities(&pos_ux_index).is_empty()
        || !scan_ux_microcopy_opportunities(&neg_ux_index).is_empty()
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opportunity_scan_runs_cleanly() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.parent().unwrap().parent().unwrap();
        let report = scan_all_workspace_opportunities(root);
        assert!(verify_synthetic_opportunity_probes());
        let _ = report.total_opportunities_discovered;
    }

    #[test]
    fn test_bodiless_trait_methods_do_not_produce_clones() {
        let trait_code = r#"
            pub trait ExampleRepo {
                async fn find_one(&self, id: &str) -> Result<Option<String>>;
                async fn find_many(&self, filter: &str) -> Result<Vec<String>>;
                async fn delete_one(&self, id: &str) -> Result<bool>;
                async fn update_status(&self, id: &str, status: &str) -> Result<()>;
            }
        "#;
        let file = IndexedSourceFile {
            path: PathBuf::from("traits.rs"),
            rel_path: PathBuf::from("traits.rs"),
            extension: "rs".to_string(),
            clean_content: trait_code.to_string(),
        };
        let index = WorkspaceSourceIndex {
            root: PathBuf::from("."),
            inventory: WorkspaceInventory::default(),
            rust_files: vec![file.clone()],
            polyglot_files: vec![file],
        };
        let clones = scan_clone_consolidation_opportunities(&index);
        assert!(
            clones.is_empty(),
            "Bodiless trait declarations must not produce clone signals"
        );
    }
}
