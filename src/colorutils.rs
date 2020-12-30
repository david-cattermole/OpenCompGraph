use fastapprox::faster;

/// https://www.excamera.com/sphinx/article-srgb.html
pub fn convert_srgb_to_linear(x: f32) -> f32 {
    let a: f32 = 0.055;
    if x <= 0.04045 {
        x * (1.0 / 12.92)
    } else {
        faster::pow((x + a) * (1.0 / (1.0 + a)), 2.4)
    }
}

/// https://www.excamera.com/sphinx/article-srgb.html
pub fn convert_linear_to_srgb(x: f32) -> f32 {
    let a = 0.055;
    if x <= 0.0031308 {
        x * 12.92
    } else {
        (1.0 + a) * faster::pow(x, 1.0 / 2.4) - a
    }
}
