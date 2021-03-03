use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::ParameterType;
use crate::deformer::Deformer;
use crate::hashutils::HashableF32;
use crate::ldpk_utils::LensDistortionPlugin;
use crate::mathutils;

// Note: All names end with a null terminator character because the
// C++ plug-ins expect it.
static PARM_NAME_FOCAL_LENGTH: &str = "tde4_focal_length_cm\0";
static PARM_NAME_FILM_BACK_WIDTH: &str = "tde4_filmback_width_cm\0";
static PARM_NAME_FILM_BACK_HEIGHT: &str = "tde4_filmback_height_cm\0";
static PARM_NAME_PIXEL_ASPECT: &str = "tde4_pixel_aspect\0";
static PARM_NAME_LENS_CENTER_OFFSET_X: &str = "tde4_lens_center_offset_x_cm\0";
static PARM_NAME_LENS_CENTER_OFFSET_Y: &str = "tde4_lens_center_offset_y_cm\0";
static PARM_NAME_FOCUS_DISTANCE: &str = "tde4_custom_focus_distance_cm\0";
// Actual distortion parameter names.
static PARM_NAME_DISTORTION: &str = "Distortion\0";
static PARM_NAME_ANAMORPHIC_SQUEEZE: &str = "Anamorphic Squeeze\0";
static PARM_NAME_CURVATURE_X: &str = "Curvature X\0";
static PARM_NAME_CURVATURE_Y: &str = "Curvature Y\0";
static PARM_NAME_QUARTIC_DISTORTION: &str = "Quartic Distortion\0";

#[derive(Debug, Clone)]
pub struct DeformerTde4Classic {
    /// Enable/disable the deformer.
    enable: i32,

    // Built-in LDPK parameters
    focal_length: f32,
    film_back_width: f32,
    film_back_height: f32,
    pixel_aspect: f32,
    lens_center_offset_x: f32,
    lens_center_offset_y: f32,

    // 3DE Lens Model parameters
    distortion: f32,
    anamorphic_squeeze: f32,
    curvature_x: f32,
    curvature_y: f32,
    quartic_distortion: f32,

    /// LDPK Plug-in.
    plugin: LensDistortionPlugin,
}

impl hash::Hash for DeformerTde4Classic {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        HashableF32::new(self.focal_length).hash(state);
        HashableF32::new(self.film_back_width).hash(state);
        HashableF32::new(self.film_back_height).hash(state);
        HashableF32::new(self.pixel_aspect).hash(state);
        HashableF32::new(self.lens_center_offset_x).hash(state);
        HashableF32::new(self.lens_center_offset_y).hash(state);
        HashableF32::new(self.distortion).hash(state);
        HashableF32::new(self.anamorphic_squeeze).hash(state);
        HashableF32::new(self.curvature_x).hash(state);
        HashableF32::new(self.curvature_y).hash(state);
        HashableF32::new(self.quartic_distortion).hash(state);
    }
}

impl Default for DeformerTde4Classic {
    fn default() -> Self {
        let plugin = LensDistortionPlugin::new();

        let focal_length = 3.0; // 30.0 mm
        let film_back_width = 3.6; // 36.0 mm
        let film_back_height = 2.4; // 24.0 mm
        let pixel_aspect = 1.0;
        let lens_center_offset_x = 0.0;
        let lens_center_offset_y = 0.0;

        let distortion = 0.0;
        let anamorphic_squeeze = 1.0;
        let curvature_x = 0.0;
        let curvature_y = 0.0;
        let quartic_distortion = 0.0;

        Self {
            enable: 1,

            focal_length,
            film_back_width,
            film_back_height,
            pixel_aspect,
            lens_center_offset_x,
            lens_center_offset_y,

            distortion,
            anamorphic_squeeze,
            curvature_x,
            curvature_y,
            quartic_distortion,

            plugin,
        }
    }
}

impl Deformer for DeformerTde4Classic {
    fn hash_deformer(&self) -> u64 {
        let mut state = DefaultHasher::default();
        self.hash(&mut state);
        state.finish()
    }

