use log::{debug, error, info, warn};

// This assumes the 'pixels' have RGBA channels.
#[inline]
pub fn get_pixel_rgba(
    pixels: &[f32],
    width: i32,
    height: i32,
    x_stride: usize,
    y_stride: usize,
    x: f32,
    y: f32,
) -> (f32, f32, f32, f32) {
    // Clamp the pixel dimensions to the image.
    let sx: usize = (x.round() as i32).max(0).min(width - 1) as usize;
    let sy: usize = (y.round() as i32).max(0).min(height - 1) as usize;

    let index_x = sx * x_stride;
    let index_y = sy * y_stride;
    let index = index_x + index_y;

    let mut out_r = 0.0;
    let mut out_g = 0.0;
    let mut out_b = 0.0;
    let mut out_a = 0.0;
    if index < pixels.len() {
        out_r = pixels[index + 0];
        out_g = pixels[index + 1];
        out_b = pixels[index + 2];
        out_a = pixels[index + 3];
    }

    return (out_r, out_g, out_b, out_a);
}

// This assumes the 'pixels' have RGB channels.
#[inline]
pub fn get_pixel_rgb(
    pixels: &[f32],
    width: i32,
    height: i32,
    x_stride: usize,
    y_stride: usize,
    x: f32,
    y: f32,
) -> (f32, f32, f32) {
    // Clamp the pixel dimensions to the image.
    let sx: usize = (x.round() as i32).max(0).min(width - 1) as usize;
    let sy: usize = (y.round() as i32).max(0).min(height - 1) as usize;

    let index_x = sx * x_stride;
    let index_y = sy * y_stride;
    let index = index_x + index_y;

    let mut out_r = 0.0;
    let mut out_g = 0.0;
    let mut out_b = 0.0;
    if index < pixels.len() {
        out_r = pixels[index + 0];
        out_g = pixels[index + 1];
        out_b = pixels[index + 2];
    }

    return (out_r, out_g, out_b);
}
