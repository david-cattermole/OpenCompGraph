// OCG Configuration.
//
// OCG configuration has multiple levels with flexibility for a
// pipeline in mind.
//

use serde::Deserialize;
use std::env;
use std::fs;

use crate::cxxbridge::ffi::get_total_system_memory_as_bytes;
use crate::cxxbridge::ffi::ConfigImplShared;

#[derive(Debug, PartialEq, Deserialize)]
struct ConfigCache {
    /// How much System RAM to use for the cache? (This does not
    /// account for or use Swap/Page memory.)
    ram_capacity_percent: f32,
}

impl ConfigCache {
    pub fn ram_capacity_bytes(&self) -> usize {
        let ram_bytes = get_total_system_memory_as_bytes();
        let ram_factor = self.ram_capacity_percent();
        let capacity_bytes = (ram_bytes as f32 * ram_factor) as usize;
        capacity_bytes
    }

    pub fn ram_capacity_percent(&self) -> f32 {
        self.ram_capacity_percent
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigImpl {
    cache: ConfigCache,
}

fn load_config(file_path: &str) -> Result<ConfigImpl, serde_yaml::Error> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file!");
    let deserialized: ConfigImpl = serde_yaml::from_str(&contents)?;
    Ok(deserialized)
}

impl ConfigImpl {
    pub fn cache_ram_capacity_bytes(&self) -> usize {
        self.cache.ram_capacity_bytes()
    }

    pub fn cache_ram_capacity_percent(&self) -> f32 {
        self.cache.ram_capacity_percent()
    }
}

pub fn get_config() -> ConfigImplShared {
    let config = load_config("C:\\OpenCompGraph\\config\\open_comp_graph.yaml")
        .expect("Config file should exist.");
    ConfigImplShared {
        inner: Box::new(config),
    }
}
