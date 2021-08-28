/*
 * Copyright (C) 2021 David Cattermole.
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
 * Viewer node to be used to apply last-minute changes to the image.
 */

use log::{debug, warn};
use shellexpand;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::DiskCacheImageType;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::data::COLOR_SPACE_NAME_SRGB;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::ops::bake;
use crate::ops::imagecrop;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Viewer,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ViewerOperation::new()),
        validate: Box::new(ViewerValidate::new()),
        attr_block: Box::new(ViewerAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ViewerOperation {}

#[derive(Debug, Clone, Default, hash::Hash)]
pub struct ViewerAttrs {
    pub enable: i32,
    pub bake_option: i32,          // index for a BakeOption.
    pub bake_pixel_data_type: i32, // index for PixelDataType.
    pub bake_color_space: String,
    pub crop_to_format: i32,
    pub disk_cache: i32,
    pub disk_cache_image_type: i32, // index for DiskCacheImageType.
    pub disk_cache_dir: String,
}

impl ViewerOperation {
    pub fn new() -> ViewerOperation {
        ViewerOperation {}
    }
}

impl ViewerAttrs {
    pub fn new() -> ViewerAttrs {
        ViewerAttrs {
            enable: 1,
            bake_option: 0,          // 0 = BakeOption::Nothing
            bake_pixel_data_type: 1, // 1 = PixelDataType::Half16
            bake_color_space: COLOR_SPACE_NAME_LINEAR.to_string(),
            crop_to_format: 0,
            disk_cache: 0,
            disk_cache_image_type: 0, // 0 = DiskCacheImageType::JPEG_UInt8
            disk_cache_dir: "".to_string(),
        }
    }
}

fn expand_directory(directory: &str) -> String {
    let expanded_dir = match shellexpand::full(directory) {
        Ok(v) => (*v).to_string(),
        Err(e) => {
            warn!(
                "Environment variable unknown: name={} cause={}",
                e.var_name, e.cause
            );
            directory.to_string()
        }
    };
    expanded_dir.to_string()
}

fn compute_file_path(directory: &str, hash_value: HashValue, file_ext: &str) -> String {
    let expanded_dir = expand_directory(directory);
    let format_path = format!(
        "{}{}ocg_disk_cache_{}.{}",
        expanded_dir, MAIN_SEPARATOR, hash_value, file_ext
    );
    let file_path = match fs::canonicalize(&format_path) {
        Ok(v) => match v.to_str() {
            Some(s) => s.to_string(),
            None => format_path.to_string(),
        },
        Err(_) => format_path.to_string(),
    };
    file_path
}

fn get_disk_cache_image_type_values<'a, 'b, 'c>(
    value: DiskCacheImageType,
) -> (&'a str, &'b str, &'c str, PixelDataType) {
    match value {
        DiskCacheImageType::JPEG_UInt8 => {
            ("jpg", "lossy", COLOR_SPACE_NAME_SRGB, PixelDataType::UInt8)
        }
        DiskCacheImageType::JPEG_2000_Lossy_UInt8 => {
            ("jp2", "lossy", COLOR_SPACE_NAME_SRGB, PixelDataType::UInt8)
        }
        DiskCacheImageType::JPEG_2000_Lossless_UInt8 => (
            "jp2",
            "lossless",
            COLOR_SPACE_NAME_SRGB,
            PixelDataType::UInt8,
        ),
        DiskCacheImageType::EXR_Lossy_Half16 => (
            "exr",
            "lossy",
            COLOR_SPACE_NAME_LINEAR,
            PixelDataType::Half16,
        ),
        DiskCacheImageType::EXR_Lossless_Half16 => (
            "exr",
            "lossless",
            COLOR_SPACE_NAME_LINEAR,
            PixelDataType::Half16,
        ),
        _ => panic!("DiskCacheImageType is unsupported: {:#?}", value),
    }
}

