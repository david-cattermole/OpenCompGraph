use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// use crate::cxxbridge::create_stream_data_shared;
// use crate::cxxbridge::ffi::AttrState;
// use crate::cxxbridge::ffi::ExecuteStatus;
// use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
// use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
// use crate::data::{EdgeIdx, EdgeWeight, GraphIdx, Identifier, NodeIdx, NodeWeight};
// use crate::graphiter::UpstreamEvalSearch;
// use crate::node::NodeImpl;

type CacheKey = u64;
type CacheValue = StreamDataImplShared;

#[derive(Debug)]
pub struct CacheImpl {
    pub map: FxHashMap<CacheKey, CacheValue>,
    // pub map: HashMap<CacheKey, CacheValue>,
}

impl CacheImpl {
    pub fn new() -> CacheImpl {
        let map = FxHashMap::default();
        CacheImpl { map }
    }

    pub fn insert(&mut self, key: CacheKey, value: CacheValue) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheValue> {
        self.map.get(key)
    }
}

// pub fn create_cache() -> CacheImpl {
//     CacheImpl::new();
// }
