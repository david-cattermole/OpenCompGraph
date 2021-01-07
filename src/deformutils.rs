use log::{debug, error, info, warn};

use crate::deformer::brownian;
use crate::deformer::Deformer;
use crate::deformer::DeformerType;

pub fn apply_deformers_to_positions(deformers: &Vec<Deformer>, buffer: &mut [f32]) {
    for deformer in deformers {
        debug!("apply_deformer...");
        let enable = deformer.get_attr_i32("enable");
        if enable == 0 {
            continue;
        }

        let deformer_type = deformer.deformer_type();
        debug!("Deformer Type: {:?}", deformer_type);
        match deformer_type {
            DeformerType::Brownian => {
                let k1 = deformer.get_attr_f32("k1");
                let k2 = deformer.get_attr_f32("k2");
                let xc = deformer.get_attr_f32("center_x");
                let yc = deformer.get_attr_f32("center_y");
                let parameters = brownian::Parameters::new(xc, yc, k1, k2);
                deform_slice_by_3_in_place(buffer, parameters)
            }
            DeformerType::Null => (),
            DeformerType::Unknown => warn!("Unknown deformer type."),
        }
    }
}

// TODO: Provide a function to calculate the new bounding box, and
// hence the size of the new image. We need to allocate the new image
// before using this function.
pub fn apply_deformers_to_pixels(
    deformers: &Vec<Deformer>,
    // display_window: BBox2Df,
    // data_window: BBox2Df,
    src_width: u32,
    src_height: u32,
    src_num_channels: u8,
    src: &[f32],
    dst_width: u32,
    dst_height: u32,
    dst_num_channels: u8,
    dst: &mut [f32],
) {
    if src.len() != dst.len() {
        error!("The given source and destination buffer lengths are not equal.");
        return;
    }

    for deformer in deformers {
        let enable = deformer.get_attr_i32("enable");
        if enable == 0 {
            continue;
        }

        let deformer_type = deformer.deformer_type();
        match deformer_type {
            DeformerType::Brownian => {
                let k1 = deformer.get_attr_f32("k1");
                let k2 = deformer.get_attr_f32("k2");
                let xc = deformer.get_attr_f32("center_x");
                let yc = deformer.get_attr_f32("center_y");
                let parameters = brownian::Parameters::new(k1, k2, xc, yc);
                deform_image_slice_by_3(
                    &src,
                    dst,
                    parameters,
                    src_width,
                    src_height,
                    src_num_channels,
                    dst_width,
                    dst_height,
                    dst_num_channels,
                )
            }
            DeformerType::Null => (),
            DeformerType::Unknown => warn!("Unknown deformer type."),
        }
    }
}

// pub fn remove_deformers(deformers: Vec<Deformer>) {}

pub fn deform_image_slice_by_3(
    src: &[f32],
    dst: &mut [f32],
    parameters: brownian::Parameters,
    src_width: u32,
    src_height: u32,
    src_num_channels: u8,
    dst_width: u32,
    dst_height: u32,
    dst_num_channels: u8,
) {
    let xc = parameters.xc;
    let yc = parameters.yc;
    let k1 = parameters.k1;
    let k2 = parameters.k2;

    let dst_y_stride = dst_width as usize * dst_num_channels as usize;
    let dst_x_stride = dst_num_channels as usize;

    let mut count = 0;
    for dy in 0..dst_height as usize {
        for dx in 0..dst_width as usize {
            let dst_index = (dy * dst_y_stride) + (dx * dst_x_stride);

            let r = src[dst_index + 0];
            let g = src[dst_index + 1];
            let b = src[dst_index + 2];

            dst[dst_index + 0] = r;
            dst[dst_index + 1] = g;
            dst[dst_index + 2] = b;
            count += 1;
        }
    }
}

/// Given a slice of values, each 3 elements are expected to be a XYZ,
/// where only X and Y are deformed.
pub fn deform_slice_by_3_in_place(buffer: &mut [f32], parameters: brownian::Parameters) {
    debug!(
        "deform_slice_by_3_in_place: {} {:#?}",
        buffer.len(),
        parameters
    );
    let xc = parameters.xc;
    let yc = parameters.yc;
    let k1 = parameters.k1;
    let k2 = parameters.k2;

    let step_num = 3;
    let count = buffer.len() / step_num;
    for i in 0..count {
        let index = i * step_num;

        let x = buffer[index + 0];
        let y = buffer[index + 1];
        // Z is never changed, only X and Y are modified.

        let (xu, yu) = brownian::apply_distortion_model_brownian_degree4(x, y, xc, yc, k1, k2);

        buffer[index + 0] = xu;
        buffer[index + 1] = yu;
    }
}
