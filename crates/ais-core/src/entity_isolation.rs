//! Legal-Entity Isolation Boundary Scanner (BND-ENT-001..007)
//!
//! Statically audits webmail, Postfix alias routing, Documenso K8s deployments,
//! Traefik routers, Cloudflare edge workers, and entity ServiceAccount / fencing isolation.

use crate::findings::{Confidence, Finding, Severity};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub fn host_legal_entity(host: &str) -> Option<LegalEntity> {
    let mut current = host
        .trim()
        .trim_matches(|c| matches!(c, '`' | '"' | '\''))
        .trim_end_matches('.')
        .to_ascii_lowercase();
    while !current.is_empty() {
        if let Some(ent) = domain_legal_entity(&current) {
            return Some(ent);
        }
        let Some((_, rest)) = current.split_once('.') else {
            break;
        };
        current = rest.to_string();
    }
    None
}

fn service_namespace_legal_entity(url: &str) -> Option<LegalEntity> {
    let before_svc = url.split(".svc.cluster.local").next()?;
    let ns = before_svc.rsplit('.').next()?.trim();
    match ns {
        "freshcredit" => Some(LegalEntity::FreshCreditInc),
        "freshcredit-org" => Some(LegalEntity::FreshCreditOrg),
        "freshcredit-xyz" => Some(LegalEntity::FreshCreditXyz),
        "zaplit" => Some(LegalEntity::ZaplitInc),
        "personal-tism" => Some(LegalEntity::DevonPersonal),
        _ => None,
    }
}

