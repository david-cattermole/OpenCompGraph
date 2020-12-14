#[allow(unused_imports)]
use petgraph;
use std::hash::{Hash, Hasher};
use std::mem;

pub type Identifier = u64;
pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = ();
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

#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct PixelBlock {
    pub data: bool,
    pub width: u32,
    pub height: u32,
    pub num_channels: u8,
    pub data_size: u8,
    // pub pixels: Vec<f32>,
}

#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct Matrix4 {
    pub data: u32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct BoundingBox2D {
    pub ax: f32,
    pub ay: f32,
    pub bx: f32,
    pub by: f32,
}

impl Hash for BoundingBox2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Distance32::new(self.ax).hash(state);
        Distance32::new(self.ay).hash(state);
        Distance32::new(self.bx).hash(state);
        Distance32::new(self.by).hash(state);
    }
}
