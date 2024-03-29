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

use log::{debug, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::CropOnWrite;
use crate::cxxbridge::ffi::DataType;
use crate::cxxbridge::ffi::ExrCompression;
use crate::cxxbridge::ffi::ImageCompression;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::JpegChromaSubSampling;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::data::COLOR_SPACE_NAME_SRGB;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::ops;
use crate::pathutils;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::WriteImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(WriteImageOperation::new()),
        validate: Box::new(WriteImageValidate::new()),
        attr_block: Box::new(WriteImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageOperation {}

// TODO: Add attributes:
//
// - Pre-multiply (?)
// - Output Channels
// - Output colour space.
//
#[derive(Debug, Clone, Default, hash::Hash)]
pub struct WriteImageAttrs {
    pub enable: i32,
    pub execute: i32,
    pub file_path: String,

    // Crop the input image the display window before writing?
    pub crop_on_write: i32, // index for WriteImageCropOnWrite.

    // The pixel data type for pixels written out.
    pub pixel_data_type: i32, // index for DataType.

    // EXR Compression
    pub exr_compression: i32, // index for ExrCompression
    pub exr_dwa_compression_level: i32,

    // PNG Compression
    //
    // Compression level for zip/deflate compression, on a scale from
    // 0 (fastest, minimal compression) to 9 (slowest, maximal
    // compression). The default is 6. PNG compression is always
    // lossless.
    pub png_compression_level: i32,

    // JPEG Compression
    pub jpeg_compression_level: i32,
    pub jpeg_subsampling: i32, // index for JpegChromaSubSampling
    pub jpeg_progressive: i32,
}

impl WriteImageOperation {
    pub fn new() -> WriteImageOperation {
        WriteImageOperation {}
    }
}

impl WriteImageAttrs {
    pub fn new() -> WriteImageAttrs {
        WriteImageAttrs {
            enable: 1,
            execute: 1,
            file_path: "".to_string(),
            crop_on_write: 2,              // 2 = CropOnWrite::Auto
            pixel_data_type: 255,          // 255 = DataType::Unknown
            exr_compression: 0,            // 0 = ExrCompression::Default
            exr_dwa_compression_level: 45, // Good default for DWA compression
            png_compression_level: 6,      // 0 to 9; 6 is default.
            jpeg_compression_level: 90,    // 1 to 100; higher is larger.
            jpeg_subsampling: 0,           // 0 = JpegChromaSubSampling::Default
            jpeg_progressive: 0,           // Not progressive JPEG
        }
    }
}

fn do_image_process(
    input: &Rc<StreamDataImpl>,
    file_path: &str,
    frame: FrameValue,
    bake_pixel_data_type: DataType,
    crop_on_write: CropOnWrite,
    compress: ImageCompression,
) -> bool {
    let frame_num = frame.round().trunc() as i32;
    let path_expanded = pathutils::expand_string(file_path.to_string(), frame_num);

    // Copy input data
    let mut copy = &mut (**input).clone();
    let display_window = copy.display_window();
    let mut data_window = copy.data_window();
    let mut pixel_block = copy.clone_pixel_block();
    let mut image_spec = copy.clone_image_spec();
    let from_color_space = &image_spec.color_space();

    // The Pixels will be baked into this data type before
    // writing out.
    let out_pixel_data_type = match bake_pixel_data_type {
        DataType::Unknown => copy.pixel_data_type(),
        _ => bake_pixel_data_type,
    };

    // The color space to write to.
    let to_color_space = if path_expanded.ends_with(".jpg")
        || path_expanded.ends_with(".png")
        || path_expanded.ends_with(".jpeg")
        || path_expanded.ends_with(".jpe")
        || path_expanded.ends_with(".jif")
        || path_expanded.ends_with(".jfif")
        || path_expanded.ends_with(".jfi")
    {
        COLOR_SPACE_NAME_SRGB.to_string()
    } else {
        COLOR_SPACE_NAME_LINEAR.to_string()
    };

    let bake_option = BakeOption::All;
    ops::bake::do_process(
        bake_option,
        &mut pixel_block,
        display_window,
        &mut data_window,
        &mut image_spec,
        &mut copy,
        &from_color_space,
        &to_color_space,
        out_pixel_data_type,
    );

    // Write pixels
    let pixel_block_box = Box::new(pixel_block);
    let image = ImageShared {
        pixel_block: pixel_block_box,
        display_window,
        data_window,
        spec: image_spec,
    };
    let num_threads = 0;

    // Should the image be cropped to the display window
    // before writing?
    let do_crop = match crop_on_write {
        CropOnWrite::Disable => false,
        CropOnWrite::Enable => true,
        CropOnWrite::Auto => {
            // When If the image format does not support data
            // windows, then crop off the extra information outside
            // the display window.
            let format_supports_data_window = path_expanded.ends_with(".exr");
            !format_supports_data_window
        }
        _ => panic!("Invalid crop_on_write value: {:?}", crop_on_write),
    };

    let ok = imageio::write_image(&image, &path_expanded, num_threads, do_crop, compress);
    ok
}

impl Operation for WriteImageOperation {
    fn compute(
        &mut self,
        frame: FrameValue,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        _hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("WriteImageOperation.compute()");
        debug!(
            "WriteImageOperation NodeComputeMode={:#?}",
            node_compute_mode
        );
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        let enable = attr_block.get_attr_i32("enable") != 0;
        if enable == false {
            let stream_data = StreamDataImpl::new();
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Valid;
        }

        match inputs.len() {
            0 => {
                // No input given, return an empty default stream.
                let stream_data = StreamDataImpl::new();
                *output = std::rc::Rc::new(stream_data);
                NodeStatus::Warning
            }
            _ => {
                let execute = attr_block.get_attr_i32("execute") != 0;
                if execute == false {
                    *output = inputs[0].clone();
                    NodeStatus::Valid
                } else {
                    let input = &inputs[0].clone();
                    let file_path = attr_block.get_attr_str("file_path");
                    let bake_pixel_data_type =
                        DataType::from(attr_block.get_attr_i32("pixel_data_type"));
                    let crop_on_write = CropOnWrite::from(attr_block.get_attr_i32("crop_on_write"));
                    let exr_compression =
                        ExrCompression::from(attr_block.get_attr_i32("exr_compression"));
                    let exr_dwa_compression_level =
                        attr_block.get_attr_i32("exr_dwa_compression_level");
                    let png_compression_level = attr_block.get_attr_i32("png_compression_level");
                    let jpeg_compression_level = attr_block.get_attr_i32("jpeg_compression_level");
                    let jpeg_subsampling =
                        JpegChromaSubSampling::from(attr_block.get_attr_i32("jpeg_subsampling"));
                    let jpeg_progressive = attr_block.get_attr_i32("jpeg_progressive") != 0;

                    let compress = ImageCompression {
                        exr_compression,
                        exr_dwa_compression_level,
                        png_compression_level,
                        jpeg_compression_level,
                        jpeg_subsampling,
                        jpeg_progressive,
                    };

                    let ok = do_image_process(
                        input,
                        file_path,
                        frame,
                        bake_pixel_data_type,
                        crop_on_write,
                        compress,
                    );
                    if ok == false {
                        warn!("Failed to write image: status={}", ok);
                    }
                    debug!("Success: {}", ok);

                    *output = inputs[0].clone();
                    NodeStatus::Valid
                }
            }
        }
    }
}

impl AttrBlock for WriteImageAttrs {
    fn attr_hash(&self, frame: FrameValue, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 1 {
            let frame_num = frame.round().trunc() as i32;
            let path_expanded = pathutils::expand_string(self.file_path.to_string(), frame_num);
            path_expanded.hash(state);
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "execute" => AttrState::Exists,
            "file_path" => AttrState::Exists,
            "crop_on_write" => AttrState::Exists,
            "pixel_data_type" => AttrState::Exists,
            "exr_compression" => AttrState::Exists,
            "exr_dwa_compression_level" => AttrState::Exists,
            "png_compression_level" => AttrState::Exists,
            "jpeg_compression_level" => AttrState::Exists,
            "jpeg_subsampling" => AttrState::Exists,
            "jpeg_progressive" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            "execute" => self.execute,
            "crop_on_write" => self.crop_on_write,
            "pixel_data_type" => self.pixel_data_type,
            "exr_compression" => self.exr_compression,
            "exr_dwa_compression_level" => self.exr_dwa_compression_level,
            "png_compression_level" => self.png_compression_level,
            "jpeg_compression_level" => self.jpeg_compression_level,
            "jpeg_subsampling" => self.jpeg_subsampling,
            "jpeg_progressive" => self.jpeg_progressive,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "execute" => self.execute = value,
            "crop_on_write" => self.crop_on_write = value,
            "pixel_data_type" => self.pixel_data_type = value,
            "exr_compression" => self.exr_compression = value,
            "exr_dwa_compression_level" => self.exr_dwa_compression_level = value,
            "png_compression_level" => self.png_compression_level = value,
            "jpeg_compression_level" => self.jpeg_compression_level = value,
            "jpeg_subsampling" => self.jpeg_subsampling = value,
            "jpeg_progressive" => self.jpeg_progressive = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, _name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, _name: &str, _value: f32) {
        ()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageValidate {}

impl WriteImageValidate {
    pub fn new() -> WriteImageValidate {
        WriteImageValidate {}
    }
}

impl Validate for WriteImageValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "writeImageValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
        let mut node_compute_modes = Vec::new();
        if input_nodes.len() > 0 {
            node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL);
            for _ in input_nodes.iter().skip(1) {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
            }
        }
        node_compute_modes
    }
}
