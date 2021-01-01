use log::{debug, error, info, warn};
use rustc_hash::FxHashMap;

use crate::cxxbridge::ffi::StreamDataImplShared;

type CacheKey = u64;
type CacheValue = StreamDataImplShared;

#[derive(Debug)]
pub struct CacheImpl {
    map: FxHashMap<CacheKey, CacheValue>,
}

impl CacheImpl {
    pub fn new() -> CacheImpl {
        let map = FxHashMap::default();
        CacheImpl { map }
    }

    pub fn get_id(&self) -> u64 {
        42
    }

    pub fn len(&self) -> usize {
        let value = self.map.len();
        debug!("count value: {}", value);
        value
    }

    pub fn insert(&mut self, key: CacheKey, value: CacheValue) {
        debug!("Insert into Cache: key={}", key);
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheValue> {
        self.map.get(key)
    }
}

/*
 * // Disk-based cache.
 * ocg::DiskCache<ocg::Hash, ocg::BaseNodeResult> cacheB();
 * cacheB.setPath("/tmp/openCompGraph");
 * cacheB.setCapacity(10 * GIGABYTES_TO_BYTES);
 *
 * // RAM-based cache.
 * ocg::MemoryCache<ocg::Hash, ocg::BaseNodeResult> cacheA();
 * cacheA.setCapacity(1024 * MEGABYTES_TO_BYTES);
 *
 * std::vector<BaseCache> cacheList;
 * cacheList.push_back(cacheA);
 * cacheList.push_back(cacheB);
 */
