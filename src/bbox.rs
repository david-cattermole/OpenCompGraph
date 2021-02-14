use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::hashutils;

impl BBox2Df {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> BBox2Df {
        BBox2Df {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

impl Hash for BBox2Df {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hashutils::HashableF32::new(self.min_x).hash(state);
        hashutils::HashableF32::new(self.min_y).hash(state);
        hashutils::HashableF32::new(self.max_x).hash(state);
        hashutils::HashableF32::new(self.max_y).hash(state);
    }
}

impl BBox2Di {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> BBox2Di {
        BBox2Di {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }
}
