use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::BBox2D;
use crate::hashutils;

impl BBox2D {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> BBox2D {
        BBox2D {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

impl Hash for BBox2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hashutils::HashableF32::new(self.min_x).hash(state);
        hashutils::HashableF32::new(self.min_y).hash(state);
        hashutils::HashableF32::new(self.max_x).hash(state);
        hashutils::HashableF32::new(self.max_y).hash(state);
    }
}
