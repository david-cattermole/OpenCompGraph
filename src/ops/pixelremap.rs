use log::{debug, error, info, warn};

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::math;
use crate::pixel::get_pixel_rgb;
use crate::pixel::get_pixel_rgba;
use crate::pixelblock::PixelBlock;

/// Move each pixel to a new position using a slice of 2D pixel
/// coordinates.
pub fn pixels_remap_coords(
    display_window: BBox2Di,
    pixel_coords: &[f32],
    src_pixel_block: &PixelBlock,
    src_data_window: BBox2Di,
    dst_pixel_block: &mut PixelBlock,
    dst_data_window: &mut BBox2Di,
) {
    let display_window_f32 = BBox2Df::from(display_window);
    warn!("display_window = {:?}", display_window);
    warn!("display_window_f32 = {:?}", display_window_f32);
    warn!("display_window.width() = {}", display_window.width());
    warn!("display_window.height() = {}", display_window.height());

    let dst_width = dst_pixel_block.width();
    let dst_height = dst_pixel_block.height();
    let dst_num_channels = dst_pixel_block.num_channels();
    let dst_num_pixels = (dst_width as usize) * (dst_height as usize);
    if dst_num_pixels != (pixel_coords.len() / 2) {
        error!("Destination pixel count and pixel coordinates do not match.");
        return;
    }
    let mut dst_pixels = dst_pixel_block.as_slice_mut();
    warn!("dst_width = {}", dst_width);
    warn!("dst_height = {}", dst_height);
    warn!("dst_num_channels = {}", dst_num_channels);

    let src_width = src_pixel_block.width();
    let src_height = src_pixel_block.height();
    let src_num_channels = src_pixel_block.num_channels();
    let src_pixels = src_pixel_block.as_slice();
    warn!("src_width = {}", src_width);
    warn!("src_height = {}", src_height);
    warn!("src_num_channels = {}", src_num_channels);

    let src_x_stride = src_num_channels;
    let dst_x_stride = dst_num_channels;
    let src_y_stride = src_width * src_x_stride;
    let dst_y_stride = dst_width * dst_x_stride;

    let mut pixel_coord_index = 0;
    for dy in 0..dst_height as usize {
        for dx in 0..dst_width as usize {
            let dst_index: usize = (dy * dst_y_stride as usize) + (dx * dst_x_stride as usize);

            // X and Y source pixel coordinate to fetch the pixel
            // value. The pixel coordinates are relative to the
            // 'display window'.
            let x = pixel_coords[pixel_coord_index + 0];
            let y = pixel_coords[pixel_coord_index + 1];

            let x = math::interp::remap(
                display_window_f32.min_x,
                display_window_f32.max_x,
                src_data_window.min_x as f32,
                (src_data_window.max_x - 1) as f32,
                x,
            );
            let y = math::interp::remap(
                display_window_f32.min_y,
                display_window_f32.max_y,
                src_data_window.min_y as f32,
                (src_data_window.max_y - 1) as f32,
                y,
            );

            if src_num_channels == 4 {
                let (r, g, b, a) = get_pixel_rgba(
                    src_pixels,
                    src_width,
                    src_height,
                    src_x_stride,
                    src_y_stride,
                    x,
                    y,
                );
                dst_pixels[dst_index + 0] = r;
                dst_pixels[dst_index + 1] = g;
                dst_pixels[dst_index + 2] = b;
                dst_pixels[dst_index + 3] = a;
            } else if src_num_channels == 3 {
                let (r, g, b) = get_pixel_rgb(
                    src_pixels,
                    src_width,
                    src_height,
                    src_x_stride,
                    src_y_stride,
                    x,
                    y,
                );
                dst_pixels[dst_index + 0] = r;
                dst_pixels[dst_index + 1] = g;
                dst_pixels[dst_index + 2] = b;
            }
            pixel_coord_index += 2;
        }
    }
}
