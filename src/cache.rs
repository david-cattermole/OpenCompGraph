use log::{debug, error, info, warn};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::BYTES_TO_GIGABYTES;

type CacheTimestamp = usize;
type CacheIndex = usize;
type CacheKey = u64;
type CacheValue = StreamDataImplShared;

#[derive(Debug, PartialEq)]
struct OldestTimestamp(CacheTimestamp);

impl Eq for OldestTimestamp {}

impl PartialOrd for OldestTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for OldestTimestamp {
    fn cmp(&self, other: &OldestTimestamp) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct CacheEntry {
    timestamp: CacheTimestamp,
    value: CacheValue,
}

#[derive(Debug)]
pub struct CacheImpl {
    // Mapping data structure.
    key_to_entry: FxHashMap<CacheKey, CacheEntry>,

    // Least Recently Used priority queue.
    lru_timestamps: BinaryHeap<OldestTimestamp>,
    timestamp_to_key: FxHashMap<CacheTimestamp, CacheKey>,
    last_timestamp: usize,

    // Keep track of how much memory is used in the cache.
    capacity_bytes: usize,
    used_bytes: usize,

    // Diagnostics; hits vs misses.
    hits: usize,
    misses: usize,
    inserts: usize,
    evictions: usize,
}

impl CacheImpl {
    /// An empty cache, without any capacity.
    pub fn new() -> CacheImpl {
        let capacity_bytes = 0;
        CacheImpl::with_capacity(capacity_bytes)
    }

    /// Create a new cache with the given capacity (in bytes).
    pub fn with_capacity(capacity_bytes: usize) -> CacheImpl {
        let key_to_entry = FxHashMap::default();
        let lru_timestamps = BinaryHeap::new();
        let timestamp_to_key = FxHashMap::default();

        let used_bytes = 0;
        let capacity_bytes = 0;
        let last_timestamp = 0;
        let hits = 0;
        let misses = 0;
        let inserts = 0;
        let evictions = 0;

        CacheImpl {
            key_to_entry,
            lru_timestamps,
            timestamp_to_key,
            last_timestamp,
            capacity_bytes,
            used_bytes,
            hits,
            misses,
            inserts,
            evictions,
        }
    }

    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        let value = self.key_to_entry.len();
        debug!("count value: {}", value);
        value
    }

    /// Amount of memory used by the cache.
    pub fn used_bytes(&self) -> usize {
        let value = self.used_bytes;
        debug!("used_bytes: {}", value);
        value
    }

    /// Amount of memory that can be put into the cache.
    pub fn capacity_bytes(&self) -> usize {
        let value = self.capacity_bytes;
        debug!("capacity_bytes: {}", value);
        value
    }

    /// Set the capacity of the cache.
    ///
    /// If the new capacity value is larger than the currently used
    /// memory entries will be evicted from the cache until there is
    /// enough memory.
    pub fn set_capacity_bytes(&mut self, value: usize) {
        if value < self.used_bytes {
            let free_bytes_needed = self.used_bytes - value;
            self.evict_bytes(free_bytes_needed);
        }
        self.capacity_bytes = value;
    }

    /// Insert a new cache value into to the Cache. If a value already
    /// exists with the same 'key' it will be evicted from the cache.
    pub fn insert(&mut self, key: CacheKey, value: CacheValue) {
        debug!("Insert into Cache: key={}", key);
        if self.capacity_bytes == 0 {
            error!("Cannot insert into Cache; capacity is zero.");
            return;
        }

        let value_bytes = value.inner.size_bytes();

        // Make space if necessary
        let expected_bytes = self.used_bytes + value_bytes;
        if expected_bytes >= self.capacity_bytes {
            let require_used = self.capacity_bytes - value_bytes;
            let free_bytes_needed = self.used_bytes - require_used;
            self.evict_bytes(free_bytes_needed);
        }

        // Register the newest key to be "used" by our cache. This key
        // is therefore "hot", where as only "cold" keys will be lower
        // in the heap and will be removed as needed.
        let entry_timestamp = self.last_timestamp + 1;
        self.lru_timestamps.push(OldestTimestamp(entry_timestamp));
        self.timestamp_to_key.insert(entry_timestamp, key);

        // Put our value into the map.
        let entry = CacheEntry {
            timestamp: entry_timestamp,
            value,
        };
        self.key_to_entry.insert(key, entry);

        self.last_timestamp = entry_timestamp;
        self.used_bytes += value_bytes;
        self.inserts += 1;
    }

