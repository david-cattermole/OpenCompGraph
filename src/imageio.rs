use image;
use image::GenericImageView;
use image::ImageBuffer;
use image::RgbaImage;
use log::{debug, error, info, warn};
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::{Duration, Instant};

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
    // TODO: use the display and data windows.
    debug!("Writing... {:?}", path);
    let start = Instant::now();

    let use_oiio = false;
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
                let img = pixelblock::create_image_buffer_rgb_u8(&image.pixel_block);
                ok = match img.save(path) {
                    Ok(value) => true,
                    Err(_) => false,
                };
            }
            if num_channels == 4 {
                let img = pixelblock::create_image_buffer_rgba_u8(&image.pixel_block);
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
