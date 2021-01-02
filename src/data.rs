use image;
use petgraph;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = u8;
pub type NodeIdx = petgraph::graph::NodeIndex<GraphIdx>;
pub type EdgeIdx = petgraph::graph::EdgeIndex<GraphIdx>;
pub type HashValue = u64;
pub type Identifier = u64;

// https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
fn integer_decode_f64(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

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
struct Distance64((u64, i16, i8));

impl Distance64 {
    fn new(val: f64) -> Distance64 {
        Distance64(integer_decode_f64(val))
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Distance32((u64, i16, i8));

impl Distance32 {
    pub fn new(val: f32) -> Distance32 {
        Distance32(integer_decode_f32(val))
    }
}

#[derive(Clone)]
pub struct PixelBlock {
    pub width: u32,
    pub height: u32,
    pub num_channels: u8,
    pub pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(width: u32, height: u32, num_channels: u8) -> PixelBlock {
        let size = (width * height * num_channels as u32) as usize;
        let pixels = vec![0.5 as f32; size];
        PixelBlock {
            width,
            height,
            num_channels,
            pixels,
        }
    }

    pub fn set_pixels(&mut self, pixels: Vec<f32>) {
        self.pixels = pixels;
    }
}

impl Hash for PixelBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.num_channels.hash(state);
        // Note: Skipping the 'pixels' data.
    }
}

impl fmt::Debug for PixelBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelBlock")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("num_channels", &self.num_channels)
            .finish()
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
