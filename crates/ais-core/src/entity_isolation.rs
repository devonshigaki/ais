//! Legal-Entity Isolation Boundary Scanner (BND-ENT-001..004)
//!
//! Statically audits webmail, Postfix alias routing, Documenso K8s deployments,
//! Traefik routers, and Cloudflare edge workers for cross-entity leakage.

use crate::findings::{Confidence, Finding, Severity};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalEntity {
    FreshCreditInc,
    FreshCreditOrg,
    FreshCreditXyz,
    ZaplitInc,
    DevonPersonal,
}

pub fn domain_legal_entity(domain: &str) -> Option<LegalEntity> {
    let d = domain.trim().trim_end_matches('.').to_ascii_lowercase();
    match d.as_str() {
        "freshcredit.com" | "freshcredit.ai" | "freshcredit.app" => {
            Some(LegalEntity::FreshCreditInc)
        }
        "freshcredit.org" => Some(LegalEntity::FreshCreditOrg),
        "freshcredit.xyz" => Some(LegalEntity::FreshCreditXyz),
        "zaplit.com" | "zaplit.org" => Some(LegalEntity::ZaplitInc),
        "tism.ing" | "devonshigaki.com" => Some(LegalEntity::DevonPersonal),
        _ => None,
    }
}

pub fn check_roundcube_config_content(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    if content.contains("$config['skin']") && !content.contains("$config['use_https'] = true") {
        findings.push(
            Finding::new(
                "BND-ENT-001",
                Severity::High,
                "Roundcube config missing `$config['use_https'] = true;` (session cookie_secure=false)",
                path.to_path_buf(),
                1,
            )
            .with_confidence(Confidence::new(0.98)),
        );
    }
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.contains("$config['username_domain']") {
            if let Some(rhs) = trimmed.split('=').nth(1) {
                if rhs.contains("=>") || rhs.contains('[') || rhs.contains("array(") {
                    findings.push(
                        Finding::new(
                            "BND-ENT-001",
                            Severity::Critical,
                            "Roundcube `username_domain` uses multi-domain array dropdown across entities",
                            path.to_path_buf(),
                            idx + 1,
                        )
                        .with_confidence(Confidence::new(0.99)),
                    );
                }
            }
        }
    }
    findings
}

pub fn check_roundcube_skin_content(skin: &str, path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let lower = content.to_ascii_lowercase();
    let forbidden: &[&str] = match skin {
        "freshcredit" => &["zaplit", "tism.ing"],
        "freshcredit-org" | "freshcredit-xyz" => &["zaplit", "freshcredit c corp", "tism.ing"],
        "zaplit" => &["freshcredit", "tism.ing"],
        "tism" => &["freshcredit", "zaplit"],
        _ => &[],
    };
    for kw in forbidden {
        if lower.contains(kw) {
            findings.push(
                Finding::new(
                    "BND-ENT-001",
                    Severity::Critical,
                    format!("Roundcube skin `{skin}` leaks cross-entity branding keyword `{kw}`"),
                    path.to_path_buf(),
                    1,
                )
                .with_confidence(Confidence::new(0.97)),
            );
        }
    }
    findings
}

pub fn check_postfix_virtual_content(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut cols = trimmed.split_whitespace();
        let Some(lhs) = cols.next() else {
            continue;
        };
        if lhs.starts_with('@') {
            findings.push(
                Finding::new(
                    "BND-ENT-002",
                    Severity::Critical,
                    format!("Wildcard catch-all `{lhs}` forbidden in Postfix virtual map"),
                    path.to_path_buf(),
                    idx + 1,
                )
                .with_confidence(Confidence::new(0.99)),
            );
            continue;
        }
        let Some(rhs) = cols.next() else {
            continue;
        };
        let Some(lhs_dom) = lhs.split('@').nth(1) else {
            continue;
        };
        let Some(rhs_dom) = rhs.split('@').nth(1) else {
            continue;
        };
        if let (Some(e1), Some(e2)) = (domain_legal_entity(lhs_dom), domain_legal_entity(rhs_dom)) {
            if e1 != e2 {
                findings.push(
                    Finding::new(
                        "BND-ENT-002",
                        Severity::Critical,
                        format!(
                            "Postfix alias `{lhs} -> {rhs}` crosses legal-entity boundary ({e1:?} -> {e2:?})"
                        ),
                        path.to_path_buf(),
                        idx + 1,
                    )
                    .with_confidence(Confidence::new(0.99)),
                );
            }
        }
    }
    findings
}

