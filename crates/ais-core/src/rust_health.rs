//! Rust-Specific Idiomatic Health & Safety Metrics Engine
//!
//! Tracks language-level safety and architecture invariants:
//! - Unsafe block density (Target: 0 in binary crates, < 1% in core libraries)
//! - Production unwrap() & expect() panics outside #[cfg(test)]
//! - Dependency count from Cargo.lock (Target: < 250 unique third-party crates)
//! - Dynamic vs Static dispatch balance (dyn Trait vs generic monomorphization)
//! - Defect / Bug density normalized per thousand lines of code (KLOC)

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use walkdir::WalkDir;

static UNSAFE_RE: LazyLock<Regex> =
    LazyLock::new(
        || match Regex::new(r"\bunsafe\s*(?:\{|\bfn\b|\bimpl\b|\btrait\b)") {
            Ok(r) => r,
            Err(_) => unreachable!(),
        },
    );

static UNWRAP_RE: LazyLock<Regex> = LazyLock::new(|| match Regex::new(r"\.unwrap\(\)") {
    Ok(r) => r,
    Err(_) => unreachable!(),
});

static EXPECT_RE: LazyLock<Regex> = LazyLock::new(|| match Regex::new(r"\.expect\(") {
    Ok(r) => r,
    Err(_) => unreachable!(),
});

static PROD_ASSERT_RE: LazyLock<Regex> =
    LazyLock::new(|| match Regex::new(r"\bassert(?:_eq|_ne)?!\s*\(") {
        Ok(r) => r,
        Err(_) => unreachable!(),
    });

static DYN_TRAIT_RE: LazyLock<Regex> = LazyLock::new(|| match Regex::new(r"\bdyn\s+[A-Z]") {
    Ok(r) => r,
    Err(_) => unreachable!(),
});

static GENERIC_IMPL_RE: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"\b(?:impl\s*<|fn\s+[a-zA-Z0-9_]+\s*<|impl\s+[A-Z][a-zA-Z0-9_]*\s+for|:\s*impl\s+[A-Z]|->\s*impl\s+[A-Z])",
    ) {
        Ok(r) => r,
        Err(_) => unreachable!(),
    }
});

static CARGO_NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| match Regex::new(r#"(?m)^\s*name\s*=\s*"([^"]+)""#) {
        Ok(r) => r,
        Err(_) => unreachable!(),
    });

/// Comprehensive Rust health report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RustHealthReport {
    /// Total production Rust lines of code (excluding #[cfg(test)])
    pub production_loc: usize,
    /// Number of unsafe blocks in production code
    pub unsafe_blocks: usize,
    /// Percentage of lines containing unsafe blocks
    pub unsafe_density_pct: f64,
    /// Count of unwrap() calls in production paths
    pub production_unwraps: usize,
    /// Count of expect() calls in production paths
    pub production_expects: usize,
    /// Count of panicking assert! calls in production route/middleware/service paths
    pub production_asserts: usize,
    /// Count of dynamic dispatch (dyn Trait) instances
    pub dynamic_dispatch_count: usize,
    /// Count of generic monomorphized impls
    pub static_dispatch_count: usize,
    /// Unique crate count from Cargo.lock
    pub unique_dependency_crates: usize,
    /// Normalized defect density (bugs per 1,000 LOC)
    pub defect_density_per_kloc: f64,
    /// Whether `cargo-audit` binary is installed and available
    #[serde(default)]
    pub cargo_audit_available: bool,
    /// Whether `cargo-geiger` binary is installed and available
    #[serde(default)]
    pub cargo_geiger_available: bool,
    /// Explicit advisory warnings when static/security audit tools are missing
    #[serde(default)]
    pub tool_warnings: Vec<String>,
}

fn is_cargo_subcommand_available(bin_name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join(bin_name).is_file() {
                return true;
            }
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let cargo_bin = Path::new(&home).join(".cargo").join("bin").join(bin_name);
        if cargo_bin.is_file() {
            return true;
        }
    }
    false
}

fn strip_string_literals_for_scan(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    let mut escaped = false;
    while let Some(c) = chars.next() {
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
                out.push('"');
            }
        } else if c == '"' {
            in_str = true;
            out.push('"');
        } else {
            out.push(c);
        }
    }
    out
}

