use cxx::{CxxString, UniquePtr};
use log::{debug, error, info, warn};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rustc_hash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::cache::CacheImpl;
use crate::data::BoundingBox2D;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::Matrix4;
use crate::data::PixelBlock;
use crate::graph::GraphImpl;
use crate::logger::initialize;
use crate::node::create_node;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

#[rustfmt::skip]
#[cxx::bridge(namespace = "open_comp_graph")]
pub mod ffi {
    #[namespace = "open_comp_graph::shared"]
    struct SharedThing {
        z: i32,
        y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct GraphImplShared {
        inner: Box<GraphImpl>,
    }

    #[derive(Debug, Hash, Clone)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct StreamDataImplShared {
        inner: Box<StreamDataImpl>,
    }
    impl Vec<StreamDataImplShared> {}

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct CacheImplShared {
        inner: Box<CacheImpl>,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum ExecuteStatus {
        #[cxx_name = "kError"]
        Error = 0,
        #[cxx_name = "kSuccess"]
        Success = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum NodeType {
        // Creation / Input / Output
        #[cxx_name = "kNull"]
        Null = 0,
        #[cxx_name = "kReadImage"]
        ReadImage = 1,
        #[cxx_name = "kWriteImage"]
        WriteImage = 2,
        // ColorBars,
        // Constant,

        // // Pattern
        // Checkerboard,
        // Noise,

        // // Time????
        // FrameHold,

        // // Transform
        // TransformImage,
        // CropImage,
        // ReformatImage,

        // // Merges
        // MergeImage,

        // // Color
        // ColorCorrect,
        #[cxx_name = "kGrade"]
        Grade = 3,

        // // Deform / Warp
        // LensDistort,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum NodeStatus {
        #[cxx_name = "kError"]
        Error = 0,
        #[cxx_name = "kValid"]
        Valid = 1,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 2,
        // Warning = 2,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum AttrState {
        #[cxx_name = "kMissing"]
        Missing = 0,
        #[cxx_name = "kExists"]
        Exists = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum StreamDataState {
        #[cxx_name = "kInvalid"]
        Invalid = 0,
        #[cxx_name = "kValid"]
        Valid = 1,
    }

    // ThingC
    #[namespace = "open_comp_graph::cpp"]
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("opencompgraph/symbol_export.h");
        include!("opencompgraph/cpp.h");

        type ThingC;
        fn make_thingc(appname: &str) -> UniquePtr<ThingC>;
        fn get_name(thing: &ThingC) -> &CxxString;
        fn run_sharedthing(state: SharedThing);
    }

    // ThingR
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type ThingR;
        fn print_r(r: &ThingR);
    }

    // PixelBlock
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type PixelBlock;
    }

