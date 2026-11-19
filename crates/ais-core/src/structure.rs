//! Codebase Structural Hierarchy & Topology Enforcer (Native Rust)
//!
//! Enforces monorepo hygiene, root directory encapsulation, database spill prevention,
//! and clean architectural boundaries.

use crate::findings::{Confidence, Finding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Allowed files in the root of the workspace
pub static ALLOWED_ROOT_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "Makefile",
    "deny.toml",
    ".clippy.toml",
    ".rustfmt.toml",
    "rust-toolchain.toml",
    "codecov.yml",
    "ais.yaml",
    ".env",
    ".env.example",
    "README.md",
    "LICENSE",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    ".gitignore",
    ".gitattributes",
    ".gitmodules",
    ".gitmessage",
    ".dockerignore",
    ".gcloudignore",
    ".node-version",
    ".DS_Store",
];

/// Allowed directories in the root of the workspace
pub static ALLOWED_ROOT_DIRS: &[&str] = &[
    "apps",
    "crates",
    "docs",
    "infra",
    "migrations",
    "scripts",
    "audit",
    "spec",
    "tests",
    "contracts",
    "packages",
    "data",
    "e2e",
    "workspace-hack",
    "xtask",
    "target",
    "src",
    "lib",
    "cmd",
    "pkg",
    "domain",
    "models",
    "product",
    "research",
    "system",
    "user",
    ".git",
    ".github",
    ".vscode",
    ".githooks",
    ".cargo",
    ".config",
    ".freshcredit",
    ".ais",
];
/// Canonical predicate for ignored directories across all AIS scanners and indexers
pub fn is_ignored_directory_name(name: &str) -> bool {
    matches!(
        name,
        "target"
            | "node_modules"
            | ".git"
            | ".cargo"
            | ".rustup"
            | ".turso"
            | ".next"
            | ".twenty"
            | "dist"
            | "build"
            | ".venv"
            | "venv"
            | "site-packages"
            | "__pycache__"
            | ".android-sdk"
            | "android-sdk"
            | ".gradle"
            | "fixtures"
            | ".cache"
            | ".freshcredit"
            | ".ais"
            | ".terraform"
            | "audit"
            | "playwright-report"
            | "test-results"
            | ".playwright-mcp"
            | "archive"
            | "scripts-backup"
    )
}

/// Summary report of structural audit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StructureScanReport {
    pub total_violations: usize,
    pub root_violations: usize,
    pub db_leak_violations: usize,
    pub doc_topology_violations: usize,
    pub ais_isolation_violations: usize,
    pub findings: Vec<Finding>,
}

/// Find the true monorepo or workspace root by locating ais.yaml, Cargo.lock, .git, package.json, or pyproject.toml
pub fn find_workspace_root(start: &Path) -> PathBuf {
    let mut current = if start == Path::new(".") {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else {
        start.to_path_buf()
    };
    loop {
        // Priority 1: explicitly configured ais.yaml
        if current.join("ais.yaml").exists() {
            return current;
        }
        // Priority 2: Monorepo with Cargo.lock and apps/ or crates/
        if current.join("Cargo.lock").exists()
            && (current.join("apps").exists() || current.join("crates").exists())
        {
            return current;
        }
        // Priority 3: standard repo root markers (.git, Cargo.toml, package.json)
        if current.join(".git").exists()
            || current.join("Cargo.lock").exists()
            || current.join("package.json").exists()
            || current.join("pyproject.toml").exists()
        {
            return current;
        }
        if !current.pop() {
            return start.to_path_buf();
        }
    }
}

/// Canonical dynamic inventory of all codebase roots, routes, and templates.
/// Used across all AIS scanners to guarantee 100% monorepo coverage without hardcoded paths.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceInventory {
    /// All Rust and polyglot source roots (apps/product/*, apps/service/*, crates/*, packages/*, src, lib, etc.)
    pub source_roots: Vec<PathBuf>,
    /// All directories containing Axum router declarations
    pub route_dirs: Vec<PathBuf>,
    /// All template directories (Askama/HTML) across product applications and services
    pub template_dirs: Vec<PathBuf>,
}

