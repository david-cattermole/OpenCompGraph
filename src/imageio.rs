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
use log::debug;
use std::time::Instant;

use crate::cxxbridge::ffi::oiio_get_thread_count;
use crate::cxxbridge::ffi::oiio_read_image;
use crate::cxxbridge::ffi::oiio_set_thread_count;
use crate::cxxbridge::ffi::oiio_write_image;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::PixelDataType;
use crate::imagebuffer::create_image_buffer_rgb_u8;
use crate::imagebuffer::create_image_buffer_rgba_u8;
use crate::ops::imagecrop;
use crate::pixelblock;

pub fn read_image(path: &String, num_threads: i32) -> ImageShared {
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
                spec: ImageSpec::new(),
            };

            // Overrides the number of threads used for reading.
            let mut old_num_threads = 0;
            oiio_get_thread_count(&mut old_num_threads);
            oiio_set_thread_count(num_threads);
            oiio_read_image(&path, &mut image);
            oiio_set_thread_count(old_num_threads);

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
                spec: ImageSpec::new(),
            }
        }
    };
    let duration = start.elapsed();
    debug!("Reading total time: {:?}", duration);

    image
}

pub fn write_image(
    image: &ImageShared,
    path: &String,
    num_threads: i32,
    crop_to_display_window: bool,
) -> bool {
    debug!("Writing... {:?}", path);
    debug!("Crop to Display Window: {:?}", crop_to_display_window);
    let start = Instant::now();

    let mut new_pixel_block = (*image.pixel_block).clone();
    let new_display_window = image.display_window;
    let new_data_window = image.data_window;
    let crop_window = image.display_window;

    let mut image_out = ImageShared {
        pixel_block: Box::new(new_pixel_block),
        display_window: new_display_window,
        data_window: new_data_window,
        spec: ImageSpec::new(),
    };

    if crop_to_display_window == true {
        let reformat = true;
        let black_outside = false;
        let intersect = false;
        let crop_ok = imagecrop::crop_image_in_place(
            &mut image_out,
            crop_window,
            reformat,
            black_outside,
            intersect,
        );
        debug!("Crop Ok: {:?}", crop_ok);
    }

    let use_oiio = true;
    let ok = match use_oiio {
        true => {
            // Use OpenImageIO C++ library to write the image path.

            // Overrides the number of threads used for writing.
            let mut old_num_threads = 0;
            oiio_get_thread_count(&mut old_num_threads);
            oiio_set_thread_count(num_threads);

            let ok = oiio_write_image(&path, &image_out);

            oiio_set_thread_count(old_num_threads);

            ok
        }
        false => {
            // Use Rust "image" Crate.
            let mut ok = false;
            let num_channels = image.pixel_block.num_channels();
            if num_channels == 3 {
                let img = create_image_buffer_rgb_u8(&image_out);
                ok = match img.save(path) {
                    Ok(_value) => true,
                    Err(_) => false,
                };
            }
            if num_channels == 4 {
                let img = create_image_buffer_rgba_u8(&image_out);
                ok = match img.save(path) {
                    Ok(_value) => true,
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
