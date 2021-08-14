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

#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;


void generate_mesh(const bool debug_print,
                   const uint32_t divisions_x,
                   const uint32_t divisions_y,
                   ocg::StreamData &stream_data,
                   const char* file_path,
                   size_t &pos_count,
                   size_t &uv_count,
                   size_t &tri_count) {
    assert(divisions_x > 1);
    assert(divisions_y > 1);
    if (debug_print) {
        std::cout << "divisions: " << divisions_x << "x" << divisions_y << '\n';
    }

    auto center_x = -0.5f;
    auto center_y = -0.5f;
    auto size_x = 1.0f;
    auto size_y = 1.0f;
    auto geom = ocg::internal::create_geometry_plane_box(
        center_x, center_y,
        size_x, size_y,
        divisions_x, divisions_y);

    pos_count = geom->calc_buffer_size_vertex_positions();
    uv_count = geom->calc_buffer_size_vertex_uvs();
    tri_count = geom->calc_buffer_size_index_tris();
    if (debug_print) {
        std::cout << "position count: " << pos_count << '\n';
        std::cout << "      uv count: " << uv_count << '\n';
        std::cout << "triangle count: " << tri_count << '\n';
    }

    // Allocate exactly enough memory
    std::vector<float> vertex_pos;
    std::vector<float> vertex_uvs;
    std::vector<uint32_t> index_tris;
    vertex_pos.reserve(pos_count);
    vertex_uvs.reserve(uv_count);
    index_tris.reserve(tri_count);

    // Generate mesh data.
    rust::Slice<float> slice_vertex_pos{vertex_pos.data(), vertex_pos.capacity()};
    rust::Slice<float> slice_vertex_uvs{vertex_uvs.data(), vertex_uvs.capacity()};
    rust::Slice<uint32_t> slice_index_tris{index_tris.data(), index_tris.capacity()};
    geom->fill_buffer_vertex_positions(slice_vertex_pos);    
    geom->fill_buffer_vertex_uvs(slice_vertex_uvs);
    geom->fill_buffer_index_tris(slice_index_tris);

    // Apply deformations
    if (stream_data.deformers_len() > 0) {
        auto display_window = ocg::BBox2Df();
        display_window.min_x = -0.5;
        display_window.min_y = -0.5;
        display_window.max_x = +0.5;
        display_window.max_y = +0.5;

        auto data_window = ocg::BBox2Df();
        data_window.min_x = -0.5;
        data_window.min_y = -0.5;
        data_window.max_x = +0.5;
        data_window.max_y = +0.5;
        stream_data.apply_deformers(
            slice_vertex_pos,
            display_window,
            data_window);
    }

    // TODO: Apply the image 2D transforms (from the StreamData).

    // TODO: Apply the 3D transform (from the StreamData).

    // Export
    rust::Slice<const float> const_slice_vertex_pos(
        vertex_pos.data(), vertex_pos.capacity());
    rust::Slice<const float> const_slice_vertex_uvs(
        vertex_uvs.data(), vertex_uvs.capacity());
    rust::Slice<const uint32_t> const_slice_index_tris(
        index_tris.data(), index_tris.capacity());
    ocg::internal::export_mesh(
        const_slice_vertex_pos,
        const_slice_vertex_uvs,
        const_slice_index_tris,
        file_path);
}