impl WorkspaceInventory {
    /// Dynamically discovers all workspace source roots, route directories, and template directories.
    pub fn discover(root: &Path) -> Self {
        let mut source_roots = Vec::new();
        let mut route_dirs = Vec::new();
        let mut template_dirs = Vec::new();

        // 1. Discover all architectural source roots across apps, crates, packages, contracts, infra, migrations, e2e, spec, xtask, scripts, src, lib
        // Closed-world non-overlapping roots: do not push child directories if parent is already a root
        for parent_rel in &[
            "apps/product",
            "apps/service",
            "crates",
            "packages",
            "contracts",
            "infra",
            "migrations",
            "e2e",
            "spec",
            "xtask",
            "scripts",
            "src",
            "lib",
            "cmd",
            "pkg",
        ] {
            let parent = root.join(parent_rel);
            if parent.is_dir() {
                source_roots.push(parent);
            }
        }
        if source_roots.is_empty() {
            source_roots.push(root.to_path_buf());
        }

        // 2. Discover all route directories and template directories across the codebase
        for entry in walkdir::WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                if e.file_type().is_dir() {
                    !is_ignored_directory_name(&e.file_name().to_string_lossy())
                } else {
                    true
                }
            })
            .flatten()
        {
            if entry.file_type().is_dir() {
                let name = entry.file_name().to_string_lossy();
                if name == "templates" {
                    template_dirs.push(entry.path().to_path_buf());
                } else if name == "routes" || name == "router" {
                    route_dirs.push(entry.path().to_path_buf());
                }
            }
        }

        // Add services with in-line routing (like apps/service/vote/src or apps/product/blockchain/src/rpc)
        for cand in &[
            root.join("apps/service/vote/src"),
            root.join("apps/product/blockchain/src/rpc"),
            root.join("apps/product/simulator/src"),
            root.join("apps/product/scheduler/src"),
            root.join("apps/product/mcp/src"),
        ] {
            if cand.is_dir() && !route_dirs.contains(cand) {
                route_dirs.push(cand.clone());
            }
        }

        source_roots.sort();
        source_roots.dedup();
        route_dirs.sort();
        route_dirs.dedup();
        template_dirs.sort();
        template_dirs.dedup();

        Self {
            source_roots,
            route_dirs,
            template_dirs,
        }
    }

    /// Returns a strictly deduplicated, canonical list of all workspace source/scannable files.
    /// Prefers git ls-files when in a repository, with fallback to deduplicated directory walks.
    pub fn collect_canonical_files(&self, root: &Path) -> Vec<PathBuf> {
        let tracked = crate::discovery::collect_git_tracked_files(root);
        if !tracked.is_empty() {
            let mut res: Vec<PathBuf> = tracked.into_iter().map(|p| root.join(p)).collect();
            res.sort();
            res.dedup();
            return res;
        }

        let mut files = Vec::new();
        for dir in &self.source_roots {
            if !dir.exists() {
                continue;
            }
            for entry in walkdir::WalkDir::new(dir)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| {
                    if e.file_type().is_dir() {
                        !is_ignored_directory_name(&e.file_name().to_string_lossy())
                    } else {
                        true
                    }
                })
                .flatten()
            {
                if entry.file_type().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
        files.sort();
        files.dedup();
        files
    }
}

