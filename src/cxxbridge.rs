use cxx::{CxxString, UniquePtr};
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

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct NodeImplShared {
        inner: Box<NodeImpl>,
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
        Error = 0,
        Success = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum NodeType {
        // Creation / Input / Output
        Null = 0,
        ReadImage = 1,
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
        Grade = 3,

        // // Deform / Warp
        // LensDistort,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum NodeStatus {
        Error = 0,
        Valid = 1,
        Uninitialized = 2,
        // Warning = 2,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum AttrState {
        Missing = 0,
        Exists = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum StreamDataState {
        Invalid = 0,
        Valid = 1,
    }

    // ThingC
    #[namespace = "open_comp_graph::cpp"]
    unsafe extern "C++" {
        include!("rust/cxx.h");
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
        fn get_hash(&self) -> u64;
        fn get_bounding_box(&self) -> &Box<BoundingBox2D>;
        fn get_color_matrix(&self) -> &Box<Matrix4>;
        fn get_transform_matrix(&self) -> &Box<Matrix4>;
        fn get_pixel_block(&self) -> &PixelBlock;
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
        fn connect(&mut self, src_index: usize, dst_index: usize, input_num: u8);
        fn execute(&mut self, start_index: usize, cache: &mut Box<CacheImpl>) -> ExecuteStatus;
    }

    // Struct Creation
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        #[cxx_name = "create_node_box"]
        fn create_node_box(node_type: NodeType) -> Box<NodeImpl>;
        #[cxx_name = "create_node_box"]
        fn create_node_box_with_name(node_type: NodeType, name: &str) -> Box<NodeImpl>;
        #[cxx_name = "create_node_box"]
        fn create_node_box_with_id(node_type: NodeType, id: u64) -> Box<NodeImpl>;

        #[cxx_name = "create_node_shared"]
        fn create_node_shared(node_type: NodeType) -> NodeImplShared;
        #[cxx_name = "create_node_shared"]
        fn create_node_shared_with_name(node_type: NodeType, name: &str) -> NodeImplShared;
        #[cxx_name = "create_node_shared"]
        fn create_node_shared_with_id(node_type: NodeType, id: u64) -> NodeImplShared;

        fn create_cache_box() -> Box<CacheImpl>;
        fn create_cache_shared() -> CacheImplShared;

        fn create_graph_box() -> Box<GraphImpl>;
        fn create_graph_shared() -> GraphImplShared;

        fn create_stream_data_box() -> Box<StreamDataImpl>;
        fn create_stream_data_shared() -> StreamDataImplShared;
        fn create_vec_stream_data_shared() -> Vec<StreamDataImplShared>;
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

pub fn create_node_box(node_type: ffi::NodeType) -> Box<NodeImpl> {
    let id = generate_random_id();
    create_node_box_with_id(node_type, id)
}

pub fn create_node_box_with_name(node_type: ffi::NodeType, name: &str) -> Box<NodeImpl> {
    let id = generate_id_from_name(name);
    create_node_box_with_id(node_type, id)
}

pub fn create_node_box_with_id(node_type: ffi::NodeType, id: Identifier) -> Box<NodeImpl> {
    // println!("create_node_box(node_type={:?}, id={:?})", node_type, id);
    Box::new(create_node(node_type, id))
}

fn create_node_shared(node_type: ffi::NodeType) -> ffi::NodeImplShared {
    let id = generate_random_id();
    create_node_shared_with_id(node_type, id)
}

fn create_node_shared_with_name(node_type: ffi::NodeType, name: &str) -> ffi::NodeImplShared {
    let id = generate_id_from_name(name);
    create_node_shared_with_id(node_type, id)
}

fn create_node_shared_with_id(node_type: ffi::NodeType, id: Identifier) -> ffi::NodeImplShared {
    // println!("create_node_shared(node_type={:?}, id={:?})", node_type, id,);
    ffi::NodeImplShared {
        inner: create_node_box_with_id(node_type, id),
    }
}

fn create_cache_shared() -> ffi::CacheImplShared {
    // println!("create_cache_shared()");
    ffi::CacheImplShared {
        inner: create_cache_box(),
    }
}

pub fn create_cache_box() -> Box<CacheImpl> {
    // println!("create_cache_box()");
    Box::new(CacheImpl::new())
}

fn create_graph_shared() -> ffi::GraphImplShared {
    // println!("create_graph_shared()");
    ffi::GraphImplShared {
        inner: create_graph_box(),
    }
}

fn create_graph_box() -> Box<GraphImpl> {
    // println!("create_graph_box()");
    Box::new(GraphImpl::new())
}

pub fn create_vec_stream_data_shared() -> Vec<ffi::StreamDataImplShared> {
    // println!("create_stream_data_shared()");
    Vec::<ffi::StreamDataImplShared>::new()
}

pub fn create_stream_data_shared() -> ffi::StreamDataImplShared {
    // println!("create_stream_data_shared()");
    ffi::StreamDataImplShared {
        inner: create_stream_data_box(),
    }
}

pub fn create_stream_data_box() -> Box<StreamDataImpl> {
    // println!("create_stream_data_box()");
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
