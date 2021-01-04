#include <iostream>
#include <opencompgraph.h>


namespace ocg = open_comp_graph;

void generate_mesh(const uint32_t divisions_x,
                   const uint32_t divisions_y,
                   const char* file_path,
                   size_t &pos_count,
                   size_t &uv_count,
                   size_t &tri_count) {
    assert(divisions_x > 1);
    assert(divisions_y > 1);
    std::cout << "divisions: " << divisions_x << "x" << divisions_y << '\n';
    pos_count = ocg::internal::calc_buffer_size_vertex_positions(
        divisions_x, divisions_y);
    uv_count = ocg::internal::calc_buffer_size_vertex_uvs(
        divisions_x, divisions_y);
    tri_count = ocg::internal::calc_buffer_size_index_tris(
        divisions_x, divisions_y);
    std::cout << "position count: " << pos_count << '\n';
    std::cout << "      uv count: " << uv_count << '\n';
    std::cout << "triangle count: " << tri_count << '\n';

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
    ocg::internal::fill_buffer_vertex_positions(
        divisions_x,
        divisions_y,
        slice_vertex_pos);
    ocg::internal::fill_buffer_vertex_uvs(
        divisions_x,
        divisions_y,
        slice_vertex_uvs);
    ocg::internal::fill_buffer_index_tris(
        divisions_x,
        divisions_y,
        slice_index_tris);

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

int test_i() {
    std::cout << "=========================================== test_i()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    size_t tri_count = 0;

    // 2 x 2 divisions = 1 face (2 triangles).
    divisions_x = 2;
    divisions_y = 2;
    const char* file_path_2x2 = "./tests/data/out/test_i_2x2_out.obj";
    generate_mesh(
        divisions_x,
        divisions_y,
        file_path_2x2,
        pos_count,
        uv_count,
        tri_count);

    // 3 x 3 divisions = 4 faces (8 triangles).
    divisions_x = 3;
    divisions_y = 3;
    const char* file_path_3x3 = "./tests/data/out/test_i_3x3_out.obj";
    generate_mesh(
        divisions_x,
        divisions_y,
        file_path_3x3,
        pos_count,
        uv_count,
        tri_count);

    // 4 x 4 divisions = 9 faces (18 triangles).
    divisions_x = 4;
    divisions_y = 4;
    const char* file_path_4x4 = "./tests/data/out/test_i_4x4_out.obj";
    generate_mesh(
        divisions_x,
        divisions_y,
        file_path_4x4,
        pos_count,
        uv_count,
        tri_count);

    bench.stop();
    bench.print("Test I:");

    return 0;
}