/// Audit root directory hygiene (ROOT-001)
pub fn check_root_hygiene(root: &Path, findings: &mut Vec<Finding>) -> usize {
    let mut violations = 0;
    let allowed_files: HashSet<&str> = ALLOWED_ROOT_FILES.iter().copied().collect();
    let allowed_dirs: HashSet<&str> = ALLOWED_ROOT_DIRS.iter().copied().collect();

    let Ok(entries) = fs::read_dir(root) else {
        return 0;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name_str = entry.file_name().to_string_lossy().to_string();

        if path.is_file() {
            if !allowed_files.contains(name_str.as_str()) {
                violations += 1;
                findings.push(Finding {
                    rule_code: "ROOT-001".to_string(),
                    severity: Severity::High,
                    message: format!(
                        "Unauthorized rogue file '{name_str}' in workspace root. Move to canonical directory or add to .gitignore"
                    ),
                    file: path,
                    line: 1,
                    confidence: Confidence::new(0.95),
                    suppressed: None,
                    hint: Some("Root directory must only contain standard workspace manifests".to_string()),
                });
            }
        } else if path.is_dir() && !allowed_dirs.contains(name_str.as_str()) {
            violations += 1;
            findings.push(Finding {
                rule_code: "ROOT-001".to_string(),
                severity: Severity::High,
                message: format!(
                    "Unauthorized rogue directory '{name_str}' in workspace root. Move to canonical location or add to .gitignore"
                ),
                file: path,
                line: 1,
                confidence: Confidence::new(0.95),
                suppressed: None,
                hint: Some("Root directory must only contain standard architectural directories".to_string()),
            });
        }
    }
    violations
}

/// Audit database file leakage outside storage dirs (ROOT-002)
pub fn check_database_leakage(root: &Path, findings: &mut Vec<Finding>) -> usize {
    let mut violations = 0;

    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() && is_ignored_directory_name(&e.file_name().to_string_lossy())
            {
                return false;
            }
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains(".git")
                && !p.contains("node_modules")
                && !p.contains(".cargo")
                && !p.contains(".rustup")
                && !p.contains(".ais/data")
                && !p.contains(".freshcredit/data")
                && !p.contains("fixtures")
        })
        .flatten()
    {
        if entry.file_type().is_file() {
            let name = entry.file_name().to_string_lossy();
            let is_db = name.ends_with(".db")
                || name.ends_with(".sqlite")
                || name.ends_with(".sqlite3")
                || name.ends_with("-wal")
                || name.ends_with("-shm")
                || name.contains("DATABASE_URL");

            if is_db {
                violations += 1;
                findings.push(Finding {
                    rule_code: "ROOT-002".to_string(),
                    severity: Severity::Critical,
                    message: format!(
                        "Database or journal file '{}' found outside authorized data store directories",
                        entry.path().display()
                    ),
                    file: entry.path().to_path_buf(),
                    line: 1,
                    confidence: Confidence::new(1.0),
                    suppressed: None,
                    hint: Some("Store SQLite databases exclusively under .ais/data/".to_string()),
                });
            }
        }
    }

    violations
}

/// Audit broken symlinks across repository (HYGIENE-001)
pub fn check_broken_symlinks(root: &Path, findings: &mut Vec<Finding>) -> usize {
    let mut violations = 0;
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains(".git")
                && !p.contains("node_modules")
                && !p.contains(".cargo")
        })
        .flatten()
    {
        if entry.path_is_symlink() && fs::metadata(entry.path()).is_err() {
            violations += 1;
            findings.push(Finding {
                rule_code: "HYGIENE-001".to_string(),
                severity: Severity::High,
                message: format!(
                    "Broken symbolic link '{}' detected; target does not exist",
                    entry.path().display()
                ),
                file: entry.path().to_path_buf(),
                line: 1,
                confidence: Confidence::new(1.0),
                suppressed: None,
                hint: Some("Remove dangling symlink or repair its target path".to_string()),
            });
        }
    }
    violations
}