    fn commit_data(&mut self) -> Result<(), String> {
        self.plugin
            .set_parameter_value_f64(PARM_NAME_FOCAL_LENGTH, self.focal_length.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_FILM_BACK_WIDTH, self.film_back_width.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_FILM_BACK_HEIGHT, self.film_back_height.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_PIXEL_ASPECT, self.pixel_aspect.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(
                PARM_NAME_LENS_CENTER_OFFSET_X,
                self.lens_center_offset_x.into(),
            )
            .unwrap();
        self.plugin
            .set_parameter_value_f64(
                PARM_NAME_LENS_CENTER_OFFSET_Y,
                self.lens_center_offset_y.into(),
            )
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_DISTORTION, self.distortion.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_ANAMORPHIC_SQUEEZE, self.anamorphic_squeeze.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_CURVATURE_X, self.curvature_x.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_CURVATURE_Y, self.curvature_y.into())
            .unwrap();
        self.plugin
            .set_parameter_value_f64(PARM_NAME_QUARTIC_DISTORTION, self.quartic_distortion.into())
            .unwrap();
        self.plugin.initialize_parameters()
    }

    /// Tde4Classic lens distortion model.
    ///
    /// xd and yd are expected to be in FOV units.
    ///
    /// Forward == undistort
    fn apply_forward(&self, xd: f32, yd: f32) -> (f32, f32) {
        let (xu, yu) = self.plugin.undistort(xd as f64, yd as f64);
        // warn!("apply_forward: xd={} yd={} -> xu={} yu={}", xd, yd, xu, yu);
        (xu as f32, yu as f32)
    }

    /// Backward == (re)distort
    fn apply_backward(&self, xd: f32, yd: f32) -> (f32, f32) {
        let (xu, yu) = self.plugin.distort(xd as f64, yd as f64);
        // warn!("apply_backward: xd={} yd={} -> xu={} yu={}", xd, yd, xu, yu);
        (xu as f32, yu as f32)
    }

    // Forward == undistort
    fn apply_forward_bounding_box(&self, bbox: BBox2Df, image_window: BBox2Df) -> BBox2Df {
        let bbox_remap = BBox2Df::new(
            mathutils::remap(image_window.min_x, image_window.max_x, 0.0, 1.0, bbox.min_x),
            mathutils::remap(image_window.min_y, image_window.max_y, 0.0, 1.0, bbox.min_y),
            mathutils::remap(image_window.min_x, image_window.max_x, 0.0, 1.0, bbox.max_x),
            mathutils::remap(image_window.min_y, image_window.max_y, 0.0, 1.0, bbox.max_y),
        );
        let (min_x, min_y, max_x, max_y) = self.plugin.get_bounding_box_undistort(
            bbox_remap.min_x as f64,
            bbox_remap.min_y as f64,
            bbox_remap.max_x as f64,
            bbox_remap.max_y as f64,
        );
        // warn!("apply_forward_bounding_box: xd={} yd={} -> xu={} yu={}", xd, yd, xu, yu);
        // Slight extra margin, just in case our
        // getBoundingBox-Methods miss something.
        BBox2Df::new(
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_x,
                image_window.max_x,
                min_x as f32,
            ) - 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_y,
                image_window.max_y,
                min_y as f32,
            ) - 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_x,
                image_window.max_x,
                max_x as f32,
            ) + 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_y,
                image_window.max_y,
                max_y as f32,
            ) + 2.0,
        )
    }

    /// Backward == (re)distort
    fn apply_backward_bounding_box(&self, bbox: BBox2Df, image_window: BBox2Df) -> BBox2Df {
        let bbox_remap = BBox2Df::new(
            mathutils::remap(image_window.min_x, image_window.max_x, 0.0, 1.0, bbox.min_x),
            mathutils::remap(image_window.min_y, image_window.max_y, 0.0, 1.0, bbox.min_y),
            mathutils::remap(image_window.min_x, image_window.max_x, 0.0, 1.0, bbox.max_x),
            mathutils::remap(image_window.min_y, image_window.max_y, 0.0, 1.0, bbox.max_y),
        );
        let (min_x, min_y, max_x, max_y) = self.plugin.get_bounding_box_distort(
            bbox_remap.min_x as f64,
            bbox_remap.min_y as f64,
            bbox_remap.max_x as f64,
            bbox_remap.max_y as f64,
        );
        // warn!("apply_forward_bounding_box: xd={} yd={} -> xu={} yu={}", xd, yd, xu, yu);

        // Slight extra margin, just in case our
        // getBoundingBox-Methods miss something.
        BBox2Df::new(
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_x,
                image_window.max_x,
                min_x as f32,
            ) - 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_y,
                image_window.max_y,
                min_y as f32,
            ) - 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_x,
                image_window.max_x,
                max_x as f32,
            ) + 2.0,
            mathutils::remap(
                0.0,
                1.0,
                image_window.min_y,
                image_window.max_y,
                max_y as f32,
            ) + 2.0,
        )
    }

    fn data_debug_string(&self) -> String {
        debug!("Tde4Classic Debug");
        let model_name = self.plugin.get_model_name().unwrap();
        let num_params = self.plugin.get_num_parameters().unwrap();
        let mut string = format!("Model={} number of params={}", model_name, num_params);
        for i in 0..num_params {
            let parm_name = self.plugin.get_parameter_name(i).unwrap();
            let parm_type = self.plugin.get_parameter_type(&parm_name).unwrap();
            let line = match parm_type {
                ParameterType::Double | ParameterType::AdjustableDouble => {
                    let default_value = self
                        .plugin
                        .get_parameter_default_value_f64(&parm_name)
                        .unwrap();
                    let (min_value, max_value) =
                        self.plugin.get_parameter_range(&parm_name).unwrap();
                    format!(
                        "name={} type={:?} default_value={:?} range={:?} to {:?}",
                        parm_name, parm_type, default_value, min_value, max_value
                    )
                }
                _ => (format!("name={} type={:?}", parm_name, parm_type)),
            };
            debug!("{}", line);
            string.push_str(line.as_str())
        }
        string
    }
}

