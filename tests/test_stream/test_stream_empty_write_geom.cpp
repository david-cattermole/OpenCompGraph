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
 * Use an empty StreamData class, and write out geometry using the
 * empty stream.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_mesh.h"
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;


int test_stream_empty_write_geom(const bool debug_print) {
    if (debug_print) {
        std::cout << "================= test_stream_empty_write_geom()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto stream_data = ocg::StreamData();
    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    size_t tri_count = 0;

    // 2 x 2 divisions = 1 face (2 triangles).
    divisions_x = 2;
    divisions_y = 2;
    const char* file_path_2x2 = "./tests/data/out/test_stream_empty_write_geom_2x2_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_2x2,
        pos_count,
        uv_count,
        tri_count);

    // 3 x 3 divisions = 4 faces (8 triangles).
    divisions_x = 3;
    divisions_y = 3;
    const char* file_path_3x3 = "./tests/data/out/test_stream_empty_write_geom_3x3_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_3x3,
        pos_count,
        uv_count,
        tri_count);

    // 4 x 4 divisions = 9 faces (18 triangles).
    divisions_x = 4;
    divisions_y = 4;
    const char* file_path_4x4 = "./tests/data/out/test_stream_empty_write_geom_4x4_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_4x4,
        pos_count,
        uv_count,
        tri_count);

    // 2 x 4 divisions = 8 faces (16 triangles).
    divisions_x = 2;
    divisions_y = 4;
    const char* file_path_2x4 = "./tests/data/out/test_stream_empty_write_geom_2x4_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_2x4,
        pos_count,
        uv_count,
        tri_count);

    // 16 x 32 divisions = 512 faces (1024 triangles).
    divisions_x = 16;
    divisions_y = 32;
    const char* file_path_16x32 = "./tests/data/out/test_stream_empty_write_geom_16x32_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_16x32,
        pos_count,
        uv_count,
        tri_count);

    // 32 x 16 divisions = 512 faces (1024 triangles).
    divisions_x = 32;
    divisions_y = 16;
    const char* file_path_32x16 = "./tests/data/out/test_stream_empty_write_geom_32x16_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_32x16,
        pos_count,
        uv_count,
        tri_count);

    // 192 x 108 divisions = 20736 faces (41472 triangles).
    //
    // This represents 1 vertex per-10 pixels in a HD 1920 x 1080 image.
    divisions_x = 192;
    divisions_y = 108;
    const char* file_path_192x108 = "./tests/data/out/test_stream_empty_write_geom_192x108_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_192x108,
        pos_count,
        uv_count,
        tri_count);

    if (debug_print) {
        bench.stop();
        bench.print("Test Stream Empty Write Geometry:");
    }

    return 0;
}
