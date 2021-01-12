use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::AttrState;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::hashutils::HashableF32;

pub mod brownian;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash)]
pub enum DeformerType {
    Null = 0,
    Brownian = 1,
    Unknown = 255,
}

#[derive(Debug, Clone, Default)]
pub struct DeformerAttrs {
    pub enable: i32,
    pub k1: f32,
    pub k2: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl hash::Hash for DeformerAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        HashableF32::new(self.k1).hash(state);
        HashableF32::new(self.k2).hash(state);
    }
}

impl DeformerAttrs {
    pub fn new() -> DeformerAttrs {
        DeformerAttrs {
            enable: 1,
            k1: 0.0,
            k2: 0.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
}

impl AttrBlock for DeformerAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "k1" => AttrState::Exists,
            "k2" => AttrState::Exists,
            "center_x" => AttrState::Exists,
            "center_y" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        ()
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "k1" => self.k1,
            "k2" => self.k2,
            "center_x" => self.center_x,
            "center_y" => self.center_y,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "k1" => self.k1 = value,
            "k2" => self.k2 = value,
            "center_x" => self.center_x = value,
            "center_y" => self.center_y = value,
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Deformer {
    deformer_type: DeformerType,
    attr_block: DeformerAttrs,
}

impl Deformer {
    pub fn new(deformer_type: DeformerType) -> Deformer {
        Deformer {
            deformer_type,
            attr_block: DeformerAttrs::new(),
        }
    }

    pub fn deformer_type(&self) -> DeformerType {
        self.deformer_type
    }

    pub fn attr_exists(&self, name: &str) -> AttrState {
        self.attr_block.attr_exists(name)
    }

    pub fn get_attr_str(&self, name: &str) -> &str {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_str(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                ""
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_str(&mut self, name: &str, value: &str) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_str(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_i32(&self, name: &str) -> i32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_i32(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_i32(&mut self, name: &str, value: i32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_i32(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_f32(&self, name: &str) -> f32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_f32(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                0.0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_f32(&mut self, name: &str, value: f32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_f32(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn data_debug_string(&self) -> String {
        format!("{:?}", self.attr_block)
    }
}

pub fn create_deformer_box(deformer_type: DeformerType) -> Box<Deformer> {
    debug!("create_deformer_box(deformer_type={:?})", deformer_type);
    Box::new(Deformer::new(deformer_type))
}
