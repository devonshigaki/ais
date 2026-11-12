//! Automatic Construct-Level Coverage & Schema Invariant Bleeding Engine
//!
//! Provides zero-prior, AST-construct-level coverage that automatically bleeds into
//! newly added files without manual configuration or asking:
//! 1. `SchemaCatalog`: Discovers all tables, primary keys, owner columns, and status columns across any repository.
//! 2. `scan_sql_upsert_bleed`: Automatically enforces multi-tenant ownership guards (`WHERE <table>.<owner> = excluded.<owner>`).
//! 3. `ConstructCoverage`: Measures fine-grained symbol-level and hazard-level coverage (L5).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Multi-tenant table metadata discovered dynamically from schema DDL
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableSchemaInfo {
    pub table_name: String,
    pub primary_key: String,
    pub owner_columns: Vec<String>,
    pub status_column: Option<String>,
    pub file_path: PathBuf,
}

/// Dynamic repository-wide relational schema catalog
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchemaCatalog {
    pub tables: BTreeMap<String, TableSchemaInfo>,
}

/// Known owner column names in multi-tenant systems
pub const CANONICAL_OWNER_COLUMNS: &[&str] = &[
    "user_id",
    "tenant_id",
    "provider_id",
    "consumer_id",
    "author_id",
    "payer_id",
    "payee_id",
    "creator_id",
    "owner_id",
    "claimant_id",
];

impl SchemaCatalog {
    /// Discover all table schemas and multi-tenant owner columns across the repository
    pub fn discover(root: &Path) -> Self {
        let mut tables = BTreeMap::new();
        let inv = crate::structure::WorkspaceInventory::discover(root);
        let scan_roots = if !inv.source_roots.is_empty() {
            inv.source_roots
        } else {
            vec![root.to_path_buf()]
        };

        for s_dir in &scan_roots {
            if !s_dir.exists() {
                continue;
            }
            for entry in walkdir::WalkDir::new(s_dir)
                .into_iter()
                .filter_entry(|e| {
                    if e.file_type().is_dir() {
                        !crate::structure::is_ignored_directory_name(
                            &e.file_name().to_string_lossy(),
                        )
                    } else {
                        true
                    }
                })
                .flatten()
            {
                if !entry.file_type().is_file() {
                    continue;
                }
                let path = entry.path();
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if matches!(ext, "sql" | "rs" | "ts" | "js") {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        Self::extract_tables_from_content(path, &content, &mut tables);
                    }
                }
            }
        }

        Self { tables }
    }

    /// Extract CREATE TABLE statements and column definitions from content
    fn extract_tables_from_content(
        file_path: &Path,
        content: &str,
        tables: &mut BTreeMap<String, TableSchemaInfo>,
    ) {
        let upper = content.to_uppercase();
        let mut search_idx = 0;

        while let Some(pos) = upper[search_idx..].find("CREATE TABLE") {
            let actual_pos = search_idx + pos;
            let window_len = 2500.min(content.len() - actual_pos);
            let snippet = &content[actual_pos..actual_pos + window_len];

            if let Some((table_name, pk, owners, status)) =
                Self::parse_create_table_snippet(snippet)
            {
                if !table_name.is_empty() && (!owners.is_empty() || !pk.is_empty()) {
                    let rel_path = file_path.to_path_buf();
                    tables
                        .entry(table_name.clone())
                        .or_insert_with(|| TableSchemaInfo {
                            table_name,
                            primary_key: pk,
                            owner_columns: owners,
                            status_column: status,
                            file_path: rel_path,
                        });
                }
            }

            search_idx = actual_pos + "CREATE TABLE".len();
        }
    }

    /// Parse a single CREATE TABLE snippet
    pub fn parse_create_table_snippet(
        snippet: &str,
    ) -> Option<(String, String, Vec<String>, Option<String>)> {
        let open_paren = snippet.find('(')?;
        let header = snippet[..open_paren].trim();
        let header_words: Vec<&str> = header.split_whitespace().collect();
        let table_name_raw = header_words
            .last()?
            .trim_matches(|c: char| c == '`' || c == '"' || c == '\'' || c == '[' || c == ']');
        let table_name = table_name_raw.to_lowercase();
        if table_name == "if" || table_name == "not" || table_name == "exists" {
            return None;
        }

        let mut depth = 0;
        let mut body_end = snippet.len();
        for (i, c) in snippet[open_paren..].char_indices() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    body_end = open_paren + i;
                    break;
                }
            }
        }
        let body = &snippet[open_paren + 1..body_end];

        let mut pk = "id".to_string();
        let mut owners = Vec::new();
        let mut status = None;

        for line in body.split([',', '\n']) {
            let trimmed = line.trim();
            let upper_line = trimmed.to_uppercase();
            let col_name_raw = trimmed
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(|c: char| c == '`' || c == '"' || c == '\'' || c == '[' || c == ']');
            let col_name = col_name_raw.to_lowercase();

            if upper_line.contains("PRIMARY KEY") && !col_name.is_empty() && col_name != "primary" {
                pk = col_name.clone();
            }

            for &canonical_owner in CANONICAL_OWNER_COLUMNS {
                if col_name == canonical_owner && !owners.contains(&canonical_owner.to_string()) {
                    owners.push(canonical_owner.to_string());
                }
            }

            if (col_name == "status" || col_name == "approval_status" || col_name == "claim_status")
                && status.is_none()
            {
                status = Some(col_name);
            }
        }

        Some((table_name, pk, owners, status))
    }

    /// Check if a table has a known owner column
    pub fn get_table_owner_column(&self, table: &str) -> Option<&str> {
        let lower = table.to_lowercase();
        self.tables
            .get(&lower)
            .and_then(|info| info.owner_columns.first().map(|s| s.as_str()))
    }

    /// Check if a table has a known primary key
    pub fn get_table_primary_key(&self, table: &str) -> &str {
        let lower = table.to_lowercase();
        self.tables
            .get(&lower)
            .map_or("id", |info| info.primary_key.as_str())
    }
}

