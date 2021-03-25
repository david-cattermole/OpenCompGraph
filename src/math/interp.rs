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

use log::{debug, error, info, warn};
use num_traits::Float;
use num_traits::NumCast;

/// Return 'min_value' to 'max_value' linearly, for a 'mix' value
/// between 0.0 and 1.0.
pub fn lerp<T: Float>(min_value: T, max_value: T, mix: T) -> T {
    let one: T = NumCast::from(1).unwrap();
    (one - mix) * min_value + mix * max_value
}

/// Return 0.0 to 1.0 linearly, for a 'mix' value between 'min_value'
/// and 'max_value'.
pub fn inverse_lerp<T: Float>(min_value: T, max_value: T, mix: T) -> T {
    (mix - min_value) / (max_value - min_value)
}

/// Remap from the 'old_*' values to 'new_*' values, using a 'mix'
/// value between 0.0 and 1.0;
pub fn remap<T: Float>(old_min: T, old_max: T, new_min: T, new_max: T, mix: T) -> T {
    let blend = inverse_lerp::<T>(old_min, old_max, mix);
    lerp::<T>(new_min, new_max, blend)
}
