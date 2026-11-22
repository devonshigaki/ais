//! Code Quality Metrics (CCN & Maintainability Index)

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileMetricSummary {
    pub path: std::path::PathBuf,
    pub ccn: usize,
    pub mi: f64,
    pub loc: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityMetricsReport {
    pub max_ccn: usize,
    pub avg_ccn: f64,
    pub min_mi: f64,
    pub avg_mi: f64,
    pub total_loc: usize,
    pub files_scanned: usize,
    pub files_above_ccn_goal: Vec<FileMetricSummary>,
    pub files_below_mi_goal: Vec<FileMetricSummary>,
}

pub fn compute_ccn(content: &str) -> usize {
    let mut complexity = 1;
    let mut in_string = false;
    let mut in_raw_rust_str = false;
    let mut in_block_comment = false;
    let mut string_quote = ' ';
    let mut escaped = false;

    for line in content.lines() {
        if line.len() > 1000 {
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("//")
            || trimmed.starts_with("--")
            || trimmed.starts_with('#')
            || trimmed.starts_with('*')
        {
            continue;
        }

        if in_block_comment {
            if let Some(pos) = trimmed.find("*/").or_else(|| trimmed.find("-/")) {
                in_block_comment = false;
                if pos + 2 >= trimmed.len() {
                    continue;
                }
            } else {
                continue;
            }
        }

        if trimmed.starts_with("/*") || trimmed.starts_with("/-") {
            let end_pattern = if trimmed.starts_with("/*") {
                "*/"
            } else {
                "-/"
            };
            if let Some(pos) = trimmed.find(end_pattern) {
                if pos + 2 >= trimmed.len() {
                    continue;
                }
            } else {
                in_block_comment = true;
                continue;
            }
        }

        // Strip string literals so English words inside text constants
        // do not artificially inflate control flow complexity
        let mut code_only = String::with_capacity(trimmed.len());

        let mut chars = trimmed.chars().peekable();
        while let Some(ch) = chars.next() {
            if in_raw_rust_str {
                if ch == '"' && chars.peek() == Some(&'#') {
                    chars.next();
                    in_raw_rust_str = false;
                } else if ch == '"' {
                    in_raw_rust_str = false;
                }
                continue;
            }

            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' && in_string {
                escaped = true;
                continue;
            }
            if in_string {
                if ch == string_quote {
                    in_string = false;
                }
            } else if ch == 'r' && (chars.peek() == Some(&'"') || chars.peek() == Some(&'#')) {
                in_raw_rust_str = true;
                if chars.peek() == Some(&'#') {
                    chars.next();
                }
                chars.next();
            } else if ch == '"' || ch == '\'' || ch == '`' {
                in_string = true;
                string_quote = ch;
            } else {
                code_only.push(ch);
            }
        }

        for token in &[
            "if ", "match ", "while ", "for ", "&&", "||", "case ", "catch ", "elif ",
        ] {
            // POT-002: Bounded by line length (start increases each iteration)
            let mut start = 0;
            while let Some(pos) = code_only[start..].find(token) {
                let abs_pos = start + pos;

                // Enforce word boundaries for keyword tokens
                if (*token).ends_with(' ') && abs_pos > 0 {
                    if let Some(prev_ch) = code_only[..abs_pos].chars().next_back() {
                        if prev_ch.is_alphanumeric() || prev_ch == '_' {
                            start = abs_pos + token.len();
                            continue;
                        }
                    }
                }

                if *token == "case " {
                    // In Swift and TypeScript, enum variant declarations (e.g. `case foo = "foo"`, `case foo`)
                    // are declarative type definitions, not control-flow decision branches.
                    // A true control-flow switch case statement terminates with a colon `:`.
                    let line_suffix = &code_only[abs_pos..];
                    if !line_suffix.contains(':') || line_suffix.contains('=') {
                        start = abs_pos + token.len();
                        continue;
                    }
                }

                if *token == "||" {
                    // Check if this || is a Rust zero-argument closure (e.g. || { ... }, move || ...)
                    let before = code_only[..abs_pos].trim_end();
                    if before.is_empty()
                        || before.ends_with('(')
                        || before.ends_with(',')
                        || before.ends_with('=')
                        || before.ends_with('{')
                        || before.ends_with(';')
                        || before.ends_with("move")
                    {
                        start = abs_pos + token.len();
                        continue;
                    }

                    // Check if this || is a default value fallback rather than control branching
                    let after = code_only[abs_pos + 2..].trim_start();
                    if after.starts_with("''")
                        || after.starts_with("\"\"")
                        || after.starts_with("0")
                        || after.starts_with("{}")
                        || after.starts_with("[]")
                        || after.starts_with("null")
                        || after.starts_with("false")
                        || after.starts_with("undefined")
                    {
                        start = abs_pos + token.len();
                        continue;
                    }
                }
                complexity += 1;
                start = abs_pos + token.len();
            }
        }
    }
    complexity
}

pub fn compute_maintainability_index(ccn: usize, loc: usize) -> f64 {
    if loc == 0 {
        return 100.0;
    }
    let v = ((loc as f64) * 8.0).max(1.0);
    let loc_f = (loc as f64).max(1.0);
    let ccn_f = ccn as f64;

    let raw_mi = 171.0 - 5.2 * v.ln() - 0.23 * ccn_f - 16.2 * loc_f.ln();
    // Oman-Hagemeister (1992) / SEI canonical maintainability index on [0, 100]
    raw_mi.clamp(0.0, 100.0)
}

fn is_scannable_source_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    if crate::provenance::is_test_or_mock_path(path)
        || path_str.contains("fixtures")
        || path_str.contains(".spec.")
        || path_str.contains(".min.")
        || path_str.contains(".bundle.")
        || path_str.contains("/vendor/")
        || path_str.contains("static/js/vendor")
        || path_str.contains("smoldot-bundle")
        || path_str.contains("nomt-wasm")
        || path_str.contains("/.twenty/")
        || path_str.contains("/output/")
        || path_str.contains("/dist/")
        || path_str.contains("/build/")
        || path_str.contains("node_modules")
        || path_str.contains("target/")
        || path_str.contains("/.cargo/")
        || path_str.contains("/.rustup/")
    {
        return false;
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(
        ext,
        "rs" | "ts"
            | "tsx"
            | "js"
            | "mjs"
            | "jsx"
            | "py"
            | "swift"
            | "kt"
            | "kts"
            | "sol"
            | "lua"
            | "sh"
            | "sql"
            | "tf"
            | "hcl"
            | "lean"
    )
}

fn count_functions(content: &str, ext: &str) -> usize {
    if ext == "rs" {
        static RUST_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]+"\s+)?fn\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        static RUST_DECL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:pub(?:\([^\)]+\))?\s+)?(?:struct|enum|trait|type|const|static|impl|mod)\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        RUST_FN_RE.find_iter(content).count() + RUST_DECL_RE.find_iter(content).count()
    } else if ext == "py" {
        static PY_DEF_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*(?:async\s+)?def\s+[a-zA-Z0-9_]+"#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        static PY_CLASS_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*class\s+[a-zA-Z0-9_]+"#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        PY_DEF_RE.find_iter(content).count() + PY_CLASS_RE.find_iter(content).count()
    } else if ext == "swift" {
        static SWIFT_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:(?:public|private|internal|fileprivate|open|nonisolated)\s+)?(?:(?:static|class|mutating|override|final)\s+)?(?:func\s+[a-zA-Z0-9_]+|init\s*(?:<[^>]+>)?\s*\(|var\s+[a-zA-Z0-9_]+\s*:[^{]+?\{)"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        static SWIFT_DECL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:(?:public|private|internal|fileprivate|open)\s+)?(?:struct|enum|class|protocol|actor)\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        SWIFT_FN_RE.find_iter(content).count() + SWIFT_DECL_RE.find_iter(content).count()
    } else if ext == "kt" || ext == "kts" {
        static KT_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:(?:public|private|internal|protected|override)\s+)?(?:suspend\s+)?fun\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        static KT_DECL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:(?:public|private|internal|protected|sealed|data|inner)\s+)*(?:class|interface|object|enum\s+class)\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        KT_FN_RE.find_iter(content).count() + KT_DECL_RE.find_iter(content).count()
    } else if ext == "sol" {
        static SOL_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*function\s+[a-zA-Z0-9_]+"#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        SOL_FN_RE.find_iter(content).count()
    } else if ext == "lua" {
        static LUA_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*(?:local\s+)?function\s+[a-zA-Z0-9_]+"#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        LUA_FN_RE.find_iter(content).count()
    } else if ext == "sh" {
        static SH_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*(?:function\s+)?[a-zA-Z0-9_-]+\s*\(\)\s*\{"#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        SH_FN_RE.find_iter(content).count()
    } else if ext == "sql" {
        static SQL_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?im)^\s*CREATE\s+(?:OR\s+REPLACE\s+)?(?:TRIGGER|VIEW|TABLE|PROCEDURE|FUNCTION)\s+[a-zA-Z0-9_]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        SQL_FN_RE.find_iter(content).count()
    } else if ext == "tf" || ext == "hcl" {
        static TF_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(r#"(?m)^\s*(?:resource|module|data)\s+"[^"]+"\s+"[^"]+""#) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        TF_FN_RE.find_iter(content).count()
    } else if ext == "lean" {
        static LEAN_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:def|theorem|lemma|instance|structure|inductive)\s+[a-zA-Z0-9_'.]+"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        LEAN_FN_RE.find_iter(content).count()
    } else {
        static TS_FN_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)(?:export\s+(?:default\s+)?)?(?:async\s+)?function(?:\s+[a-zA-Z0-9_]+)?|(?:const|let|var)\s+[a-zA-Z0-9_]+(?:\s*:\s*[^=]+)?\s*=\s*(?:async\s*)?(?:<[^>]+>)?\s*(?:\([^)]*\)|[a-zA-Z0-9_]+)\s*=>|(?:async\s+)?[a-zA-Z0-9_]+\s*\([^)]*\)\s*(?::\s*[^\{]+)?\s*\{|\buse[A-Z]\w*\(|\bdefineLogicFunction\(|(?:handler|execute|run|resolve):\s*(?:async\s*)?(?:\([^)]*\)|[a-zA-Z0-9_]+)\s*=>"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        static TS_DECL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            match regex::Regex::new(
                r#"(?m)^\s*(?:export\s+(?:default\s+)?)?(?:class|interface|type|enum)\s+[a-zA-Z0-9_]+|^\s*(?:export\s+)?const\s+[a-zA-Z0-9_]+\s*(?::\s*[^=]+)?\s*=\s*[\[\{]"#,
            ) {
                Ok(r) => r,
                Err(_) => unreachable!(),
            }
        });
        TS_FN_RE.find_iter(content).count() + TS_DECL_RE.find_iter(content).count()
    }
}

