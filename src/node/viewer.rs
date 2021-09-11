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

use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::COLOR_SPACE_NAME_LINEAR;
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

#[derive(Debug, Clone, Default)]
pub struct ViewerAttrs {
    pub enable: i32,
    pub bake_option: i32,          // index for a BakeOption.
    pub bake_pixel_data_type: i32, // index for PixelDataType.
    pub bake_color_space: String,
    pub crop_to_format: i32,
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
        }
    }
}

fn do_viewer_bake(
    mut stream_data: &mut StreamDataImpl,
    bake_option: BakeOption,
    bake_pixel_data_type: PixelDataType,
    crop_to_format: bool,
    to_color_space: &str,
) -> (Rc<PixelBlock>, ImageSpec, BBox2Di, BBox2Di) {
    debug!("ViewerOperation::do_viewer_bake(bake_option={:#?}, bake_pixel_data_type={:#?}, to_color_space={:#?})", bake_option, bake_pixel_data_type, to_color_space);

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

impl Operation for ViewerOperation {
    fn compute(
        &mut self,
        _frame: FrameValue,
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

        let mut stream_data = match inputs.len() {
            0 => {
                // No input given, return an empty default stream.
                StreamDataImpl::new()
            }
            _ => {
                let input = &inputs[0].clone();
                (**input).clone()
            }
        };

        let bake_option_initial = BakeOption::from(attr_block.get_attr_i32("bake_option"));
        debug!("bake_option={:#?}", bake_option_initial);

        if bake_option_initial != BakeOption::Nothing {
            let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
            let bake_option = if crop_to_format {
                BakeOption::All
            } else {
                BakeOption::from(bake_option_initial)
            };

            let bake_pixel_data_type =
                PixelDataType::from(attr_block.get_attr_i32("bake_pixel_data_type"));
            debug!("crop_to_format={:#?}", crop_to_format);
            debug!("bake_pixel_data_type={:#?}", bake_pixel_data_type);

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
                    // Bake the image data down and add to
                    // the image cache.

                    let bake_color_space = attr_block.get_attr_str("bake_color_space");
                    debug!("bake_color_space: {:?}", bake_color_space);

                    let (pixel_block_rc, image_spec, data_window, display_window) = do_viewer_bake(
                        &mut stream_data,
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
                stream_data.clear_deformers();
            }

            stream_data.set_data_window(data_window);
            stream_data.set_display_window(display_window);
            stream_data.set_hash(hash_value);
            stream_data.set_pixel_block(pixel_block);
            stream_data.set_image_spec(image_spec);
        }

        *output = std::rc::Rc::new(stream_data);
        NodeStatus::Valid
    }
}

impl AttrBlock for ViewerAttrs {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 0 {
            return;
        }

        let bake_option = BakeOption::from(self.bake_option);
        if bake_option != BakeOption::Nothing {
            self.bake_option.hash(state);
            self.crop_to_format.hash(state);
            self.bake_pixel_data_type.hash(state);
            self.bake_color_space.hash(state);
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

            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        match name {
            "bake_color_space" => &self.bake_color_space,
            _ => "",
        }
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        match name {
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
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "crop_to_format" => self.crop_to_format = value,
            "bake_option" => self.bake_option = value,
            "bake_pixel_data_type" => self.bake_pixel_data_type = value,
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

            let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
            let bake_option_num = attr_block.get_attr_i32("bake_option");
            let bake_option_initial = BakeOption::from(bake_option_num);
            let bake_option = if bake_option_initial != BakeOption::Nothing {
                if crop_to_format {
                    // If cropping, then All must be used because
                    // we'll affect the pixel data when cropping.
                    BakeOption::All
                } else {
                    BakeOption::from(bake_option_num)
                }
            } else {
                BakeOption::Nothing
            };

            if enable == false {
                node_compute_modes.push(NodeComputeMode::ALL);
            } else {
                match bake_option {
                    BakeOption::Nothing => {
                        node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL)
                    }
                    BakeOption::ColorSpace => node_compute_modes.push(
                        node_compute_mode & (NodeComputeMode::ALL - NodeComputeMode::COLOR_SPACE),
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
            for _ in input_nodes.iter().skip(1) {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
            }
        }
        node_compute_modes
    }
}