fn do_process(
    mut stream_data: &mut StreamDataImpl,
    bake_option: BakeOption,
    bake_pixel_data_type: PixelDataType,
    crop_to_format: bool,
    to_color_space: &str,
) -> (Rc<PixelBlock>, ImageSpec, BBox2Di, BBox2Di) {
    debug!("ViewerOperation::do_process(bake_option={:#?}, bake_pixel_data_type={:#?}, to_color_space={:#?})", bake_option, bake_pixel_data_type, to_color_space);

    // Stream input data
    let mut display_window = stream_data.display_window();
    let mut data_window = stream_data.data_window();
    let mut pixel_block = stream_data.clone_pixel_block();
    let mut image_spec = stream_data.clone_image_spec();

    let pixel_data_type = PixelDataType::Float32;
    let from_color_space = &image_spec.color_space();
    bake::do_process(
        bake_option,
        &mut pixel_block,
        display_window,
        &mut data_window,
        &mut image_spec,
        &mut stream_data,
        &from_color_space,
        to_color_space,
        pixel_data_type,
    );

    if crop_to_format {
        let mut img = ImageShared {
            pixel_block: Box::new(pixel_block),
            display_window: display_window,
            data_window: data_window,
            spec: image_spec,
        };

        let reformat = true;
        let black_outside = false;
        let intersect = false;
        let _ok = imagecrop::crop_image_in_place(
            &mut img,
            display_window,
            reformat,
            black_outside,
            intersect,
        );

        pixel_block = *img.pixel_block;
        display_window = img.display_window;
        data_window = img.data_window;
        image_spec = img.spec;
    }
    debug!(
        "ViewerOperation: convert to baked pixel_data_type={:#?}",
        bake_pixel_data_type,
    );
    pixel_block.convert_into_pixel_data_type(bake_pixel_data_type);

    let pixel_block_rc = Rc::new(pixel_block);
    (
        pixel_block_rc.clone(),
        image_spec,
        data_window,
        display_window,
    )
}

fn get_image_from_disk_cache(
    hash_value: HashValue,
    mut copy: &mut StreamDataImpl,
    disk_cache_dir: &str,
    disk_cache_image_type: DiskCacheImageType,
    bake_option: BakeOption,
    crop_to_format: bool,
) -> (Rc<PixelBlock>, ImageSpec, BBox2Di, BBox2Di) {
    // Compute file path
    // TODO: Use compression hints.
    let (file_ext, compression_mode, disk_color_space, disk_pixel_data_type) =
        get_disk_cache_image_type_values(disk_cache_image_type);
    debug!("file_ext: {:?}", file_ext);
    debug!("compression_mode: {:?}", compression_mode);
    debug!("disk_color_space: {:?}", disk_color_space);
    debug!("disk_pixel_data_type: {:?}", disk_pixel_data_type);

    let file_path = compute_file_path(disk_cache_dir, hash_value, file_ext);
    debug!("file_path: {:?}", file_path);

    // Try to read file path.
    let file_exists = Path::new(&file_path).exists();
    if file_exists {
        // Read file (that we know exists)
        let num_threads = 0;
        let mut img = imageio::read_image(&file_path, num_threads);
        let from_color_space = &img.spec.color_space();
        let to_color_space = COLOR_SPACE_NAME_LINEAR;
        bake::do_process(
            BakeOption::ColorSpace,
            &mut img.pixel_block,
            img.display_window,
            &mut img.data_window,
            &mut img.spec,
            &mut copy,
            &from_color_space,
            &to_color_space,
            disk_pixel_data_type,
        );

        let pixel_block_rc = Rc::new(*img.pixel_block);
        (
            pixel_block_rc,
            img.spec,
            img.data_window,
            img.display_window,
        )
    } else {
        // Image was not found on-disk.

        // Compute image
        let to_color_space = disk_color_space;
        let (pixel_block_rc, image_spec, data_window, display_window) = do_process(
            &mut copy,
            bake_option,
            disk_pixel_data_type,
            crop_to_format,
            &to_color_space,
        );

        // Write image
        let img = ImageShared {
            pixel_block: Box::new((*pixel_block_rc).clone()),
            display_window: display_window,
            data_window: data_window,
            spec: image_spec.clone(),
        };
        let do_crop = false;
        let num_threads = 0;
        let ok = imageio::write_image(&img, &file_path, num_threads, do_crop);
        if ok == false {
            warn!("Failed to write image: status={}", ok);
        }
        debug!("Success: {}", ok);

        (pixel_block_rc, image_spec, data_window, display_window)
    }
}

