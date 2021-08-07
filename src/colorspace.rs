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

use crate::cxxbridge::ffi::oiio_color_convert_inplace;

pub fn color_convert_inplace(
    pixel_data: &mut [f32],
    width: i32,
    height: i32,
    num_channels: i32,
    alpha_channel_index: i32,
    unassociated_alpha: bool,
    src_color_space: &String,
    dst_color_space: &String,
) -> bool {
    oiio_color_convert_inplace(
        pixel_data,
        width,
        height,
        num_channels,
        alpha_channel_index,
        unassociated_alpha,
        src_color_space,
        dst_color_space,
    )
}
