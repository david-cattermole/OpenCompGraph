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

/// OCG Configuration.
///
/// OCG configuration has multiple levels with flexibility for a
/// pipeline in mind.
///
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::default::Default;
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

impl Default for ConfigCache {
    fn default() -> Self {
        ConfigCache {
            ram_capacity_percent: 0.0,
        }
    }
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

#[derive(Debug, Default, PartialEq, Deserialize)]
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

    /// Convert the graph into a human-readable string, for debug
    /// printing.
    pub fn data_debug_string(&self) -> String {
        debug!("Config Debug");
        let string = format!(
            "cache_ram_capacity_percent={} cache_ram_capacity_bytes={}",
            self.cache_ram_capacity_percent(),
            self.cache_ram_capacity_bytes(),
        );
        string
    }
}

/// Get a valid file path for the file name and environment variable.
fn get_config_path(file_name: &str, envvar_name: &str) -> String {
    // let envvar_name = var_name;
    let mut config_path = String::new();
    match env::var_os(envvar_name) {
        Some(paths) => {
            for split_path_buf in env::split_paths(&paths) {
                // println!("'{}'", split_path_buf.display());
                let mut path_buf = split_path_buf.clone();
                path_buf.push(file_name);
                let path = path_buf.as_path();
                if path.is_file() {
                    match path.to_str() {
                        Some(v) => {
                            config_path = v.to_string();
                            break;
                        }
                        None => println!("{} is not defined in the environment.", envvar_name),
                    }
                }
            }
        }
        None => println!("{} is not defined in the environment.", envvar_name),
    }
    config_path
}

pub fn get_config(file_name: &str) -> ConfigImplShared {
    let envvar_name = "OPENCOMPGRAPH_CONFIG_PATH";
    let path = get_config_path(&file_name, envvar_name);
    let config = match path.as_str() {
        "" => ConfigImpl::default(),
        _ => load_config(&path).expect("Config file should exist."),
    };
    ConfigImplShared {
        inner: Box::new(config),
    }
}
