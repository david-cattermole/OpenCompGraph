use log::{debug, error, info, warn};
use std::time::{Duration, Instant};

use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::MergeImageMode;
use crate::cxxbridge::ffi::PixelDataType;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::deformer::Deformer;
use crate::deformutils;
use crate::pixelblock;

#[inline]
fn get_pixel_rgba(image: &ImageShared, x: i32, y: i32) -> (f32, f32, f32, f32) {
    // TODO: Somehow this function seems to incorrectly wrap the X
    // (and Y?) pixel location.
    //
    // TODO: This function should support 'clamp' and 'black outside'
    // features.
    let pixels = image.pixel_block.as_slice();
    let num_channels = image.pixel_block.num_channels();
    let image_x = x - image.data_window.min_x;
    let image_y = y - image.data_window.min_y;
    let index = image.pixel_block.get_pixel_index(image_x, image_y);
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 1.0; // Default to full opaque.
    if index >= 0 {
        let i = index as usize;
        r = pixels[i + 0];
        g = pixels[i + 1];
        b = pixels[i + 2];
        if num_channels == 4 {
            a = pixels[i + 3];
        }
    }
    (r, g, b, a)
}

/// Merge images together.
///
/// Supports RGB or RGBA. Nothing more.
///
/// References:
///
///  - "Compositing Digital Images", Thomas Porter and Tom Duff,
///  Computer Graphics Volume 18, Number 3 July 1984,
///  https://keithp.com/~keithp/porterduff/p253-porter.pdf
///
/// Assumes:
///
///  - Images are already f32 pixel data types.
///
///  - All input image data windows are consistent compared to the
///    image_b display window.
pub fn merge(
    mode: MergeImageMode,
    image_a: &ImageShared,
    image_b: &ImageShared,
    mix: f32,
    image_out: &mut ImageShared,
) -> bool {
    debug!("merge...");
    if mode == MergeImageMode::Uninitialized {
        return false;
    }

    assert!(image_a.pixel_block.pixel_data_type() == PixelDataType::Float32);
    assert!(image_b.pixel_block.pixel_data_type() == PixelDataType::Float32);

    debug!("Image A: Data Window {:?}", image_a.data_window);
    debug!("Image B: Data Window {:?}", image_b.data_window);

    debug!("Image A: Display Window {:?}", image_a.display_window);
    debug!("Image B: Display Window {:?}", image_b.display_window);

    // Compute the output image dimensions and allocate needed memory.
    let out_data_window = BBox2Di::combine(image_a.data_window, image_b.data_window);
    image_out.data_window = out_data_window;
    image_out.pixel_block.data_resize(
        out_data_window.width(),
        out_data_window.height(),
        image_b.pixel_block.num_channels(),
        image_b.pixel_block.pixel_data_type(),
    );
    image_out.display_window = image_b.display_window;

    debug!(
        "Image OUT: WxH = {}x{}",
        image_out.pixel_block.width(),
        image_out.pixel_block.height()
    );

    debug!("Image OUT: Data Window {:?}", image_out.data_window);
    debug!("Image OUT: Display Window {:?}", image_out.display_window);

    let stride = image_out.pixel_block.num_channels() as usize;
    let mut out_pixels = image_out.pixel_block.as_slice_mut();
    let mut index = 0;
    for row in out_data_window.min_y..out_data_window.max_y {
        for col in out_data_window.min_x..out_data_window.max_x {
            let (r_a, g_a, b_a, a_a) = get_pixel_rgba(image_a, col, row);
            let (r_b, g_b, b_b, a_b) = get_pixel_rgba(image_b, col, row);

            let (r, g, b, a) = match mode {
                MergeImageMode::Add => {
                    let mask = mix;
                    let r = r_a + (r_b * mask);
                    let g = g_a + (g_b * mask);
                    let b = b_a + (b_b * mask);
                    let a = a_a + (a_b * mask);
                    (r, g, b, a)
                }
                MergeImageMode::Over => {
                    // A Over B
                    let mask = (1.0 - a_a) * mix;
                    let r = r_a + (r_b * mask);
                    let g = g_a + (g_b * mask);
                    let b = b_a + (b_b * mask);
                    let a = a_a + (a_b * mask);
                    (r, g, b, a)
                }
                MergeImageMode::Multiply => {
                    let mask = mix;
                    let r = r_a * (r_b * mask);
                    let g = g_a * (g_b * mask);
                    let b = b_a * (b_b * mask);
                    let a = a_a * (a_b * mask);
                    (r, g, b, a)
                }
                _ => panic!("unsupported merge mode"),
            };

            // Set output
            out_pixels[index + 0] = r;
            out_pixels[index + 1] = g;
            out_pixels[index + 2] = b;
            if stride == 4 {
                out_pixels[index + 3] = a;
            }
            index += stride;
        }
    }
    true
}