impl Operation for ViewerOperation {
    fn compute(
        &mut self,
        _frame: i32,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ViewerOperation.compute()");
        debug!("ViewerOperation NodeComputeMode={:#?}", node_compute_mode);
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            return NodeStatus::Valid;
        }

        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let input = &inputs[0].clone();
                let mut copy = (**input).clone();

                let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
                let bake_option_num = if crop_to_format {
                    3 as i32 // 3 == Bake All.
                } else {
                    attr_block.get_attr_i32("bake_option")
                };
                let bake_option = BakeOption::from(bake_option_num);
                let bake_pixel_data_type =
                    PixelDataType::from(attr_block.get_attr_i32("bake_pixel_data_type"));
                let disk_cache = attr_block.get_attr_i32("disk_cache") != 0;
                let disk_cache_image_type =
                    DiskCacheImageType::from(attr_block.get_attr_i32("disk_cache_image_type"));

                debug!("Viewer bake_option={:#?}", bake_option);
                debug!("Viewer bake_pixel_data_type={:#?}", bake_pixel_data_type);
                debug!("Viewer disk_cache={:#?}", disk_cache);
                debug!("Viewer disk_cache_image_type={:#?}", disk_cache_image_type);

                // TODO: Determine if we really need to change the
                // pixels or not. For example, Check if the color
                // space is already set correctly, check if there are
                // any deformers, check if the color matrix is not
                // identity.
                let change_pixels = (bake_option != BakeOption::Nothing)
                    || crop_to_format
                    || (bake_pixel_data_type != copy.pixel_data_type());

