//! Architectural Coupling & Instability Engine (SDP Compliance)
//!
//! Computes Afferent Coupling (Ca), Efferent Coupling (Ce), and the
//! Instability Index (I = Ce / (Ca + Ce)) across workspace crates
//! following Robert C. Martin's Stable Dependencies Principle (SDP).

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Coupling and stability metrics for a workspace crate or module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateCouplingMetric {
    /// Crate name
    pub crate_name: String,
    /// Crate relative directory path
    pub path: PathBuf,
    /// Afferent Coupling: number of other workspace crates depending on this crate
    pub ca_inbound: usize,
    /// Efferent Coupling: number of other workspace crates this crate depends on
    pub ce_outbound: usize,
    /// Instability Index: I = Ce / (Ca + Ce), range [0.0, 1.0]
    pub instability: f64,
    /// Architectural archetype classification
    pub archetype: CrateArchetype,
}

/// Architectural role of a component based on stability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrateArchetype {
    /// Highly stable core foundation (I <= 0.35): difficult to change, many dependents
    StableCore,
    /// Balanced mediator (0.35 < I < 0.75): intermediate service or pipeline
    Balanced,
    /// Volatile edge/leaf (I >= 0.75): easy to change, few/no downstream dependents
    VolatileEdge,
}

/// Full workspace coupling report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceCouplingReport {
    /// Total workspace crates analyzed
    pub total_crates: usize,
    /// Critical path depth of dependencies in the workspace DAG
    pub critical_path_depth: usize,
    /// Estimated parallel compilation build span in seconds (Brent's Theorem)
    pub estimated_parallel_span_seconds: f64,
    /// Average instability across foundation/shared crates (SDP invariant)
    pub avg_instability: f64,
    /// Detailed per-crate coupling metrics
    pub crates: Vec<CrateCouplingMetric>,
}

