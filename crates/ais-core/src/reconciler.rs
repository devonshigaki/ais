//! Rule Contradiction Reconciler & Precedence Matrix (Native Rust)

use crate::findings::Finding;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RulePriorityTier {
    ArchitecturalPurity = 1,
    ErgonomicConvenience = 2,
    PerformanceLatency = 3,
    UserPsychologyAndUX = 4,
    DataIntegrityAndDDIA = 5,
    SecurityAndPrivacy = 6,
    ZeroCustodyCryptography = 7,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciledConflict {
    pub surface_location: String,
    pub rule_a: String,
    pub rule_b: String,
    pub winning_rule: String,
    pub winning_tier: RulePriorityTier,
    pub common_sense_resolution: String,
    pub prescribed_recipe: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReconciliationReport {
    pub total_findings_analyzed: usize,
    pub conflicts_detected: usize,
    pub conflicts_reconciled: usize,
    pub resolutions: Vec<ReconciledConflict>,
}

pub fn classify_rule_tier(rule_code: &str) -> RulePriorityTier {
    if rule_code.starts_with("ZERO")
        || rule_code.starts_with("CHAIN")
        || rule_code.starts_with("DP")
    {
        RulePriorityTier::ZeroCustodyCryptography
    } else if rule_code.starts_with("SEC")
        || rule_code.starts_with("AUTH")
        || rule_code.starts_with("PII")
        || rule_code.starts_with("HTMX-002")
    {
        RulePriorityTier::SecurityAndPrivacy
    } else if rule_code.starts_with("DDIA")
        || rule_code.starts_with("MIGRATE")
        || rule_code.starts_with("DB")
    {
        RulePriorityTier::DataIntegrityAndDDIA
    } else if rule_code.starts_with("BEHAV")
        || rule_code.starts_with("UX")
        || rule_code.starts_with("A11Y")
        || rule_code.starts_with("JOURNEY")
    {
        RulePriorityTier::UserPsychologyAndUX
    } else if rule_code.starts_with("PERF")
        || rule_code.starts_with("WASM")
        || rule_code.starts_with("OPP-PERF")
    {
        RulePriorityTier::PerformanceLatency
    } else if rule_code.starts_with("DDD")
        || rule_code.starts_with("POT")
        || rule_code.starts_with("ARCH")
    {
        RulePriorityTier::ArchitecturalPurity
    } else {
        RulePriorityTier::ErgonomicConvenience
    }
}

pub fn reconcile_rule_contradictions(findings: &[Finding]) -> ReconciliationReport {
    let mut report = ReconciliationReport {
        total_findings_analyzed: findings.len(),
        ..Default::default()
    };

    let mut site_map: HashMap<String, Vec<&Finding>> = HashMap::new();
    for f in findings {
        let key = format!("{}:{}", f.file.display(), f.line);
        site_map.entry(key).or_default().push(f);
    }

    for (site, group) in site_map {
        if group.len() < 2 {
            continue;
        }
        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let f_a = group[i];
                let f_b = group[j];
                let tier_a = classify_rule_tier(&f_a.rule_code);
                let tier_b = classify_rule_tier(&f_b.rule_code);

                if tier_a != tier_b {
                    let (winner, winning_tier, loser) = if tier_a > tier_b {
                        (&f_a.rule_code, tier_a, &f_b.rule_code)
                    } else {
                        (&f_b.rule_code, tier_b, &f_a.rule_code)
                    };

                    let (res, recipe) = resolve_tension(winner, loser);
                    report.conflicts_detected += 1;
                    report.conflicts_reconciled += 1;
                    report.resolutions.push(ReconciledConflict {
                        surface_location: site.clone(),
                        rule_a: f_a.rule_code.clone(),
                        rule_b: f_b.rule_code.clone(),
                        winning_rule: winner.clone(),
                        winning_tier,
                        common_sense_resolution: res,
                        prescribed_recipe: recipe,
                    });
                }
            }
        }
    }

    report
}

fn resolve_tension(winner: &str, loser: &str) -> (String, String) {
    if (winner.contains("SEC") || winner.contains("HTMX-002")) && loser.contains("BEHAV-005") {
        (
            "Security takes precedence over sub-400ms speed.".to_string(),
            "Prescribe non-blocking confirmation dialog with optimistic hx-indicator.".to_string(),
        )
    } else if winner.contains("DDIA") && loser.contains("DDD") {
        (
            "Data Integrity takes precedence over DDD abstraction purity.".to_string(),
            "Grant scoped adapter borrow with zero-copy projection.".to_string(),
        )
    } else {
        (
            format!("Rule [{winner}] governs over [{loser}]."),
            "Apply higher-tier constraint with scoped exemption for secondary rule.".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_security_overrules_ergonomics_tension() {
        let f1 = Finding::new(
            "HTMX-002",
            crate::findings::Severity::High,
            "CSRF missing",
            PathBuf::from("transfer.html"),
            10,
        );
        let f2 = Finding::new(
            "BEHAV-005",
            crate::findings::Severity::Medium,
            "Missing indicator",
            PathBuf::from("transfer.html"),
            10,
        );

        let report = reconcile_rule_contradictions(&[f1, f2]);
        assert_eq!(report.conflicts_detected, 1);
        assert_eq!(report.resolutions[0].winning_rule, "HTMX-002");
    }

    #[test]
    fn test_precedence_tier_ordering() {
        assert!(RulePriorityTier::ZeroCustodyCryptography > RulePriorityTier::SecurityAndPrivacy);
        assert!(RulePriorityTier::SecurityAndPrivacy > RulePriorityTier::DataIntegrityAndDDIA);
        assert!(RulePriorityTier::DataIntegrityAndDDIA > RulePriorityTier::UserPsychologyAndUX);
        assert!(RulePriorityTier::UserPsychologyAndUX > RulePriorityTier::PerformanceLatency);
        assert!(RulePriorityTier::PerformanceLatency > RulePriorityTier::ArchitecturalPurity);
    }
}
