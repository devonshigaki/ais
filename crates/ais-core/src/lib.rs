//! FreshCredit Autonomous Iteration System (AIS) Core Engine
//!
//! Agnostic, modular, AST-aware architecture and code quality framework:
//! - Finding & Severity primitives
//! - Risk and RICE scoring engine
//! - Quality metrics (CCN, Maintainability Index, LOC)
//! - Content-hash incremental scanning cache
//! - Universal Baseline Ratchet
//! - Meta-audit, anti-thrashing, and disconnected rule detection
//! - Multi-model complexity routing & attribution watermarking
//! - Universal language AST abstractions
// POT-010: AIS Core library internal documentation
#![allow(missing_docs)]

pub mod ast;
pub mod ast_queries;
pub mod cohesion;
pub mod coupling;
pub mod hotspots;
pub mod rust_health;
pub use cohesion::*;

pub mod test_gaps;
pub use test_gaps::*;

pub use coupling::*;
pub use hotspots::*;
pub use rust_health::*;

pub mod allure;
pub use allure::*;
pub mod discovery;
pub use discovery::*;
pub mod coverage_bleed;
pub use coverage_bleed::*;
pub mod doctor;
pub use doctor::*;

pub mod cache;
pub mod churn;
pub mod dedup;
pub mod findings;
pub mod graph;
pub mod healer;
pub mod merit;
pub mod meta;
pub mod metrics;
pub mod mutation;
pub mod opportunities;
pub use opportunities::*;

pub mod posture;
pub mod provenance;
pub use provenance::*;
pub mod ratchet;
pub mod reconciler;
pub mod risk;
pub mod structure;
pub use structure::*;

pub use posture::*;

pub mod router;
pub use reconciler::*;

pub mod telemetry;
pub use merit::*;

pub use ast::*;
pub use ast_queries::*;
pub use cache::*;
pub use churn::*;
pub use dedup::*;
pub use findings::*;
pub use graph::*;
pub use healer::*;
pub use meta::*;
pub use metrics::*;
pub use mutation::*;
pub use ratchet::*;
pub use risk::*;

pub use telemetry::*;

pub use router::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_smoke() {
        let f = Finding::new(
            "SMOKE-001",
            Severity::High,
            "Smoke test",
            std::path::PathBuf::from("test.rs"),
            1,
        );
        let risk = evaluate_risk(&f);
        assert!(risk.contextual_risk_score > 0.0);
        let (model, _tier) = route_model("GOF-001", 16, 85.0);
        assert_eq!(model, "gemini-2.5-pro");
    }
}
