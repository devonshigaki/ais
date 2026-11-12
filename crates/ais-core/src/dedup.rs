//! Semantic Deduplication & Structural Clone Detection Engine (MinHash LSH)
//!
//! Provides linear-time AST-shingled clone detection across Rust, TypeScript, and Python:
//! - Normalizes concrete syntax tokens (stripping identifiers & literal values)
//! - Generates 64-permutation MinHash dense signatures per subroutine
//! - Utilizes Locality-Sensitive Hashing (LSH) banding to discover near-duplicate pairs (Jaccard similarity >= 85%)

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const NUM_PERMUTATIONS: usize = 64;
const NUM_BANDS: usize = 16;
const ROWS_PER_BAND: usize = 4; // 16 * 4 = 64
const SHINGLE_SIZE: usize = 4;

pub(crate) static FN_DECL_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    match regex::Regex::new(
        r#"(?:pub\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]+"\s+)?(?:fun|func|function|fn)\s+([a-zA-Z0-9_]+)\s*(?:<[^>]+>)?\s*\("#,
    ) {
        Ok(r) => r,
        Err(e) => panic!("invalid regex: {e}"),
    }
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDuplicateCluster {
    pub similarity_pct: f64,
    pub function_a: String,
    pub file_a: PathBuf,
    pub line_a: usize,
    pub function_b: String,
    pub file_b: PathBuf,
    pub line_b: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionFingerprint {
    pub file: PathBuf,
    pub name: String,
    pub line: usize,
    pub loc: usize,
    pub signature: Vec<u64>,
}

fn tokenize_and_normalize(code: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    let keywords = [
        "fn", "function", "def", "let", "const", "var", "if", "else", "match", "for", "while",
        "return", "await", "async", "pub", "struct", "class",
    ];

    let common_ops = [
        "insert", "query", "execute", "get", "set", "lock", "unwrap", "expect", "clone", "connect",
        "select", "update", "delete", "send", "receive", "encode", "decode", "parse", "format",
        "Result", "Option", "Some", "None", "Ok", "Err", "String", "Vec", "Self", "AppState",
    ];

    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // String literals: "..."
        if b == b'"' {
            let start = i;
            i += 1;
            let mut escaped = false;
            while i < len {
                if escaped {
                    escaped = false;
                } else if bytes[i] == b'\\' {
                    escaped = true;
                } else if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let lit = &code[start..i.min(len)];
            if lit.contains("SELECT")
                || lit.contains("INSERT")
                || lit.contains("UPDATE")
                || lit.contains("DELETE")
                || lit.contains("FROM")
            {
                let mut hasher = DefaultHasher::new();
                lit.hash(&mut hasher);
                tokens.push(format!("SQL_{:x}", hasher.finish()));
            } else {
                tokens.push("LIT_STR".to_string());
            }
            continue;
        }

        // Character literal vs Rust lifetime: 'a' vs 'a
        if b == b'\'' {
            // Check if char literal with closing quote within 4 bytes: 'c', '\n'
            let mut found_char_end = None;
            for offset in 2..=4 {
                if i + offset < len && bytes[i + offset] == b'\'' && bytes[i + offset - 1] != b'\\'
                {
                    found_char_end = Some(i + offset + 1);
                    break;
                }
            }
            if let Some(end) = found_char_end {
                tokens.push("LIT_CHAR".to_string());
                i = end;
                continue;
            }

            // Rust lifetime like 'a or 'static
            let start = i;
            i += 1;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            if i > start + 1 {
                tokens.push("LIFETIME".to_string());
            } else {
                tokens.push("'".to_string());
            }
            continue;
        }

        // Numeric literals: 123, 123.45
        if b.is_ascii_digit() {
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'_') {
                i += 1;
            }
            tokens.push("LIT_NUM".to_string());
            continue;
        }

        // Identifiers & keywords: [a-zA-Z_][a-zA-Z0-9_]*
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &code[start..i];
            if keywords.contains(&word) {
                tokens.push(word.to_uppercase());
            } else if common_ops.contains(&word) {
                tokens.push(word.to_string());
            } else if word.starts_with(|c: char| c.is_ascii_uppercase()) {
                tokens.push(format!("TYPE_{word}"));
            } else {
                tokens.push("ID".to_string());
            }
            continue;
        }

        // Single punctuation or operator
        let ch_len = code[i..].chars().next().map_or(1, |c| c.len_utf8());
        tokens.push(code[i..i + ch_len].to_string());
        i += ch_len;
    }

    tokens
}

