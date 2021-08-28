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

use std::collections::hash_map::DefaultHasher;

use crate::cxxbridge::ffi::AttrState;
use crate::data::FrameValue;

pub trait AttrBlock: std::fmt::Debug {
    fn attr_hash(&self, frame: FrameValue, state: &mut DefaultHasher);

    fn attr_exists(&self, name: &str) -> AttrState;

    fn get_attr_str(&self, name: &str) -> &str;
    fn set_attr_str(&mut self, name: &str, value: &str);

    fn get_attr_i32(&self, name: &str) -> i32;
    fn set_attr_i32(&mut self, name: &str, value: i32);

    fn get_attr_f32(&self, name: &str) -> f32;
    fn set_attr_f32(&mut self, name: &str, value: f32);
}
