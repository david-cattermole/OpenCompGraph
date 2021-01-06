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

    auto geom = ocg::internal::create_geometry_plane_box(divisions_x, divisions_y);

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

    // Generate mesh.
    rust::Slice<float> slice_vertex_pos{vertex_pos.data(), vertex_pos.capacity()};
    rust::Slice<float> slice_vertex_uvs{vertex_uvs.data(), vertex_uvs.capacity()};
    rust::Slice<uint32_t> slice_index_tris{index_tris.data(), index_tris.capacity()};
    geom->fill_buffer_vertex_positions(slice_vertex_pos);    
    geom->fill_buffer_vertex_uvs(slice_vertex_uvs);
    geom->fill_buffer_index_tris(slice_index_tris);

    // Apply deformations
    if (stream_data.deformers_len() > 0) {
        stream_data.apply_deformers(slice_vertex_pos);
    }

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
