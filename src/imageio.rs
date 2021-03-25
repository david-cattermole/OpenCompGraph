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

use image;
use image::GenericImageView;
use image::ImageBuffer;
use image::RgbaImage;
use log::{debug, error, info, log_enabled, warn, Level};
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant};

use crate::colorutils::convert_linear_to_srgb;
use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::oiio_read_image;
use crate::cxxbridge::ffi::oiio_write_image;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::PixelDataType;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::deformer::Deformer;
use crate::deformutils;
use crate::imagebuffer::create_image_buffer_rgb_u8;
use crate::imagebuffer::create_image_buffer_rgba_u8;
use crate::pixelblock;

pub fn read_image(path: &String) -> ImageShared {
    debug!("Reading... {:?}", path);
    let start = Instant::now();

    let use_oiio = true;
    let image = match use_oiio {
        true => {
            // Use OpenImageIO C++ library to read the image path.
            let mut image = ImageShared {
                pixel_block: Box::new(pixelblock::PixelBlock::empty(PixelDataType::Float32)),
                display_window: BBox2Di::new(0, 0, 0, 0),
                data_window: BBox2Di::new(0, 0, 0, 0),
            };
            let ok = oiio_read_image(&path, &mut image);
            image
        }
        false => {
            // Use Rust "image" Crate.
            let img = image::open(path).unwrap();
            let pixel_block = Box::new(pixelblock::from_dynamic_image(img));
            let display_window = BBox2Di::new(0, 0, pixel_block.width(), pixel_block.height());
            let data_window = display_window.clone();
            ImageShared {
                pixel_block,
                display_window,
                data_window,
            }
        }
    };
    let duration = start.elapsed();
    debug!("Reading total time: {:?}", duration);

    image
}

pub fn write_image(image: &ImageShared, path: &String) -> bool {
    debug!("Writing... {:?}", path);
    let start = Instant::now();

    let use_oiio = true;
    let ok = match use_oiio {
        true => {
            // Use OpenImageIO C++ library to write the image path.
            oiio_write_image(&path, &image)
        }
        false => {
            // Use Rust "image" Crate.
            let mut ok = false;
            let num_channels = image.pixel_block.num_channels();
            if num_channels == 3 {
                let img = create_image_buffer_rgb_u8(&image);
                ok = match img.save(path) {
                    Ok(value) => true,
                    Err(_) => false,
                };
            }
            if num_channels == 4 {
                let img = create_image_buffer_rgba_u8(&image);
                ok = match img.save(path) {
                    Ok(value) => true,
                    Err(_) => false,
                };
            }
            ok
        }
    };
    let duration = start.elapsed();
    debug!("Writing total time: {:?}", duration);

    ok
}
