//! AIS Universal Baseline Ratchet Engine

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricDirection {
    HigherIsBetter,
    LowerIsBetter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalTier {
    /// Direct codebase-controlled engineering invariant (hard quality gate)
    HardEngineering,
    /// External vendor pass-through or macro commercial cost model (soft tracking goal)
    SoftVendorEconomics,
}

/// Exhaustive canonical North-Star Goals, Polarities, and Governance Tiers for all 69 ratchet dimensions
pub fn canonical_metric_spec(name: &str) -> (f64, MetricDirection, GoalTier) {
    use GoalTier::{HardEngineering, SoftVendorEconomics};
    use MetricDirection::{HigherIsBetter, LowerIsBetter};
    match name {
        // Pillar A. Static Correctness & Power of Ten
        "maintainability_index_avg" => (90.0, HigherIsBetter, HardEngineering),
        "maintainability_index_min" => (80.0, HigherIsBetter, HardEngineering),
        "complexity_ccn_max" => (10.0, LowerIsBetter, HardEngineering),
        "complexity_ccn_avg" => (1.10, LowerIsBetter, HardEngineering),
        "production_unwraps_max" => (0.0, LowerIsBetter, HardEngineering),
        "production_expects_max" => (0.0, LowerIsBetter, HardEngineering),
        "static_dispatch_min" => (1500.0, HigherIsBetter, HardEngineering),
        "dynamic_dispatch_max" => (150.0, LowerIsBetter, HardEngineering),
        "unsafe_blocks_production" => (0.0, LowerIsBetter, HardEngineering),
        "duplication_clusters_count" => (0.0, LowerIsBetter, HardEngineering),
        "code_duplication_medium" => (0.0, LowerIsBetter, HardEngineering),
        "active_exemptions_max" => (0.0, LowerIsBetter, HardEngineering),
        "hardcoded_scanner_metrics_max" => (0.0, LowerIsBetter, HardEngineering),

        // Pillar B. Security, Taint & Shadow-IT
        "compliance_critical" => (0.0, LowerIsBetter, HardEngineering),
        "compliance_high" => (0.0, LowerIsBetter, HardEngineering),
        "compliance_medium" => (0.0, LowerIsBetter, HardEngineering),
        "entropy_secrets_high" => (0.0, LowerIsBetter, HardEngineering),
        "gcp_iam_least_privilege_critical" => (0.0, LowerIsBetter, HardEngineering),
        "taint_analysis_critical" => (0.0, LowerIsBetter, HardEngineering),
        "shadow_it_critical" => (0.0, LowerIsBetter, HardEngineering),
        "supply_chain_vulns_critical" => (0.0, LowerIsBetter, HardEngineering),
        "license_compliance_critical" => (0.0, LowerIsBetter, HardEngineering),
        "llm_security_critical" => (0.0, LowerIsBetter, HardEngineering),
        "k8s_security_high" => (0.0, LowerIsBetter, HardEngineering),

        // Pillar C. Architectural Coupling & Cohesion
        "coupling_instability_avg" => (0.35, LowerIsBetter, HardEngineering),
        "circular_dependency_cycles" => (0.0, LowerIsBetter, HardEngineering),
        "lcom4_cohesion_avg" => (1.00, LowerIsBetter, HardEngineering),
        "cohesion_violations_max" => (0.0, LowerIsBetter, HardEngineering),
        "architecture_audit_high" => (0.0, LowerIsBetter, HardEngineering),
        "router_wiring_high" => (0.0, LowerIsBetter, HardEngineering),
        "cross_service_contract_high" => (0.0, LowerIsBetter, HardEngineering),
        "api_contract_drift_high" => (0.0, LowerIsBetter, HardEngineering),
        "folder_taxonomy_high" => (0.0, LowerIsBetter, HardEngineering),
        "design_system_health_min" => (100.0, HigherIsBetter, HardEngineering),
        "design_system_high" => (0.0, LowerIsBetter, HardEngineering),
        "design_system_medium" => (0.0, LowerIsBetter, HardEngineering),
        "design_system_low" => (0.0, LowerIsBetter, HardEngineering),
        "a11y_compliance_high" => (0.0, LowerIsBetter, HardEngineering),
        "i18n_naked_strings_medium" => (0.0, LowerIsBetter, HardEngineering),
        "mobile_compliance_high" => (0.0, LowerIsBetter, HardEngineering),
        "bundle_budget_high" => (0.0, LowerIsBetter, HardEngineering),
        "structural_gaps_total_max" => (0.0, LowerIsBetter, HardEngineering),
        "unmapped_workspace_findings_max" => (0.0, LowerIsBetter, HardEngineering),

        // Pillar D. FinOps Unit Economics & Cloud Infrastructure (SoftVendorEconomics)
        "unit_cost_user_onboarding_usd_max" => (4.50, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_monthly_active_usd_max" => (0.50, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_phone_lifecycle_usd_max" => (9.50, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_credit_card_lifecycle_usd_max" => {
            (12.00, LowerIsBetter, SoftVendorEconomics)
        }
        "unit_cost_user_education_lifecycle_usd_max" => (25.00, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_car_lifecycle_usd_max" => (65.00, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_business_lifecycle_usd_max" => (60.00, LowerIsBetter, SoftVendorEconomics),
        "unit_cost_user_home_lifecycle_usd_max" => (450.00, LowerIsBetter, SoftVendorEconomics),
        "infra_projected_monthly_usd_max" => (1200.00, LowerIsBetter, SoftVendorEconomics),
        "infra_satellite_projected_monthly_usd_max" => (500.00, LowerIsBetter, SoftVendorEconomics),
        "db_performance_high" => (0.0, LowerIsBetter, HardEngineering),
        "build_duration_seconds_max" => (300.00, LowerIsBetter, HardEngineering),

        // Pillar E. Metrology, Epistemics & Lean 4 Formal Proofs
        "coverage_line_min" => (100.0, HigherIsBetter, HardEngineering),
        "coverage_branch_min" => (100.0, HigherIsBetter, HardEngineering),
        "mutation_score_min" => (100.0, HigherIsBetter, HardEngineering),
        "discovery_coverage_pct_min" => (100.0, HigherIsBetter, HardEngineering),
        "scorecard_min_health_pct" => (100.0, HigherIsBetter, HardEngineering),
        "pattern_handler_comparison_coverage_pct" => (100.0, HigherIsBetter, HardEngineering),
        "scanner_canary_count_min" => (400.0, HigherIsBetter, HardEngineering),
        "defect_density_per_kloc" => (0.0, LowerIsBetter, HardEngineering),
        "churn_hazard_spikes" => (0.0, LowerIsBetter, HardEngineering),
        "test_flakiness_max" => (0.0, LowerIsBetter, HardEngineering),
        "sbom_components_count_min" => (1070.0, HigherIsBetter, HardEngineering),
        "log_traceability_high" => (0.0, LowerIsBetter, HardEngineering),
        "rice_score_top" => (0.0, LowerIsBetter, HardEngineering),
        "crs_score_top" => (15.0, LowerIsBetter, HardEngineering),

        // Universal fallback for new or extension metrics
        _ => {
            if name.ends_with("_min") || name.contains("coverage") || name.contains("health") {
                (100.0, HigherIsBetter, HardEngineering)
            } else {
                (0.0, LowerIsBetter, HardEngineering)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetPayload {
    pub version: String,
    pub description: String,
    pub metrics: BTreeMap<String, f64>,
    #[serde(default)]
    pub goals: BTreeMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct MetricEvaluation {
    pub name: String,
    pub current: f64,
    pub baseline: f64,
    pub delta: f64,
    pub status: MetricStatus,
    pub is_higher_better: bool,
    pub goal: f64,
    pub gap_to_goal: f64,
    pub surplus_beyond_goal: f64,
    pub goal_met: bool,
    pub tier: GoalTier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricStatus {
    Held,
    Improved,
    Regression,
}

impl std::fmt::Display for MetricStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Held => write!(f, "HELD"),
            Self::Improved => write!(f, "IMPROVED"),
            Self::Regression => write!(f, "REGRESSION"),
        }
    }
}

pub fn load_ratchet_baseline(path: &Path) -> BTreeMap<String, f64> {
    if !path.exists() {
        return BTreeMap::new();
    }
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(payload) = serde_json::from_str::<RatchetPayload>(&content) {
            return payload.metrics;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(metrics_obj) = val.get("metrics").and_then(|m| m.as_object()) {
                let mut map = BTreeMap::new();
                for (k, v) in metrics_obj {
                    if let Some(num) = v.as_f64() {
                        map.insert(k.clone(), num);
                    }
                }
                return map;
            }
        }
    }
    BTreeMap::new()
}

pub fn save_ratchet_baseline(path: &Path, metrics: &BTreeMap<String, f64>) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let mut goals = BTreeMap::new();
    for k in metrics.keys() {
        let (g, _, _) = canonical_metric_spec(k);
        goals.insert(k.clone(), g);
    }
    let payload = RatchetPayload {
        version: "2.0.0".to_string(),
        description: "Autonomous Iteration System (AIS) dual-horizon universal ratchet (monotonic baseline floor + North-Star goals). Any baseline regression triggers a CI block.".to_string(),
        metrics: metrics.clone(),
        goals,
    };
    if let Ok(json) = serde_json::to_string_pretty(&payload) {
        let _ = fs::write(path, json);
    }
}

/// Merges current metrics into baseline strictly monotonically:
/// - "Higher is better" metrics can only increase (floor ratchets up).
/// - "Lower is better" metrics can only decrease (ceiling ratchets down).
///
/// This prevents any regressions from lowering baselines or raising caps during `--update`.
pub fn update_ratchet_baseline_monotonic(
    baseline: &BTreeMap<String, f64>,
    current: &BTreeMap<String, f64>,
) -> BTreeMap<String, f64> {
    let mut updated = baseline.clone();
    for (name, curr_val) in current {
        let (_, direction, _) = canonical_metric_spec(name);
        let is_higher_better = direction == MetricDirection::HigherIsBetter;

        let new_val = if let Some(base_val) = baseline.get(name) {
            if is_higher_better {
                base_val.max(*curr_val)
            } else {
                base_val.min(*curr_val)
            }
        } else {
            *curr_val
        };
        updated.insert(name.clone(), new_val);
    }
    updated
}

pub fn evaluate_ratchet_metrics(
    current: &BTreeMap<String, f64>,
    baseline: &BTreeMap<String, f64>,
) -> (Vec<MetricEvaluation>, usize) {
    let mut evaluations = Vec::new();
    let mut regressions = 0;

    for (name, curr_val) in current {
        let base_val = baseline.get(name).copied().unwrap_or(*curr_val);
        let (goal, direction, tier) = canonical_metric_spec(name);
        let is_higher_better = direction == MetricDirection::HigherIsBetter;

        let delta = if is_higher_better {
            curr_val - base_val
        } else {
            base_val - curr_val
        };

        const EPSILON: f64 = 1e-6;
        let status = if delta < -EPSILON {
            regressions += 1;
            MetricStatus::Regression
        } else if delta > EPSILON {
            MetricStatus::Improved
        } else {
            MetricStatus::Held
        };

        let (gap_to_goal, surplus_beyond_goal, goal_met) = if is_higher_better {
            if *curr_val >= goal - EPSILON {
                (0.0, *curr_val - goal, true)
            } else {
                (goal - *curr_val, 0.0, false)
            }
        } else if *curr_val <= goal + EPSILON {
            (0.0, goal - *curr_val, true)
        } else {
            (*curr_val - goal, 0.0, false)
        };

        evaluations.push(MetricEvaluation {
            name: name.clone(),
            current: *curr_val,
            baseline: base_val,
            delta,
            status,
            is_higher_better,
            goal,
            gap_to_goal,
            surplus_beyond_goal,
            goal_met,
            tier,
        });
    }

    (evaluations, regressions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_metric_specs_and_dual_horizon() {
        let (goal_mi, dir_mi, tier_mi) = canonical_metric_spec("maintainability_index_avg");
        assert_eq!(goal_mi, 90.0);
        assert_eq!(dir_mi, MetricDirection::HigherIsBetter);
        assert_eq!(tier_mi, GoalTier::HardEngineering);

        let (goal_ccn, dir_ccn, tier_ccn) = canonical_metric_spec("complexity_ccn_max");
        assert_eq!(goal_ccn, 10.0);
        assert_eq!(dir_ccn, MetricDirection::LowerIsBetter);
        assert_eq!(tier_ccn, GoalTier::HardEngineering);

        let (goal_phone, _, tier_phone) =
            canonical_metric_spec("unit_cost_user_phone_lifecycle_usd_max");
        assert_eq!(goal_phone, 9.50);
        assert_eq!(tier_phone, GoalTier::SoftVendorEconomics);

        let mut current = BTreeMap::new();
        current.insert("maintainability_index_avg".to_string(), 92.5);
        current.insert("complexity_ccn_max".to_string(), 12.0);

        let mut baseline = BTreeMap::new();
        baseline.insert("maintainability_index_avg".to_string(), 85.0);
        baseline.insert("complexity_ccn_max".to_string(), 12.0);

        let (evals, regressions) = evaluate_ratchet_metrics(&current, &baseline);
        assert_eq!(regressions, 0);

        let mi_eval = evals
            .iter()
            .find(|e| e.name == "maintainability_index_avg")
            .unwrap();
        assert_eq!(mi_eval.status, MetricStatus::Improved);
        assert_eq!(mi_eval.goal, 90.0);
        assert!(mi_eval.goal_met);
        assert_eq!(mi_eval.gap_to_goal, 0.0);
        assert!((mi_eval.surplus_beyond_goal - 2.5).abs() < 1e-4);

        let ccn_eval = evals
            .iter()
            .find(|e| e.name == "complexity_ccn_max")
            .unwrap();
        assert_eq!(ccn_eval.status, MetricStatus::Held);
        assert_eq!(ccn_eval.goal, 10.0);
        assert!(!ccn_eval.goal_met);
        assert_eq!(ccn_eval.gap_to_goal, 2.0);
        assert_eq!(ccn_eval.surplus_beyond_goal, 0.0);
    }
}
