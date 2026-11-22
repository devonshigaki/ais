//! Contextual Risk Scoring (CRS) & RICE Prioritization Engine

use crate::findings::{Finding, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub finding: Finding,
    pub contextual_risk_score: f32,
    pub financial: FinancialEngineeringDebt,
    pub priority: ActionPriority,
    pub rice: RiceTriage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialEngineeringDebt {
    pub principal_cost_usd: f32,
    pub annual_interest_usd: f32,
    pub remediation_hours: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActionPriority {
    P0Blocker,
    P1Urgent,
    P2Standard,
    P3Backlog,
}

impl std::fmt::Display for ActionPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P0Blocker => write!(f, "P0 [IMMEDIATE BLOCKER]"),
            Self::P1Urgent => write!(f, "P1 [URGENT]"),
            Self::P2Standard => write!(f, "P2 [STANDARD]"),
            Self::P3Backlog => write!(f, "P3 [LOW RISK]"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiceTriage {
    pub reach: f32,
    pub impact: f32,
    pub confidence: f32,
    pub effort_hours: f32,
    pub rice_score: f32,
    pub batch_category: String,
}

fn calculate_factors(path_str: &str, severity: Severity) -> (f32, f32, f32) {
    let path = path_str.to_lowercase();
    let reach = if path.contains("tests/") || path.contains("test_") || path.contains("fixtures/") {
        0.25
    } else if path.contains("apps/product/api/src/routes/")
        || path.contains("routes/v1/")
        || path.contains("infra/cloudflare/")
    {
        1.30
    } else if path.contains("apps/product/app/src/routes/")
        || path.contains("routes/")
        || path.contains("api/")
        || path.contains("infra/k8s/")
        || path.contains("infra/mail/")
    {
        1.15
    } else if path.contains("infra/terraform")
        || path.contains("infra/gcp")
        || path.contains("infra/scripts/")
        || path.contains("crates/shigaki-srv/src/verify/")
        || path.contains("crates/shigaki-srv/src/psi")
        || path.contains("crates/shigaki-srv/src/ebpf_trace")
        || path.contains("apps/product/native/")
        || path.contains("apps/native/")
    {
        1.10
    } else if path.contains("crates/") || path.contains("services/") {
        1.00
    } else if path.contains("scripts/") {
        0.70
    } else {
        0.90
    };

    let domain_criticality = if path.contains("vault")
        || path.contains("secret")
        || path.contains("crypto")
        || path.contains("auth")
        || path.contains("kms")
        || path.contains("unseal")
        || path.contains("failover")
        || path.contains("fencing")
    {
        1.35
    } else if path.contains("payment")
        || path.contains("billing")
        || path.contains("stripe")
        || path.contains("settlement")
        || path.contains("wallet")
        || path.contains("dr_backup")
        || path.contains("dr_restore")
        || path.contains("wal_stream")
        || path.contains("rpo_hourly")
    {
        1.30
    } else if path.contains("pii")
        || path.contains("identity")
        || path.contains("kyc")
        || path.contains("user_profile")
        || path.contains("postgres")
        || path.contains("sqld")
        || path.contains("traefik")
        || path.contains("prometheus")
    {
        1.25
    } else if path.contains("template")
        || path.contains("static")
        || path.contains("css")
        || path.contains("i18n")
    {
        0.80
    } else {
        1.00
    };

    let (sev_multiplier, base_score) = match severity {
        Severity::Critical => (1.5, 90.0),
        Severity::High => (1.2, 75.0),
        Severity::Medium => (1.0, 45.0),
        Severity::Low => (0.8, 20.0),
        Severity::Info => (0.5, 5.0),
    };

    let criticality = domain_criticality * sev_multiplier;
    (reach, criticality, base_score)
}

fn calculate_priority(contextual_risk_score: f32) -> ActionPriority {
    if contextual_risk_score >= 80.0 {
        ActionPriority::P0Blocker
    } else if contextual_risk_score >= 60.0 {
        ActionPriority::P1Urgent
    } else if contextual_risk_score >= 35.0 {
        ActionPriority::P2Standard
    } else {
        ActionPriority::P3Backlog
    }
}

pub fn evaluate_risk(finding: &Finding) -> RiskAssessment {
    let path_str = finding.file.to_string_lossy();
    let (reach, criticality, base_score) = calculate_factors(&path_str, finding.severity);

    let contextual_risk_score =
        (base_score * reach * criticality * finding.confidence.0).clamp(0.0, 100.0);
    let priority = calculate_priority(contextual_risk_score);

    let effort_hours = match finding.severity {
        Severity::Critical => 4.0,
        Severity::High => 3.0,
        Severity::Medium => 2.0,
        _ => 1.0,
    };

    let impact = criticality;
    let rice_score = if effort_hours > 0.0 {
        ((reach * impact * finding.confidence.0 * 100.0) / effort_hours * 10.0).round() / 10.0
    } else {
        100.0
    };

    let hourly_rate = 150.0;
    let principal_cost_usd = effort_hours * hourly_rate;
    let interest_multiplier = match priority {
        ActionPriority::P0Blocker => 2.0,
        ActionPriority::P1Urgent => 1.0,
        ActionPriority::P2Standard => 0.5,
        ActionPriority::P3Backlog => 0.1,
    };
    let annual_interest_usd = principal_cost_usd * interest_multiplier;

    let batch_category =
        if priority == ActionPriority::P0Blocker || priority == ActionPriority::P1Urgent {
            "Strategic Batch (Sequenced)".to_string()
        } else {
            "Opportunistic (Opportunistic Cleanups)".to_string()
        };

    RiskAssessment {
        finding: finding.clone(),
        contextual_risk_score,
        financial: FinancialEngineeringDebt {
            principal_cost_usd,
            annual_interest_usd,
            remediation_hours: effort_hours,
        },
        priority,
        rice: RiceTriage {
            reach,
            impact,
            confidence: finding.confidence.0,
            effort_hours,
            rice_score,
            batch_category,
        },
    }
}

pub fn rank_findings_by_risk(findings: &mut [Finding]) {
    findings.sort_by(|a, b| {
        let risk_a = evaluate_risk(a).contextual_risk_score;
        let risk_b = evaluate_risk(b).contextual_risk_score;
        risk_b
            .partial_cmp(&risk_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::findings::Confidence;
    use std::path::PathBuf;

    #[test]
    fn prioritizes_sovereign_dr_and_edge_failover_above_generic_scripts() {
        let edge_finding = Finding::new(
            "TEST-001",
            Severity::Medium,
            "edge timeout",
            PathBuf::from("infra/cloudflare/failover-worker.js"),
            1,
        )
        .with_confidence(Confidence::new(0.9));
        let dr_finding = Finding::new(
            "TEST-002",
            Severity::Medium,
            "dr script gap",
            PathBuf::from("infra/scripts/unified_dr_backup.sh"),
            1,
        )
        .with_confidence(Confidence::new(0.9));
        let generic_script = Finding::new(
            "TEST-003",
            Severity::Medium,
            "helper script",
            PathBuf::from("scripts/dev_helper.sh"),
            1,
        )
        .with_confidence(Confidence::new(0.9));

        assert!(
            evaluate_risk(&edge_finding).contextual_risk_score
                > evaluate_risk(&generic_script).contextual_risk_score
        );
        assert!(
            evaluate_risk(&dr_finding).contextual_risk_score
                > evaluate_risk(&generic_script).contextual_risk_score
        );
    }
}
