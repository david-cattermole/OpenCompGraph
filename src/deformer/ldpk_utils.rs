/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use cxx::UniquePtr;
use std::cell::RefCell;
use std::fmt;
use std::os::raw::c_char;
// use log::debug;

use crate::cxxbridge::ffi::ldpk_new_plugin;
use crate::cxxbridge::ffi::OcgLdPluginBase;
use crate::cxxbridge::ffi::ParameterType;

// #[derive(Clone, Debug)]
pub struct LensDistortionPlugin {
    /// The internal C++ plug-in.
    //
    // Unfortunately the internal C++ plug-in has mutable state which
    // is often changed even when calling "get" functions. The C++
    // plug-in has internal LUTs and other such optimisation features
    // that internally mutate the state, too. We wrap the plug-in with
    // a RefCell to allow rust to pretend the mutation is not happening.
    plugin: RefCell<UniquePtr<OcgLdPluginBase>>,
}

impl fmt::Debug for LensDistortionPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LensDistortionPlugin")
    }
}

/// Clone the lens distortion plugin.
///
/// NOTE: This does NOT clone the underlying lens distortion plug-in,
/// and in fact only creates a new default class instance.
//
/// It is expected that after a clone happens the parameters (saved
/// elsewhere) will be set again, and re-initialised before the
/// plug-in is used.
impl Clone for LensDistortionPlugin {
    fn clone(&self) -> LensDistortionPlugin {
        LensDistortionPlugin::new()
    }
}

impl LensDistortionPlugin {
    pub fn new() -> Self {
        LensDistortionPlugin {
            plugin: RefCell::new(ldpk_new_plugin()),
        }
    }