fn extract_traefik_rule_hosts(rule: &str) -> Vec<String> {
    let mut hosts = Vec::new();
    let mut rest = rule;
    while let Some(pos) = rest.find("Host(") {
        let after = &rest[pos + 5..];
        let Some(end) = after.find(')') else {
            break;
        };
        let inner = &after[..end];
        for raw in inner.split(',') {
            let cleaned = raw
                .trim()
                .trim_matches(|c| matches!(c, '`' | '"' | '\'' | ' '))
                .to_ascii_lowercase();
            if !cleaned.is_empty() {
                hosts.push(cleaned);
            }
        }
        rest = &after[end + 1..];
    }
    hosts
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

fn parse_traefik_service_entities(content: &str) -> HashMap<String, LegalEntity> {
    let mut service_entities = HashMap::new();
    let mut in_services = false;
    let mut cur_svc: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with("  services:") {
            in_services = true;
            cur_svc = None;
            continue;
        } else if line.starts_with("  routers:") || !line.starts_with(' ') {
            in_services = false;
            cur_svc = None;
            continue;
        }
        if !in_services {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent == 4 && trimmed.ends_with(':') {
            cur_svc = Some(trimmed.trim_end_matches(':').to_string());
        } else if let (Some(ref svc), Some(url_raw)) = (&cur_svc, trimmed.strip_prefix("- url:")) {
            let url = url_raw.trim().trim_matches('"').trim_matches('\'');
            if let Some(ent) = service_namespace_legal_entity(url) {
                service_entities.insert(svc.clone(), ent);
            }
        }
    }
    service_entities
}

#[derive(Default)]
struct TraefikRouterInfo {
    name: String,
    line_num: usize,
    hosts: Vec<String>,
    service: Option<String>,
}

fn parse_traefik_routers(content: &str) -> Vec<TraefikRouterInfo> {
    let mut routers = Vec::new();
    let mut in_routers = false;
    let mut in_tls_domains = false;
    let mut in_sans = false;
    let mut current: Option<TraefikRouterInfo> = None;

    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with("  routers:") {
            in_routers = true;
            in_tls_domains = false;
            in_sans = false;
            continue;
        } else if line.starts_with("  services:") || !line.starts_with(' ') {
            if let Some(r) = current.take() {
                routers.push(r);
            }
            in_routers = false;
            in_tls_domains = false;
            in_sans = false;
            continue;
        }
        if !in_routers {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent == 4 && trimmed.ends_with(':') {
            if let Some(r) = current.take() {
                routers.push(r);
            }
            current = Some(TraefikRouterInfo {
                name: trimmed.trim_end_matches(':').to_string(),
                line_num: idx + 1,
                ..Default::default()
            });
            in_tls_domains = false;
            in_sans = false;
            continue;
        }
        let Some(ref mut r) = current else {
            continue;
        };
        if let Some(rule_val) = trimmed.strip_prefix("rule:") {
            r.hosts.extend(extract_traefik_rule_hosts(rule_val));
        } else if let Some(svc_val) = trimmed.strip_prefix("service:") {
            r.service = Some(
                svc_val
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
        } else if trimmed == "domains:" {
            in_tls_domains = true;
            in_sans = false;
        } else if in_tls_domains {
            if let Some(main_val) = trimmed.strip_prefix("- main:") {
                in_sans = false;
                let dom = main_val.trim().trim_matches('"').trim_matches('\'');
                if !dom.is_empty() {
                    r.hosts.push(dom.to_string());
                }
            } else if trimmed == "sans:" {
                in_sans = true;
            } else if in_sans && trimmed.starts_with("- ") {
                let dom = trimmed
                    .trim_start_matches("- ")
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                if !dom.is_empty() {
                    r.hosts.push(dom.to_string());
                }
            } else if indent <= 6 {
                in_tls_domains = false;
                in_sans = false;
            }
        }
    }
    if let Some(r) = current.take() {
        routers.push(r);
    }
    routers
}
pub fn check_traefik_and_worker(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if fname == "dynamic-k8s.yml" {
        let service_entities = parse_traefik_service_entities(content);
        for r in parse_traefik_routers(content) {
            let mut router_entities: HashSet<LegalEntity> = HashSet::new();
            for h in &r.hosts {
                if let Some(ent) = host_legal_entity(h) {
                    router_entities.insert(ent);
                }
            }
            if router_entities.len() > 1 {
                findings.push(
                    Finding::new(
                        "BND-ENT-004",
                        Severity::Critical,
                        format!(
                            "Traefik router `{}` mixes hosts from multiple legal entities ({:?})",
                            r.name, router_entities
                        ),
                        path.to_path_buf(),
                        r.line_num,
                    )
                    .with_confidence(Confidence::new(0.98)),
                );
            } else if let (Some(&host_ent), Some(ref svc_name)) =
                (router_entities.iter().next(), &r.service)
            {
                if let Some(&svc_ent) = service_entities.get(svc_name) {
                    if svc_ent != host_ent {
                        findings.push(
                            Finding::new(
                                "BND-ENT-004",
                                Severity::Critical,
                                format!(
                                    "Traefik router `{}` routes {:?} host(s) to `{svc_name}` in {:?} namespace",
                                    r.name, host_ent, svc_ent
                                ),
                                path.to_path_buf(),
                                r.line_num,
                            )
                            .with_confidence(Confidence::new(0.98)),
                        );
                    }
                }
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

/// Audits Prometheus config (`09-prometheus.yaml`) for legal_entity chargeback labeling (`BND-ENT-005`).
pub fn check_prometheus_entity_attribution(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    if content.contains("entity:memory_rss_bytes:sum") {
        if !content.contains("target_label: legal_entity") {
            findings.push(
                Finding::new(
                    "BND-ENT-005",
                    Severity::High,
                    "Prometheus k3s-cadvisor scrape lacks `target_label: legal_entity` mapping for namespace chargeback",
                    path.to_path_buf(),
                    1,
                )
                .with_confidence(Confidence::new(0.95)),
            );
        }
        if content.contains("sum by (namespace) (") {
            findings.push(
                Finding::new(
                    "BND-ENT-005",
                    Severity::High,
                    "Prometheus `entity:*` recording rule aggregates `by (namespace)` without `legal_entity` label",
                    path.to_path_buf(),
                    1,
                )
                .with_confidence(Confidence::new(0.95)),
            );
        }
    }
    findings
}

/// Audits multi-entity DR backup, RPO/WAL streaming, and restore verification scripts (`BND-ENT-006`).
pub fn check_multi_entity_dr_parity(infra_dir: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let scripts_dir = infra_dir.join("scripts");
    let unified_path = scripts_dir.join("unified_dr_backup.sh");
    if let Ok(unified) = fs::read_to_string(&unified_path) {
        if unified.contains("freshcredit_xyz_vote_data") {
            for rpo_script in ["rpo_hourly_db_sync.sh", "wal_stream_push.sh"] {
                let p = scripts_dir.join(rpo_script);
                if let Ok(txt) = fs::read_to_string(&p) {
                    if !txt.contains("freshcredit_xyz_vote_data") {
                        findings.push(
                            Finding::new(
                                "BND-ENT-006",
                                Severity::High,
                                format!("{rpo_script} omits `freshcredit_xyz_vote_data` (FreshCredit.xyz LLC RPO gap)"),
                                p,
                                1,
                            )
                            .with_confidence(Confidence::new(0.95)),
                        );
                    }
                }
            }
        }
    }
    let verify_path = scripts_dir.join("verify_dr_restore.sh");
    if let Ok(verify_txt) = fs::read_to_string(&verify_path) {
        for db in ["documenso_org", "documenso_tism", "documenso_xyz"] {
            if !verify_txt.contains(db) {
                findings.push(
                    Finding::new(
                        "BND-ENT-006",
                        Severity::High,
                        format!("verify_dr_restore.sh omits PostgreSQL entity database `{db}`"),
                        verify_path.clone(),
                        1,
                    )
                    .with_confidence(Confidence::new(0.95)),
                );
            }
        }
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
            if p.file_name().and_then(|n| n.to_str()) == Some("09-prometheus.yaml") {
                findings.extend(check_prometheus_entity_attribution(p, &content));
            }
        } else if p.file_name().and_then(|n| n.to_str()) == Some("dynamic-k8s.yml")
            || p.file_name().and_then(|n| n.to_str()) == Some("failover-worker.js")
        {
            findings.extend(check_traefik_and_worker(p, &content));
        }
    }

    findings.extend(check_multi_entity_dr_parity(&infra_dir));
    findings.extend(check_entity_sa_and_fence_isolation(root));
    findings
}

/// BND-ENT-007: Verifies entity namespaces disable default SA token automount and
/// split-brain write fencing scales stateful workloads across all entity namespaces.
pub fn check_entity_sa_and_fence_isolation(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let ns_yaml = root.join("infra/k8s/00-namespace.yaml");
    if let Ok(content) = fs::read_to_string(&ns_yaml) {
        findings.extend(check_namespace_sa_automount_content(&ns_yaml, &content));
    }
    let psi_rs = root.join("crates/shigaki-srv/src/psi.rs");
    if let Ok(content) = fs::read_to_string(&psi_rs) {
        findings.extend(check_multi_entity_fence_coverage(&psi_rs, &content));
    }
    findings
}

pub fn check_namespace_sa_automount_content(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let entity_namespaces = [
        "freshcredit",
        "zaplit",
        "freshcredit-org",
        "freshcredit-xyz",
        "personal-tism",
    ];
    let docs: Vec<&str> = content.split("\n---").collect();
    for ns in entity_namespaces {
        let has_disabled_default = docs.iter().any(|doc| {
            doc.contains("kind: ServiceAccount")
                && doc.contains("name: default")
                && doc.contains(&format!("namespace: {ns}"))
                && doc.contains("automountServiceAccountToken: false")
        });
        if !has_disabled_default {
            findings.push(
                Finding::new(
                    "BND-ENT-007",
                    Severity::High,
                    format!(
                        "Namespace `{ns}` default ServiceAccount missing `automountServiceAccountToken: false`"
                    ),
                    path.to_path_buf(),
                    1,
                )
                .with_confidence(Confidence::new(0.95)),
            );
        }
    }
    findings
}

pub fn check_multi_entity_fence_coverage(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    if !content.contains("fn k8s_fence_deployments") {
        return findings;
    }
    let required = [
        ("freshcredit-xyz", "freshcredit-vote"),
        ("freshcredit-org", "freshcredit-sign-org"),
        ("freshcredit-xyz", "freshcredit-sign-xyz"),
        ("personal-tism", "tism-sign"),
    ];
    for (ns, dep) in required {
        if !content.contains(ns) || !content.contains(dep) {
            findings.push(
                Finding::new(
                    "BND-ENT-007",
                    Severity::Critical,
                    format!(
                        "Split-brain write fence omits stateful entity deployment `{ns}/{dep}`"
                    ),
                    path.to_path_buf(),
                    1,
                )
                .with_confidence(Confidence::new(0.96)),
            );
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

    #[test]
    fn detects_prometheus_missing_legal_entity_attribution() {
        let bad = "- record: entity:memory_rss_bytes:sum\n  expr: sum by (namespace) (container_memory_rss{})\n";
        let f = check_prometheus_entity_attribution(Path::new("09-prometheus.yaml"), bad);
        assert_eq!(f.len(), 2);
        assert!(f.iter().all(|x| x.rule_code == "BND-ENT-005"));

        let good = "target_label: legal_entity\n- record: entity:memory_rss_bytes:sum\n  expr: sum by (namespace, legal_entity) (container_memory_rss{})\n";
        assert!(
            check_prometheus_entity_attribution(Path::new("09-prometheus.yaml"), good).is_empty()
        );
    }
}
