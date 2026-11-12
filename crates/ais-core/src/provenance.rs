//! Code Provenance & Triage Classification Engine
//!
//! Categorizes every file and finding across `/Users/devonshigaki/Developer/freshcredit/*`
//! into formal ownership tiers and actionable remediation paths:
//! - First-Party Proprietary (apps/product/*, crates/*, packages/*, infra/*, xtask/*)
//! - Forked & Satellite Services (apps/service/*)
//! - Vendored Third-Party Bundles (static/js/vendor/*, smoldot-bundle, nomt-wasm)

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceTier {
    /// Core FreshCredit proprietary intellectual property
    FirstPartyProprietary,
    /// Satellite services adapted from open-source engines (Cal.com, Twenty, Documenso, etc.)
    ForkedSatellite,
    /// Upstream vendor bundles, third-party libraries, and compiled wasm
    VendoredThirdParty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriageAction {
    /// Remediate directly in this repository (auto-heal or manual commit)
    UpdateOurselves,
    /// Submit pull request or issue to upstream open-source project
    SubmitUpstreamPr,
    /// Patch local satellite adapter wrapper AND submit upstream pull request
    BothLocalAndUpstreamPr,
    /// Upgrade pinned vendor bundle version or request vendor security patch
    UpgradeVendorBundle,
}

impl ProvenanceTier {
    pub fn label(&self) -> &'static str {
        match self {
            Self::FirstPartyProprietary => "First-Party Proprietary",
            Self::ForkedSatellite => "Forked Satellite Service",
            Self::VendoredThirdParty => "Vendored Third-Party Asset",
        }
    }
}

impl TriageAction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::UpdateOurselves => "[UPDATE-OURSELVES]",
            Self::SubmitUpstreamPr => "[SUBMIT-UPSTREAM-PR]",
            Self::BothLocalAndUpstreamPr => "[BOTH-LOCAL-AND-UPSTREAM-PR]",
            Self::UpgradeVendorBundle => "[UPGRADE-VENDOR-BUNDLE]",
        }
    }
}

/// Classify path into provenance tier and recommended action
pub fn classify_path(path: &Path) -> (ProvenanceTier, TriageAction) {
    let path_str = path.to_string_lossy();

    // 1. Vendored third-party assets
    if path_str.contains("static/js/vendor")
        || path_str.contains("static/simulator/js/vendor")
        || path_str.contains("smoldot-bundle")
        || path_str.contains("nomt-wasm")
        || path_str.contains(".twenty")
    {
        return (
            ProvenanceTier::VendoredThirdParty,
            TriageAction::UpgradeVendorBundle,
        );
    }

    // 2. Forked satellite services
    if path_str.contains("apps/service/") {
        if path_str.contains("/src/") && path_str.ends_with(".rs") {
            return (
                ProvenanceTier::ForkedSatellite,
                TriageAction::BothLocalAndUpstreamPr,
            );
        }
        return (
            ProvenanceTier::ForkedSatellite,
            TriageAction::SubmitUpstreamPr,
        );
    }

    // 3. First-party proprietary core
    (
        ProvenanceTier::FirstPartyProprietary,
        TriageAction::UpdateOurselves,
    )
}

/// Canonical, robust test/mock path checker using exact directory components and filename suffixes.
/// Eliminates substring false positives like "testnet-agent-bridge" or "attestation".
pub fn is_test_or_mock_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    if path_str.contains("scanner_canaries") || path_str.contains("canary") {
        return false;
    }

    let test_dir_names = [
        "tests",
        "test",
        "__tests__",
        "benches",
        "bench",
        "fixtures",
        "test_fixtures",
        "test-util",
        "test_util",
        "test-helpers",
        "test_helpers",
        "testing",
    ];

    for comp in path.components() {
        if let std::path::Component::Normal(c) = comp {
            if let Some(s) = c.to_str() {
                if test_dir_names.contains(&s) {
                    return true;
                }
            }
        }
    }

    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    file_name == "tests.rs"
        || file_name == "test.rs"
        || file_name.starts_with("test_")
        || file_name.starts_with("tests_")
        || file_name.starts_with("mock_")
        || file_name.ends_with("_test.rs")
        || file_name.ends_with("_tests.rs")
        || file_name.ends_with(".test.ts")
        || file_name.ends_with(".test.tsx")
        || file_name.ends_with(".spec.ts")
        || file_name.ends_with(".spec.tsx")
        || file_name.ends_with(".test.js")
        || file_name.ends_with(".spec.js")
        || file_name == "tests.rs"
        || file_name == "test.rs"
        || file_name == "mock.rs"
        || file_name.ends_with("_mock.rs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testnet_crate_is_not_blinded() {
        let testnet_path = Path::new("crates/testnet-agent-bridge/src/lib.rs");
        assert!(
            !is_test_or_mock_path(testnet_path),
            "testnet-agent-bridge must not be classified as a test file"
        );
    }

    #[test]
    fn test_real_test_paths_are_recognized() {
        assert!(is_test_or_mock_path(Path::new(
            "crates/core/tests/integration.rs"
        )));
        assert!(is_test_or_mock_path(Path::new(
            "apps/product/app/src/foo_test.rs"
        )));
        assert!(is_test_or_mock_path(Path::new(
            "apps/product/website/components/button.test.tsx"
        )));
        assert!(is_test_or_mock_path(Path::new(
            "crates/engine-ports/src/wallet/mock.rs"
        )));
    }

    #[test]
    fn test_provenance_classification() {
        let (tier1, action1) = classify_path(Path::new("apps/product/app/src/main.rs"));
        assert_eq!(tier1, ProvenanceTier::FirstPartyProprietary);
        assert_eq!(action1, TriageAction::UpdateOurselves);

        let (tier2, _) = classify_path(Path::new("apps/service/crm/src/main.rs"));
        assert_eq!(tier2, ProvenanceTier::ForkedSatellite);

        let (tier3, action3) =
            classify_path(Path::new("apps/product/app/static/js/vendor/idkit.js"));
        assert_eq!(tier3, ProvenanceTier::VendoredThirdParty);
        assert_eq!(action3, TriageAction::UpgradeVendorBundle);
    }
}