                if change_pixels {
                    let (pixel_block, image_spec, data_window, display_window) = match cache
                        .get(&hash_value)
                    {
                        Some(cached_img) => {
                            debug!("Cache Hit");
                            (
                                cached_img.pixel_block.clone(),
                                cached_img.spec.clone(),
                                cached_img.data_window,
                                cached_img.display_window,
                            )
                        }
                        _ => {
                            debug!("Cache Miss");
                            if disk_cache {
                                let disk_cache_dir = attr_block.get_attr_str("disk_cache_dir");
                                debug!("disk_cache_dir: {:?}", disk_cache_dir);
                                get_image_from_disk_cache(
                                    hash_value,
                                    &mut copy,
                                    disk_cache_dir,
                                    disk_cache_image_type,
                                    bake_option,
                                    crop_to_format,
                                )
                            } else {
                                // Bake the image data down and add to
                                // the image cache.

                                let bake_color_space = attr_block.get_attr_str("bake_color_space");
                                debug!("bake_color_space: {:?}", bake_color_space);

                                let (pixel_block_rc, image_spec, data_window, display_window) =
                                    do_process(
                                        &mut copy,
                                        bake_option,
                                        bake_pixel_data_type,
                                        crop_to_format,
                                        &bake_color_space,
                                    );
                                let cached_img = CachedImage {
                                    pixel_block: pixel_block_rc.clone(),
                                    spec: image_spec.clone(),
                                    data_window: data_window,
                                    display_window: display_window,
                                };
                                cache.insert(hash_value, cached_img);
                                (pixel_block_rc, image_spec, data_window, display_window)
                            }
                        }
                    };

                    // Ensure the stream does not double up on
                    // deformers if the image is expected to already
                    // have baked deformations in it.
                    //
                    // If the cached image was generated while baking
                    // deformations to pixels, then we must remove
                    // deformers from the stream so that we do not get
                    // a double-deformation effect when reading from
                    // the cache.
                    if bake_option == BakeOption::All {
                        copy.clear_deformers();
                    }

                    copy.set_data_window(data_window);
                    copy.set_display_window(display_window);
                    copy.set_hash(hash_value);
                    copy.set_pixel_block(pixel_block);
                    copy.set_image_spec(image_spec);
                    *output = std::rc::Rc::new(copy);
                }

                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for ViewerAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 1 {
            self.bake_option.hash(state);
            self.crop_to_format.hash(state);
            if self.disk_cache == 1 {
                self.disk_cache.hash(state);
                self.disk_cache_image_type.hash(state);
                // NOTE: Hash is the same regardless of 'disk_cache_dir'.
            } else {
                self.bake_pixel_data_type.hash(state);
                self.bake_color_space.hash(state);
            }
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        /*
         * Add attributes:
         *
         * - Image Exposure
         * - Image Gamma
         * - Image Soft-clip
         * - Crop image to the image display window.
         * - Down-res image
         */
        match name {
            "enable" => AttrState::Exists,

            // "color_exposure" => AttrState::Exists,
            // "color_gamma" => AttrState::Exists,
            // "color_soft_clip" => AttrState::Exists,

            // "down_res" => AttrState::Exists,
            "crop_to_format" => AttrState::Exists,

            "bake_option" => AttrState::Exists,
            "bake_pixel_data_type" => AttrState::Exists,
            "bake_color_space" => AttrState::Exists,

            "disk_cache" => AttrState::Exists,
            "disk_cache_image_type" => AttrState::Exists,
            "disk_cache_dir" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        match name {
            "disk_cache_dir" => &self.disk_cache_dir,
            "bake_color_space" => &self.bake_color_space,
            _ => "",
        }
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        match name {
            "disk_cache_dir" => self.disk_cache_dir = value.to_string(),
            "bake_color_space" => self.bake_color_space = value.to_string(),
            _ => (),
        };
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            "crop_to_format" => self.crop_to_format,
            "bake_option" => self.bake_option,
            "bake_pixel_data_type" => self.bake_pixel_data_type,
            "disk_cache" => self.disk_cache,
            "disk_cache_image_type" => self.disk_cache_image_type,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "crop_to_format" => self.crop_to_format = value,
            "bake_option" => self.bake_option = value,
            "bake_pixel_data_type" => self.bake_pixel_data_type = value,
            "disk_cache" => self.disk_cache = value,
            "disk_cache_image_type" => self.disk_cache_image_type = value,
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
pub struct ViewerValidate {}

impl ViewerValidate {
    pub fn new() -> ViewerValidate {
        ViewerValidate {}
    }
}

impl Validate for ViewerValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "ViewerValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
        let mut node_compute_modes = Vec::new();

        if input_nodes.len() > 0 {
            let enable = attr_block.get_attr_i32("enable") != 0;
            let disk_cache = attr_block.get_attr_i32("disk_cache") != 0;
            let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
            let bake_option_num = if crop_to_format {
                3 as i32 // 3 == Bake All.
            } else {
                attr_block.get_attr_i32("bake_option")
            };
            let bake_option = BakeOption::from(bake_option_num);
            let disk_cache_image_type =
                DiskCacheImageType::from(attr_block.get_attr_i32("disk_cache_image_type"));

            if enable == false {
                node_compute_modes.push(NodeComputeMode::ALL);
            } else {
                if disk_cache == true {
                    let disk_cache_dir = attr_block.get_attr_str("disk_cache_dir");
                    let (file_ext, _compression_mode, _disk_color_space, _disk_pixel_data_type) =
                        get_disk_cache_image_type_values(disk_cache_image_type);
                    let file_path = compute_file_path(disk_cache_dir, hash_value, file_ext);

                    // Try to read file path.
                    let file_exists = Path::new(&file_path).exists();
                    if file_exists == false {
                        node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL)
                    } else {
                        match bake_option {
                            BakeOption::Nothing => {
                                node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL)
                            }
                            BakeOption::ColorSpace => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL - NodeComputeMode::COLOR_SPACE),
                            ),
                            BakeOption::ColorSpaceAndGrade => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL
                                        - NodeComputeMode::COLOR_SPACE
                                        - NodeComputeMode::COLOR),
                            ),
                            BakeOption::All => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL
                                        - NodeComputeMode::COLOR_SPACE
                                        - NodeComputeMode::COLOR
                                        - NodeComputeMode::DEFORMER
                                        - NodeComputeMode::PIXEL),
                            ),
                            _ => panic!("BakeOption is invalid value: {:#?}", bake_option),
                        }
                    }
                } else {
                    node_compute_modes.push(NodeComputeMode::ALL);
                }
            }
            for _ in input_nodes.iter().skip(1) {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
            }
        }
        node_compute_modes
    }
}
