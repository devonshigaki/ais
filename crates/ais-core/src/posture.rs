//! Runtime Behavioral Archetype & Cognitive Posture Resolver (Native Rust)
//!
//! Maps 14 psychological archetypes (Analysts, Sentinels, Diplomats, Explorers)
//! to adaptive frontend layout density and interaction modes:
//! - HighDensityGrid: Analytical metrics, data element breakdowns, factor weights
//! - TactileSandbox: Interactive sliders, rapid trial-and-error, <100ms feedback
//! - EmpatheticSummary: Human-centered copy, community impact, plain language

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchetypeGroup {
    Analysts,
    Sentinels,
    Diplomats,
    Explorers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitivePosture {
    HighDensityGrid,
    TactileSandbox,
    EmpatheticSummary,
    StandardBalanced,
}

impl CognitivePosture {
    pub fn show_high_density_table(&self) -> bool {
        matches!(self, Self::HighDensityGrid)
    }

    pub fn show_tactile_sandbox(&self) -> bool {
        matches!(self, Self::TactileSandbox)
    }

    pub fn show_factor_weights(&self) -> bool {
        matches!(self, Self::HighDensityGrid | Self::StandardBalanced)
    }

    pub fn default_view_mode(&self) -> &'static str {
        match self {
            Self::HighDensityGrid => "table",
            Self::TactileSandbox => "interactive",
            Self::EmpatheticSummary => "summary",
            Self::StandardBalanced => "cards",
        }
    }
}

pub fn resolve_archetype_group(archetype: &str) -> ArchetypeGroup {
    let lower = archetype.to_lowercase();
    if lower.contains("architect")
        || lower.contains("logician")
        || lower.contains("commander")
        || lower.contains("debater")
        || lower.contains("intj")
        || lower.contains("intp")
        || lower.contains("entj")
        || lower.contains("entp")
    {
        ArchetypeGroup::Analysts
    } else if lower.contains("logistician")
        || lower.contains("defender")
        || lower.contains("executive")
        || lower.contains("consul")
        || lower.contains("istj")
        || lower.contains("isfj")
        || lower.contains("estj")
        || lower.contains("esfj")
    {
        ArchetypeGroup::Sentinels
    } else if lower.contains("advocate")
        || lower.contains("mediator")
        || lower.contains("protagonist")
        || lower.contains("campaigner")
        || lower.contains("infj")
        || lower.contains("infp")
        || lower.contains("enfj")
        || lower.contains("enfp")
    {
        ArchetypeGroup::Diplomats
    } else {
        ArchetypeGroup::Explorers
    }
}

pub fn resolve_cognitive_posture(archetype: &str) -> CognitivePosture {
    match resolve_archetype_group(archetype) {
        ArchetypeGroup::Analysts | ArchetypeGroup::Sentinels => CognitivePosture::HighDensityGrid,
        ArchetypeGroup::Explorers => CognitivePosture::TactileSandbox,
        ArchetypeGroup::Diplomats => CognitivePosture::EmpatheticSummary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_analyst_posture() {
        let posture = resolve_cognitive_posture("Architect");
        assert_eq!(posture, CognitivePosture::HighDensityGrid);
        assert!(posture.show_high_density_table());
        assert!(posture.show_factor_weights());
        assert_eq!(posture.default_view_mode(), "table");
    }

    #[test]
    fn test_resolve_explorer_posture() {
        let posture = resolve_cognitive_posture("Virtuoso");
        assert_eq!(posture, CognitivePosture::TactileSandbox);
        assert!(posture.show_tactile_sandbox());
        assert_eq!(posture.default_view_mode(), "interactive");
    }

    #[test]
    fn test_resolve_diplomat_posture() {
        let posture = resolve_cognitive_posture("Advocate");
        assert_eq!(posture, CognitivePosture::EmpatheticSummary);
        assert_eq!(posture.default_view_mode(), "summary");
    }
}
