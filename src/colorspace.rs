use fastapprox::faster;
use log::{debug, error, info, warn};
use nalgebra as na;

use crate::cxxbridge::ffi::ocio_color_convert_inplace;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::Matrix4;
use crate::math::colorxform;
use crate::pixelblock::PixelBlock;

pub fn color_convert_inplace(
    pixel_data: &mut [f32],
    width: i32,
    height: i32,
    num_channels: i32,
    src_color_space: &String,
    dst_color_space: &String,
) -> bool {
    ocio_color_convert_inplace(
        pixel_data,
        width,
        height,
        num_channels,
        src_color_space,
        dst_color_space,
    )
}
