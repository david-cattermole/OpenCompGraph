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

use crate::colorop::ColorOp;

pub fn apply_color_ops_to_pixels(
    color_ops: &Vec<Box<dyn ColorOp>>,
    pixels: &mut [f32],
    num_channels: i32,
) {
    assert!(num_channels > 0);

    if color_ops.len() == 0 {
        return;
    }
    let enabled_color_ops: Vec<_> = color_ops
        .iter()
        .filter(|x| x.get_attr_i32("enable") == 1)
        .collect();
    if enabled_color_ops.len() == 0 {
        return;
    }

    for color_op in enabled_color_ops {
        color_op.apply_slice_in_place(pixels, num_channels);
    }
}