fn check_documenso_k8s_isolation(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut seen_secrets: HashSet<String> = HashSet::new();
    let mut current_ns = String::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(ns) = trimmed.strip_prefix("namespace:") {
            current_ns = ns.trim().trim_matches('"').to_string();
        }
        if trimmed.contains("secretKeyRef:") && line.contains("SIGN_DATABASE_URL") {
            if let Some(name_start) = line.find("name:") {
                let rest = &line[name_start + 5..];
                let sec_name = rest
                    .split([',', '}'])
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"');
                if !sec_name.is_empty() {
                    let prefix = format!("SIGN_DB:{sec_name}:");
                    if seen_secrets
                        .iter()
                        .any(|s| s.starts_with(&prefix) && !s.ends_with(&format!(":{current_ns}")))
                    {
                        findings.push(
                            Finding::new(
                                "BND-ENT-003",
                                Severity::Critical,
                                format!(
                                    "Documenso DB secret `{sec_name}` shared across namespaces"
                                ),
                                path.to_path_buf(),
                                idx + 1,
                            )
                            .with_confidence(Confidence::new(0.95)),
                        );
                    }
                    seen_secrets.insert(format!("{prefix}{current_ns}"));
                }
            }
        }
    }
    findings
}

fn check_traefik_and_worker(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if fname == "dynamic-k8s.yml" {
        if let Some(idx) = content.find("webmail-https:") {
            let block = &content[idx..content.len().min(idx + 1200)];
            if block.contains("webmail.freshcredit.com") && block.contains("webmail.zaplit.com") {
                findings.push(
                    Finding::new(
                        "BND-ENT-004",
                        Severity::High,
                        "Traefik `webmail-https` router mixes FreshCredit and Zaplit SANs in one TLS cert",
                        path.to_path_buf(),
                        1,
                    )
                    .with_confidence(Confidence::new(0.96)),
                );
            }
        }
    } else if fname == "failover-worker.js"
        && content.contains("\"X-FreshCredit-Origin\": \"split-brain-fence\"")
    {
        findings.push(
            Finding::new(
                "BND-ENT-004",
                Severity::High,
                "Cloudflare failover-worker hardcodes `X-FreshCredit-Origin` on all entity 503 fences",
                path.to_path_buf(),
                1,
            )
            .with_confidence(Confidence::new(0.95)),
        );
    }
    findings
}

/// Scans `root` for legal-entity isolation violations across mail, K8s, and edge configs.
#[must_use]
pub fn scan_entity_isolation(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let infra_dir = root.join("infra");
    if !infra_dir.is_dir() {
        return findings;
    }

    for entry in WalkDir::new(&infra_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let p = entry.path();
        let s = p.to_string_lossy();
        let Ok(content) = fs::read_to_string(p) else {
            continue;
        };
        if s.contains("/roundcube/config/") && s.ends_with(".php") {
            findings.extend(check_roundcube_config_content(p, &content));
        } else if s.contains("/roundcube/skins/")
            && matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("html" | "css" | "json")
            )
        {
            if let Some(skin) = s
                .split("/roundcube/skins/")
                .nth(1)
                .and_then(|rest| rest.split('/').next())
            {
                findings.extend(check_roundcube_skin_content(skin, p, &content));
            }
        } else if p.file_name().and_then(|n| n.to_str()) == Some("postfix-virtual.cf") {
            findings.extend(check_postfix_virtual_content(p, &content));
        } else if s.contains("/infra/k8s/") && s.ends_with(".yaml") {
            findings.extend(check_documenso_k8s_isolation(p, &content));
        } else if p.file_name().and_then(|n| n.to_str()) == Some("dynamic-k8s.yml")
            || p.file_name().and_then(|n| n.to_str()) == Some("failover-worker.js")
        {
            findings.extend(check_traefik_and_worker(p, &content));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_roundcube_cross_entity_dropdown_and_missing_https() {
        let bad = "<?php\n$config['skin'] = 'freshcredit';\n$config['username_domain'] = ['freshcredit.com' => 'FC', 'zaplit.com' => 'Zaplit'];\n";
        let f = check_roundcube_config_content(Path::new("zz.php"), bad);
        assert_eq!(f.len(), 2);
        assert!(f.iter().all(|x| x.rule_code == "BND-ENT-001"));

        let good = "<?php\n$config['use_https'] = true;\n$config['skin'] = 'zaplit';\n$config['username_domain'] = 'zaplit.com';\n";
        assert!(check_roundcube_config_content(Path::new("zz.php"), good).is_empty());
    }

    #[test]
    fn detects_postfix_cross_entity_alias() {
        let bad = "hudson@freshcredit.xyz hudson.schenck@freshcredit.com\n";
        let f = check_postfix_virtual_content(Path::new("postfix-virtual.cf"), bad);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule_code, "BND-ENT-002");

        let good =
            "hudson@freshcredit.xyz hello@freshcredit.xyz\npostmaster@tism.ing admin@tism.ing\n";
        assert!(check_postfix_virtual_content(Path::new("postfix-virtual.cf"), good).is_empty());
    }

    #[test]
    fn detects_roundcube_skin_branding_leak() {
        let f = check_roundcube_skin_content(
            "zaplit",
            Path::new("login.html"),
            "<span>&copy; FreshCredit C Corp.</span>",
        );
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].rule_code, "BND-ENT-001");
    }
}
