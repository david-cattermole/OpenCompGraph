#[allow(unused_imports)]
use petgraph;
use std::hash::{Hash, Hasher};
use std::mem;

pub type Identifier = u64;
pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = u8;
pub type NodeIdx = petgraph::graph::NodeIndex<GraphIdx>;
pub type EdgeIdx = petgraph::graph::EdgeIndex<GraphIdx>;

/// Returns the mantissa, exponent and sign as integers.
fn integer_decode_f32(val: f32) -> (u64, i16, i8) {
    let bits: u32 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0x7fffff) << 1
    } else {
        (bits & 0x7fffff) | 0x800000
    };
    // Exponent bias + mantissa shift
    exponent -= 127 + 23;
    (mantissa as u64, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
struct Distance32((u64, i16, i8));

impl Distance32 {
    fn new(val: f32) -> Distance32 {
        Distance32(integer_decode_f32(val))
    }
}

#[derive(Debug, Clone)]
pub struct PixelBlock {
    pub data: bool,
    pub width: u32,
    pub height: u32,
    pub num_channels: u8,
    pub data_type: u8,
    pub pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(width: u32, height: u32, num_channels: u8) -> PixelBlock {
        let pixels = Vec::<f32>::new();
        let data = true;
        let data_type = 0;
        PixelBlock {
            data,
            width,
            height,
            num_channels,
            data_type,
            pixels,
        }
    }
}

impl Hash for PixelBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.num_channels.hash(state);
        self.data_type.hash(state);
        // Note: Skipping the 'pixels' data.
    }
}

#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct Matrix4 {
    pub data: u32,
}

impl Matrix4 {
    pub fn new() -> Matrix4 {
        Matrix4 { data: 0 }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct BoundingBox2D {
    pub ax: f32,
    pub ay: f32,
    pub bx: f32,
    pub by: f32,
}

impl BoundingBox2D {
    pub fn new() -> BoundingBox2D {
        BoundingBox2D {
            ax: 0.0,
            ay: 0.0,
            bx: 0.0,
            by: 0.0,
        }
    }
}

impl Hash for BoundingBox2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Distance32::new(self.ax).hash(state);
        Distance32::new(self.ay).hash(state);
        Distance32::new(self.bx).hash(state);
        Distance32::new(self.by).hash(state);
    }
}