/// Analyze workspace crate coupling and instability
pub fn analyze_workspace_coupling(root: &Path) -> WorkspaceCouplingReport {
    let cargo_toml_path = root.join("Cargo.toml");
    let Ok(cargo_str) = fs::read_to_string(&cargo_toml_path) else {
        return WorkspaceCouplingReport::default();
    };

    let members = extract_workspace_members(&cargo_str);
    if members.is_empty() {
        return WorkspaceCouplingReport::default();
    }

    let mut crate_paths: HashMap<String, PathBuf> = HashMap::new();
    let mut crate_names: HashMap<PathBuf, String> = HashMap::new();

    for m in &members {
        let member_path = root.join(m);
        let pkg_cargo = member_path.join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&pkg_cargo) {
            if let Some(name) = extract_package_name(&content) {
                crate_paths.insert(name.clone(), PathBuf::from(m));
                crate_names.insert(PathBuf::from(m), name);
            }
        }
    }

    let mut outbound_deps: HashMap<String, HashSet<String>> = HashMap::new();
    let mut inbound_deps: HashMap<String, HashSet<String>> = HashMap::new();

    for name in crate_paths.keys() {
        outbound_deps.insert(name.clone(), HashSet::new());
        inbound_deps.insert(name.clone(), HashSet::new());
    }

    for (name, path) in &crate_paths {
        let pkg_cargo = root.join(path).join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&pkg_cargo) {
            let deps = extract_dependencies(&content);
            for d in deps {
                if crate_paths.contains_key(&d) && &d != name {
                    outbound_deps
                        .entry(name.clone())
                        .or_default()
                        .insert(d.clone());
                    inbound_deps.entry(d).or_default().insert(name.clone());
                }
            }
        }
    }

    let mut metrics = Vec::new();

    for (name, path) in &crate_paths {
        let ca = inbound_deps.get(name).map(|s| s.len()).unwrap_or(0);
        let ce = outbound_deps.get(name).map(|s| s.len()).unwrap_or(0);
        let instability = if ca + ce > 0 {
            ce as f64 / (ca + ce) as f64
        } else {
            0.0
        };

        let archetype = if instability <= 0.35 {
            CrateArchetype::StableCore
        } else if instability >= 0.75 {
            CrateArchetype::VolatileEdge
        } else {
            CrateArchetype::Balanced
        };

        metrics.push(CrateCouplingMetric {
            crate_name: name.clone(),
            path: path.clone(),
            ca_inbound: ca,
            ce_outbound: ce,
            instability: (instability * 100.0).round() / 100.0,
            archetype,
        });
    }

    metrics.sort_by(|a, b| {
        a.instability
            .partial_cmp(&b.instability)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.ca_inbound.cmp(&a.ca_inbound))
    });

    // Foundation Core Instability Index (Martin 2002 SDP invariant):
    // Measures the stability of shared foundation crates that other crates depend on (Ca > 0).
    // In any directed graph, the sum of all Ce strictly equals the sum of all Ca, making raw total_ce/total_degree
    // mathematically invariant at 0.50 (Euler's Handshaking Lemma). SDP specifically dictates that
    // core/shared dependencies (Ca > 0) must maintain low instability (I <= 0.35).
    let foundation_crates: Vec<_> = metrics.iter().filter(|c| c.ca_inbound > 0).collect();
    let total_ca_weight: usize = foundation_crates.iter().map(|c| c.ca_inbound).sum();
    let avg_instability = if total_ca_weight > 0 {
        let weighted_sum: f64 = foundation_crates
            .iter()
            .map(|c| c.instability * (c.ca_inbound as f64))
            .sum();
        ((weighted_sum / total_ca_weight as f64) * 100.0).round() / 100.0
    } else {
        0.35
    };

    // DAG Critical Path Depth via memoized depth calculation
    let mut depth_memo: HashMap<String, usize> = HashMap::new();
    fn get_chain_depth(
        node: &str,
        outbound: &HashMap<String, HashSet<String>>,
        visited_in_path: &mut HashSet<String>,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if let Some(&d) = memo.get(node) {
            return d;
        }
        if visited_in_path.contains(node) {
            return 1;
        }
        visited_in_path.insert(node.to_string());
        let mut max_child_depth = 0;
        if let Some(deps) = outbound.get(node) {
            for dep in deps {
                let d = get_chain_depth(dep, outbound, visited_in_path, memo);
                if d > max_child_depth {
                    max_child_depth = d;
                }
            }
        }
        visited_in_path.remove(node);
        let total = 1 + max_child_depth;
        memo.insert(node.to_string(), total);
        total
    }

    let mut visited_in_path = HashSet::new();
    let critical_path_depth = crate_paths
        .keys()
        .map(|k| get_chain_depth(k, &outbound_deps, &mut visited_in_path, &mut depth_memo))
        .max()
        .unwrap_or(1);

    // Brent's Theorem parallel compilation span estimation (8-core CI standard):
    // Critical path compiles serially (~22s per stage), non-critical path compiles concurrently.
    let total_crates = metrics.len();
    let off_path_crates = total_crates.saturating_sub(critical_path_depth);
    let parallel_span_seconds = ((critical_path_depth as f64 * 22.0)
        + ((off_path_crates as f64) * 6.0) / 8.0)
        .clamp(60.0, 480.0);
    let parallel_span_seconds = (parallel_span_seconds * 100.0).round() / 100.0;

    WorkspaceCouplingReport {
        total_crates: metrics.len(),
        critical_path_depth,
        estimated_parallel_span_seconds: parallel_span_seconds,
        avg_instability,
        crates: metrics,
    }
}

fn extract_workspace_members(content: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut in_members = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("members") && trimmed.contains('[') {
            in_members = true;
            continue;
        }

        if in_members {
            if trimmed.starts_with(']') {
                break;
            }
            // Strip inline comments before extracting member path
            let line_no_comment = trimmed.split('#').next().unwrap_or("").trim();
            let cleaned = line_no_comment.trim_matches(|c| c == '"' || c == ',' || c == ' ');
            if !cleaned.is_empty() {
                members.push(cleaned.to_string());
            }
        }
    }

    members
}

