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

use log::debug;
use std::time::Instant;

use crate::cxxbridge::ffi::oiio_image_resample;
use crate::cxxbridge::ffi::ImageShared;

pub fn image_resample(
    src_image: &mut ImageShared,
    dst_image: &mut ImageShared,
    factor_num: i32,
    interpolate: bool,
) -> bool {
    let start = Instant::now();
    let ok = oiio_image_resample(src_image, dst_image, factor_num, interpolate);
    let duration = start.elapsed();
    debug!("Total time: {:?}", duration);
    ok
}
