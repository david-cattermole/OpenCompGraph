use crate::cxxbridge::ffi::AttrState;
use crate::traits::AttrBlock;
use crate::traits::Compute;

#[derive(Debug, Clone, Default)]
pub struct ReadImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub file_path: String,
}

impl Compute for ReadImageCompute {
    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>) -> Result<bool, &'static str> {
        println!("ReadImageCompute.compute()");
        println!("file_name={}", attr_block.get_attr_string("file_path"));
        Ok(true)
    }
}

impl AttrBlock for ReadImageAttrs {
    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "file_path" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_string(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }
}