fn extract_package_name(content: &str) -> Option<String> {
    let re = regex::Regex::new(r#"(?m)^\s*name\s*=\s*"([^"]+)""#).ok()?;
    re.captures(content).map(|c| c[1].to_string())
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_deps_section = false;
    let mut current_sub_dep = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = trimmed.trim_matches(|c| c == '[' || c == ']');
            if section.starts_with("dependencies.") || section.starts_with("build-dependencies.") {
                in_deps_section = false;
                let sub = section.split('.').nth(1).unwrap_or("").trim();
                if !sub.is_empty() {
                    current_sub_dep = Some(sub.to_string());
                    deps.push(sub.to_string());
                }
            } else if section == "dependencies"
                || section == "build-dependencies"
                || section.ends_with(".dependencies")
            {
                in_deps_section = true;
                current_sub_dep = None;
            } else {
                in_deps_section = false;
                current_sub_dep = None;
            }
            continue;
        }

        let line_no_comment = trimmed.split('#').next().unwrap_or("").trim();
        if let Some(ref sub) = current_sub_dep {
            // Check if sub-table renames the package: package = "real_pkg"
            if line_no_comment.starts_with("package =") {
                if let Some(cap) = line_no_comment.split('=').nth(1) {
                    let real_name = cap.trim().trim_matches('"').trim();
                    if !real_name.is_empty() {
                        if let Some(pos) = deps.iter().position(|d| d == sub) {
                            deps[pos] = real_name.to_string();
                        }
                    }
                }
            }
        } else if in_deps_section {
            if let Some(idx) = line_no_comment.find('=') {
                let dep_name = line_no_comment[..idx].trim();
                let rest = line_no_comment[idx + 1..].trim();
                // Check if inline table renames the package: foo = { package = "bar", ... }
                let real_name = if rest.contains("package =") {
                    rest.split("package =")
                        .nth(1)
                        .and_then(|p| p.split('"').nth(1))
                        .unwrap_or(dep_name)
                } else {
                    dep_name
                };
                if !real_name.is_empty() {
                    deps.push(real_name.to_string());
                }
            }
        }
    }

    deps
}
/// Detect circular dependencies in workspace crates via depth-first search cycle detection
pub fn detect_circular_dependencies(root: &Path) -> usize {
    let cargo_toml_path = root.join("Cargo.toml");
    let Ok(cargo_str) = fs::read_to_string(&cargo_toml_path) else {
        return 0;
    };

    let members = extract_workspace_members(&cargo_str);
    if members.is_empty() {
        return 0;
    }

    let mut crate_paths: HashMap<String, PathBuf> = HashMap::new();
    for m in &members {
        let member_path = root.join(m);
        let pkg_cargo = member_path.join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&pkg_cargo) {
            if let Some(name) = extract_package_name(&content) {
                crate_paths.insert(name.clone(), PathBuf::from(m));
            }
        }
    }

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for (name, path) in &crate_paths {
        let pkg_cargo = root.join(path).join("Cargo.toml");
        if let Ok(content) = fs::read_to_string(&pkg_cargo) {
            let deps = extract_dependencies(&content);
            let mut list = Vec::new();
            for d in deps {
                if crate_paths.contains_key(&d) && &d != name {
                    list.push(d);
                }
            }
            adj.insert(name.clone(), list);
        }
    }

    let mut visited: HashMap<String, u8> = HashMap::new(); // 0 = unvisited, 1 = visiting, 2 = visited
    let mut cycles = 0;

    fn dfs(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashMap<String, u8>,
        cycles: &mut usize,
    ) {
        visited.insert(node.to_string(), 1);
        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                match visited.get(neighbor).copied().unwrap_or(0) {
                    1 => *cycles += 1,
                    0 => dfs(neighbor, adj, visited, cycles),
                    _ => {}
                }
            }
        }
        visited.insert(node.to_string(), 2);
    }

    for node in adj.keys() {
        if visited.get(node).copied().unwrap_or(0) == 0 {
            dfs(node, &adj, &mut visited, &mut cycles);
        }
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_package_name() {
        let toml = r#"
[package]
name = "my-test-crate"
version = "0.1.0"
"#;
        assert_eq!(
            extract_package_name(toml),
            Some("my-test-crate".to_string())
        );
    }

    #[test]
    fn test_extract_workspace_members_with_inline_comments() {
        let toml = r#"
[workspace]
members = [
    "crates/engine-ports",        # Port trait definitions (48+ traits for adapters)
    "crates/shared-wiring",       # Common service wiring to reduce ~800 lines duplication
    "apps/product/engine",
]
"#;
        let members = extract_workspace_members(toml);
        assert_eq!(members.len(), 3);
        assert_eq!(members[0], "crates/engine-ports");
        assert_eq!(members[1], "crates/shared-wiring");
        assert_eq!(members[2], "apps/product/engine");
    }

    #[test]
    fn test_workspace_coupling_discovers_all_members() {
        let root = crate::structure::find_workspace_root(Path::new("."));
        let report = analyze_workspace_coupling(&root);
        assert!(
            report.total_crates >= 1,
            "Discovered {} crates",
            report.total_crates
        );
        assert_eq!(
            detect_circular_dependencies(&root),
            0,
            "No circular dependencies in workspace DAG"
        );
    }
}
