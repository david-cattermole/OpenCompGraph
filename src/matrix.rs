use nalgebra;
use std::hash::{Hash, Hasher};

use crate::hashutils;

#[derive(Debug, Copy, Clone, Default)]
pub struct Matrix4 {
    pub data: nalgebra::Matrix4<f32>,
}

impl Matrix4 {
    pub fn new() -> Matrix4 {
        let data = nalgebra::Matrix4::<f32>::identity();
        Matrix4 { data }
    }
}

impl Hash for Matrix4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.data.as_slice().iter() {
            hashutils::HashableF32::new(*i).hash(state);
        }
    }
}