    // BoundingBox2D
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type BoundingBox2D;
    }

    // Matrix4
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type Matrix4;
    }

    // StreamData
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type StreamDataImpl;
        fn get_state(&self) -> StreamDataState;
        fn get_state_id(&self) -> u8;
        fn get_hash(&self) -> u64;
        fn get_bounding_box(&self) -> &Box<BoundingBox2D>;
        fn get_color_matrix(&self) -> &Box<Matrix4>;
        fn get_transform_matrix(&self) -> &Box<Matrix4>;
        fn get_pixel_block(&self) -> &PixelBlock;
        fn get_pixel_buffer(&self) -> &[f32];
        fn get_pixel_width(&self) -> u32;
        fn get_pixel_height(&self) -> u32;
        fn get_pixel_num_channels(&self) -> u8;
    }

    // Node
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type NodeImpl;
        fn get_id(&self) -> u64;
        fn get_node_type(&self) -> NodeType;
        fn get_node_type_id(&self) -> u8;
        fn get_status(&self) -> NodeStatus;
        fn get_status_id(&self) -> u8;

        // Compute
        fn hash(&self, inputs: &Vec<StreamDataImplShared>) -> u64;
        fn compute(&mut self, inputs: &Vec<StreamDataImplShared>, output: &mut StreamDataImplShared) -> NodeStatus;

        // AttrBlock
        fn attr_exists(&self, name: &str) -> AttrState;
        fn get_attr_f32(&self, name: &str) -> f32;
        fn get_attr_i32(&self, name: &str) -> i32;
        unsafe fn get_attr_str<'a, 'b>(&'b self, name: &'a str) -> &'b str;
        fn set_attr_f32(&mut self, name: &str, value: f32);
        fn set_attr_i32(&mut self, name: &str, value: i32);
        fn set_attr_str(&mut self, name: &str, value: &str);
    }

    // Cache
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type CacheImpl;
        fn get_id(&self) -> u64;
        fn len(&self) -> usize;
    }

    // Graph
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type GraphImpl;
        fn add_node(&mut self, op_box: Box<NodeImpl>) -> usize;
        fn remove_node(&mut self, node_id: u64) -> bool;

        fn node_attr_exists(&self, node_id: u64, name: &str) -> AttrState;
        fn get_node_attr_f32(&self, node_id: u64, name: &str) -> f32;
        fn get_node_attr_i32(&self, node_id: u64, name: &str) -> i32;
        unsafe fn get_node_attr_str<'a, 'b>(&'b self, node_id: u64, name: &'a str) -> &'b str;
        fn set_node_attr_f32(&mut self, node_id: u64, name: &str, value: f32);
        fn set_node_attr_i32(&mut self, node_id: u64, name: &str, value: i32);
        fn set_node_attr_str(&mut self, node_id: u64, name: &str, value: &str);

        fn node_exists(&mut self, node_id: u64) -> bool;
        // fn find_node(&mut self, node_id: u64) -> &Box<NodeImpl>;
        fn connect(&mut self, src_node_id: u64, dst_node_id: u64, input_num: u8);
        fn execute(&mut self, node_id: u64, cache: &mut Box<CacheImpl>) -> ExecuteStatus;
        fn output_stream(&self) -> StreamDataImplShared;
    }

    // Struct Creation
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        #[cxx_name = "create_node_box"]
        fn create_node_box_with_id(node_type: NodeType, id: u64) -> Box<NodeImpl>;

        fn create_cache_box() -> Box<CacheImpl>;
        fn create_cache_shared() -> CacheImplShared;

        fn create_graph_box() -> Box<GraphImpl>;
        fn create_graph_shared() -> GraphImplShared;

        fn create_stream_data_box() -> Box<StreamDataImpl>;
        fn create_stream_data_shared() -> StreamDataImplShared;
        fn create_stream_data_shared_box(data: Box<StreamDataImpl>) -> StreamDataImplShared;
        fn create_vec_stream_data_shared() -> Vec<StreamDataImplShared>;

        fn generate_random_id() -> u64;
        fn generate_id_from_name(name: &str) -> u64;
    }

    // Logging
    #[namespace = "open_comp_graph::log"]
    extern "Rust" {
        fn initialize() -> bool;
    }
}

pub struct ThingR(usize);

fn print_r(r: &ThingR) {
    println!("called back with r={}", r.0);
}

#[allow(dead_code)]
fn my_test() {
    let x = ffi::make_thingc("demo of cxx::bridge");
    println!("this is a \"{}\"", ffi::get_name(x.as_ref().unwrap()));
    ffi::run_sharedthing(ffi::SharedThing {
        z: 222,
        y: Box::new(ThingR(333)),
        x,
    });
}

pub fn create_node_box_with_id(node_type: ffi::NodeType, id: Identifier) -> Box<NodeImpl> {
    debug!("create_node_box(node_type={:?}, id={:?})", node_type, id);
    Box::new(create_node(node_type, id))
}

fn create_cache_shared() -> ffi::CacheImplShared {
    debug!("create_cache_shared()");
    ffi::CacheImplShared {
        inner: create_cache_box(),
    }
}

pub fn create_cache_box() -> Box<CacheImpl> {
    debug!("create_cache_box()");
    Box::new(CacheImpl::new())
}

fn create_graph_shared() -> ffi::GraphImplShared {
    debug!("create_graph_shared()");
    ffi::GraphImplShared {
        inner: create_graph_box(),
    }
}

fn create_graph_box() -> Box<GraphImpl> {
    debug!("create_graph_box()");
    Box::new(GraphImpl::new())
}

pub fn create_vec_stream_data_shared() -> Vec<ffi::StreamDataImplShared> {
    debug!("create_stream_data_shared()");
    Vec::<ffi::StreamDataImplShared>::new()
}

pub fn create_stream_data_shared() -> ffi::StreamDataImplShared {
    debug!("create_stream_data_shared()");
    ffi::StreamDataImplShared {
        inner: create_stream_data_box(),
    }
}

pub fn create_stream_data_shared_box(data: Box<StreamDataImpl>) -> ffi::StreamDataImplShared {
    debug!("create_stream_data_shared_box()");
    ffi::StreamDataImplShared { inner: data }
}

pub fn create_stream_data_box() -> Box<StreamDataImpl> {
    debug!("create_stream_data_box()");
    Box::new(StreamDataImpl::new())
}

fn calculate_hash<T: Hash>(t: &T) -> HashValue {
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}

fn generate_id_from_name(name: &str) -> HashValue {
    calculate_hash::<&str>(&name)
}

fn generate_random_id() -> HashValue {
    // Create small, cheap to initialize and fast RNG with a random seed.
    // The randomness is supplied by the operating system.
    let mut rng = SmallRng::from_entropy();
    rng.gen()
}
