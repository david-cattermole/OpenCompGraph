/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use linked_hash_map::LinkedHashMap;
use log::{debug, error};
use std::rc::Rc;

use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageSpec;
use crate::data::BYTES_TO_GIGABYTES;
use crate::pixelblock::pixelblock::PixelBlock;

#[derive(Debug, Clone)]
pub struct CachedImage {
    pub pixel_block: Rc<PixelBlock>,
    pub spec: ImageSpec,
    pub display_window: BBox2Di,
    pub data_window: BBox2Di,
}

impl CachedImage {
    pub fn size_bytes(&self) -> usize {
        self.pixel_block.size_bytes()
    }
}

type CacheKey = u64;
type CacheValue = CachedImage;

#[derive(Debug)]
pub struct CacheImpl {
    lru_hash_map: LinkedHashMap<CacheKey, CacheValue>,

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
        let lru_hash_map = LinkedHashMap::new();

        let used_bytes = 0;
        let hits = 0;
        let misses = 0;
        let inserts = 0;
        let evictions = 0;

        CacheImpl {
            lru_hash_map,
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
        let value = self.lru_hash_map.len();
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

        let value_bytes = value.size_bytes();

        // Make space if necessary
        let expected_bytes = self.used_bytes + value_bytes;
        if expected_bytes >= self.capacity_bytes {
            let require_used = self.capacity_bytes - value_bytes;
            let free_bytes_needed = self.used_bytes - require_used;
            self.evict_bytes(free_bytes_needed);
        }

        self.lru_hash_map.insert(key, value);
        self.used_bytes += value_bytes;
        self.inserts += 1;
    }

    /// Get a value from the Cache, if it exists.
    pub fn get(&mut self, key: &CacheKey) -> Option<&CacheValue> {
        debug!("Query Cache: key={}", key);
        let value = self.lru_hash_map.get_refresh(key);
        match value {
            Some(value) => {
                self.hits += 1;
                Some(value)
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
        if self.lru_hash_map.len() == 0 {
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

        while (self.used_bytes >= requested_capacity) && (self.lru_hash_map.len() > 0) {
            self.evict();
        }
    }

    pub fn evict_all(&mut self) {
        debug!("EVICT ALL: count={}", self.lru_hash_map.len());
        // Keep evicting until all entries are removed from the cache.
        while self.lru_hash_map.len() > 0 {
            self.evict();
        }
    }

    pub fn evict(&mut self) -> bool {
        let mut success = false;
        // We assume 'self.evict()' will always update
        // 'self.used_bytes' each time it runs and the used_bytes will
        // always decrease after each call.
        //
        // Identify least recently used key, and remove the key.
        if let Some((_key, value)) = self.lru_hash_map.pop_front() {
            let value_bytes = value.size_bytes();
            // There should not be any overflow problems with this
            // subtraction since we add and subtract exactly goes
            // into the cache. If there is an overflow here, it
            // means we must have skipped counting a cache entry's
            // size somewhere.
            self.used_bytes -= value_bytes;

            success = true;
            self.evictions += 1;
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
            self.lru_hash_map.len()
        );
        string
    }
}

pub fn create_cache_box_with_capacity(capacity_bytes: usize) -> Box<CacheImpl> {
    debug!("create_cache_box_with_capacity()");
    Box::new(CacheImpl::with_capacity(capacity_bytes))
}
