use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::AttrState;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::hashutils::HashableF32;

#[derive(Debug)]
pub struct Parameters {
    xc: f32,
    yc: f32,
    k1: f32,
    k2: f32,
}

impl Parameters {
    pub fn new(xc: f32, yc: f32, k1: f32, k2: f32) -> Parameters {
        Parameters { xc, yc, k1, k2 }
    }
}

/// Brownian lens distortion model.
///
/// xu = xd + ((xd - xc) * ((k1 * r2) + (k2 * r4)));
/// yu = yd + ((yd - yc) * ((k1 * r2) + (k2 * r4)));
///
/// where:
///   xu = undistorted image point
///   xd = distorted image point
///   xc = distortion center
///   k1, k2, etc = Nth radial distortion coefficent
///   p1, p2, etc = Nth tangential distortion coefficent
///   r = sqrt(pow(xd - xc, 2) + pow(yd - yc, 2))
///
#[inline]
fn apply_distortion_model_brownian_degree4(
    xd: f32,
    yd: f32,
    xc: f32,
    yc: f32,
    k1: f32,
    k2: f32,
) -> (f32, f32) {
    let r: f32 = ((xd - xc).powi(2) + (yd - yc).powi(2)).sqrt();
    let r2: f32 = r.powi(2);
    let r4: f32 = r.powi(4) * 2.0;

    let xu = xd + ((xd - xc) * ((k1 * r2) + (k2 * r4)));
    let yu = yd + ((yd - yc) * ((k1 * r2) + (k2 * r4)));

    (xu, yu)
}

/// Given a slice of values, each 3 elements are expected to be a XYZ,
/// where only X and Y are deformed.
pub fn deform_slice_by_3_in_place(buffer: &mut [f32], parameters: Parameters) {
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

        let (xu, yu) = apply_distortion_model_brownian_degree4(x, y, xc, yc, k1, k2);

        buffer[index + 0] = xu;
        buffer[index + 1] = yu;
    }
}
