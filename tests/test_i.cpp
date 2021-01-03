#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

void generate_mesh(const uint32_t divisions_x,
                   const uint32_t divisions_y,
                   const char* file_path,
                   size_t &pos_count,
                   size_t &uv_count,
                   // size_t &line_count,
                   size_t &tri_count) {
    assert(divisions_x > 0);
    assert(divisions_y > 0);
    std::cout << "divisions: " << divisions_x << "x" << divisions_y << '\n';
    pos_count = ocg::internal::calc_buffer_size_vertex_positions(divisions_x, divisions_y);
    uv_count = ocg::internal::calc_buffer_size_vertex_uvs(divisions_x, divisions_y);
    // line_count = ocg::internal::calc_buffer_size_index_lines(divisions_x, divisions_y);
    tri_count = ocg::internal::calc_buffer_size_index_tris(divisions_x, divisions_y);
    std::cout << "position count: " << pos_count << '\n';
    std::cout << "      uv count: " << uv_count << '\n';
    // std::cout << "    line count: " << line_count << '\n';
    std::cout << "triangle count: " << tri_count << '\n';

    // Allocate exactly enough memory
    std::vector<float> vertex_pos;
    std::vector<float> vertex_uvs;
    std::vector<uint32_t> index_tris;
    // std::vector<uint32_t> index_lines;
    vertex_pos.reserve(pos_count);
    vertex_uvs.reserve(uv_count);
    index_tris.reserve(tri_count);
    // index_lines.reserve(line_count);

    // Generate mesh.
    rust::Slice<float> slice_vertex_pos{vertex_pos.data(), vertex_pos.capacity()};
    rust::Slice<float> slice_vertex_uvs{vertex_uvs.data(), vertex_uvs.capacity()};
    rust::Slice<uint32_t> slice_index_tris{index_tris.data(), index_tris.capacity()};
    // rust::Slice<uint32_t> slice_index_lines{index_lines.data(), index_lines.capacity()};
    ocg::internal::fill_all_buffers(
        divisions_x,
        divisions_y,
        slice_vertex_pos,
        slice_vertex_uvs,
        slice_index_tris
        // slice_index_lines
        );

    // Export
    ocg::internal::export_mesh(
        slice_vertex_pos,
        slice_vertex_uvs,
        slice_index_tris,
        file_path);
}

int test_i() {
    std::cout << "=========================================== test_i()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    // size_t line_count = 0;
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
        // line_count,
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
        // line_count,
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
        // line_count,
        tri_count);

    bench.stop();
    bench.print("Test I:");

    return 0;
}
