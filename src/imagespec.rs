/*
 * Copyright (C) 2021 David Cattermole.
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

use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::ImageOrientation;
use crate::cxxbridge::ffi::ImageSpec;
use crate::hashutils;

impl ImageSpec {
    pub fn new() -> ImageSpec {
        ImageSpec {
            color_space: "Linear".to_string(),
            gamma: 1.0,
            pixel_aspect: 1.0,
            orientation: ImageOrientation::Normal,
            unassociated_alpha: false,
            dither: 0,
        }
    }

    pub fn color_space(&self) -> String {
        self.color_space.clone()
    }

    pub fn set_color_space(&mut self, value: String) {
        self.color_space = value;
    }

    pub fn gamma(&self) -> f32 {
        self.gamma
    }

    pub fn set_gamma(&mut self, value: f32) {
        self.gamma = value;
    }

    pub fn pixel_aspect(&self) -> f32 {
        self.pixel_aspect
    }

    pub fn set_pixel_aspect(&mut self, value: f32) {
        self.pixel_aspect = value;
    }

    pub fn orientation(&self) -> ImageOrientation {
        self.orientation
    }

    pub fn set_orientation(&mut self, value: ImageOrientation) {
        self.orientation = value;
    }

    pub fn unassociated_alpha(&self) -> bool {
        self.unassociated_alpha
    }

    pub fn set_unassociated_alpha(&mut self, value: bool) {
        self.unassociated_alpha = value;
    }

    pub fn dither(&self) -> i32 {
        self.dither
    }

    pub fn set_dither(&mut self, value: i32) {
        self.dither = value;
    }
}

impl Hash for ImageSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color_space.hash(state);
        hashutils::HashableF32::new(self.gamma).hash(state);
        hashutils::HashableF32::new(self.pixel_aspect).hash(state);
        self.orientation.hash(state);
        self.unassociated_alpha.hash(state);
        self.dither.hash(state);
    }
}
