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
pub mod plane;

/// Export arrays of data as an .obj file - for debug.
pub fn export_mesh(positions: &[f32], uvs: &[f32], indices: &[u32], file_path: &str) {
    debug!("Exporting Mesh: {}", file_path);
    // debug!("positions: size={} {:#?}", positions.len(), positions);
    // debug!("indices: size={} {:#?}", indices.len(), indices);
    let mut data =
        "# This file uses centimeters as units for non-parametric coordinates.\n\n".to_string();

    let vert_count = positions.len() / 3;
    debug!("vert_count: {}", vert_count);
    for i in 0..vert_count {
        let index = i * 3;
        let x = positions[index + 0];
        let y = positions[index + 1];
        let z = positions[index + 2];
        data.push_str(&format!("v {} {} {}\n", x, y, z));
    }

    let uv_count = uvs.len() / 2;
    debug!("uv_count: {}", uv_count);
    for i in 0..uv_count {
        let index = i * 2;
        let x = uvs[index + 0];
        let y = uvs[index + 1];
        data.push_str(&format!("vt {} {}\n", x, y));
    }

    let tri_count = indices.len() / 3;
    debug!("tri_count: {}", tri_count);
    let mut tri_index = 0;
    for i in 0..tri_count {
        // Note: Wavefront OBJ is a one-based format, so we must add 1
        // to the face index number.
        let x = 1 + indices[tri_index + 0];
        let y = 1 + indices[tri_index + 1];
        let z = 1 + indices[tri_index + 2];
        data.push_str(&format!("f {0}/{0} {1}/{1} {2}/{2}\n", x, y, z));
        tri_index += 3;
    }

    // Write the ASCII string to disk.
    std::fs::write(file_path, data).unwrap();
}