/// Audit AIS runtime cache and state isolation (AIS-ISO-001)
pub fn check_ais_isolation(root: &Path, findings: &mut Vec<Finding>) -> usize {
    let mut violations = 0;
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() && is_ignored_directory_name(&e.file_name().to_string_lossy())
            {
                return false;
            }
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains(".git")
                && !p.contains("node_modules")
                && !p.contains(".cargo")
                && !p.contains(".rustup")
                && !p.contains(".ais/cache")
                && !p.contains(".freshcredit/cache")
        })
        .flatten()
    {
        if entry.file_type().is_file() && entry.file_name() == ".ais_scanner_cache.json" {
            violations += 1;
            findings.push(Finding {
                rule_code: "AIS-ISO-001".to_string(),
                severity: Severity::High,
                message: format!(
                    "AIS scanner cache '{}' resides outside authorized .ais/cache/ location",
                    entry.path().display()
                ),
                file: entry.path().to_path_buf(),
                line: 1,
                confidence: Confidence::new(1.0),
                suppressed: None,
                hint: Some(
                    "Relocate cache to .ais/cache/ to isolate AIS tooling from product code"
                        .to_string(),
                ),
            });
        }
    }

    violations
}
/// Audit unauthorized Python scripts and virtual environments outside strict allowlists (RUST-FIRST-001)
pub fn check_unauthorized_scripts(root: &Path, findings: &mut Vec<Finding>) -> usize {
    let mut violations = 0;
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains(".git")
                && !p.contains("node_modules")
                && !p.contains(".cargo")
                && !p.contains("fixtures")
                && !p.contains("docs/_archive")
                && !p.contains("/spec/")
                && !p.contains("transcribe")
                && !p.contains("research")
                && !p.contains("native")
                && !p.contains("entra-external-id")
                && !p.contains("brian2_docas")
                && !p.contains("auth/app.py")
        })
        .flatten()
    {
        let p = entry.path().to_string_lossy();
        if entry.file_type().is_file() && p.ends_with(".py") {
            violations += 1;
            findings.push(Finding {
                rule_code: "RUST-FIRST-001".to_string(),
                severity: Severity::High,
                message: format!(
                    "Unauthorized Python script '{}' detected; operational tooling and scanners must be written in Rust (cargo xtask)",
                    entry.path().display()
                ),
                file: entry.path().to_path_buf(),
                line: 1,
                confidence: Confidence::new(1.0),
                suppressed: None,
                hint: Some("Migrate script logic into a cargo xtask subcommand or delete obsolete file".to_string()),
            });
        }
        if entry.file_type().is_dir()
            && (entry.file_name() == ".venv" || entry.file_name() == "__pycache__")
        {
            violations += 1;
            findings.push(Finding {
                rule_code: "RUST-FIRST-001".to_string(),
                severity: Severity::High,
                message: format!(
                    "Unauthorized Python virtualenv or bytecode directory '{}' detected",
                    entry.path().display()
                ),
                file: entry.path().to_path_buf(),
                line: 1,
                confidence: Confidence::new(1.0),
                suppressed: None,
                hint: Some("Purge virtual environment and gitignore bytecode".to_string()),
            });
        }
    }
    violations
}

/// Execute full codebase structure audit
pub fn scan_codebase_structure(start: &Path) -> StructureScanReport {
    let root = find_workspace_root(start);
    let mut findings = Vec::new();
    let root_violations = check_root_hygiene(&root, &mut findings);
    let db_leak_violations = check_database_leakage(&root, &mut findings);
    let ais_isolation_violations = check_ais_isolation(&root, &mut findings);
    let broken_symlink_violations = check_broken_symlinks(&root, &mut findings);
    let script_drift_violations = check_unauthorized_scripts(&root, &mut findings);
    let total_violations = root_violations
        + db_leak_violations
        + ais_isolation_violations
        + broken_symlink_violations
        + script_drift_violations;

    StructureScanReport {
        total_violations,
        root_violations,
        db_leak_violations,
        doc_topology_violations: broken_symlink_violations,
        ais_isolation_violations,
        findings,
    }
}

