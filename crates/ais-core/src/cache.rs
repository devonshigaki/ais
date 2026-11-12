//! Incremental Content-Hash Scanner & Layer Cache Engine
//!
//! Provides Docker-layer-style content-addressed caching for:
//! 1. File metrics (CCN, Maintainability Index, LOC)
//! 2. Structural cohesion (LCOM4 class and struct components)
//! 3. Subroutine fingerprints (64-permutation MinHash signatures)
//! 4. Scanner findings and opportunity reports

use crate::cohesion::StructCohesion;
use crate::dedup::FunctionFingerprint;
use crate::findings::Finding;
use crate::opportunities::OpportunityScanReport;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

pub const DEFAULT_CACHE_FILE: &str = ".freshcredit/cache/ais_scanner_cache.json";
pub const AIS_LAYER_CACHE_FILE: &str = ".freshcredit/cache/ais_layer_cache.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScannerCache {
    pub rules_count: usize,
    pub scanner_fingerprint: u64,
    pub entries: BTreeMap<String, CacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub hash: u64,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileMetrics {
    pub hash: u64,
    pub ccn: usize,
    pub mi: f64,
    pub loc: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileCohesion {
    pub hash: u64,
    pub structs: Vec<StructCohesion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileFingerprints {
    pub hash: u64,
    pub fingerprints: Vec<FunctionFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AisLayerCache {
    pub engine_fingerprint: u64,
    pub metrics: BTreeMap<String, CachedFileMetrics>,
    pub cohesion: BTreeMap<String, CachedFileCohesion>,
    pub fingerprints: BTreeMap<String, CachedFileFingerprints>,
    pub cached_opportunities: Option<(u64, OpportunityScanReport)>,
}

pub fn compute_content_hash(s: &str) -> u64 {
    debug_assert!(s.is_char_boundary(0), "char boundary valid");
    debug_assert!(s.len() < 100_000_000, "string bounded");
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Computes a self-invalidating content-addressed fingerprint across ais-core engine source files
pub fn compute_engine_fingerprint(root: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    let src_dir = root.join("crates/ais-core/src");
    if let Ok(entries) = std::fs::read_dir(&src_dir) {
        let mut paths: Vec<_> = entries
            .filter_map(|e| e.ok().map(|ent| ent.path()))
            .filter(|p| p.extension().is_some_and(|e| e == "rs"))
            .collect();
        paths.sort();
        for p in paths {
            if let Ok(content) = std::fs::read_to_string(&p) {
                p.file_name().unwrap_or_default().hash(&mut hasher);
                content.hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

pub fn load_layer_cache(root: &Path, engine_fingerprint: u64) -> AisLayerCache {
    let cache_path = root.join(AIS_LAYER_CACHE_FILE);
    if let Ok(content) = std::fs::read_to_string(&cache_path) {
        if let Ok(cache) = serde_json::from_str::<AisLayerCache>(&content) {
            if cache.engine_fingerprint == engine_fingerprint {
                return cache;
            }
        }
    }
    AisLayerCache {
        engine_fingerprint,
        ..Default::default()
    }
}

pub fn save_layer_cache(root: &Path, cache: &AisLayerCache) {
    let cache_path = root.join(AIS_LAYER_CACHE_FILE);
    if let Some(parent) = cache_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string(cache) {
        let _ = std::fs::write(&cache_path, json);
    }
}