fn generate_shingles(tokens: &[String]) -> HashSet<u64> {
    let mut shingles = HashSet::new();
    if tokens.len() < SHINGLE_SIZE {
        let mut hasher = DefaultHasher::new();
        tokens.hash(&mut hasher);
        shingles.insert(hasher.finish());
        return shingles;
    }

    for window in tokens.windows(SHINGLE_SIZE) {
        let mut hasher = DefaultHasher::new();
        window.hash(&mut hasher);
        shingles.insert(hasher.finish());
    }
    shingles
}

fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn compute_minhash_signature(shingles: &HashSet<u64>) -> [u64; NUM_PERMUTATIONS] {
    let mut sig = [u64::MAX; NUM_PERMUTATIONS];
    if shingles.is_empty() {
        return sig;
    }

    for &shingle in shingles {
        for (idx, slot) in sig.iter_mut().enumerate() {
            // Carter-Wegman universal hashing with SplitMix64 avalanche finalizer
            let seed = (idx as u64).wrapping_mul(0x9e3779b97f4a7c15);
            let mixed = splitmix64(shingle.wrapping_add(seed));
            if mixed < *slot {
                *slot = mixed;
            }
        }
    }
    sig
}

pub fn estimate_jaccard_similarity(sig_a: &[u64], sig_b: &[u64]) -> f64 {
    let mut matching = 0;
    let limit = NUM_PERMUTATIONS.min(sig_a.len()).min(sig_b.len());
    for i in 0..limit {
        if sig_a[i] == sig_b[i] {
            matching += 1;
        }
    }
    (matching as f64 / NUM_PERMUTATIONS as f64 * 100.0 * 10.0).round() / 10.0
}

pub fn fingerprint_code_snippet(
    file: PathBuf,
    name: String,
    line: usize,
    code: &str,
) -> FunctionFingerprint {
    let tokens = tokenize_and_normalize(code);
    let shingles = generate_shingles(&tokens);
    let signature = compute_minhash_signature(&shingles);
    let loc = code.lines().filter(|l| !l.trim().is_empty()).count();
    FunctionFingerprint {
        file,
        name,
        line,
        loc,
        signature: signature.to_vec(),
    }
}

pub fn detect_lsh_duplicate_clusters(
    fingerprints: &[FunctionFingerprint],
    similarity_threshold: f64,
) -> Vec<SemanticDuplicateCluster> {
    let mut band_buckets: Vec<HashMap<u64, Vec<usize>>> = vec![HashMap::new(); NUM_BANDS];

    for (fn_idx, fp) in fingerprints.iter().enumerate() {
        for (band, bucket_map) in band_buckets.iter_mut().enumerate() {
            let start = band * ROWS_PER_BAND;
            let end = start + ROWS_PER_BAND;
            let mut hasher = DefaultHasher::new();
            fp.signature[start..end].hash(&mut hasher);
            let band_hash = hasher.finish();
            bucket_map.entry(band_hash).or_default().push(fn_idx);
        }
    }

    let mut candidate_pairs = BTreeSet::new();
    for band_map in &band_buckets {
        for bucket in band_map.values() {
            if bucket.len() > 1 {
                if bucket.len() <= 50 {
                    for i in 0..bucket.len() {
                        for j in (i + 1)..bucket.len() {
                            let idx_a = bucket[i].min(bucket[j]);
                            let idx_b = bucket[i].max(bucket[j]);
                            candidate_pairs.insert((idx_a, idx_b));
                        }
                    }
                } else {
                    // For dense collision buckets, sort by full 64-word MinHash signature and compare within sliding window
                    let mut sorted_bucket = bucket.clone();
                    sorted_bucket.sort_by(|&a_idx, &b_idx| {
                        fingerprints[a_idx]
                            .signature
                            .cmp(&fingerprints[b_idx].signature)
                    });
                    const WINDOW: usize = 32;
                    for i in 0..sorted_bucket.len() {
                        for j in (i + 1)..(i + 1 + WINDOW).min(sorted_bucket.len()) {
                            let idx_a = sorted_bucket[i].min(sorted_bucket[j]);
                            let idx_b = sorted_bucket[i].max(sorted_bucket[j]);
                            candidate_pairs.insert((idx_a, idx_b));
                        }
                    }
                }
            }
        }
    }

    let mut clusters = Vec::new();
    for (idx_a, idx_b) in candidate_pairs {
        let a = &fingerprints[idx_a];
        let b = &fingerprints[idx_b];
        if a.file == b.file && a.name == b.name && a.line == b.line {
            continue;
        }

        let sim = estimate_jaccard_similarity(&a.signature, &b.signature);
        if sim >= similarity_threshold {
            clusters.push(SemanticDuplicateCluster {
                similarity_pct: sim,
                function_a: a.name.clone(),
                file_a: a.file.clone(),
                line_a: a.line,
                function_b: b.name.clone(),
                file_b: b.file.clone(),
                line_b: b.line,
            });
        }
    }

    clusters.sort_by(|a, b| {
        b.similarity_pct
            .partial_cmp(&a.similarity_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.file_a.cmp(&b.file_a))
            .then_with(|| a.line_a.cmp(&b.line_a))
            .then_with(|| a.function_a.cmp(&b.function_a))
            .then_with(|| a.file_b.cmp(&b.file_b))
            .then_with(|| a.line_b.cmp(&b.line_b))
            .then_with(|| a.function_b.cmp(&b.function_b))
    });
    clusters
}