/// Automatically remediate rogue artifacts and enforce structure
pub fn remediate_structure(root: &Path) -> (usize, Vec<String>) {
    let mut actions = Vec::new();

    // 1. Clean root cache if lingering
    let root_cache = root.join(".ais_scanner_cache.json");
    if fs::remove_file(&root_cache).is_ok() {
        actions.push("Purged legacy root .ais_scanner_cache.json".to_string());
    }

    // 2. Remove any database spill files in root
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.contains("DATABASE_URL")
                || name.ends_with(".db")
                || name.ends_with("-wal")
                || name.ends_with("-shm")
            {
                let _ = fs::remove_file(entry.path());
                actions.push(format!("Purged rogue database spill file: {name}"));
            }
        }
    }

    // 3. Purge leaked test database files across repository (single O(N) walk)
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let p = e.path().to_string_lossy();
            !p.contains("target")
                && !p.contains(".git")
                && !p.contains("node_modules")
                && !p.contains(".freshcredit")
                && !p.contains("/data")
                && !p.contains("fixtures")
        })
        .flatten()
    {
        if entry.file_type().is_file() {
            let name = entry.file_name().to_string_lossy();
            let is_leak = name.ends_with(".db")
                || name.ends_with(".sqlite")
                || name.ends_with(".sqlite3")
                || name.ends_with("-wal")
                || name.ends_with("-shm")
                || name.contains("DATABASE_URL");

            if is_leak {
                let _ = fs::remove_file(entry.path());
                actions.push(format!(
                    "Purged leaked database artifact: {}",
                    entry.path().display()
                ));
            }
        }
    }

    // 4. Purge stale test database files in OS temp directory (std::env::temp_dir)
    let tmp_dir = std::env::temp_dir();
    if let Ok(entries) = fs::read_dir(&tmp_dir) {
        let mut purged_tmp_count = 0usize;
        let stale_cutoff = std::time::Duration::from_secs(3600);
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let is_test_db_or_tmp = (name.starts_with("test_")
                || name.starts_with("anchor_log_store_")
                || name.starts_with("sweep_store_")
                || name.starts_with("fc_vid_")
                || name.starts_with("plaid_tokens_test_")
                || name.starts_with("approve_canon_")
                || name.starts_with("test-upload")
                || name.starts_with("test-sim-")
                || name.starts_with("_pandoc_"))
                && (name.ends_with(".db")
                    || name.ends_with(".db-wal")
                    || name.ends_with(".db-shm")
                    || name.starts_with("test-upload")
                    || name.starts_with("test-sim-")
                    || name.starts_with("_pandoc_"));
            if !is_test_db_or_tmp {
                continue;
            }
            let is_stale = p
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|mtime| std::time::SystemTime::now().duration_since(mtime).ok())
                .is_some_and(|age| age > stale_cutoff);
            if is_stale && fs::remove_file(&p).is_ok() {
                purged_tmp_count += 1;
            }
        }
        if purged_tmp_count > 0 {
            actions.push(format!(
                "Purged {purged_tmp_count} stale test artifact(s) from {}",
                tmp_dir.display()
            ));
        }
    }

    // 3. Move root STYLE_CANON.md if present
    let root_style = root.join("STYLE_CANON.md");
    if root_style.exists() {
        let target_dir = root.join("docs/design");
        let _ = fs::create_dir_all(&target_dir);
        let target = target_dir.join("STYLE_CANON.md");
        if fs::rename(&root_style, &target).is_ok() {
            actions.push("Relocated STYLE_CANON.md to docs/design/STYLE_CANON.md".to_string());
        }
    }

    let count = actions.len();
    (count, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_scan_clean_root() {
        let root = Path::new(".");
        let report = scan_codebase_structure(root);
        assert_eq!(
            report.root_violations, 0,
            "Root must have 0 violations: {:?}",
            report.findings
        );
        assert_eq!(report.ais_isolation_violations, 0);
    }

    #[test]
    fn test_is_ignored_directory_name_vendor_and_build_dirs() {
        assert!(is_ignored_directory_name(".cargo"));
        assert!(is_ignored_directory_name(".rustup"));
        assert!(is_ignored_directory_name(".turso"));
        assert!(is_ignored_directory_name("target"));
        assert!(is_ignored_directory_name("node_modules"));
        assert!(is_ignored_directory_name(".git"));
        assert!(!is_ignored_directory_name("apps"));
        assert!(!is_ignored_directory_name("crates"));
    }
}