    /// Get a value from the Cache, if it exists.
    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheValue> {
        debug!("Query Cache: key={}", key);
        let entry = self.key_to_entry.get(key);
        match entry {
            Some(entry) => {
                // TODO: Remove the timestamp from
                // 'self.lru_timestamps' heap (which is not possible
                // in the Rust API).
                self.timestamp_to_key.remove(&entry.timestamp);
                let entry_timestamp = self.last_timestamp + 1;
                self.timestamp_to_key.insert(entry_timestamp, *key);
                self.lru_timestamps.push(OldestTimestamp(entry_timestamp));
                self.hits += 1;
                self.last_timestamp = entry_timestamp;
                Some(&entry.value)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// How much memory (in bytes) do you want to evict?
    ///
    /// Strategy for removing items is Least Recently Used (LRU).
    pub fn evict_bytes(&mut self, n_bytes: usize) {
        if self.lru_timestamps.len() == 0 {
            debug!("Cache Data: {}", self.data_debug_string());
            error!("Could not evict bytes, no timestamps!");
            return;
        }
        if self.used_bytes == 0 {
            // There is nothing to evict.
            return;
        }
        if self.capacity_bytes == 0 {
            self.evict_all();
            return;
        }

        let requested_capacity = match n_bytes > self.used_bytes {
            true => {
                // If the user requests to free more bytes than we have
                // used, then we simply remove everything.
                //
                // This ensures we do not have integer overflow problems
                // when comparing self.used_bytes and n_bytes.
                0
            }
            false => self.used_bytes - n_bytes,
        };

        while (self.used_bytes >= requested_capacity) && (self.lru_timestamps.len() > 0) {
            self.evict();
        }
    }

    pub fn evict_all(&mut self) {
        debug!("EVICT ALL: count={}", self.lru_timestamps.len());
        // Keep evicting until all entries are removed from the cache.
        while self.lru_timestamps.len() > 0 {
            self.evict();
        }
    }

    fn pop_next_valid_key_to_evict(&mut self) -> Option<CacheKey> {
        let mut valid_timestamp_key = None;
        while let Some(next_oldest_timestamp) = self.lru_timestamps.pop() {
            let next_oldest_entry = next_oldest_timestamp.0;
            if let Some(key) = self.timestamp_to_key.remove(&next_oldest_entry) {
                valid_timestamp_key = Some(key);
                break;
            }
            // If no key exists, the timestamp has already been
            // evicted and we can pop another timestamp off and try to
            // evict that one instead.
        }
        valid_timestamp_key
    }

    pub fn evict(&mut self) -> bool {
        let mut success = false;
        // We assume 'self.evict()' will always update
        // 'self.used_bytes' each time it runs and the used_bytes will
        // always decrease after each call.
        //
        // Identify least recently used key, and remove the key.
        if let Some(key) = self.pop_next_valid_key_to_evict() {
            if let Some(entry) = self.key_to_entry.remove(&key) {
                let value = entry.value;
                let value_bytes = value.inner.size_bytes();
                // There should not be any overflow problems with this
                // subtraction since we add and subtract exactly goes
                // into the cache. If there is an overflow here, it
                // means we must have skipped counting a cache entry's
                // size somewhere.
                self.used_bytes -= value_bytes;

                success = true;
                self.evictions += 1;
            }
        };
        success
    }

    /// Convert the graph into a human-readable string, for debug
    /// printing.
    pub fn data_debug_string(&self) -> String {
        debug!("Cache Debug");
        let string = format!(
            "capacity={}GB used={}GB hit_ratio={}% hits={} misses={} insert_ratio={}% inserts={} evictions={} count={}",
            self.capacity_bytes as f64 / BYTES_TO_GIGABYTES as f64,
            self.used_bytes as f64 / BYTES_TO_GIGABYTES as f64,
            (self.hits as f64 / self.misses as f64) * 100.0,
            self.hits,
            self.misses,
            (self.inserts as f64 / self.evictions as f64) * 100.0,
            self.inserts,
            self.evictions,
            self.key_to_entry.len()
        );
        string
    }
}

pub fn create_cache_box_with_capacity(capacity_bytes: usize) -> Box<CacheImpl> {
    debug!("create_cache_box_with_capacity()");
    Box::new(CacheImpl::with_capacity(capacity_bytes))
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