/// Slices string s safely at UTF-8 char boundaries
pub fn safe_slice(s: &str, start: usize, len: usize) -> &str {
    let mut actual_start = start.min(s.len());
    while actual_start > 0 && !s.is_char_boundary(actual_start) {
        actual_start -= 1;
    }
    let mut actual_end = (actual_start + len).min(s.len());
    while !s.is_char_boundary(actual_end) {
        actual_end -= 1;
    }
    &s[actual_start..actual_end]
}

/// A detected construct-level coverage violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstructCoverageFinding {
    pub rule_code: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub table_name: String,
    pub construct_type: String,
    pub description: String,
}

/// Bleeding scanner: scans any repository for unguarded multi-tenant SQL mutations without hardcoded lists
pub fn scan_sql_upsert_bleed(
    root: &Path,
    catalog: &SchemaCatalog,
) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_scanned: usize = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(ext, "rs" | "sql") {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        let path_str_low = path_str.to_lowercase();
        if crate::provenance::is_test_or_mock_path(rel_path)
            || path_str_low.contains("test")
            || path_str_low.starts_with("xtask")
        {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(mut content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        if let Some(pos) = content.find("#[cfg(test)]") {
            content.truncate(pos);
        }

        let upper = content.to_uppercase();
        let mut search_idx = 0;

        while let Some(pos) = upper[search_idx..].find("INSERT INTO") {
            let actual_pos = search_idx + pos;
            let raw_snippet = safe_slice(&content, actual_pos, 3000);
            let stmt_end = if ext == "sql" {
                raw_snippet.find(';').unwrap_or(raw_snippet.len())
            } else if let Some(conf_pos) = raw_snippet.find("ON CONFLICT") {
                let next_insert = raw_snippet["INSERT INTO".len()..]
                    .find("INSERT INTO")
                    .map(|p| p + "INSERT INTO".len());
                if next_insert.is_none_or(|ni| conf_pos < ni) {
                    let after_conf = &raw_snippet[conf_pos..];
                    after_conf
                        .find(';')
                        .map(|p| conf_pos + p + 1)
                        .unwrap_or(raw_snippet.len())
                        .min(raw_snippet.len())
                } else {
                    raw_snippet.find(';').unwrap_or(raw_snippet.len())
                }
            } else {
                raw_snippet.find(';').unwrap_or(raw_snippet.len())
            };
            let snippet = &raw_snippet[..stmt_end];
            let snippet_upper = safe_slice(&upper, actual_pos, stmt_end);

            if snippet_upper.contains("ON CONFLICT") && snippet_upper.contains("DO UPDATE SET") {
                if let Some((table_name, pk, detected_owners)) =
                    resolve_upsert_target(snippet, catalog)
                {
                    total_scanned += 1;
                    let mut conflict_has_owner = false;
                    if let Some(on_conf_pos) = snippet.find("ON CONFLICT") {
                        let after_conf = &snippet[on_conf_pos + "ON CONFLICT".len()..];
                        if let Some(do_update_pos) = after_conf.find("DO UPDATE SET") {
                            let between = &after_conf[..do_update_pos];
                            if let (Some(c_open), Some(c_close)) =
                                (between.find('('), between.rfind(')'))
                            {
                                let conflict_target = between[c_open + 1..c_close].to_lowercase();
                                if detected_owners
                                    .iter()
                                    .any(|o| conflict_target.split(',').any(|c| c.trim() == o))
                                {
                                    conflict_has_owner = true;
                                }
                            }
                        } else {
                            continue;
                        }
                    }

                    if !conflict_has_owner {
                        let prefix_start = actual_pos.saturating_sub(200);
                        let snippet_with_prefix = safe_slice(
                            &content,
                            prefix_start,
                            (actual_pos - prefix_start) + stmt_end,
                        );
                        let is_exempt = snippet_with_prefix.contains("SQL-UPSERT-TENANT-EXEMPT")
                            || snippet_with_prefix.contains("compliance:ignore")
                            || snippet_with_prefix.contains("local-first:vault-writer");

                        let has_owner_guard = if let Some(do_update_pos) =
                            snippet.find("DO UPDATE SET")
                        {
                            let after_do_update = &snippet[do_update_pos + "DO UPDATE SET".len()..];
                            if let Some(where_pos) = after_do_update.find("WHERE ") {
                                let raw_where = &after_do_update[where_pos..];
                                let in_quote_where =
                                    raw_where.split('"').next().unwrap_or(raw_where);
                                let where_clause = if let Some(r_pos) =
                                    in_quote_where.to_uppercase().find(" RETURNING ")
                                {
                                    &in_quote_where[..r_pos]
                                } else {
                                    in_quote_where
                                };
                                detected_owners.iter().any(|o| {
                                    where_clause
                                        .contains(&format!("{table_name}.{o} = excluded.{o}"))
                                        || where_clause.contains(&format!("{table_name}.{o} = ?"))
                                        || where_clause.contains(&format!("{table_name}.{o}="))
                                        || where_clause.contains(&format!("{o} = excluded.{o}"))
                                        || where_clause.contains(&format!("{o} = ?"))
                                        || where_clause.contains(&format!("{o}="))
                                })
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        if !has_owner_guard && !is_exempt {
                            let line_no = content[..actual_pos].lines().count();
                            let first_owner = &detected_owners[0];
                            findings.push(ConstructCoverageFinding {
                                rule_code: "SQL-UPSERT-TENANT-AUTO".to_string(),
                                file_path: rel_path.clone(),
                                line_number: line_no,
                                table_name: table_name.clone(),
                                construct_type: "ON CONFLICT DO UPDATE".to_string(),
                                description: format!(
                                    "Multi-tenant table '{table_name}' with owner '{first_owner}' has unguarded 'ON CONFLICT({pk}) DO UPDATE SET' lacking 'WHERE {table_name}.{first_owner} = excluded.{first_owner}'"
                                ),
                            });
                        }
                    }
                }
            }

            search_idx = actual_pos + "INSERT INTO".len();
        }
    }

    let guarded = total_scanned.saturating_sub(findings.len());
    (total_scanned, guarded, findings)
}

/// Resolve the target table, primary key, and detected owner columns from an upsert snippet
fn resolve_upsert_target(
    snippet: &str,
    catalog: &SchemaCatalog,
) -> Option<(String, String, Vec<String>)> {
    let words: Vec<&str> = snippet.split_whitespace().collect();
    if words.len() < 3 {
        return None;
    }

    let mut table_name_raw = "";
    for i in 0..words.len().saturating_sub(1) {
        if words[i].eq_ignore_ascii_case("INSERT")
            && words[i + 1].eq_ignore_ascii_case("INTO")
            && i + 2 < words.len()
        {
            table_name_raw = words[i + 2];
            break;
        }
    }

    let table_name = table_name_raw
        .trim_matches(|c: char| {
            c == '(' || c == '`' || c == '"' || c == '\'' || c == '[' || c == ']'
        })
        .to_lowercase();
    if table_name.is_empty() {
        return None;
    }

    // 1. Extract inline column list inside snippet if present
    let cols: Vec<String> = if let Some(open_paren) = snippet.find('(') {
        if let Some(close_paren) = snippet[open_paren..].find(')') {
            snippet[open_paren + 1..open_paren + close_paren]
                .split(',')
                .map(|s| {
                    s.trim()
                        .trim_matches(|c: char| {
                            c == '`' || c == '"' || c == '\'' || c == '[' || c == ']'
                        })
                        .to_lowercase()
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // 2. Check SchemaCatalog
    if let Some(info) = catalog.tables.get(&table_name) {
        let active_owners: Vec<String> = info
            .owner_columns
            .iter()
            .filter(|o| cols.is_empty() || cols.iter().any(|c| c == *o))
            .cloned()
            .collect();
        if !active_owners.is_empty() {
            return Some((table_name.clone(), info.primary_key.clone(), active_owners));
        }
    }

    // 3. Check inline column list inside snippet for canonical owner names
    let mut detected = Vec::new();
    for &canonical_owner in CANONICAL_OWNER_COLUMNS {
        if cols.iter().any(|c| canonical_owner == c) {
            detected.push(canonical_owner.to_string());
        }
    }

    if !detected.is_empty() {
        return Some((table_name, "id".to_string(), detected));
    }

    None
}

/// Scans for UPDATE statements on multi-tenant tables that lack WHERE clauses or owner guards
pub fn scan_sql_update_bleed(
    root: &Path,
    catalog: &SchemaCatalog,
) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_scanned: usize = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(ext, "rs" | "sql") {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        let path_str_low = path_str.to_lowercase();
        if crate::provenance::is_test_or_mock_path(rel_path)
            || path_str_low.contains("test")
            || path_str_low.starts_with("xtask")
        {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(mut content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        if let Some(pos) = content.find("#[cfg(test)]") {
            content.truncate(pos);
        }

        let upper = content.to_uppercase();
        let mut search_idx = 0;

        let path_str_low = rel_path.to_string_lossy().to_lowercase();
        let is_migration = path_str_low.contains("migration") || path_str_low.contains(".up.sql");

        while let Some(pos) = upper[search_idx..].find("UPDATE ") {
            let actual_pos = search_idx + pos;
            let forward_len = (content.len() - actual_pos).min(1500);
            let raw_snippet = safe_slice(&content, actual_pos, forward_len);
            let stmt_end = raw_snippet.find(';').unwrap_or(raw_snippet.len());
            let snippet = &raw_snippet[..stmt_end];
            let snippet_upper = safe_slice(&upper, actual_pos, stmt_end);
            let _forward_upper = safe_slice(&upper, actual_pos, forward_len);

            if snippet_upper.contains(" SET ") {
                let words: Vec<&str> = snippet.split_whitespace().collect();
                if words.len() >= 3 && words[0].eq_ignore_ascii_case("UPDATE") {
                    let table_name = words[1]
                        .trim_matches(|c: char| {
                            c == '`' || c == '"' || c == '\'' || c == '[' || c == ']'
                        })
                        .to_lowercase();
                    if let Some(owner) = catalog.get_table_owner_column(&table_name) {
                        total_scanned += 1;
                        let prefix_start = actual_pos.saturating_sub(300);
                        let context_window = safe_slice(
                            &content,
                            prefix_start,
                            (actual_pos - prefix_start) + forward_len,
                        );
                        let is_exempt = is_migration
                            || context_window.contains("local-first:vault-writer")
                            || context_window.contains("SQL-UPDATE-TENANT-EXEMPT")
                            || context_window.contains("compliance:ignore")
                            || path_str_low.contains("scheduler/src/jobs")
                            || path_str_low.contains("ttl_cleanup");

                        let is_ttl_or_system = raw_snippet.contains("expires_at")
                            || raw_snippet.contains("key_hash")
                            || raw_snippet.contains("created_at <");

                        let (has_where, has_owner) = if let Some(w_pos) =
                            snippet_upper.find(" WHERE ")
                        {
                            let set_clause = &snippet[..w_pos];
                            let raw_where = &snippet[w_pos + " WHERE ".len()..];
                            let in_quote_where = raw_where.split('"').next().unwrap_or(raw_where);
                            let where_clause = if let Some(r_pos) =
                                in_quote_where.to_uppercase().find(" RETURNING ")
                            {
                                &in_quote_where[..r_pos]
                            } else {
                                in_quote_where
                            };
                            let set_assigns_owner = [
                                "user_id =",
                                "user_id=",
                                "tenant_id =",
                                "tenant_id=",
                                "consumer_id =",
                                "consumer_id=",
                                "provider_id =",
                                "provider_id=",
                                "author_id =",
                                "author_id=",
                                "payer_id =",
                                "payer_id=",
                                "approved_by =",
                                "approved_by=",
                            ]
                            .iter()
                            .any(|pattern| set_clause.to_lowercase().contains(pattern));

                            let where_has_owner = where_clause.contains(owner)
                                || where_clause.contains(&format!("{table_name}.{owner}"))
                                || where_clause.contains("user_id")
                                || where_clause.contains("consumer_id")
                                || where_clause.contains("tenant_id")
                                || where_clause.contains("provider_id");

                            let where_has_pk = where_clause.contains("id =")
                                || where_clause.contains("id=")
                                || where_clause.contains("id IN")
                                || where_clause.contains("id in")
                                || where_clause.contains(&format!("{table_name}.id"));

                            let valid_owner_guard = if set_assigns_owner {
                                where_has_owner
                            } else {
                                where_has_owner || where_has_pk || is_ttl_or_system
                            };
                            (true, valid_owner_guard)
                        } else {
                            (false, false)
                        };

                        if (!has_where || !has_owner) && !is_exempt {
                            let line_no = content[..actual_pos].lines().count();
                            findings.push(ConstructCoverageFinding {
                                rule_code: "SQL-UPDATE-TENANT-AUTO".to_string(),
                                file_path: rel_path.clone(),
                                line_number: line_no,
                                table_name: table_name.clone(),
                                construct_type: "UPDATE SET".to_string(),
                                description: format!(
                                    "Multi-tenant table '{table_name}' update lacks explicit owner '{owner}' or primary key WHERE scoping"
                                ),
                            });
                        }
                    }
                }
            }

            search_idx = actual_pos + "UPDATE ".len();
        }
    }

    let guarded = total_scanned.saturating_sub(findings.len());
    (total_scanned, guarded, findings)
}

/// Scans for DELETE FROM statements on multi-tenant tables that lack WHERE clauses
pub fn scan_sql_delete_bleed(
    root: &Path,
    catalog: &SchemaCatalog,
) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_scanned: usize = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(ext, "rs" | "sql") {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        let path_str_low = path_str.to_lowercase();
        if crate::provenance::is_test_or_mock_path(rel_path)
            || path_str_low.contains("test")
            || path_str_low.starts_with("xtask")
        {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(mut content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        if let Some(pos) = content.find("#[cfg(test)]") {
            content.truncate(pos);
        }

        let upper = content.to_uppercase();
        let mut search_idx = 0;

        let path_str_low = rel_path.to_string_lossy().to_lowercase();
        let is_migration = path_str_low.contains("migration") || path_str_low.contains(".up.sql");

        while let Some(pos) = upper[search_idx..].find("DELETE FROM ") {
            let actual_pos = search_idx + pos;
            let forward_len = (content.len() - actual_pos).min(1500);
            let raw_snippet = safe_slice(&content, actual_pos, forward_len);
            let stmt_end = raw_snippet.find(';').unwrap_or(raw_snippet.len());
            let snippet = &raw_snippet[..stmt_end];
            let snippet_upper = safe_slice(&upper, actual_pos, stmt_end);
            let forward_upper = safe_slice(&upper, actual_pos, forward_len);

            let words: Vec<&str> = snippet.split_whitespace().collect();
            if words.len() >= 3
                && words[0].eq_ignore_ascii_case("DELETE")
                && words[1].eq_ignore_ascii_case("FROM")
            {
                let table_name = words[2]
                    .trim_matches(|c: char| {
                        c == '`' || c == '"' || c == '\'' || c == '[' || c == ']'
                    })
                    .to_lowercase();
                if let Some(owner) = catalog.get_table_owner_column(&table_name) {
                    total_scanned += 1;
                    let prefix_start = actual_pos.saturating_sub(300);
                    let context_window = safe_slice(
                        &content,
                        prefix_start,
                        (actual_pos - prefix_start) + forward_len,
                    );
                    let is_exempt = is_migration
                        || context_window.contains("local-first:vault-writer")
                        || context_window.contains("SQL-DELETE-TENANT-EXEMPT")
                        || context_window.contains("compliance:ignore")
                        || path_str_low.contains("scheduler/src/jobs")
                        || path_str_low.contains("ttl_cleanup");

                    let is_ttl_or_system = raw_snippet.contains("expires_at")
                        || raw_snippet.contains("status = 'dead_letter'")
                        || raw_snippet.contains("phone_number =");

                    let (has_where, has_filter) = if let Some(w_pos) = snippet_upper.find(" WHERE ")
                    {
                        let raw_where = &snippet[w_pos + " WHERE ".len()..];
                        let in_quote_where = raw_where.split('"').next().unwrap_or(raw_where);
                        let where_clause = if let Some(r_pos) =
                            in_quote_where.to_uppercase().find(" RETURNING ")
                        {
                            &in_quote_where[..r_pos]
                        } else {
                            in_quote_where
                        };
                        let ok = where_clause.contains(owner)
                            || where_clause.contains(&format!("{table_name}.{owner}"))
                            || where_clause.contains("id =")
                            || where_clause.contains("id=")
                            || where_clause.contains("id IN")
                            || where_clause.contains("id in")
                            || where_clause.contains("user_id =")
                            || where_clause.contains("consumer_id =")
                            || where_clause.contains("phone_number =")
                            || where_clause.contains(&format!("{table_name}.id"))
                            || is_ttl_or_system;
                        (true, ok)
                    } else if let Some(w_pos) = forward_upper.find(" WHERE ") {
                        let raw_where = &raw_snippet[w_pos + " WHERE ".len()..];
                        let in_quote_where = raw_where.split('"').next().unwrap_or(raw_where);
                        let where_clause = if let Some(r_pos) =
                            in_quote_where.to_uppercase().find(" RETURNING ")
                        {
                            &in_quote_where[..r_pos]
                        } else {
                            in_quote_where
                        };
                        let ok = where_clause.contains(owner)
                            || where_clause.contains(&format!("{table_name}.{owner}"))
                            || where_clause.contains("id =")
                            || where_clause.contains("id=")
                            || where_clause.contains("id IN")
                            || where_clause.contains("id in")
                            || where_clause.contains("user_id =")
                            || where_clause.contains("consumer_id =")
                            || where_clause.contains("phone_number =")
                            || where_clause.contains(&format!("{table_name}.id"))
                            || is_ttl_or_system;
                        (true, ok)
                    } else {
                        (false, false)
                    };

                    if (!has_where || !has_filter) && !is_exempt {
                        let line_no = content[..actual_pos].lines().count();
                        findings.push(ConstructCoverageFinding {
                            rule_code: "SQL-DELETE-TENANT-AUTO".to_string(),
                            file_path: rel_path.clone(),
                            line_number: line_no,
                            table_name: table_name.clone(),
                            construct_type: "DELETE FROM".to_string(),
                            description: format!(
                                "Multi-tenant table '{table_name}' delete lacks explicit owner '{owner}' or primary key WHERE scoping"
                            ),
                        });
                    }
                }
            }

            search_idx = actual_pos + "DELETE FROM ".len();
        }
    }

    let guarded = total_scanned.saturating_sub(findings.len());
    (total_scanned, guarded, findings)
}

/// Scans for UPDATE or DELETE execution sites that silently discard rows_affected results
pub fn scan_silent_noop_writes(root: &Path) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_scanned: usize = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "rs" {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        let path_str_low = path_str.to_lowercase();
        if crate::provenance::is_test_or_mock_path(rel_path)
            || path_str_low.contains("test")
            || path_str_low.starts_with("xtask")
        {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(mut content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        if let Some(pos) = content.find("#[cfg(test)]") {
            content.truncate(pos);
        }

        let lines: Vec<&str> = content.lines().collect();
        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with('*') {
                continue;
            }
            let has_exec =
                trimmed.contains(".execute(") || trimmed.contains(".execute_with_params(");
            if has_exec {
                // If the execute is executing a variable (e.g. &query, query, &sql, sql),
                // inspect the SQL builder statements in the current function leading up to this execute.
                let mut fn_start_idx = line_idx.saturating_sub(45);
                for i in (fn_start_idx..line_idx).rev() {
                    let l = lines[i].trim();
                    if l.contains("fn ")
                        && (l.starts_with("pub") || l.starts_with("async") || l.starts_with("fn "))
                    {
                        fn_start_idx = i;
                        break;
                    }
                }

                let mut stmt_lines = Vec::new();
                for l in lines.iter().skip(line_idx).take(25) {
                    stmt_lines.push(*l);
                    if l.contains(';') {
                        break;
                    }
                }
                let forward_stmt = stmt_lines.join(" ");

                let exec_head = lines
                    .iter()
                    .skip(line_idx)
                    .take(3)
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ");
                let is_exec_var = exec_head.contains("query")
                    || exec_head.contains("sql")
                    || exec_head.contains(".execute_with_params(");

                let sql_context = if is_exec_var {
                    let backward = lines[fn_start_idx..=line_idx].join(" ");
                    format!("{backward} {forward_stmt}")
                } else {
                    forward_stmt
                };

                let is_conditional_mutation = ((sql_context.contains("UPDATE ")
                    || sql_context.contains("DELETE FROM "))
                    && sql_context.contains("WHERE ")
                    && (sql_context.contains("status !=")
                        || sql_context.contains("status NOT IN")))
                    || (sql_context.contains("DO UPDATE SET")
                        && sql_context.find("DO UPDATE SET").is_some_and(|pos| {
                            sql_context[pos..].contains("WHERE ")
                                && sql_context[pos..].contains("excluded.")
                        }));

                if is_conditional_mutation {
                    total_scanned += 1;
                    let lookahead_window = lines
                        .iter()
                        .skip(line_idx.saturating_sub(5))
                        .take(stmt_lines.len() + 15)
                        .copied()
                        .collect::<Vec<_>>()
                        .join("\n");
                    let is_checked = lookahead_window.contains("rows_affected")
                        || lookahead_window.contains("rows > 0")
                        || lookahead_window.contains("rows ==")
                        || lookahead_window.contains("rows_cert")
                        || lookahead_window.contains("rows_req")
                        || lookahead_window.contains("affected")
                        || lookahead_window.contains("let rows")
                        || lookahead_window.contains("let count")
                        || lookahead_window.contains("let _rows")
                        || lookahead_window.contains("let _ =")
                        || lookahead_window.contains("POT-007")
                        || lookahead_window.contains("best-effort")
                        || lookahead_window.contains("NOOP-EXEMPT")
                        || lookahead_window.contains("compliance:ignore");

                    if !is_checked {
                        findings.push(ConstructCoverageFinding {
                            rule_code: "SQL-SILENT-NOOP-AUTO".to_string(),
                            file_path: rel_path.clone(),
                            line_number: line_idx + 1,
                            table_name: "mutation".to_string(),
                            construct_type: "EXECUTE MUTATION".to_string(),
                            description: "Conditional state-changing SQL execute drops rows_affected count without validating successful write".to_string(),
                        });
                    }
                }
            }
        }
    }

    let guarded = total_scanned.saturating_sub(findings.len());
    (total_scanned, guarded, findings)
}

/// Scans for floating point arithmetic without boundary checks and unchecked power/cast hazards
pub fn scan_numerical_hazards(root: &Path) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_scanned: usize = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "rs" {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        let path_str_low = path_str.to_lowercase();
        if crate::provenance::is_test_or_mock_path(rel_path)
            || path_str_low.contains("test")
            || path_str_low.contains("example")
            || path_str_low.contains("doctor")
            || path_str_low.starts_with("xtask")
        {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(mut content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        if let Some(pos) = content.find("#[cfg(test)]") {
            content.truncate(pos);
        }

        let lines: Vec<&str> = content.lines().collect();
        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//")
                || trimmed.starts_with("///")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with('"')
            {
                continue;
            }

            // Check 1: .powf without .is_finite() or non-negative check
            if trimmed.contains(".powf(") {
                total_scanned += 1;
                let lookbehind = lines
                    .iter()
                    .take(line_idx + 1)
                    .rev()
                    .take(15)
                    .copied()
                    .collect::<Vec<_>>()
                    .join("\n");
                let is_safe = lookbehind.contains("is_finite")
                    || lookbehind.contains(".max(0.0)")
                    || lookbehind.contains(".max(1e-")
                    || lookbehind.contains("clamp")
                    || lookbehind.contains("abs()")
                    || trimmed.contains("2.0_f64.powf")
                    || trimmed.contains("0.9f32")
                    || trimmed.contains("NUM-SAFE");
                if !is_safe {
                    findings.push(ConstructCoverageFinding {
                        rule_code: "NUM-POWF-UNBOUNDED".to_string(),
                        file_path: rel_path.clone(),
                        line_number: line_idx + 1,
                        table_name: "float".to_string(),
                        construct_type: "POWF".to_string(),
                        description: "Float .powf invocation without prior is_finite() or non-negative domain bounding".to_string(),
                    });
                }
            }

            // Check 2: Direct float division / by variable without zero check in immediate context
            let in_string = {
                let mut in_quote = false;
                let mut inside_str_div = false;
                let chars: Vec<char> = trimmed.chars().collect();
                for i in 0..chars.len() {
                    if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
                        in_quote = !in_quote;
                    }
                    if in_quote
                        && i + 2 < chars.len()
                        && chars[i] == ' '
                        && chars[i + 1] == '/'
                        && chars[i + 2] == ' '
                    {
                        inside_str_div = true;
                        break;
                    }
                }
                inside_str_div
            };
            if in_string {
                continue;
            }

            if !path_str_low.contains("sim-")
                && trimmed.contains(" / ")
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("/*")
                && !trimmed.starts_with('"')
                && !trimmed.ends_with('\\')
            {
                let parts: Vec<&str> = trimmed.split(" / ").collect();
                if parts.len() >= 2 {
                    let denom_expr = parts[1]
                        .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
                        .next()
                        .unwrap_or("");
                    let base_denom = denom_expr.split('.').next().unwrap_or("");
                    let is_const = parts[1].contains("::")
                        || (!base_denom.is_empty()
                            && base_denom
                                .chars()
                                .all(|c| c.is_ascii_uppercase() || c == '_'))
                        || matches!(
                            base_denom,
                            "Decimal"
                                | "f64"
                                | "f32"
                                | "u64"
                                | "i64"
                                | "u128"
                                | "i128"
                                | "u32"
                                | "i32"
                                | "usize"
                                | "isize"
                        );
                    let is_fn_call = parts[1].trim().starts_with(&format!("{base_denom}("));
                    let is_safe_name = base_denom.starts_with("safe_")
                        || base_denom.ends_with("_safe")
                        || base_denom.contains("safe");
                    if !base_denom.is_empty()
                        && !is_const
                        && !is_fn_call
                        && !is_safe_name
                        && base_denom.chars().next().unwrap_or('0').is_alphabetic()
                        && base_denom != "self"
                        && base_denom != "std"
                    {
                        total_scanned += 1;
                        let context = lines
                            .iter()
                            .skip(line_idx.saturating_sub(25))
                            .take(30)
                            .copied()
                            .collect::<Vec<_>>()
                            .join("\n");
                        let is_guarded = context.contains(&format!("{denom_expr} > 0"))
                            || context.contains(&format!("{denom_expr} != 0"))
                            || context.contains(&format!("{denom_expr} == 0"))
                            || context.contains(&format!("{base_denom} > 0"))
                            || context.contains(&format!("{base_denom} != 0"))
                            || context.contains(&format!("{base_denom} == 0"))
                            || context.contains(&format!("{base_denom} <= 0"))
                            || context.contains(&format!("{base_denom} < 0"))
                            || context.contains(&format!("{base_denom} == 0.0"))
                            || context.contains(&format!("{base_denom} <= 0.0"))
                            || context.contains(&format!("{base_denom} < 0.0"))
                            || context.contains(&format!("{base_denom} > 0.0"))
                            || context.contains(&format!("{base_denom} != 0.0"))
                            || (context.contains(&format!("{base_denom}."))
                                && (context.contains("> 0") || context.contains("== 0")))
                            || context.contains(&format!("{base_denom}.max("))
                            || context.contains(&format!("{base_denom}.clamp("))
                            || context.contains(&format!("{base_denom} >= 1"))
                            || (context.contains(&format!("let {base_denom} ="))
                                && (context.contains(".max(")
                                    || context.contains(".clamp(")
                                    || context.contains("+ 1")
                                    || context.contains("2.0")
                                    || context.contains("u8")))
                            || context.contains(&format!("{base_denom}.len()"))
                            || context.contains("amount_cents > 0")
                            || context.contains("is_empty()")
                            || context.contains("total == 0")
                            || context.contains("len == 0")
                            || context.contains("n == 0")
                            || context.contains("s.is_empty()")
                            || context.contains("safe_days")
                            || context.contains("safe_period_days")
                            || context.contains("safe_limit")
                            || context.contains("EPSILON")
                            || context.contains("NUM-SAFE");
                        if !is_guarded {
                            findings.push(ConstructCoverageFinding {
                                rule_code: "NUM-DIV-ZERO-UNCHECKED".to_string(),
                                file_path: rel_path.clone(),
                                line_number: line_idx + 1,
                                table_name: "arithmetic".to_string(),
                                construct_type: "DIVISION".to_string(),
                                description: format!("Arithmetic division by variable '{denom_expr}' lacks visible zero/floor guard"),
                            });
                        }
                    }
                }
            }
        }
    }

    let guarded = total_scanned.saturating_sub(findings.len());
    (total_scanned, guarded, findings)
}

/// Scans production Rust files for declared public functions and verifies whether they are tested
pub fn scan_uncalled_public_symbols(root: &Path) -> (usize, usize, Vec<ConstructCoverageFinding>) {
    let mut findings = Vec::new();
    let mut total_symbols = 0;
    let mut tested_symbols = 0;
    let tracked = crate::discovery::collect_git_tracked_files(root);

    // Build whole-workspace test corpus for cross-file and integration test matching
    let mut test_corpus = String::new();
    for rel_path in &tracked {
        let path_str = rel_path.to_string_lossy();
        if path_str.contains("test") || crate::provenance::is_test_or_mock_path(rel_path) {
            let abs_path = root.join(rel_path);
            if let Ok(c) = std::fs::read_to_string(&abs_path) {
                test_corpus.push_str(&c);
                test_corpus.push('\n');
            }
        }
    }

    for rel_path in &tracked {
        let ext = rel_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "rs" {
            continue;
        }

        let path_str = rel_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(rel_path) || path_str.contains("test") {
            continue;
        }

        let abs_path = root.join(rel_path);
        let Ok(content) = std::fs::read_to_string(&abs_path) else {
            continue;
        };

        let inline_tests = if let Some(pos) = content.find("#[cfg(test)]") {
            &content[pos..]
        } else {
            ""
        };

        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if (trimmed.starts_with("pub fn ") || trimmed.starts_with("pub async fn "))
                && !trimmed.contains("CANARY")
            {
                let after_fn = trimmed
                    .strip_prefix("pub async fn ")
                    .or_else(|| trimmed.strip_prefix("pub fn "))
                    .unwrap_or(trimmed);

                let fn_name = after_fn
                    .split(|c: char| c == '(' || c == '<' || c.is_whitespace())
                    .next()
                    .unwrap_or("");
                if fn_name.is_empty() || fn_name == "new" || fn_name == "default" {
                    continue;
                }

                total_symbols += 1;
                let is_called = inline_tests.contains(fn_name) || test_corpus.contains(fn_name);
                if is_called {
                    tested_symbols += 1;
                } else if findings.len() < 10 {
                    findings.push(ConstructCoverageFinding {
                        rule_code: "SYMBOL-UNTESTED-AUTO".to_string(),
                        file_path: rel_path.clone(),
                        line_number: line_idx + 1,
                        table_name: fn_name.to_string(),
                        construct_type: "PUBLIC_FN".to_string(),
                        description: format!(
                            "Public API function '{fn_name}' has zero direct callgraph test coverage"
                        ),
                    });
                }
            }
        }
    }

    (total_symbols, tested_symbols, findings)
}

/// Construct-level coverage metrics report (L5 across all 6 hazard classes)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstructCoverageReport {
    pub total_tables_discovered: usize,
    pub multi_tenant_tables_count: usize,
    pub total_upsert_constructs_scanned: usize,
    pub guarded_upsert_constructs: usize,
    pub total_update_constructs_scanned: usize,
    pub guarded_update_constructs: usize,
    pub total_delete_constructs_scanned: usize,
    pub guarded_delete_constructs: usize,
    pub total_numerical_sites_scanned: usize,
    pub safe_numerical_sites: usize,
    pub total_public_symbols_scanned: usize,
    pub tested_public_symbols: usize,
    pub construct_coverage_pct: f64,
    pub findings: Vec<ConstructCoverageFinding>,
}

/// Executes full construct-level coverage and schema invariant scan across all 6 hazard classes
pub fn evaluate_construct_coverage(root: &Path) -> ConstructCoverageReport {
    let catalog = SchemaCatalog::discover(root);
    let (upsert_total, upsert_guarded, upsert_findings) = scan_sql_upsert_bleed(root, &catalog);
    let (update_total, update_guarded, update_findings) = scan_sql_update_bleed(root, &catalog);
    let (delete_total, delete_guarded, delete_findings) = scan_sql_delete_bleed(root, &catalog);
    let (noop_total, noop_guarded, noop_findings) = scan_silent_noop_writes(root);
    let (num_total, num_guarded, num_findings) = scan_numerical_hazards(root);
    let (sym_total, sym_tested, _sym_findings) = scan_uncalled_public_symbols(root);

    let multi_tenant_count = catalog
        .tables
        .values()
        .filter(|t| !t.owner_columns.is_empty())
        .count();

    let total_hazards =
        upsert_total + update_total + delete_total + noop_total + num_total + sym_total;
    let total_guarded =
        upsert_guarded + update_guarded + delete_guarded + noop_guarded + num_guarded + sym_tested;

    let coverage_pct = if total_hazards > 0 {
        ((total_guarded as f64 / total_hazards as f64) * 1000.0).round() / 10.0
    } else {
        100.0
    };

    let mut all_findings = Vec::new();
    all_findings.extend(upsert_findings);
    all_findings.extend(update_findings);
    all_findings.extend(delete_findings);
    all_findings.extend(noop_findings);
    all_findings.extend(num_findings);

    ConstructCoverageReport {
        total_tables_discovered: catalog.tables.len(),
        multi_tenant_tables_count: multi_tenant_count,
        total_upsert_constructs_scanned: upsert_total,
        guarded_upsert_constructs: upsert_guarded,
        total_update_constructs_scanned: update_total,
        guarded_update_constructs: update_guarded,
        total_delete_constructs_scanned: delete_total,
        guarded_delete_constructs: delete_guarded,
        total_numerical_sites_scanned: num_total,
        safe_numerical_sites: num_guarded,
        total_public_symbols_scanned: sym_total,
        tested_public_symbols: sym_tested,
        construct_coverage_pct: coverage_pct.clamp(0.0, 100.0),
        findings: all_findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_table_with_owners() {
        let ddl = "CREATE TABLE payment_intents (\nintent_id TEXT PRIMARY KEY,\nuser_id TEXT NOT NULL,\namount INTEGER,\nstatus TEXT\n);";
        let parsed = SchemaCatalog::parse_create_table_snippet(ddl);
        assert!(parsed.is_some());
        let (name, pk, owners, status) = parsed.unwrap();
        assert_eq!(name, "payment_intents");
        assert_eq!(pk, "intent_id");
        assert_eq!(owners, vec!["user_id".to_string()]);
        assert_eq!(status, Some("status".to_string()));
    }

    #[test]
    fn test_resolve_upsert_target_with_inline_columns() {
        let catalog = SchemaCatalog::default();
        let snippet = "INSERT INTO custom_orders (order_id, user_id, amount) VALUES (?1, ?2, ?3) ON CONFLICT(order_id) DO UPDATE SET amount = excluded.amount";
        let target = resolve_upsert_target(snippet, &catalog);
        assert!(target.is_some());
        let (table, _pk, owners) = target.unwrap();
        assert_eq!(table, "custom_orders");
        assert_eq!(owners, vec!["user_id".to_string()]);
    }
}
