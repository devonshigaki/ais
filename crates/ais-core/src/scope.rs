//! AIS Rule Scope Taxonomy: Universal Agnostic vs. Project-Scoped Custom Rules.

use serde::{Deserialize, Serialize};

/// Classification of an AIS rule's applicability scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RuleScope {
    /// Agnostic: Universal invariants enforced across all repositories and languages.
    /// (e.g. Memory safety, Power-of-Ten, GoF, DDD ubiquitous language, Vendor agnosticism, Brand eradication, Secrets, AST deadlocks).
    Agnostic,
    /// Custom: Project-specific rules constrained to a specific codebase.
    /// E.g. "shigaki-srv" for bare-metal hardware/eBPF/K3s, or "product" for fintech multi-tenant SaaS.
    Custom(&'static str),
}

impl std::fmt::Display for RuleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Agnostic => write!(f, "agnostic"),
            Self::Custom(project) => write!(f, "custom:{project}"),
        }
    }
}

impl RuleScope {
    /// Returns true if this rule scope is active for the target project.
    #[must_use]
    pub fn is_active_for(&self, target_project: Option<&str>, active_scopes: &[String]) -> bool {
        // If "all" is explicitly passed in active_scopes, everything is active
        if active_scopes.iter().any(|s| s == "all" || s == "*") {
            return true;
        }

        match self {
            Self::Agnostic => {
                // Agnostic rules are active by default everywhere, unless explicitly suppressed
                !active_scopes.iter().any(|s| s == "no-agnostic")
            }
            Self::Custom(scope_project) => {
                // If active_scopes explicitly names this project or custom scope
                if active_scopes
                    .iter()
                    .any(|s| s == *scope_project || s == &format!("custom:{scope_project}"))
                {
                    return true;
                }
                // If target_project matches
                if let Some(tp) = target_project {
                    if tp == *scope_project {
                        return true;
                    }
                }
                false
            }
        }
    }
}

/// Status record for displaying which rules are enabled vs disabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleStatusEntry {
    pub code: String,
    pub name: String,
    pub scope: String,
    pub severity: String,
    pub enabled: bool,
    pub probe_status: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agnostic_rule_active_by_default() {
        let scope = RuleScope::Agnostic;
        assert!(scope.is_active_for(Some("shigaki-srv"), &[]));
        assert!(scope.is_active_for(Some("product"), &[]));
        assert!(scope.is_active_for(None, &[]));
    }

    #[test]
    fn test_custom_rule_isolation() {
        let srv_rule = RuleScope::Custom("shigaki-srv");
        assert!(srv_rule.is_active_for(Some("shigaki-srv"), &[]));
        assert!(!srv_rule.is_active_for(Some("product"), &[]));
        assert!(!srv_rule.is_active_for(None, &[]));

        let prod_rule = RuleScope::Custom("product");
        assert!(!prod_rule.is_active_for(Some("shigaki-srv"), &[]));
        assert!(prod_rule.is_active_for(Some("product"), &[]));
    }

    #[test]
    fn test_explicit_all_scopes_enables_everything() {
        let srv_rule = RuleScope::Custom("shigaki-srv");
        let active = vec!["all".to_string()];
        assert!(srv_rule.is_active_for(Some("product"), &active));
    }
}
