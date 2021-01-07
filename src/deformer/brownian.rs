use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct Parameters {
    pub xc: f32,
    pub yc: f32,
    pub k1: f32,
    pub k2: f32,
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
pub fn apply_distortion_model_brownian_degree4(
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