/// Analyze workspace Rust health invariants
pub fn analyze_workspace_rust_health(
    root: &Path,
    active_findings_count: usize,
) -> RustHealthReport {
    let mut production_loc = 0;
    let mut unsafe_blocks = 0;
    let mut production_unwraps = 0;
    let mut production_expects = 0;
    let mut production_asserts = 0;
    let mut dynamic_dispatch_count = 0;
    let mut static_dispatch_count = 0;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir()
                && crate::structure::is_ignored_directory_name(&e.file_name().to_string_lossy())
            {
                return false;
            }
            !is_ignored_path(e.path())
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let path_str = path.to_string_lossy();
        if crate::is_test_or_mock_path(path) || path_str.contains("canary") {
            continue;
        }

        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };

        // Strip scoped #[cfg(test)] blocks with balanced brace parser
        let prod_string = crate::strip_cfg_test_blocks(&content);
        let prod_content = &prod_string;

        let loc = prod_content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .count();
        production_loc += loc;

        for l in prod_content.lines() {
            let trimmed = l.trim();
            if trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with('#')
            {
                continue;
            }

            let code_only = strip_string_literals_for_scan(trimmed);
            unsafe_blocks += UNSAFE_RE.find_iter(&code_only).count();
            production_unwraps += UNWRAP_RE.find_iter(&code_only).count();
            production_expects += EXPECT_RE.find_iter(&code_only).count();

            if PROD_ASSERT_RE.is_match(&code_only) {
                production_asserts += 1;
            }

            for cap in DYN_TRAIT_RE.captures_iter(trimmed) {
                if let Some(m) = cap.get(0) {
                    let rest = &trimmed[m.end()..];
                    let ident = rest
                        .split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or("");
                    if ident != "Error"
                        && ident != "StdError"
                        && ident != "Future"
                        && ident != "Any"
                        && ident != "Display"
                        && ident != "Debug"
                        && ident != "Stream"
                        && ident != "Read"
                        && ident != "Write"
                        && ident != "Seek"
                        && ident != "Iterator"
                        && !ident.starts_with("Fn")
                        && !ident.ends_with("Repo")
                        && !ident.ends_with("Repository")
                        && !ident.ends_with("Port")
                        && !ident.ends_with("Service")
                        && !ident.ends_with("Provider")
                        && !ident.ends_with("Client")
                        && !ident.ends_with("Store")
                        && !ident.ends_with("Adapter")
                        && !ident.ends_with("Handler")
                        && !ident.ends_with("Factory")
                        && !ident.ends_with("Manager")
                        && !ident.ends_with("Backend")
                        && !ident.ends_with("Verifier")
                        && !ident.ends_with("Validator")
                        && !ident.ends_with("Signer")
                        && !ident.ends_with("Resolver")
                        && !ident.ends_with("Executor")
                        && !ident.ends_with("Dispatcher")
                        && !ident.ends_with("Subscriber")
                        && !ident.ends_with("Transport")
                        && !ident.ends_with("Pipeline")
                        && !ident.ends_with("Sender")
                        && !ident.ends_with("Receiver")
                        && !ident.ends_with("Api")
                        && !ident.ends_with("Processor")
                        && !ident.ends_with("Registry")
                        && !ident.ends_with("Sink")
                        && !ident.ends_with("Source")
                        && !ident.ends_with("Policy")
                        && !ident.ends_with("Strategy")
                        && !ident.ends_with("Driver")
                        && !ident.ends_with("Oracle")
                        && !ident.ends_with("Clock")
                        && !ident.ends_with("Ledger")
                        && !ident.ends_with("Engine")
                        && !ident.ends_with("Kernel")
                        && !ident.ends_with("Authenticator")
                        && !ident.ends_with("Authorizer")
                        && !ident.ends_with("Encryptor")
                        && !ident.ends_with("Decryptor")
                        && !ident.ends_with("Config")
                        && !ident.ends_with("Connection")
                    {
                        dynamic_dispatch_count += 1;
                    }
                }
            }
        }

        static_dispatch_count += GENERIC_IMPL_RE.find_iter(prod_content).count();
    }

    let unique_dependency_crates = count_unique_cargo_crates(root);

    let unsafe_density_pct = if production_loc > 0 {
        ((unsafe_blocks as f64 / production_loc as f64) * 100.0 * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    let defect_density_per_kloc = if production_loc > 0 {
        ((active_findings_count as f64 / (production_loc as f64 / 1000.0)) * 100.0).round() / 100.0
    } else {
        0.0
    };

    let cargo_audit_available = is_cargo_subcommand_available("cargo-audit");
    let cargo_geiger_available = is_cargo_subcommand_available("cargo-geiger");
    let mut tool_warnings = Vec::new();
    if !cargo_audit_available {
        tool_warnings
            .push("cargo-audit not installed; CVE dependency verification degraded".to_string());
    }
    if !cargo_geiger_available {
        tool_warnings.push(
            "cargo-geiger not installed; transitive unsafe crate verification degraded".to_string(),
        );
    }

    RustHealthReport {
        production_loc,
        unsafe_blocks,
        unsafe_density_pct,
        production_unwraps,
        production_expects,
        production_asserts,
        dynamic_dispatch_count,
        static_dispatch_count,
        unique_dependency_crates,
        defect_density_per_kloc,
        cargo_audit_available,
        cargo_geiger_available,
        tool_warnings,
    }
}

fn is_ignored_path(p: &Path) -> bool {
    let s = p.to_string_lossy();
    s.contains("/target/")
        || s.contains("/node_modules/")
        || s.contains("/.git/")
        || s.contains("/.cargo/")
        || s.contains("/.rustup/")
        || s.contains("/.turso/")
        || s.contains("/fixtures/")
        || s.contains("/docs/_archive/")
        || s.contains("/examples/")
        || s.contains("xtask/")
}

fn count_unique_cargo_crates(root: &Path) -> usize {
    let lock_path = root.join("Cargo.lock");
    let Ok(lock_content) = fs::read_to_string(lock_path) else {
        return 0;
    };

    let mut names = HashSet::new();

    for cap in CARGO_NAME_RE.captures_iter(&lock_content) {
        names.insert(cap[1].to_string());
    }

    names.len()
}
