use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct GeometryPlaneImpl {
    divisions_x: u32,
    divisions_y: u32,
}

impl GeometryPlaneImpl {
    pub fn new(divisions_x: u32, divisions_y: u32) -> GeometryPlaneImpl {
        GeometryPlaneImpl {
            divisions_x,
            divisions_y,
        }
    }

    pub fn divisions_x(&self) -> u32 {
        self.divisions_x
    }

    pub fn divisions_y(&self) -> u32 {
        self.divisions_y
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

    /// Vertex Buffer Positions
    pub fn fill_buffer_vertex_positions(&self, buffer: &mut [f32]) -> bool {
        let mut index = 0;
        let per_vertex_num = 3;
        let square_size_x: f32 = 1.0 / ((self.divisions_x - 1) as f32);
        let square_size_y: f32 = 1.0 / ((self.divisions_y - 1) as f32);
        for row in 0..self.divisions_y {
            for col in 0..self.divisions_x {
                buffer[index + 0] = -0.5 + ((col as f32) * square_size_x);
                buffer[index + 1] = -0.5 + ((row as f32) * square_size_y);
                buffer[index + 2] = 0.0;
                index += per_vertex_num;
            }
        }
        true
    }

    /// Vertex Buffer UVs
    pub fn fill_buffer_vertex_uvs(&self, buffer: &mut [f32]) -> bool {
        let mut index = 0;
        let per_vertex_num = 2;
        let square_size_x: f32 = 1.0 / ((self.divisions_x - 1) as f32);
        let square_size_y: f32 = 1.0 / ((self.divisions_y - 1) as f32);
        for row in 0..self.divisions_y {
            for col in 0..self.divisions_x {
                // Flip Y coordinates because the texture data starts at the
                // top-left, but our UVs start at bottom-left.
                buffer[index + 0] = (col as f32) * square_size_x;
                buffer[index + 1] = 1.0 + ((row as f32) * square_size_y * -1.0);
                index += per_vertex_num;
            }
        }
        true
    }

    /// Index Buffer Triangles
    pub fn fill_buffer_index_tris(&self, buffer: &mut [u32]) -> bool {
        let mut index = 0;
        for col in 0..(self.divisions_y - 1) {
            for row in 0..(self.divisions_x - 1) {
                let index_top_left = ((col * self.divisions_x) + row) as u32;
                let index_bottom_left = (index_top_left + self.divisions_x) as u32;
                let index_top_right = index_top_left + 1;
                let index_bottom_right = index_bottom_left + 1;

                // First triangle
                buffer[index + 0] = index_top_left;
                buffer[index + 1] = index_bottom_left;
                buffer[index + 2] = index_bottom_right;

                // Second triangle
                buffer[index + 3] = index_top_left;
                buffer[index + 4] = index_bottom_right;
                buffer[index + 5] = index_top_right;

                // We have adjusted 6 indexes, so lets move to the next 6.
                index += 6;
            }
        }
        true
    }
}

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
