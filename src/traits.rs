use crate::cxxbridge::ffi::AttrState;

pub trait AttrBlock {
    fn attr_exists(&self, name: &str) -> AttrState;
    fn get_attr_string(&self, name: &str) -> &str;
    fn set_attr_string(&mut self, name: &str, value: &str);
}

pub trait Compute {
    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>) -> Result<bool, &'static str>;
}
