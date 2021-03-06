/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use log::debug;

use crate::cache::create_cache_box_with_capacity;
use crate::cache::CacheImpl;
use crate::config::get_config;
use crate::config::ConfigImpl;
use crate::geom::export_mesh;
use crate::geom::plane::create_geometry_plane_box;
use crate::geom::plane::GeometryPlaneImpl;
use crate::graph::create_graph_box;
use crate::graph::GraphImpl;
use crate::hashutils::generate_id_from_name;
use crate::hashutils::generate_random_id;
use crate::imagemetadata::ImageMetadata;
use crate::logger::initialize;
use crate::node::create_node_box_with_id;
use crate::node::NodeImpl;
use crate::pixelblock::channel_size_bytes;
use crate::pixelblock::stride_num_channels;
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
        metadata: Box<ImageMetadata>,
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
        #[cxx_name = "kCropImage"]
        CropImage = 7,
        // ReformatImage,

        // Merges
        #[cxx_name = "kMergeImage"]
        MergeImage = 6,

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

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum LensDistortDirection {
        #[cxx_name = "kUndistort"]
        Undistort = 0,
        #[cxx_name = "kDistort"]
        Distort = 1,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
    }

    // This matches the LDPK's 'tde4_ldp_ptype' enum values.
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph::internal"]
    pub(crate) enum ParameterType {
        #[cxx_name = "kString"]
        String = 0,
        #[cxx_name = "kDouble"]
        Double = 1,
        #[cxx_name = "kInt"]
        Int = 2,
        #[cxx_name = "kFile"]
        File = 3,
        #[cxx_name = "kBoolean"]
        Boolean = 4,
        #[cxx_name = "kAdjustableDouble"]
        AdjustableDouble = 5,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum MergeImageMode {
        // NOTE: Keep these indexes in-line with the "From" trait
        // below.
        #[cxx_name = "kAdd"]
        Add = 0,
        #[cxx_name = "kOver"]
        Over = 1,
        #[cxx_name = "kMultiply"]
        Multiply = 2,
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
    }

    // Color Spaces
    // The orientation of an image.
    //
    // https://openimageio.readthedocs.io/en/release-2.2.8.0/stdmetadata.html#cmdoption-arg-Orientation
    //
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "open_comp_graph"]
    pub(crate) enum ImageOrientation {
        // NOTE: Keep these indexes in-line with the "From" trait
        // below.
        //
        // 0 = normal (top to bottom, left to right)
        #[cxx_name = "kNormal"]
        Normal = 0,
        //
        // 1 = flipped horizontally (top to botom, right to left)
        #[cxx_name = "kFlippedHorizontally"]
        FlippedHorizontally = 1,
        //
        // 2 = rotated 180 (bottom to top, right to left)
        #[cxx_name = "kRotated180"]
        Rotated180 = 2,
        //
        // 3 = flipped vertically (bottom to top, left to right)
        #[cxx_name = "kFlippedVertically"]
        FlippedVertically = 3,
        //
        // 4 = transposed (left to right, top to bottom)
        #[cxx_name = "kTransposed"]
        Transposed = 4,
        //
        // 5 = rotated 90 clockwise (right to left, top to bottom)
        #[cxx_name = "kRotated90Clockwise"]
        Rotated90Clockwise = 5,
        //
        // 6 = transverse (right to left, bottom to top)
        #[cxx_name = "kTransverse"]
        Transverse = 6,
        //
        // 7 = rotated 90 counter-clockwise (left to right, bottom to top)
        #[cxx_name = "kRotated90CounterClockwise"]
        Rotated90CounterClockwise = 7,
        //
        #[cxx_name = "kUninitialized"]
        Uninitialized = 255,
    }
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/internal/colorspace.h");

        fn ocio_print_color_spaces() -> bool;

        fn ocio_color_convert_inplace(
            pixel_data: &mut [f32],
            width: i32,
            height: i32,
            num_channels: i32,
            src_color_space: &String,
            dst_color_space: &String) -> bool;
    }

    // LDPK and the tde4_ld_plugin class.
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/internal/ldpk_utils.h");

        type OcgLdPluginBase;

        fn get_version_string(self: Pin<&OcgLdPluginBase>) -> &str;

        fn get_model_name(
            self: Pin<&mut OcgLdPluginBase>,
            model_name: &mut [u8]
        ) -> bool;

        fn get_num_parameters(
            self: Pin<&mut OcgLdPluginBase>,
            value: &mut i32,
        ) -> bool;

        fn get_parameter_name(
            self: Pin<&mut OcgLdPluginBase>,
            value: i32,
            identifier: &mut [u8]
        ) -> bool;

        fn get_parameter_type(
            self: Pin<&mut OcgLdPluginBase>,
            identifier: &str,
            value: &mut i32,
        ) -> bool;

        fn get_parameter_default_value_f64(
            self: Pin<&mut OcgLdPluginBase>,
            identifier: &str,
            value: &mut f64,
        ) -> bool;

        fn get_parameter_range(
            self: Pin<&mut OcgLdPluginBase>,
            identifier: &str,
            min_value: &mut f64,
            max_value: &mut f64,
        ) -> bool;

        fn set_parameter_value_f64(
            self: Pin<&mut OcgLdPluginBase>,
            identifier: &str,
            value: f64
        ) -> bool;

        fn initialize_parameters(self: Pin<&mut OcgLdPluginBase>) -> bool;

        fn undistort(
            self: Pin<&mut OcgLdPluginBase>,
            x0: f64, y0: f64,
            x1: &mut f64, y1: &mut f64
        ) -> bool;

        fn distort(
            self: Pin<&mut OcgLdPluginBase>,
            x0: f64, y0: f64,
            x1: &mut f64, y1: &mut f64
        ) -> bool;

        fn distort_with_guess(
            self: Pin<&mut OcgLdPluginBase>,
            x0: f64, y0: f64,
            x1_start: f64, y1_start: f64,
            x1: &mut f64, y1: &mut f64
        ) -> bool;

        fn provides_parameter_derivatives(
            self: Pin<&mut OcgLdPluginBase>) -> bool;

        fn get_bounding_box_undistort(
            self: Pin<&mut OcgLdPluginBase>,
            xa_in: f64, ya_in: f64,
            xb_in: f64, yb_in: f64,
            xa_out: &mut f64, ya_out: &mut f64,
            xb_out: &mut f64, yb_out: &mut f64,
            nx: i32, ny: i32);

        fn get_bounding_box_distort(
            self: Pin<&mut OcgLdPluginBase>,
            xa_in: f64, ya_in: f64,
            xb_in: f64, yb_in: f64,
            xa_out: &mut f64, ya_out: &mut f64,
            xb_out: &mut f64, yb_out: &mut f64,
            nx: i32, ny: i32);

        fn ldpk_new_plugin() -> UniquePtr<OcgLdPluginBase>;
    }

    // Image IO
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/internal/imageio.h");

        fn oiio_get_thread_count(num_threads: &mut i32) -> bool;
        fn oiio_set_thread_count(num_threads: i32) -> bool;
        fn oiio_read_image(file_path: &String, image: &mut ImageShared) -> bool;
        fn oiio_write_image(file_path: &String, image: &ImageShared) -> bool;
    }

    // System Memory Utilities
    #[namespace = "open_comp_graph::internal"]
    unsafe extern "C++" {
        include!("opencompgraph/internal/systemmemory.h");
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

        fn stride_num_channels(num_channels: i32, pixel_data_type: PixelDataType) -> usize;
        fn channel_size_bytes(pixel_data_type: PixelDataType) -> usize;

        fn data_resize(
            &mut self,
            width: i32,
            height: i32,
            num_channels: i32,
            pixel_data_type: PixelDataType);
    }

    // ImageMetadata
    #[namespace = "open_comp_graph::internal"]
    extern "Rust" {
        type ImageMetadata;

        fn color_space(&self) -> String;
        fn set_color_space(&mut self, value: String);

        fn pixel_aspect(&self) -> f32;
        fn set_pixel_aspect(&mut self, value: f32);

        fn orientation(&self) -> ImageOrientation;
        fn set_orientation(&mut self, value: ImageOrientation);

        fn unassociated_alpha(&self) -> bool;
        fn set_unassociated_alpha(&mut self, value: bool);
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
        fn deformers_len(&self) -> usize;
        fn apply_deformers(
            &self,
            buffer: &mut [f32],
            image_window: BBox2Df,
        );
        fn pixel_buffer(&self) -> &[f32];
        fn pixel_width(&self) -> i32;
        fn pixel_height(&self) -> i32;
        fn pixel_num_channels(&self) -> i32;
        fn pixel_data_type(&self) -> PixelDataType;

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
            center_x: f32,
            center_y: f32,
            size_x: f32,
            size_y: f32,
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

impl From<i32> for ffi::MergeImageMode {
    fn from(value: i32) -> Self {
        match value {
            0 => ffi::MergeImageMode::Add,
            1 => ffi::MergeImageMode::Over,
            2 => ffi::MergeImageMode::Multiply,
            _ => ffi::MergeImageMode::Uninitialized,
        }
    }
}

impl From<i32> for ffi::LensDistortDirection {
    fn from(value: i32) -> Self {
        match value {
            0 => ffi::LensDistortDirection::Undistort,
            1 => ffi::LensDistortDirection::Distort,
            _ => ffi::LensDistortDirection::Uninitialized,
        }
    }
}

impl From<i32> for ffi::ImageOrientation {
    fn from(value: i32) -> Self {
        match value {
            0 => ffi::ImageOrientation::Normal,
            1 => ffi::ImageOrientation::FlippedHorizontally,
            2 => ffi::ImageOrientation::Rotated180,
            3 => ffi::ImageOrientation::FlippedVertically,
            4 => ffi::ImageOrientation::Transposed,
            5 => ffi::ImageOrientation::Rotated90Clockwise,
            6 => ffi::ImageOrientation::Transverse,
            7 => ffi::ImageOrientation::Rotated90CounterClockwise,
            _ => ffi::ImageOrientation::Uninitialized,
        }
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
