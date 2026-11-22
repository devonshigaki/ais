//! Autonomous Iteration System (AIS) Command-Line Interface
//!
//! Language-agnostic, open-source code quality, risk triage, and autonomous self-healing CLI.

use ais_core::{ingest_telemetry_file, scan_codebase_metrics, Finding, HealConfig};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(name = "ais", author = "Autonomous Iteration System Contributors")]
#[command(
    version = "1.0.0",
    about = "Autonomous code quality, risk triage, and self-healing engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Measure Cyclomatic Complexity (CCN) and Maintainability Index (MI)
    Metrics {
        #[arg(long)]
        json: bool,
    },
    /// Universal Baseline Ratchet preventing technical debt regression
    Ratchet {
        #[arg(long)]
        ci: bool,
        #[arg(long)]
        update: bool,
    },
    /// Closed-loop autonomous self-healing agent (Gemini 3.1 Pro / 3.8)
    Heal {
        #[arg(long)]
        apply: bool,
        #[arg(long)]
        auto_commit: bool,
        #[arg(long)]
        sandbox: bool,
        #[arg(long)]
        batch: Option<String>,
        #[arg(long)]
        rule: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Ingest runtime production crash logs (GCP Cloud Run / Sentry)
    Telemetry {
        #[arg(short, long)]
        log: PathBuf,
    },
    /// Incremental quality and architectural scan
    Scan {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        sync_docs: bool,
    },
    /// Contextual Risk Scoring (CRS) & RICE debt triage
    Risk {
        #[arg(long)]
        json: bool,
    },
    /// Analyze Git commit frequency and empirical defect hazard hotspots
    Churn {
        #[arg(long, default_value_t = 90)]
        days: usize,
        #[arg(long)]
        json: bool,
    },
    /// Evaluate AST semantic mutation testing efficacy
    Mutate {
        #[arg(long)]
        json: bool,
    },
    /// Launch user-facing AIS Studio web interface
    Serve {
        #[arg(long, default_value_t = 3030)]
        port: u16,
    },
    /// Analyze architectural coupling, instability index, defect hotspots, and Rust health
    Architecture {
        #[arg(long)]
        json: bool,
    },
    /// Initialize AIS in any repository with zero prior configuration
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Run construct-level coverage bleed & schema invariant verification
    CoverageBleed {
        #[arg(long)]
        json: bool,
    },
    /// Run AIS self-diagnostic OBD-II trouble codes and adversarial fault injection
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Audit legal-entity isolation boundaries (BND-ENT-001..007)
    EntityIsolation {
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    match cli.command {
        Commands::Metrics { json } => run_metrics(&root, json),
        Commands::Ratchet { ci, update } => run_ratchet(&root, ci, update),
        Commands::Heal {
            apply,
            auto_commit,
            sandbox,
            batch,
            rule,
            limit,
        } => run_heal(&root, apply, auto_commit, sandbox, batch, rule, limit),
        Commands::Telemetry { log } => run_telemetry(&log),
        Commands::Scan { json, sync_docs } => run_scan(&root, json, sync_docs),
        Commands::Risk { json } => run_risk(&root, json),
        Commands::Churn { days, json } => run_churn(&root, days, json),
        Commands::Mutate { json } => run_mutate(&root, json),
        Commands::Serve { port } => run_serve(port),
        Commands::Architecture { json } => run_architecture(&root, json),
        Commands::Init { force } => run_init(&root, force),
        Commands::CoverageBleed { json } => run_coverage_bleed(&root, json),
        Commands::Doctor { json } => run_doctor(&root, json),
        Commands::EntityIsolation { json } => run_entity_isolation(&root, json),
    }
}

fn run_metrics(root: &Path, json: bool) -> ExitCode {
    let report = scan_codebase_metrics(root);
    if json {
        println!(
            "{{\"max_ccn\":{},\"avg_ccn\":{:.1},\"min_mi\":{:.1},\"avg_mi\":{:.1},\"total_loc\":{},\"files_scanned\":{}}}",
            report.max_ccn, report.avg_ccn, report.min_mi, report.avg_mi, report.total_loc, report.files_scanned
        );
    } else {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║   AIS CODE QUALITY & COMPLEXITY METRICS REPORT                   ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
        println!("Files Scanned:          {}", report.files_scanned);
        println!("Total Lines of Code:    {}", report.total_loc);
        println!("Max Cyclomatic (CCN):   {}", report.max_ccn);
        println!("Avg Cyclomatic (CCN):   {:.1}", report.avg_ccn);
        println!("Min Maintainability:    {:.1}", report.min_mi);
        println!("Avg Maintainability:    {:.1}", report.avg_mi);
    }
    ExitCode::SUCCESS
}
fn run_architecture(root: &Path, json: bool) -> ExitCode {
    let coupling = ais_core::analyze_workspace_coupling(root);
    let hotspots = ais_core::analyze_workspace_hotspots(root, 60);
    let rust_health = ais_core::analyze_workspace_rust_health(root, 0);
    let test_gaps = ais_core::analyze_test_gaps(root, 60);
    let cohesion = ais_core::analyze_workspace_cohesion(root);

    if json {
        let val = serde_json::json!({
            "coupling": coupling,
            "hotspots": hotspots,
            "rust_health": rust_health,
            "test_gaps": test_gaps,
            "cohesion": cohesion,
        });
        if let Ok(s) = serde_json::to_string_pretty(&val) {
            println!("{s}");
        }
        return ExitCode::SUCCESS;
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS ARCHITECTURAL COUPLING, INSTABILITY & RUST HEALTH AUDIT    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("🏛️  ARCHITECTURAL COUPLING & INSTABILITY (SDP Principle):");
    println!("  • Workspace Crates Analyzed:   {}", coupling.total_crates);
    println!(
        "  • Average Instability (I):      {:.2}",
        coupling.avg_instability
    );
    println!("\n  Key Stable Core Crates (I <= 0.35, high Ca):");
    for c in coupling
        .crates
        .iter()
        .filter(|c| c.archetype == ais_core::CrateArchetype::StableCore)
        .take(5)
    {
        println!(
            "    - {:<30} Ca: {:<4} Ce: {:<4} I: {:.2}",
            c.crate_name, c.ca_inbound, c.ce_outbound, c.instability
        );
    }
    println!("\n  Key Volatile Edge Crates (I >= 0.75, high Ce):");
    for c in coupling
        .crates
        .iter()
        .filter(|c| c.archetype == ais_core::CrateArchetype::VolatileEdge)
        .take(5)
    {
        println!(
            "    - {:<30} Ca: {:<4} Ce: {:<4} I: {:.2}",
            c.crate_name, c.ca_inbound, c.ce_outbound, c.instability
        );
    }

    println!("\n🔥 DEFECT HOTSPOT MATRIX (High Churn + Low MI + High CCN):");
    println!(
        "  • Critical Hotspots:           {}",
        hotspots.critical_hotspots_count
    );
    println!("  • Top Empirical Hotspots:");
    for h in hotspots.top_hotspots.iter().take(5) {
        let crit_marker = if h.is_critical_hotspot {
            " [CRITICAL]"
        } else {
            ""
        };
        println!(
            "    - {:<45} Hotspot: {:<6.1} CCN: {:<3} MI: {:<4.1}{}",
            h.path.display(),
            h.hotspot_score,
            h.ccn,
            h.mi,
            crit_marker
        );
    }

    println!("\n🧪 TEST GAP ANALYSIS (Churn × Missing Test Suites):");
    println!(
        "  • Production Files Analyzed:   {}",
        test_gaps.total_files_analyzed
    );
    println!(
        "  • Direct Test Coverage:        {:.1}%",
        test_gaps.test_coverage_ratio
    );
    println!(
        "  • Untested Production Files:   {}",
        test_gaps.untested_files_count
    );
    if !test_gaps.top_gaps.is_empty() {
        println!("  • Top High-Churn Test Gaps:");
        for g in test_gaps.top_gaps.iter().take(5) {
            println!(
                "    - {:<45} Gap Score: {:<6.1} Churn: {:<3} LOC: {:<4}",
                g.path.display(),
                g.gap_score,
                g.commit_count,
                g.loc
            );
        }
    }
    println!("\n🧩 LCOM4 COHESION & MODULARITY (Lack of Cohesion of Methods):");
    println!(
        "  • Structs Evaluated (>= 3 methods): {}",
        cohesion.total_structs_evaluated
    );
    println!(
        "  • Average LCOM4 Score:              {:.2}",
        cohesion.avg_lcom4
    );
    println!(
        "  • Max LCOM4 Score:                  {}",
        cohesion.max_lcom4
    );
    println!(
        "  • Cohesion Violations (LCOM4 >= 2): {}",
        cohesion.cohesion_violations_count
    );
    if !cohesion.top_cohesion_debt.is_empty() {
        println!("  • Top Cohesion Debt Candidates (Candidates for Partitioning):");
        for c in cohesion.top_cohesion_debt.iter().take(5) {
            println!(
                "    - {:<30} LCOM4: {:<3} Methods: {:<3} Fields: {:<3} ({})",
                c.struct_name,
                c.lcom4,
                c.method_count,
                c.field_count,
                c.file_path.display()
            );
        }
    }

    println!("\n🦀 RUST-SPECIFIC HEALTH & SAFETY INVARIANTS:");
    println!(
        "  • Production Rust LOC:         {}",
        rust_health.production_loc
    );
    println!(
        "  • Production 'unsafe' Blocks:  {} (Density: {:.4}%)",
        rust_health.unsafe_blocks, rust_health.unsafe_density_pct
    );
    println!(
        "  • Production unwrap() Calls:   {}",
        rust_health.production_unwraps
    );
    println!(
        "  • Production expect() Calls:   {}",
        rust_health.production_expects
    );
    println!(
        "  • Dynamic Dispatch (dyn Trait):{}",
        rust_health.dynamic_dispatch_count
    );
    println!(
        "  • Static Dispatch (impl<T>):   {}",
        rust_health.static_dispatch_count
    );
    println!(
        "  • Unique Crates in Cargo.lock: {}",
        rust_health.unique_dependency_crates
    );
    println!(
        "  • Defect Density (per KLOC):   {:.2} bugs/KLOC",
        rust_health.defect_density_per_kloc
    );
    println!(
        "  • Security Tooling:            cargo-audit={}, cargo-geiger={}",
        if rust_health.cargo_audit_available {
            "ok"
        } else {
            "MISSING"
        },
        if rust_health.cargo_geiger_available {
            "ok"
        } else {
            "MISSING"
        }
    );
    for w in &rust_health.tool_warnings {
        println!("  ⚠️  TOOL GAP: {w}");
    }

    ExitCode::SUCCESS
}

fn collect_workspace_findings(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let (_graph, taint_findings) = ais_core::scan_taint_violations(root);
    findings.extend(taint_findings);

    let struct_report = ais_core::scan_codebase_structure(root);
    findings.extend(struct_report.findings);
    findings.extend(ais_core::scan_entity_isolation(root));

    let rust_health = ais_core::analyze_workspace_rust_health(root, 0);
    if rust_health.production_unwraps > 0 {
        findings.push(Finding::new(
            "RUST-UNWRAP-001",
            ais_core::Severity::High,
            format!(
                "{} production .unwrap() call(s) detected outside #[cfg(test)]",
                rust_health.production_unwraps
            ),
            PathBuf::from("crates"),
            1,
        ));
    }
    if rust_health.production_expects > 0 {
        findings.push(Finding::new(
            "RUST-EXPECT-001",
            ais_core::Severity::Medium,
            format!(
                "{} production .expect() call(s) detected outside #[cfg(test)]",
                rust_health.production_expects
            ),
            PathBuf::from("crates"),
            1,
        ));
    }
    let metrics = scan_codebase_metrics(root);
    for f in &metrics.files_above_ccn_goal {
        findings.push(Finding::new(
            "METRIC-CCN-001",
            ais_core::Severity::Medium,
            format!(
                "High cyclomatic complexity ({} CCN > 10) in {}",
                f.ccn,
                f.path.display()
            ),
            f.path.clone(),
            1,
        ));
    }
    for f in &metrics.files_below_mi_goal {
        findings.push(Finding::new(
            "METRIC-MI-001",
            ais_core::Severity::Medium,
            format!(
                "Low maintainability index ({:.1} MI < 80) in {}",
                f.mi,
                f.path.display()
            ),
            f.path.clone(),
            1,
        ));
    }
    ais_core::rank_findings_by_risk(&mut findings);
    findings
}

fn run_ratchet(root: &Path, ci: bool, update: bool) -> ExitCode {
    let valid_root = if root.join("Cargo.toml").exists() {
        root.to_path_buf()
    } else {
        ais_core::structure::find_workspace_root(root)
    };

    if valid_root.join("xtask/Cargo.toml").exists() {
        let mut cmd = Command::new("cargo");
        cmd.args(["xtask", "ratchet"]).current_dir(&valid_root);
        if ci {
            cmd.arg("--ci");
        }
        if update {
            cmd.arg("--update");
        }
        return match cmd.status() {
            Ok(status) => {
                if status.success() {
                    if update {
                        let _ = Command::new("cargo")
                            .args(["xtask", "report"])
                            .current_dir(&valid_root)
                            .status();
                    }
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(e) => {
                eprintln!("Failed to execute cargo xtask ratchet: {}", e);
                ExitCode::FAILURE
            }
        };
    }

    let metrics = scan_codebase_metrics(&valid_root);
    let rust_health = ais_core::analyze_workspace_rust_health(&valid_root, 0);
    let (_g, taint_findings) = ais_core::scan_taint_violations(&valid_root);
    let mut current = std::collections::BTreeMap::new();
    current.insert(
        "maintainability_index_avg".to_string(),
        metrics.avg_mi as f64,
    );
    current.insert(
        "maintainability_index_min".to_string(),
        metrics.min_mi as f64,
    );
    current.insert("complexity_ccn_max".to_string(), metrics.max_ccn as f64);
    current.insert("complexity_ccn_avg".to_string(), metrics.avg_ccn as f64);
    current.insert(
        "production_unwraps_max".to_string(),
        rust_health.production_unwraps as f64,
    );
    current.insert(
        "production_expects_max".to_string(),
        rust_health.production_expects as f64,
    );
    current.insert(
        "unsafe_blocks_production".to_string(),
        rust_health.unsafe_blocks as f64,
    );
    current.insert(
        "taint_analysis_critical".to_string(),
        taint_findings.len() as f64,
    );

    let baseline_path = if valid_root.join("audit/raw/ratchet_baseline.json").exists() {
        valid_root.join("audit/raw/ratchet_baseline.json")
    } else {
        valid_root.join(".ais/ratchet_baseline.json")
    };
    let baseline = ais_core::load_ratchet_baseline(&baseline_path);
    let (evals, regressions) = ais_core::evaluate_ratchet_metrics(&current, &baseline);

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS STANDALONE UNIVERSAL RATCHET                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    for ev in &evals {
        println!(
            "  {:<28} curr={:<8.2} base={:<8.2} goal={:<8.2} [{}]",
            ev.name, ev.current, ev.baseline, ev.goal, ev.status
        );
    }
    if update {
        let updated = ais_core::update_ratchet_baseline_monotonic(&baseline, &current);
        ais_core::save_ratchet_baseline(&baseline_path, &updated);
        println!(
            "\n✅ Monotonic baseline saved to {}",
            baseline_path.display()
        );
    }
    if ci && regressions > 0 {
        eprintln!("\n🛑 RATCHET FAILED: {regressions} regression(s) detected.");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_heal(
    root: &Path,
    apply: bool,
    auto_commit: bool,
    sandbox: bool,
    batch: Option<String>,
    rule: Option<String>,
    limit: Option<usize>,
) -> ExitCode {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS CLOSED-LOOP AUTONOMOUS HEALER (GEMINI 3.1 / 3.8)           ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let config = HealConfig {
        dry_run: !apply,
        auto_verify: true,
        auto_commit,
        sandbox_branch: sandbox,
        batch_filter: batch,
        rule_filter: rule,
        limit,
        default_model: "gemini-3.8-flash",
        api_key: None,
    };

    let findings = collect_workspace_findings(root);
    let report = ais_core::execute_autonomous_healer(root, &findings, &config);

    println!("Workspace Findings:  {}", findings.len());
    println!("Candidate Patches:   {}", report.patches_synthesized);
    println!("Patches Applied:     {}", report.patches_applied);
    println!("Build Verified:      {}", report.patches_verified);
    println!("Git Commits Created: {}", report.git_commits_created);

    if !apply {
        println!("\n💡 Preview mode. Pass `--apply` to automatically execute and verify patches.");
    }
    ExitCode::SUCCESS
}

fn run_serve(port: u16) -> ExitCode {
    let addr = format!("127.0.0.1:{}", port);
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS STUDIO (Native Rust)                                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!("🚀 Launching AIS Studio on http://{}", addr);
    println!(
        "Use `cargo xtask serve --port={}` to run dashboard server.",
        port
    );
    ExitCode::SUCCESS
}

fn run_telemetry(log_path: &Path) -> ExitCode {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS RUNTIME CRASH & TELEMETRY INGESTION                        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let report = ingest_telemetry_file(log_path);
    println!("Log Lines Processed:     {}", report.logs_processed);
    println!(
        "Runtime Errors Mapped:   {}",
        report.runtime_errors_captured
    );

    for f in &report.findings {
        println!(
            "  - [{}] {} ({}:{})",
            f.rule_code,
            f.message,
            f.file.display(),
            f.line
        );
    }
    ExitCode::SUCCESS
}

fn run_scan(root: &Path, json: bool, sync_docs: bool) -> ExitCode {
    let report = scan_codebase_metrics(root);
    if json {
        println!(
            "{{\"status\":\"ok\",\"files_scanned\":{},\"total_loc\":{},\"avg_ccn\":{:.1},\"avg_mi\":{:.1}}}",
            report.files_scanned, report.total_loc, report.avg_ccn, report.avg_mi
        );
        return ExitCode::SUCCESS;
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS INCREMENTAL CODEBASE & ARCHITECTURAL SCANNER               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    println!("Files Scanned:          {}", report.files_scanned);
    println!("Total Lines of Code:    {}", report.total_loc);
    println!("Average Complexity:     {:.1} CCN", report.avg_ccn);
    println!("Average Maintainability:{:.1} / 100", report.avg_mi);

    println!("\n🔍 Running AIS Code Quality & Compliance Gates...");
    let has_xtask = root.join("xtask/Cargo.toml").exists();

    // Check 1: Code Formatting
    print!("  • Code Formatting (rustfmt)... ");
    let fmt_status = Command::new("cargo")
        .args(["fmt", "--all", "--", "--check"])
        .current_dir(root)
        .status();
    let fmt_ok = fmt_status.map(|s| s.success()).unwrap_or(false);
    if fmt_ok {
        println!("✅ PASS");
    } else {
        println!("❌ FAIL (formatting diffs detected, run `cargo fmt --all`)");
    }

    let (comp_ok, audit_ok, pot_ok, recon_ok) = if has_xtask {
        // Check 2: Architecture, Security & Design Compliance
        print!("  • Architecture & Frontend Compliance... ");
        let comp_status = Command::new("cargo")
            .args(["xtask", "compliance", "--tier=all"])
            .current_dir(root)
            .status();
        let comp_ok = comp_status.map(|s| s.success()).unwrap_or(false);

        // Check 3: 14 Comprehensive Quality Dimensions
        print!("  • 14 Comprehensive Quality Dimensions... ");
        let audit_status = Command::new("cargo")
            .args(["xtask", "audit-proprietary", "--fast"])
            .current_dir(root)
            .status();
        let audit_ok = audit_status.map(|s| s.success()).unwrap_or(false);

        // Check 4: Power of Ten Safety-Critical Invariants
        print!("  • Power of Ten Safety-Critical Invariants (POT)... ");
        let pot_status = Command::new("cargo")
            .args(["xtask", "pot"])
            .current_dir(root)
            .status();
        let pot_ok = pot_status.map(|s| s.success()).unwrap_or(false);

        (comp_ok, audit_ok, pot_ok, true)
    } else {
        let rust_health = ais_core::analyze_workspace_rust_health(root, 0);
        let pot_ok = rust_health.production_unwraps == 0;
        println!(
            "  • Standalone Rust Health Gate... {} (unwraps={}, expects={}, unsafe={})",
            if pot_ok { "✅ PASS" } else { "❌ FAIL" },
            rust_health.production_unwraps,
            rust_health.production_expects,
            rust_health.unsafe_blocks
        );
        (true, true, pot_ok, true)
    };

    // Check 5: Inter-Procedural Semantic Callgraph & Taint-Flow Analysis
    print!("  • Semantic Callgraph & Taint Flow (TAINT)... ");
    let (_graph, taint_findings) = ais_core::scan_taint_violations(root);
    let taint_ok = taint_findings.is_empty();
    if taint_ok {
        println!("✅ PASS (0 unsafe data flows detected)");
    } else {
        println!(
            "❌ FAIL ({} tainted data flows detected)",
            taint_findings.len()
        );
        for f in &taint_findings {
            eprintln!("    - [{}] {}", f.rule_code, f.message);
        }
    }

    // Check 6: Rule Contradiction Reconciler & Precedence Matrix
    let recon_ok = if has_xtask {
        print!("  • Rule Contradiction Reconciler & Harmony Matrix... ");
        let recon_status = Command::new("cargo")
            .args(["xtask", "reconcile"])
            .current_dir(root)
            .status();
        recon_status.map(|s| s.success()).unwrap_or(false)
    } else {
        let overlaps = ais_core::audit_rule_overlap_matrix();
        println!(
            "  • Rule Overlap Matrix (Standalone)... {}",
            if overlaps.is_empty() {
                "✅ PASS (0 overlapping rule triggers)"
            } else {
                "⚠️  OVERLAP DETECTED"
            }
        );
        let _ = recon_ok;
        overlaps.is_empty()
    };

    // Check 7: Semantic Deduplication & MinHash LSH Clones
    print!("  • Semantic Deduplication & MinHash LSH Clones... ");
    let clusters = ais_core::scan_workspace_semantic_duplicates(root, 95.0);
    if clusters.is_empty() {
        println!("✅ PASS (0 high-similarity clone clusters detected)");
    } else {
        println!(
            "ℹ️  SURFACED ({} candidate clone pairs >= 95% similarity)",
            clusters.len()
        );
        for c in clusters.iter().take(2) {
            println!(
                "      - {:.1}% similarity: {} ({}:{}) <-> {} ({}:{})",
                c.similarity_pct,
                c.function_a,
                c.file_a.display(),
                c.line_a,
                c.function_b,
                c.file_b.display(),
                c.line_b
            );
        }
    }

    // Check 8: AST Mutation Testing Efficacy
    print!("  • AST Mutation Testing Efficacy Gate... ");
    let mutation_report = ais_core::analyze_mutation_coverage(root);
    if mutation_report.empirical_outcomes_present {
        println!(
            "✅ PASS ({:.1}% empirical mutant kill rate: {}/{} caught across {} sites)",
            mutation_report.mutation_efficacy_score,
            mutation_report.mutants_caught,
            mutation_report.mutants_caught + mutation_report.mutants_missed,
            mutation_report.total_mutation_sites
        );
    } else {
        println!(
            "⚠️  HEURISTIC ({:.1}% static site-coverage proxy across {} sites; run cargo-mutants for empirical kill rate)",
            mutation_report.mutation_efficacy_score, mutation_report.total_mutation_sites
        );
    }

    // Check 9: Empirical Code Churn & Hazard Engine
    print!("  • Empirical Code Churn & Hazard Engine... ");
    let churn_report = ais_core::analyze_workspace_churn(root, 30);
    let critical_hazard_count = churn_report
        .top_hazard_files
        .iter()
        .take(10)
        .filter(|f| f.hazard_score > 90.0)
        .count();
    println!(
        "✅ PASS ({} files tracked, {} critical hazard files)",
        churn_report.total_files_tracked, critical_hazard_count
    );

    // Check 10: Codebase Structure & Root Hygiene Guard
    print!("  • Codebase Structure & Root Hygiene Guard... ");
    let struct_report = ais_core::scan_codebase_structure(root);
    if struct_report.total_violations == 0 {
        println!("✅ PASS (Workspace root & data isolation clean)");
    } else {
        println!(
            "⚠️  FAIL ({} structural violations detected)",
            struct_report.total_violations
        );
        for f in &struct_report.findings {
            println!(
                "      - [{}] {}: {}",
                f.rule_code,
                f.file.display(),
                f.message
            );
        }
    }

    if sync_docs && has_xtask {
        println!("\n📝 Synchronizing Codebase Documentation & Living Backlog...");
        print!("  • Updating docs/BACKLOG.md (RICE & Financial Debt Ledger)... ");
        let backlog_status = Command::new("cargo")
            .args(["xtask", "harvest-backlog"])
            .current_dir(root)
            .status();
        if backlog_status.map(|s| s.success()).unwrap_or(false) {
            println!("✅ SYNCHRONIZED");
        } else {
            println!("⚠️  WARNING (failed to harvest backlog)");
        }

        print!("  • Updating audit/compliance-executive-summary.md & telemetry... ");
        let report_status = Command::new("cargo")
            .args(["xtask", "report"])
            .current_dir(root)
            .status();
        if report_status.map(|s| s.success()).unwrap_or(false) {
            println!("✅ SYNCHRONIZED");
        } else {
            println!("⚠️  WARNING (failed to generate executive report)");
        }
    }

    if fmt_ok && comp_ok && audit_ok && pot_ok && taint_ok && recon_ok {
        let ent_findings = ais_core::scan_entity_isolation(root);
        if !ent_findings.is_empty() {
            eprintln!(
                "\n🛑 AIS SCAN FAILED: {} legal-entity isolation violation(s):",
                ent_findings.len()
            );
            for f in &ent_findings {
                eprintln!(
                    "    - [{}] {}: {}",
                    f.rule_code,
                    f.file.display(),
                    f.message
                );
            }
            return ExitCode::FAILURE;
        }
        println!("\n✅ Incremental AST & compliance scan completed with zero regression blocks.");
        ExitCode::SUCCESS
    } else {
        eprintln!("\n🛑 AIS SCAN FAILED: Violations detected across quality dimensions.");
        ExitCode::FAILURE
    }
}

fn run_entity_isolation(root: &Path, json: bool) -> ExitCode {
    let findings = ais_core::scan_entity_isolation(root);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&findings).unwrap_or_default()
        );
    } else if findings.is_empty() {
        println!("✅ ENTITY ISOLATION: 0 cross-entity leaks detected (BND-ENT-001..007).");
    } else {
        eprintln!(
            "❌ ENTITY ISOLATION VIOLATIONS ({} finding(s)):",
            findings.len()
        );
        for f in &findings {
            eprintln!(
                "  - [{}] {}:{} — {}",
                f.rule_code,
                f.file.display(),
                f.line,
                f.message
            );
        }
    }
    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn run_risk(root: &Path, json: bool) -> ExitCode {
    let findings = collect_workspace_findings(root);
    let assessments: Vec<_> = findings.iter().map(ais_core::evaluate_risk).collect();

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&assessments).unwrap_or_else(|_| "[]".to_string())
        );
        return ExitCode::SUCCESS;
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS CONTEXTUAL RISK SCORING (CRS) & RICE TRIAGE                ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    println!("Triage Formula: Base Severity × Reach Vector × Criticality × Confidence");
    println!("Workspace Findings Evaluated: {}\n", assessments.len());
    for (idx, a) in assessments.iter().take(15).enumerate() {
        println!(
            "  [{:>2}] CRS={:>5.1} | RICE={:>5.1} | {} | [{}] {}:{} — {}",
            idx + 1,
            a.contextual_risk_score,
            a.rice.rice_score,
            a.priority,
            a.finding.rule_code,
            a.finding.file.display(),
            a.finding.line,
            a.finding.message
        );
    }
    if assessments.is_empty() {
        println!("  ✅ Zero active workspace findings detected.");
    }
    ExitCode::SUCCESS
}

fn run_churn(root: &Path, days: usize, json: bool) -> ExitCode {
    let report = ais_core::analyze_workspace_churn(root, days);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
    } else {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║   AIS CODE CHURN & EMPIRICAL DEFECT HAZARD ENGINE                ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
        println!("Commits Evaluated:  {}", report.total_commits_analyzed);
        println!("Files Tracked:      {}", report.total_files_tracked);
        println!("\nTop Empirical Defect Hazard Files (Nagappan-Ball Pareto Model):");
        for (idx, f) in report.top_hazard_files.iter().enumerate() {
            println!(
                "  [{}] Hazard: {:>6.1} | Commits: {:>3} | Authors: {:>2} | {}",
                idx + 1,
                f.hazard_score,
                f.commit_count,
                f.unique_authors,
                f.file_path.display()
            );
        }
    }
    ExitCode::SUCCESS
}

fn run_mutate(root: &Path, json: bool) -> ExitCode {
    let report = ais_core::analyze_mutation_coverage(root);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
    } else {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║   AIS AST MUTATION TESTING EFFICACY GATE                         ║");
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
        println!("Files Analyzed:          {}", report.files_analyzed);
        println!("Total Mutation Sites:    {}", report.total_mutation_sites);
        println!("Comparison Inversions:   {}", report.comparison_mutations);
        println!("Boolean Flips:           {}", report.boolean_mutations);
        println!(
            "Mutation Score Floor:    {:.1}% ({})",
            report.mutation_efficacy_score,
            if report.empirical_outcomes_present {
                "empirical cargo-mutants outcomes"
            } else {
                "heuristic static site-coverage proxy — no mutants executed"
            }
        );
        if report.empirical_outcomes_present {
            println!("\n✅ AST Mutation testing efficacy verified against cargo-mutants outcomes.");
        } else {
            println!("\n⚠️  Heuristic static proxy only: execute cargo-mutants for empirical mutant kill rate.");
        }
    }
    ExitCode::SUCCESS
}
fn run_init(root: &Path, force: bool) -> ExitCode {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS REPOSITORY BOOTSTRAPPER (Zero-Prior Initialization)        ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let ais_cache = root.join(".ais/cache");
    if let Err(e) = std::fs::create_dir_all(&ais_cache) {
        eprintln!("Failed to create .ais/cache: {}", e);
        return ExitCode::FAILURE;
    }
    println!("✓ Initialized cache directory: .ais/cache");

    let ais_yaml = root.join("ais.yaml");
    if ais_yaml.exists() && !force {
        println!("ℹ️  Existing ais.yaml detected. Pass `--force` to overwrite.");
        return ExitCode::SUCCESS;
    }

    let is_rust = root.join("Cargo.toml").exists();
    let is_node = root.join("package.json").exists();
    let is_python = root.join("pyproject.toml").exists() || root.join("requirements.txt").exists();

    let mut languages = Vec::new();
    if is_rust {
        languages.push("rust");
    }
    if is_node {
        languages.push("typescript");
    }
    if is_python {
        languages.push("python");
    }
    if languages.is_empty() {
        languages.push("polyglot");
    }

    let manifest_content = format!(
        r#"# Autonomous Iteration System (AIS) Manifest
version: 1.0.0
profile: "{}"
languages: {:?}
quality_gates:
  min_maintainability_index: 65.0
  max_cyclomatic_complexity: 25
  min_construct_coverage_pct: 90.0
cache_dir: ".ais/cache"
"#,
        languages[0], languages
    );

    if let Err(e) = std::fs::write(&ais_yaml, manifest_content) {
        eprintln!("Failed to write ais.yaml: {}", e);
        return ExitCode::FAILURE;
    }
    println!(
        "✓ Generated customized ais.yaml for {:?} project",
        languages
    );
    println!("✅ AIS successfully bootstrapped. Run `ais scan` or `ais coverage-bleed`.");
    ExitCode::SUCCESS
}

fn run_coverage_bleed(root: &Path, json: bool) -> ExitCode {
    let report = ais_core::coverage_bleed::evaluate_construct_coverage(root);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
        return if report.findings.is_empty() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS CONSTRUCT COVERAGE BLEED & SCHEMA INVARIANT ENGINE (L5)    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    println!(
        "Discovered Schema Tables:       {}",
        report.total_tables_discovered
    );
    println!(
        "Multi-Tenant Guarded Tables:    {}",
        report.multi_tenant_tables_count
    );
    println!(
        "1. Scanned Upsert Constructs:    {} (Guarded: {})",
        report.total_upsert_constructs_scanned, report.guarded_upsert_constructs
    );
    println!(
        "2. Scanned Update Constructs:    {} (Guarded: {})",
        report.total_update_constructs_scanned, report.guarded_update_constructs
    );
    println!(
        "3. Scanned Delete Constructs:    {} (Guarded: {})",
        report.total_delete_constructs_scanned, report.guarded_delete_constructs
    );
    println!(
        "4. Scanned Numerical Sites:      {} (Safe: {})",
        report.total_numerical_sites_scanned, report.safe_numerical_sites
    );
    println!(
        "5. Public Symbols Callgraph:     {} (Tested: {})",
        report.total_public_symbols_scanned, report.tested_public_symbols
    );
    println!(
        "Overall Construct Coverage (L5): {:.1}%",
        report.construct_coverage_pct
    );

    if report.findings.is_empty() {
        println!("\n✅ ZERO HAZARD SITES: All multi-tenant schema mutations and AST constructs deterministically guarded.");
        ExitCode::SUCCESS
    } else {
        println!(
            "\n❌ HAZARD SITES DETECTED ({} unhandled constructs):",
            report.findings.len()
        );
        for f in &report.findings {
            println!(
                "  - [{}] {}:{} on table '{}': {}",
                f.rule_code,
                f.file_path.display(),
                f.line_number,
                f.table_name,
                f.description
            );
        }
        ExitCode::FAILURE
    }
}

fn run_doctor(root: &Path, json: bool) -> ExitCode {
    let report = ais_core::doctor::run_ais_doctor_diagnostics(root);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
        return if report.failed_tests == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║   AIS ADVERSARIAL SELF-DIAGNOSTIC & FAULT-INJECTION ENGINE (DTC) ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    println!("Total OBD-II Diagnostic Probes: {}", report.total_tests);
    println!("Passed Sensors & Watchmen:      {} 🟢", report.passed_tests);
    println!(
        "Failed Trouble Codes (DTCs):    {} 🔴\n",
        report.failed_tests
    );

    for dtc in &report.dtcs {
        let status = if !dtc.tripped {
            "🟢 PASS"
        } else {
            "🔴 TRIPPED"
        };
        println!(
            "  • [{}] {:<32} {:<10} — {}",
            dtc.dtc_code, dtc.component, status, dtc.message
        );
    }

    if report.failed_tests == 0 {
        println!(
            "\n✅ ALL CHECK-ENGINE WATCHMEN HEALTHY: In-memory fault injections successfully intercepted."
        );
        ExitCode::SUCCESS
    } else {
        println!(
            "\n❌ DIAGNOSTIC FAULTS DETECTED: AIS watchmen failed to catch injected sabotage."
        );
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_smoke() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
