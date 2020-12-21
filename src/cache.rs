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

    pub fn insert(&mut self, key: CacheKey, value: CacheValue) {
        // println!("Insert into Cache: key={}", key);
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheValue> {
        self.map.get(key)
    }
}
