use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::cxxbridge::ffi::AttrState;

pub trait AttrBlock: std::fmt::Debug {
    fn attr_hash(&self, state: &mut DefaultHasher);

    fn attr_exists(&self, name: &str) -> AttrState;

    fn get_attr_str(&self, name: &str) -> &str;
    fn set_attr_str(&mut self, name: &str, value: &str);

    fn get_attr_i32(&self, name: &str) -> i32;
    fn set_attr_i32(&mut self, name: &str, value: i32);

    fn get_attr_f32(&self, name: &str) -> f32;
    fn set_attr_f32(&mut self, name: &str, value: f32);
}
