use cxx::{CxxString, UniquePtr};
use log::{debug, error, info, warn};

use crate::cache::CacheImpl;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::geom::plane::export_mesh;
use crate::geom::plane::GeometryPlaneImpl;
use crate::graph::GraphImpl;
use crate::hashutils::calculate_hash;
use crate::hashutils::generate_id_from_name;
use crate::hashutils::generate_random_id;
use crate::logger::initialize;
use crate::node::create_node;
use crate::node::NodeImpl;
use crate::pixelblock::PixelBlock;
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

    #[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
    #[namespace = "open_comp_graph"]
    struct BBox2Df {
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
    }

    #[derive(Debug, Copy, Clone, Default, Hash, PartialEq, PartialOrd)]
    #[namespace = "open_comp_graph"]
    struct BBox2Di {
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    }

    #[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
    #[namespace = "open_comp_graph"]
    struct Matrix4 {
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        //
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        //
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        //
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
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
    pub(crate) enum GraphState {
        #[cxx_name = "kDirty"]
        Dirty = 0,
        #[cxx_name = "kClean"]
        Clean = 1,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum ExecuteStatus {
        #[cxx_name = "kError"]
        Error = 0,
        #[cxx_name = "kSuccess"]
        Success = 1,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
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

    // Each pixel has this type of data.
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum PixelDataType {
        #[cxx_name = "kFloat32"]
        Float32 = 0,
        #[cxx_name = "kHalf16"]
        Half16 = 1,
        #[cxx_name = "kUInt8"]
        UInt8 = 2,
        #[cxx_name = "kUInt16"]
        UInt16 = 3,
        #[cxx_name = "kUnknown"]
        Unknown = 255,
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
        #[cxx_name = "kNonExistent"]
        NonExistent = 3,
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

    // StreamData
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type StreamDataImpl;
        fn state(&self) -> StreamDataState;
        fn state_id(&self) -> u8;
        fn hash(&self) -> u64;
        fn display_window(&self) -> BBox2Df;
        fn set_display_window(&mut self, value: BBox2Df);
        fn data_window(&self) -> BBox2Df;
        fn set_data_window(&mut self, value: BBox2Df);
        fn color_matrix(&self) -> Matrix4;
        fn transform_matrix(&self) -> Matrix4;
        fn pixel_block(&self) -> &PixelBlock;
        fn pixel_buffer(&self) -> &[f32];
        fn pixel_width(&self) -> u32;
        fn pixel_height(&self) -> u32;
        fn pixel_num_channels(&self) -> u8;
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
        fn len(&self) -> usize;
    }

    // Graph
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type GraphImpl;
        fn state(&self) -> GraphState;
        fn execute_status(&self) -> ExecuteStatus;
        fn add_node(&mut self, op_box: Box<NodeImpl>) -> usize;
        fn remove_node(&mut self, node_id: u64) -> bool;

        fn node_attr_exists(&self, node_id: u64, name: &str) -> AttrState;
        fn node_status(&self, node_id: u64) -> NodeStatus;
        fn get_node_attr_f32(&self, node_id: u64, name: &str) -> f32;
        fn get_node_attr_i32(&self, node_id: u64, name: &str) -> i32;
        unsafe fn get_node_attr_str<'a, 'b>(&'b self, node_id: u64, name: &'a str) -> &'b str;
        fn set_node_attr_f32(&mut self, node_id: u64, name: &str, value: f32);
        fn set_node_attr_i32(&mut self, node_id: u64, name: &str, value: i32);
        fn set_node_attr_str(&mut self, node_id: u64, name: &str, value: &str);

        fn node_exists(&mut self, node_id: u64) -> bool;
        fn connect(&mut self, src_node_id: u64, dst_node_id: u64, input_num: u8);
        fn execute(&mut self, node_id: u64, cache: &mut Box<CacheImpl>) -> ExecuteStatus;
        fn data_debug_string(&self) -> String;
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

        fn create_geometry_plane_box(
            divisions_x: u32,
            divisions_y: u32
        ) -> Box<GeometryPlaneImpl>;

        fn generate_random_id() -> u64;
        fn generate_id_from_name(name: &str) -> u64;
    }

    // Geometry Plane
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type GeometryPlaneImpl;
        fn divisions_x(&self) -> u32;
        fn divisions_y(&self) -> u32;
        fn set_divisions_x(&mut self, value: u32);
        fn set_divisions_y(&mut self, value: u32);

        fn calc_count_vertex_positions(&self) -> usize;
        fn calc_count_vertex_uvs(&self) -> usize;

        fn calc_buffer_size_vertex_positions(&self) -> usize;
        fn calc_buffer_size_vertex_uvs(&self) -> usize;
        fn calc_buffer_size_index_tris(&self) -> usize;

        fn fill_buffer_vertex_positions(
            &self,
            buffer_vertex_positions: &mut [f32],
        ) -> bool;
        fn fill_buffer_vertex_uvs(
            &self,
            buffer_vertex_uvs: &mut [f32],
        ) -> bool;
        fn fill_buffer_index_tris(
            &self,
            buffer_index_tris: &mut [u32],
        ) -> bool;
    }

    // Geometry
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        fn export_mesh(
            buffer_vertex_positions: &[f32],
            buffer_vertex_uvs: &[f32],
            buffer_index_tris: &[u32],
            file_path: &str,
        );
    }

    // Logging
    //
    // TODO: Add more logging functions for C++ to use.
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

pub fn create_geometry_plane_box(divisions_x: u32, divisions_y: u32) -> Box<GeometryPlaneImpl> {
    debug!("create_geometry_plane_box()");
    Box::new(GeometryPlaneImpl::new(divisions_x, divisions_y))
}