pub fn scan_workspace_semantic_duplicates(
    root: &Path,
    threshold: f64,
) -> Vec<SemanticDuplicateCluster> {
    let mut fingerprints = Vec::new();
    let inv = crate::structure::WorkspaceInventory::discover(root);
    let canonical_files = inv.collect_canonical_files(root);

    for file_path in &canonical_files {
        let ext = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !matches!(
            ext,
            "rs" | "ts" | "tsx" | "js" | "mjs" | "swift" | "kt" | "kts" | "sol"
        ) {
            continue;
        }

        let path_str = file_path.to_string_lossy();
        if crate::provenance::is_test_or_mock_path(file_path)
            || path_str.contains("canary")
            || path_str.contains("fixtures")
            || path_str.contains(".min.")
            || path_str.contains(".bundle.")
            || path_str.contains("static/js/vendor")
            || path_str.contains("smoldot-bundle")
            || path_str.contains("nomt-wasm")
            || path_str.contains("node_modules")
            || path_str.contains("/target/")
        {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(file_path) else {
            continue;
        };

        let rel_path = file_path
            .strip_prefix(root)
            .unwrap_or(file_path)
            .to_path_buf();

        let stripped_content = if ext == "rs" {
            crate::ast::strip_cfg_test_blocks(&content)
        } else {
            content
        };
        let lines: Vec<&str> = stripped_content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = FN_DECL_RE.captures(line) {
                if let Some(fn_match) = cap.get(1) {
                    let fn_name = fn_match.as_str().to_string();
                    if fn_name.starts_with("test_") {
                        continue;
                    }

                    // Brace-balanced extraction of function body
                    let mut brace_depth = 0;
                    let mut started = false;
                    let mut end_idx = idx;
                    let mut hit_semicolon_before_brace = false;

                    for (curr_idx, curr_line) in lines.iter().enumerate().skip(idx) {
                        for ch in curr_line.chars() {
                            if ch == ';' && !started {
                                hit_semicolon_before_brace = true;
                                break;
                            }
                            if ch == '{' {
                                brace_depth += 1;
                                started = true;
                            } else if ch == '}' {
                                brace_depth -= 1;
                            }
                        }
                        if hit_semicolon_before_brace {
                            break;
                        }
                        if started && brace_depth == 0 {
                            end_idx = curr_idx + 1;
                            break;
                        }
                    }

                    if hit_semicolon_before_brace || !started || end_idx <= idx {
                        continue;
                    }

                    let fn_code = lines[idx..end_idx].join("\n");
                    let fn_loc = fn_code.lines().filter(|l| !l.trim().is_empty()).count();
                    if fn_loc >= 10 && fn_code.len() >= 180 {
                        fingerprints.push(fingerprint_code_snippet(
                            rel_path.clone(),
                            fn_name,
                            idx + 1,
                            &fn_code,
                        ));
                    }
                }
            }
        }
    }

    detect_lsh_duplicate_clusters(&fingerprints, threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minhash_detects_semantic_clones() {
        let fn1 = r#"
            pub fn calculate_credit_ratio(balance: f64, limit: f64) -> f64 {
                if limit <= 0.0 {
                    return 0.0;
                }
                let ratio = balance / limit;
                ratio * 100.0
            }
        "#;

        let fn2 = r#"
            pub fn compute_utilization_percentage(total_debt: f64, max_credit: f64) -> f64 {
                if max_credit <= 0.0 {
                    return 0.0;
                }
                let pct = total_debt / max_credit;
                pct * 100.0
            }
        "#;

        let fp1 = fingerprint_code_snippet(PathBuf::from("a.rs"), "calc_ratio".to_string(), 1, fn1);
        let fp2 =
            fingerprint_code_snippet(PathBuf::from("b.rs"), "compute_util".to_string(), 1, fn2);

        let sim = estimate_jaccard_similarity(&fp1.signature, &fp2.signature);
        assert!(
            sim >= 70.0,
            "Expected high semantic similarity, got {:.1}%",
            sim
        );

        let clusters = detect_lsh_duplicate_clusters(&[fp1, fp2], 70.0);
        assert_eq!(clusters.len(), 1);
    }
}