    pub fn get_model_name(&self) -> Result<String, String> {
        let mut model_name = [0 as c_char as u8; 100];
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_model_name(&mut model_name[..]);
        }
        // debug!("rust model_name: {:?}", model_name);
        let s = String::from_utf8_lossy(&model_name);
        // debug!("rust result: {}", s);
        match ok {
            true => Ok(s.to_string()),
            false => Err("Could not model name.".to_string()),
        }
    }

    pub fn get_num_parameters(&self) -> Result<u32, String> {
        let mut value = 0;
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_num_parameters(&mut value);
        }
        // debug!("rust num_parameters: {:?}", value);
        match ok {
            true => Ok(value as u32),
            false => Err("Could not get number of parameters.".to_string()),
        }
    }

    pub fn get_parameter_name(&self, index: u32) -> Result<String, String> {
        let mut parameter_name = [0 as c_char as u8; 100];
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_parameter_name(index as i32, &mut parameter_name[..]);
        }
        // debug!("rust parameter_name: {:?}", parameter_name);
        let s = String::from_utf8_lossy(&parameter_name);
        // debug!("rust result: {}", s);
        match ok {
            true => Ok(s.to_string()),
            false => Err("Could not get parameter name.".to_string()),
        }
    }

    pub fn get_parameter_type(&self, name: &str) -> Result<ParameterType, String> {
        let mut ok = false;
        let mut param_type = ParameterType::Uninitialized;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_parameter_type(name, &mut param_type.repr);
        }
        // debug!("rust param_type: {:?}", param_type);
        match ok {
            true => Ok(param_type),
            false => Err("Could not get parameter name.".to_string()),
        }
    }

    pub fn get_parameter_default_value_f64(&self, name: &str) -> Result<f64, String> {
        let mut ok = false;
        let mut value = 0.0;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_parameter_default_value_f64(name, &mut value);
        }
        // debug!("rust param default_value f64: {:?}", value);
        match ok {
            true => Ok(value),
            false => Err(format!("Could not get parameter default value: {}.", name).to_string()),
        }
    }

    pub fn get_parameter_range(&self, name: &str) -> Result<(f64, f64), String> {
        let mut ok = false;
        let mut min_value = 0.0;
        let mut max_value = 0.0;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.get_parameter_range(name, &mut min_value, &mut max_value);
        }
        // debug!("rust param range f64: {:?} to {:?}", min_value, max_value);
        match ok {
            true => Ok((min_value, max_value)),
            false => Err(format!("Could not get parameter range: {}.", name).to_string()),
        }
    }

    pub fn set_parameter_value_f64(&mut self, name: &str, value: f64) -> Result<(), String> {
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.set_parameter_value_f64(name, value);
        }
        match ok {
            true => Ok(()),
            false => Err(format!("Could not set parameter: {}.", name).to_string()),
        }
    }

    pub fn initialize_parameters(&self) -> Result<(), String> {
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.initialize_parameters();
        }
        // debug!("initialize_parameters: {}", ok);
        match ok {
            true => Ok(()),
            false => Err("Could not initialize parameters.".to_string()),
        }
    }

    pub fn undistort(&self, x: f64, y: f64) -> (f64, f64) {
        let mut _ok = false;
        let mut x_out = 0.0;
        let mut y_out = 0.0;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            _ok = plugin_obj.undistort(x, y, &mut x_out, &mut y_out);
        }
        // debug!(
        //     "rust param undistort: {:?} to {:?} -> {:?} to {:?} | {}",
        //     x, y, x_out, y_out, _ok
        // );
        (x_out, y_out)
    }

    pub fn distort(&self, x: f64, y: f64) -> (f64, f64) {
        let mut x_out = 0.0;
        let mut y_out = 0.0;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            plugin_obj.distort(x, y, &mut x_out, &mut y_out);
        }
        // debug!("rust param distort: {:?} to {:?}", x_out, y_out);
        (x_out, y_out)
    }

    pub fn distort_with_guess(&self, x: f64, y: f64, x_start: f64, y_start: f64) -> (f64, f64) {
        let mut x_out = 0.0;
        let mut y_out = 0.0;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            plugin_obj.distort_with_guess(x, y, x_start, y_start, &mut x_out, &mut y_out);
        }
        // debug!("rust param distort_with_guess: {:?} to {:?}", x_out, y_out);
        (x_out, y_out)
    }

    pub fn provides_parameter_derivatives(&self) -> Result<(), String> {
        let mut ok = false;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            ok = plugin_obj.provides_parameter_derivatives();
        }
        match ok {
            true => Ok(()),
            false => Err(
                "Could not determine if the lens model provides parameter derivatives".to_string(),
            ),
        }
    }

    pub fn get_bounding_box_undistort(
        &self,
        xa: f64,
        ya: f64,
        xb: f64,
        yb: f64,
    ) -> (f64, f64, f64, f64) {
        // 32x32 is the default LDPK value. 8x8 is used in the Nuke
        // LDPK nodes.
        let samples_x = 8;
        let samples_y = 8;

        let mut xa_out = xa;
        let mut ya_out = ya;
        let mut xb_out = xb;
        let mut yb_out = yb;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            plugin_obj.get_bounding_box_undistort(
                xa,
                ya,
                xb,
                yb,
                &mut xa_out,
                &mut ya_out,
                &mut xb_out,
                &mut yb_out,
                samples_x,
                samples_y,
            );
        }
        // debug!(
        //     "rust param get bounding_box_undistort: {:?} {:?} {:?} {:?}",
        //     xa_out, ya_out, xb_out, yb_out
        // );
        (xa_out, ya_out, xb_out, yb_out)
    }

    pub fn get_bounding_box_distort(
        &self,
        xa: f64,
        ya: f64,
        xb: f64,
        yb: f64,
    ) -> (f64, f64, f64, f64) {
        // 32x32 is the default LDPK value. 8x8 is used in the Nuke
        // LDPK nodes.
        let samples_x = 8;
        let samples_y = 8;

        let mut xa_out = xa;
        let mut ya_out = ya;
        let mut xb_out = xb;
        let mut yb_out = yb;
        if let Some(plugin_obj) = self.plugin.borrow_mut().as_mut() {
            plugin_obj.get_bounding_box_distort(
                xa,
                ya,
                xb,
                yb,
                &mut xa_out,
                &mut ya_out,
                &mut xb_out,
                &mut yb_out,
                samples_x,
                samples_y,
            );
        }
        // debug!(
        //     "rust param get bounding_box_distort: {:?} {:?} {:?} {:?}",
        //     xa_out, ya_out, xb_out, yb_out
        // );
        (xa_out, ya_out, xb_out, yb_out)
    }
}