fn is_brace_fn_start(trimmed: &str, ext: &str) -> bool {
    if ext == "rs" {
        return (trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("pub async fn ")
            || trimmed.starts_with("pub(crate) fn ")
            || trimmed.starts_with("pub(crate) async fn ")
            || trimmed.starts_with("pub(super) fn ")
            || trimmed.starts_with("async fn ")
            || trimmed.starts_with("unsafe fn ")
            || trimmed.starts_with("const fn "))
            && !trimmed.ends_with(';');
    }
    if ext == "kt" || ext == "kts" {
        return trimmed.contains("fun ") && trimmed.contains('{');
    }
    if ext == "swift" {
        return trimmed.contains("func ") && trimmed.contains('{');
    }
    if matches!(ext, "ts" | "tsx" | "js" | "jsx" | "mjs") {
        return (trimmed.contains("function ") || trimmed.contains("=> {"))
            && trimmed.contains('{');
    }
    false
}

pub fn compute_max_function_metrics(content: &str, ext: &str) -> Option<(usize, usize)> {
    if !matches!(
        ext,
        "rs" | "ts" | "tsx" | "js" | "jsx" | "mjs" | "kt" | "kts" | "swift"
    ) {
        return None;
    }
    let mut in_fn = false;
    let mut depth: i32 = 0;
    let mut seen_brace = false;
    let mut cur_lines: Vec<&str> = Vec::new();
    let mut max_ccn = 0usize;
    let mut max_loc = 0usize;
    let mut found = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if !in_fn && is_brace_fn_start(trimmed, ext) {
            in_fn = true;
            depth = 0;
            seen_brace = false;
            cur_lines.clear();
        }
        if in_fn {
            cur_lines.push(line);
            for ch in trimmed.chars() {
                if ch == '{' {
                    depth += 1;
                    seen_brace = true;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if !seen_brace && trimmed.ends_with(';') {
                in_fn = false;
                cur_lines.clear();
            } else if seen_brace && depth <= 0 {
                let body = cur_lines.join("\n");
                let ccn = compute_ccn(&body);
                let loc = cur_lines.iter().filter(|l| !l.trim().is_empty()).count();
                max_ccn = max_ccn.max(ccn);
                max_loc = max_loc.max(loc);
                found = true;
                in_fn = false;
                cur_lines.clear();
            }
        }
    }
    if found {
        Some((max_ccn, max_loc))
    } else {
        None
    }
}

pub fn analyze_file_metrics(path: &Path) -> Option<(usize, f64, usize)> {
    if !is_scannable_source_file(path) {
        return None;
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let content = fs::read_to_string(path).ok()?;
    let eval_content = if ext == "rs" {
        crate::ast::strip_cfg_test_blocks(&content)
    } else {
        content
    };
    let loc = eval_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();
    if loc == 0 {
        return None;
    }
    let ccn = compute_ccn(&eval_content);
    let raw_routines = count_functions(&eval_content, ext);
    let effective_routines = if raw_routines > 0 {
        raw_routines.max(1)
    } else {
        loc.div_ceil(25).max(1)
    };
    let avg_loc = (loc / effective_routines).max(1);
    let avg_ccn = (ccn / effective_routines).max(1);
    let fn_ccn = compute_max_function_metrics(&eval_content, ext)
        .map(|(m_ccn, _)| m_ccn.max(avg_ccn))
        .unwrap_or(avg_ccn);
    let mi = compute_maintainability_index(fn_ccn, avg_loc);
    Some((fn_ccn, mi, loc))
}

fn is_ignored_dir(e: &walkdir::DirEntry) -> bool {
    let name = e.file_name().to_string_lossy();
    crate::structure::is_ignored_directory_name(name.as_ref())
        || matches!(
            name.as_ref(),
            ".vscode" | ".idea" | ".cargo" | ".rustup" | ".turso"
        )
}

pub fn scan_codebase_metrics(root: &Path) -> QualityMetricsReport {
    use rayon::prelude::*;

    let engine_fp = crate::cache::compute_engine_fingerprint(root);
    let mut cache = crate::cache::load_layer_cache(root, engine_fp);
    let mut files_to_scan = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_ignored_dir(e))
        .filter_map(|e| e.ok())
    {
        let p_str = entry.path().to_string_lossy();
        if p_str.contains("xtask/") {
            continue;
        }
        if entry.file_type().is_file() && is_scannable_source_file(entry.path()) {
            files_to_scan.push(entry.path().to_path_buf());
        }
    }

    let per_file_metrics: Vec<(std::path::PathBuf, String, u64, usize, f64, usize)> = files_to_scan
        .into_par_iter()
        .filter_map(|path| {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let content = fs::read_to_string(&path).ok()?;
            let eval_content = if ext == "rs" {
                crate::ast::strip_cfg_test_blocks(&content)
            } else {
                content
            };
            let loc = eval_content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count();
            if loc == 0 {
                return None;
            }

            let rel_path = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            let rel_str = rel_path.to_string_lossy().to_string();
            let hash = crate::cache::compute_content_hash(&eval_content);

            if let Some(cached) = cache.metrics.get(&rel_str) {
                if cached.hash == hash {
                    return Some((rel_path, rel_str, hash, cached.ccn, cached.mi, cached.loc));
                }
            }

            let ccn = compute_ccn(&eval_content);
            let raw_routines = count_functions(&eval_content, ext);
            let effective_routines = if raw_routines > 0 {
                raw_routines.max(1)
            } else {
                loc.div_ceil(25).max(1)
            };
            let avg_loc = (loc / effective_routines).max(1);
            let avg_ccn = (ccn / effective_routines).max(1);
            let fn_ccn = compute_max_function_metrics(&eval_content, ext)
                .map(|(m_ccn, _)| m_ccn.max(avg_ccn))
                .unwrap_or(avg_ccn);
            let mi = compute_maintainability_index(fn_ccn, avg_loc);

            Some((rel_path, rel_str, hash, fn_ccn, mi, loc))
        })
        .collect();

    let mut max_ccn = 0;
    let mut total_ccn = 0;
    let mut min_mi = 100.0f64;
    let mut total_mi = 0.0f64;
    let mut total_loc = 0;
    let mut files_scanned = 0;
    let mut files_above_ccn_goal = Vec::new();
    let mut files_below_mi_goal = Vec::new();
    let mut updated = false;

    for (rel, rel_str, hash, ccn, mi, loc) in per_file_metrics {
        if ccn > max_ccn {
            max_ccn = ccn;
        }
        if mi < min_mi {
            min_mi = mi;
        }

        if ccn > 10 {
            files_above_ccn_goal.push(FileMetricSummary {
                path: rel.clone(),
                ccn,
                mi: (mi * 10.0).round() / 10.0,
                loc,
            });
        }
        if mi < 80.0 {
            files_below_mi_goal.push(FileMetricSummary {
                path: rel,
                ccn,
                mi: (mi * 10.0).round() / 10.0,
                loc,
            });
        }

        total_ccn += ccn;
        total_mi += mi;
        total_loc += loc;
        files_scanned += 1;

        if !cache.metrics.contains_key(&rel_str)
            || cache.metrics.get(&rel_str).map_or(0, |c| c.hash) != hash
        {
            cache.metrics.insert(
                rel_str,
                crate::cache::CachedFileMetrics { hash, ccn, mi, loc },
            );
            updated = true;
        }
    }

    if updated {
        crate::cache::save_layer_cache(root, &cache);
    }

    files_above_ccn_goal.sort_by(|a, b| b.ccn.cmp(&a.ccn).then_with(|| a.path.cmp(&b.path)));
    files_below_mi_goal.sort_by(|a, b| {
        a.mi.partial_cmp(&b.mi)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.path.cmp(&b.path))
    });

    let avg_ccn = if files_scanned > 0 {
        total_ccn as f64 / files_scanned as f64
    } else {
        1.0
    };
    let avg_mi = if files_scanned > 0 {
        total_mi / files_scanned as f64
    } else {
        100.0
    };

    QualityMetricsReport {
        max_ccn,
        avg_ccn: (avg_ccn * 10.0).round() / 10.0,
        min_mi: (min_mi * 10.0).round() / 10.0,
        avg_mi: (avg_mi * 10.0).round() / 10.0,
        total_loc,
        files_scanned,
        files_above_ccn_goal,
        files_below_mi_goal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_ccn_simple() {
        let code = "fn foo() { if a && b { return; } }";
        let ccn = compute_ccn(code);
        assert!(ccn >= 2);
    }

    #[test]
    fn test_compute_maintainability_index() {
        let mi = compute_maintainability_index(5, 50);
        assert!(mi > 30.0);
        assert!(mi <= 100.0);
    }

    #[test]
    fn test_compute_maintainability_index_zero_loc() {
        let mi = compute_maintainability_index(1, 0);
        assert_eq!(mi, 100.0);
    }

    #[test]
    fn test_monolith_function_ccn_not_diluted() {
        let mut src =
            String::from("fn tiny() {}\nfn monolith(x: i32) -> i32 {\n    let mut acc = 0;\n");
        for i in 0..25 {
            src.push_str(&format!("    if x == {i} {{ acc += {i}; }}\n"));
        }
        src.push_str("    acc\n}\n");
        let (max_ccn, _max_loc) = compute_max_function_metrics(&src, "rs").unwrap();
        assert!(max_ccn >= 26, "expected monolith CCN >= 26, got {max_ccn}");
    }
}
