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
