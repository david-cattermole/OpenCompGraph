use log::{debug, error, info, warn};
use std::rc::Rc;

use crate::cache::create_cache_box_with_capacity;
use crate::cache::CacheImpl;
use crate::config::get_config;
use crate::config::ConfigImpl;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::geom::export_mesh;
use crate::geom::plane::create_geometry_plane_box;
use crate::geom::plane::GeometryPlaneImpl;
use crate::graph::create_graph_box;
use crate::graph::GraphImpl;
use crate::hashutils::calculate_hash;
use crate::hashutils::generate_id_from_name;
use crate::hashutils::generate_random_id;
use crate::logger::initialize;
use crate::node::create_node;
use crate::node::create_node_box_with_id;
use crate::node::NodeImpl;
use crate::pixelblock::PixelBlock;
use crate::stream::create_stream_data_box;
use crate::stream::create_stream_data_box_rc;
use crate::stream::StreamDataImpl;
use crate::stream::StreamDataImplRc;

#[rustfmt::skip]
#[cxx::bridge(namespace = "open_comp_graph")]
pub mod ffi {
    // C++ includes needed for all files.
    #[namespace = "open_comp_graph"]
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("opencompgraph/symbol_export.h");
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
        inner: Box<StreamDataImplRc>,
    }
    impl Vec<StreamDataImplShared> {}

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct CacheImplShared {
        inner: Box<CacheImpl>,
    }

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct ConfigImplShared {
        inner: Box<ConfigImpl>,
    }

    #[derive(Debug)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) struct ImageShared {
        pixel_block: Box<PixelBlock>,
        display_window: BBox2Di,
        data_window: BBox2Di,
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

        // Transform
        #[cxx_name = "kTransform"]
        Transform = 5,
        // CropImage,
        // ReformatImage,

        // // Merges
        // MergeImage,

        // // Color
        // ColorCorrect,
        #[cxx_name = "kGrade"]
        Grade = 3,

        // Deform / Warp
        #[cxx_name = "kLensDistort"]
        LensDistort = 4,
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
        #[cxx_name = "kWarning"]
        Warning = 2,
        #[cxx_name = "kNonExistent"]
        NonExistent = 3,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
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
        include!("opencompgraph/cpp.h");

        // type ThingC;
        // fn make_thingc(appname: &str) -> UniquePtr<ThingC>;
        // fn get_name(thing: &ThingC) -> &CxxString;
        // fn run_sharedthing(state: SharedThing);
    }

    // Image IO
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/imageio.h");

        fn oiio_read_image(file_path: &String, image: &mut ImageShared);
        fn oiio_write_image(file_path: &String, image: &ImageShared) -> bool;
    }

    // System Memory Utilities
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/systemmemory.h");
        fn get_total_system_memory_as_bytes() -> usize;
    }


    // PixelBlock
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type PixelBlock;

        fn width(&self) -> i32;
        fn height(&self) -> i32;
        fn num_channels(&self) -> i32;
        fn pixel_data_type(&self) -> PixelDataType;

        fn as_slice(&self) -> &[f32];
        fn as_slice_mut(&mut self) -> &mut [f32];

        fn data_resize(
            &mut self,
            width: i32,
            height: i32,
            num_channels: i32,
            pixel_data_type: PixelDataType);
    }

    // StreamData (Rc)
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type StreamDataImplRc;

        fn state(&self) -> StreamDataState;
        fn state_id(&self) -> u8;
        fn hash(&self) -> u64;
        fn display_window(&self) -> BBox2Di;
        fn data_window(&self) -> BBox2Di;
        fn color_matrix(&self) -> Matrix4;
        fn transform_matrix(&self) -> Matrix4;
        fn deformers_len(&self) -> usize;
        fn apply_deformers(&self, buffer: &mut [f32]);
        fn pixel_buffer(&self) -> &[f32];
        fn pixel_width(&self) -> i32;
        fn pixel_height(&self) -> i32;
        fn pixel_num_channels(&self) -> i32;

        // Creation
        fn create_stream_data_box_rc() -> Box<StreamDataImplRc>;
    }


    // StreamData
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type StreamDataImpl;

        // Creation
        fn create_stream_data_box() -> Box<StreamDataImpl>;
        fn create_stream_data_shared() -> StreamDataImplShared;
        fn create_stream_data_shared_box(data: Box<StreamDataImplRc>) -> StreamDataImplShared;
        fn create_vec_stream_data_shared() -> Vec<StreamDataImplShared>;
    }

    // Node
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type NodeImpl;

        // Creation
        #[cxx_name = "create_node_box"]
        fn create_node_box_with_id(node_type: NodeType, id: u64) -> Box<NodeImpl>;
    }

    // Cache
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type CacheImpl;
        fn len(&self) -> usize;
        fn used_bytes(&self) -> usize;
        fn capacity_bytes(&self) -> usize;
        fn set_capacity_bytes(&mut self, value: usize);
        fn data_debug_string(&self) -> String;

        // Creation
        fn create_cache_box_with_capacity(capacity_bytes: usize) -> Box<CacheImpl>;
        fn create_cache_shared_with_capacity(capacity_bytes: usize) -> CacheImplShared;
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
        fn execute(
            &mut self,
            node_id: u64,
            frames: &[i32],
            cache: &mut Box<CacheImpl>) -> ExecuteStatus;
        fn data_debug_string(&self) -> String;
        fn output_stream(&self) -> StreamDataImplShared;

        // Creation
        fn create_graph_box() -> Box<GraphImpl>;
        fn create_graph_shared() -> GraphImplShared;
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
        fn calc_buffer_size_index_border_lines(&self) -> usize;
        fn calc_buffer_size_index_wire_lines(&self) -> usize;

        fn fill_buffer_vertex_positions(&self, buffer_vertex_positions: &mut [f32]) -> bool;
        fn fill_buffer_vertex_uvs(&self, buffer_vertex_uvs: &mut [f32]) -> bool;
        fn fill_buffer_index_tris(&self, buffer_index_tris: &mut [u32]) -> bool;
        fn fill_buffer_index_border_lines(&self, buffer_index_border_lines: &mut [u32]) -> bool;
        fn fill_buffer_index_wire_lines(&self, buffer_index_wire_lines: &mut [u32]) -> bool;

        // Creation
        fn create_geometry_plane_box(
            divisions_x: u32,
            divisions_y: u32
        ) -> Box<GeometryPlaneImpl>;
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

    // Configuration
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type ConfigImpl;
        fn cache_ram_capacity_bytes(&self) -> usize;
        fn cache_ram_capacity_percent(&self) -> f32;
        fn data_debug_string(&self) -> String;

        fn get_config(file_name: &str) -> ConfigImplShared;
    }

    // Hashing Utilities
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        fn generate_random_id() -> u64;
        fn generate_id_from_name(name: &str) -> u64;
    }
}

fn create_cache_shared_with_capacity(capacity_bytes: usize) -> ffi::CacheImplShared {
    debug!("create_cache_shared_with_capacity()");
    ffi::CacheImplShared {
        inner: create_cache_box_with_capacity(capacity_bytes),
    }
}

fn create_graph_shared() -> ffi::GraphImplShared {
    debug!("create_graph_shared()");
    ffi::GraphImplShared {
        inner: create_graph_box(),
    }
}

pub fn create_vec_stream_data_shared() -> Vec<ffi::StreamDataImplShared> {
    debug!("create_stream_data_shared()");
    Vec::<ffi::StreamDataImplShared>::new()
}

pub fn create_stream_data_shared() -> ffi::StreamDataImplShared {
    debug!("create_stream_data_shared()");
    ffi::StreamDataImplShared {
        inner: create_stream_data_box_rc(),
    }
}

pub fn create_stream_data_shared_box(data: Box<StreamDataImplRc>) -> ffi::StreamDataImplShared {
    debug!("create_stream_data_shared_box()");
    ffi::StreamDataImplShared { inner: data }
}
