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

use crate::math;
use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct GeometryPlaneImpl {
    center_x: f32,
    center_y: f32,
    size_x: f32,
    size_y: f32,
    divisions_x: u32,
    divisions_y: u32,
}

impl GeometryPlaneImpl {
    pub fn new(
        center_x: f32,
        center_y: f32,
        size_x: f32,
        size_y: f32,
        divisions_x: u32,
        divisions_y: u32,
    ) -> GeometryPlaneImpl {
        GeometryPlaneImpl {
            center_x,
            center_y,
            size_x,
            size_y,
            divisions_x,
            divisions_y,
        }
    }

    pub fn center_x(&self) -> f32 {
        self.center_x
    }

    pub fn center_y(&self) -> f32 {
        self.center_y
    }

    pub fn size_x(&self) -> f32 {
        self.size_x
    }

    pub fn size_y(&self) -> f32 {
        self.size_y
    }

    pub fn divisions_x(&self) -> u32 {
        self.divisions_x
    }

    pub fn divisions_y(&self) -> u32 {
        self.divisions_y
    }

    pub fn set_center_x(&mut self, value: f32) {
        self.center_x = value
    }

    pub fn set_center_y(&mut self, value: f32) {
        self.center_y = value
    }

    pub fn set_size_x(&mut self, value: f32) {
        self.size_x = value
    }

    pub fn set_size_y(&mut self, value: f32) {
        self.size_y = value
    }

    pub fn set_divisions_x(&mut self, value: u32) {
        self.divisions_x = value
    }

    pub fn set_divisions_y(&mut self, value: u32) {
        self.divisions_y = value
    }

    pub fn calc_count_vertex_positions(&self) -> usize {
        (self.divisions_x * self.divisions_y) as usize
    }

    pub fn calc_count_vertex_uvs(&self) -> usize {
        self.calc_count_vertex_positions()
    }

    pub fn calc_buffer_size_vertex_positions(&self) -> usize {
        let number_of_elements = 3;
        let length = self.calc_count_vertex_positions();
        (length * number_of_elements) as usize
    }

    pub fn calc_buffer_size_vertex_uvs(&self) -> usize {
        let number_of_elements = 2;
        let length = self.calc_count_vertex_uvs();
        (length * number_of_elements) as usize
    }

    pub fn calc_buffer_size_index_tris(&self) -> usize {
        let number_of_faces = ((self.divisions_x - 1) * (self.divisions_y - 1)) as usize;
        let tris_per_face = 2;
        let indexes_per_tri = 3;
        tris_per_face * indexes_per_tri * number_of_faces
    }

    pub fn calc_buffer_size_index_border_lines(&self) -> usize {
        let number_of_faces =
            (((self.divisions_x - 1) * 2) + ((self.divisions_y - 1) * 2)) as usize;
        let indexes_per_line = 2;
        indexes_per_line * number_of_faces
    }

    pub fn calc_buffer_size_index_wire_lines(&self) -> usize {
        let number_of_faces = ((self.divisions_x - 1) * (self.divisions_y - 1)) as usize;
        let lines_per_face = 4;
        let indexes_per_line = 2;
        lines_per_face * indexes_per_line * number_of_faces
    }

    fn get_indexes(&self, row: u32, col: u32) -> (u32, u32, u32, u32) {
        let top_left = ((row * self.divisions_x) + col) as u32;
        let bottom_left = (top_left + self.divisions_x) as u32;
        let top_right = top_left + 1;
        let bottom_right = bottom_left + 1;
        (top_left, bottom_left, top_right, bottom_right)
    }

    /// Vertex Buffer Positions
    pub fn fill_buffer_vertex_positions(&self, buffer: &mut [f32]) -> bool {
        let mut index = 0;
        let mut max_x = self.size_x / 2.0;
        let mut min_x = -max_x;
        let mut max_y = self.size_y / 2.0;
        let mut min_y = -max_y;
        let square_size_x: f32 = 1.0 / ((self.divisions_x - 1) as f32);
        let square_size_y: f32 = 1.0 / ((self.divisions_y - 1) as f32);
        for row in 0..self.divisions_y {
            for col in 0..self.divisions_x {
                let x = (col as f32) * square_size_x;
                let y = (row as f32) * square_size_y;
                let pos_x = math::interp::lerp(min_x, max_x, x) - self.center_x;
                let pos_y = math::interp::lerp(min_y, max_y, y) - self.center_y;
                buffer[index + 0] = pos_x;
                buffer[index + 1] = pos_y;
                buffer[index + 2] = 0.0;
                index += 3;
            }
        }
        true
    }

