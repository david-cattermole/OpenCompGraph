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
use crate::pixelblock;

pub fn create_image_buffer_rgb_u8(
    image: &ImageShared,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let pixel_block = &image.pixel_block;
    let width = pixel_block.width();
    let height = pixel_block.height();
    let pixels = &pixel_block.as_slice();

    // Get pixel statistics
    if log_enabled!(Level::Debug) {
        let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
        debug!("Min value: {}", min);
        debug!("Max value: {}", max);
        debug!("Avg value: {}", avg);
    }

    // Convert f32 pixel image to u8 ImageBuffer.
    let pixels_u8: Vec<u8> = pixels
        .iter()
        .map(|x| (convert_linear_to_srgb(*x as f32) * (u8::max_value() as f32)) as u8)
        .collect();

    match image::ImageBuffer::from_raw(width as u32, height as u32, pixels_u8) {
        Some(data) => data,
        _ => panic!("invalid image."),
    }
}

pub fn create_image_buffer_rgba_u8(
    image: &ImageShared,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let pixel_block = &image.pixel_block;
    let width = pixel_block.width();
    let height = pixel_block.height();
    let pixels = &pixel_block.as_slice();

    // Get pixel statistics
    if log_enabled!(Level::Debug) {
        let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
        debug!("Min value: {}", min);
        debug!("Max value: {}", max);
        debug!("Avg value: {}", avg);
    }

    // Convert f32 pixel image to u8 ImageBuffer.
    let pixels_u8: Vec<u8> = pixels
        .iter()
        .map(|x| (convert_linear_to_srgb(*x as f32) * (u8::max_value() as f32)) as u8)
        .collect();

    debug!(
        "RGBA u8 width={} height={} num_channels={} pixels.size()={}",
        width,
        height,
        pixel_block.num_channels(),
        pixels_u8.len()
    );
    match image::ImageBuffer::from_raw(width as u32, height as u32, pixels_u8) {
        Some(data) => data,
        _ => panic!("invalid image."),
    }
}