impl AttrBlock for DeformerTde4Classic {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            // Built in LDPK parameters
            "focal_length" => AttrState::Exists,
            "film_back_width" => AttrState::Exists,
            "film_back_height" => AttrState::Exists,
            "pixel_aspect" => AttrState::Exists,
            "lens_center_offset_x" => AttrState::Exists,
            "lens_center_offset_y" => AttrState::Exists,
            // Distortion parameters.
            "distortion" => AttrState::Exists,
            "anamorphic_squeeze" => AttrState::Exists,
            "curvature_x" => AttrState::Exists,
            "curvature_y" => AttrState::Exists,
            "quartic_distortion" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        ()
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            // Built in LDPK parameters
            "focal_length" => self.focal_length,
            "film_back_width" => self.film_back_width,
            "film_back_height" => self.film_back_height,
            "pixel_aspect" => self.pixel_aspect,
            "lens_center_offset_x" => self.lens_center_offset_x,
            "lens_center_offset_y" => self.lens_center_offset_y,

            // Distortion parameters.
            "distortion" => self.distortion,
            "anamorphic_squeeze" => self.anamorphic_squeeze,
            "curvature_x" => self.curvature_x,
            "curvature_y" => self.curvature_y,
            "quartic_distortion" => self.quartic_distortion,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            // Built in LDPK parameters
            "focal_length" => self.focal_length = value,
            "film_back_width" => self.film_back_width = value,
            "film_back_height" => self.film_back_height = value,
            "pixel_aspect" => self.pixel_aspect = value,
            "lens_center_offset_x" => self.lens_center_offset_x = value,
            "lens_center_offset_y" => self.lens_center_offset_y = value,

            // Distortion parameters.
            "distortion" => self.distortion = value,
            "anamorphic_squeeze" => self.anamorphic_squeeze = value,
            "curvature_x" => self.curvature_x = value,
            "curvature_y" => self.curvature_y = value,
            "quartic_distortion" => self.quartic_distortion = value,

            _ => (),
        }
    }
}