    /// Vertex Buffer UVs
    pub fn fill_buffer_vertex_uvs(&self, buffer: &mut [f32]) -> bool {
        let mut index = 0;
        let square_size_x: f32 = 1.0 / ((self.divisions_x - 1) as f32);
        let square_size_y: f32 = 1.0 / ((self.divisions_y - 1) as f32);
        for row in 0..self.divisions_y {
            for col in 0..self.divisions_x {
                // Flip Y coordinates because the texture data starts at the
                // top-left, but our UVs start at bottom-left.
                buffer[index + 0] = (col as f32) * square_size_x;
                buffer[index + 1] = 1.0 + ((row as f32) * square_size_y * -1.0);
                index += 2;
            }
        }
        true
    }

    /// Index Buffer Triangles
    pub fn fill_buffer_index_tris(&self, buffer: &mut [u32]) -> bool {
        let mut index = 0;
        for row in 0..(self.divisions_y - 1) {
            for col in 0..(self.divisions_x - 1) {
                let (top_left, bottom_left, top_right, bottom_right) = self.get_indexes(row, col);

                // First triangle
                buffer[index + 0] = top_left;
                buffer[index + 1] = bottom_left;
                buffer[index + 2] = bottom_right;

                // Second triangle
                buffer[index + 3] = top_left;
                buffer[index + 4] = bottom_right;
                buffer[index + 5] = top_right;

                // We have adjusted 6 indexes, so lets move to the next 6.
                index += 6;
            }
        }
        true
    }

    /// Index Buffer Border Lines
    pub fn fill_buffer_index_border_lines(&self, buffer: &mut [u32]) -> bool {
        let mut index = 0;

        // Left line.
        let left_column = 0;
        for row in 0..(self.divisions_y - 1) {
            let (top_left, bottom_left, top_right, bottom_right) =
                self.get_indexes(row, left_column);
            buffer[index + 0] = top_left;
            buffer[index + 1] = bottom_left;
            index += 2;
        }

        // Right line.
        let right_column = self.divisions_x - 1;
        for row in 0..(self.divisions_y - 1) {
            let (top_left, bottom_left, top_right, bottom_right) =
                self.get_indexes(row, right_column);
            buffer[index + 0] = top_left;
            buffer[index + 1] = bottom_left;
            index += 2;
        }

        // Top line.
        let top_row = self.divisions_y - 1;
        for col in 0..(self.divisions_x - 1) {
            let (top_left, bottom_left, top_right, bottom_right) = self.get_indexes(top_row, col);
            buffer[index + 0] = top_left;
            buffer[index + 1] = top_right;
            index += 2;
        }

        // Bottom line.
        let bottom_row = 0;
        for col in 0..(self.divisions_x - 1) {
            let (top_left, bottom_left, top_right, bottom_right) =
                self.get_indexes(bottom_row, col);
            buffer[index + 0] = top_left;
            buffer[index + 1] = top_right;
            index += 2;
        }

        true
    }

    /// Index Buffer Wire Lines
    pub fn fill_buffer_index_wire_lines(&self, buffer: &mut [u32]) -> bool {
        let mut index = 0;
        for col in 0..(self.divisions_y - 1) {
            for row in 0..(self.divisions_x - 1) {
                let (top_left, bottom_left, top_right, bottom_right) = self.get_indexes(col, row);

                // First line
                buffer[index + 0] = top_left;
                buffer[index + 1] = bottom_left;

                // Second line
                buffer[index + 2] = bottom_left;
                buffer[index + 3] = bottom_right;

                // Third line
                buffer[index + 4] = bottom_right;
                buffer[index + 5] = top_right;

                // Fourth line
                buffer[index + 6] = top_right;
                buffer[index + 7] = top_left;

                index += 8;
            }
        }
        true
    }
}

pub fn create_geometry_plane_box(
    center_x: f32,
    center_y: f32,
    size_x: f32,
    size_y: f32,
    divisions_x: u32,
    divisions_y: u32,
) -> Box<GeometryPlaneImpl> {
    debug!("create_geometry_plane_box()");
    Box::new(GeometryPlaneImpl::new(
        center_x,
        center_y,
        size_x,
        size_y,
        divisions_x,
        divisions_y,
    ))
}
