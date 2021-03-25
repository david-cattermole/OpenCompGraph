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
#include <cassert>
#include <opencompgraph.h>
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_f(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "====================================== test_f()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "readimage1");
    auto grade_overexpose_node = graph.create_node(ocg::NodeType::kGrade, "grade1");
    auto grade_underexpose_node = graph.create_node(ocg::NodeType::kGrade, "grade2");
    auto grade_dark_node = graph.create_node(ocg::NodeType::kGrade, "grade3");
    auto grade_light_node = graph.create_node(ocg::NodeType::kGrade, "grade4");
    auto null1_node = graph.create_node(ocg::NodeType::kNull, "null1");
    auto null2_node = graph.create_node(ocg::NodeType::kNull, "null2");
    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "writeimage1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "writeimage2");
    auto write3_node = graph.create_node(ocg::NodeType::kWriteImage, "writeimage3");

    graph.set_node_attr_str(read_node, "file_path", "tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_f32(grade_overexpose_node, "multiply_r", 2.0f);
    graph.set_node_attr_f32(grade_underexpose_node, "multiply_r", 0.5f);
    graph.set_node_attr_f32(grade_dark_node, "multiply_r", 0.25f);
    graph.set_node_attr_f32(grade_light_node, "multiply_r", 1.2f);
    graph.set_node_attr_str(write1_node, "file_path", "./tests/data/out/test_f_out1.png");
    graph.set_node_attr_str(write2_node, "file_path", "./tests/data/out/test_f_out2.png");
    graph.set_node_attr_str(write3_node, "file_path", "./tests/data/out/test_f_out3.png");

    graph.connect(read_node, grade_overexpose_node, 0);
    graph.connect(grade_overexpose_node, grade_underexpose_node, 0);
    graph.connect(grade_underexpose_node, null1_node, 0);

    // Write1 (unchanged)
    graph.connect(null1_node, write1_node, 0);

    // Write2 (darker)
    graph.connect(grade_underexpose_node, grade_dark_node, 0);
    graph.connect(grade_dark_node, null2_node, 0);
    graph.connect(null2_node, write2_node, 0);

    // Write3 (lighter)
    graph.connect(null1_node, grade_light_node, 0);
    graph.connect(grade_light_node, write3_node, 0);

    // Execute 1. The image should appear unchanged.
    if (debug_print) {
        std::cout << "Execute #1 =========================================" << '\n';
    }
    auto status1 = graph.execute(write1_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    assert(status1 == ocg::ExecuteStatus::kSuccess);
    auto stream_data1 = graph.output_stream();
    auto state_id1 = stream_data1.state_id();
    auto hash1 = stream_data1.hash();
    if (debug_print) {
        std::cout << "state_id1=" << state_id1 << '\n';
        std::cout << "hash1=" << hash1 << '\n';
    }
    auto pixel_buffer1 = stream_data1.pixel_buffer();
    if (debug_print) {
        std::cout << "pixel_buffer1"
                  << " data=" << &pixel_buffer1
                  << " size=" << pixel_buffer1.size() << '\n';
    }
    auto width1 = stream_data1.pixel_width();
    auto height1 = stream_data1.pixel_height();
    auto num_channels1 = stream_data1.pixel_num_channels();
    auto color_matrix1 = stream_data1.color_matrix();
    if (debug_print) {
        std::cout << "width1=" << width1 << '\n';
        std::cout << "height1=" << height1 << '\n';
        std::cout << "num_channels1=" << static_cast<uint32_t>(num_channels1) << '\n';
        std::cout << "color_matrix1:\n"
                  << " row0="
                  << color_matrix1.m00 << ","
                  << color_matrix1.m01 << ","
                  << color_matrix1.m02 << ","
                  << color_matrix1.m03 << '\n'
                  << " row1="
                  << color_matrix1.m10 << ","
                  << color_matrix1.m11 << ","
                  << color_matrix1.m12 << ","
                  << color_matrix1.m13 << '\n'
                  << " row2="
                  << color_matrix1.m20 << ","
                  << color_matrix1.m21 << ","
                  << color_matrix1.m22 << ","
                  << color_matrix1.m23 << '\n'
                  << " row3="
                  << color_matrix1.m30 << ","
                  << color_matrix1.m31 << ","
                  << color_matrix1.m32 << ","
                  << color_matrix1.m33 << '\n'
                  << '\n';
    }

    // Execute 2. The image should appear darker.
    if (debug_print) {
        std::cout << "Execute #2 =========================================" << '\n';
    }
    auto status2 = graph.execute(write2_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    assert(status2 == ocg::ExecuteStatus::kSuccess);
    auto stream_data2 = graph.output_stream();
    auto pixel_buffer2 = stream_data2.pixel_buffer();
    if (debug_print) {
        std::cout << "pixel_buffer2"
                  << " data=" << &pixel_buffer2
                  << " size=" << pixel_buffer2.size() << '\n';
    }
    auto width2 = stream_data2.pixel_width();
    auto height2 = stream_data2.pixel_height();
    auto num_channels2 = stream_data2.pixel_num_channels();
    auto color_matrix2 = stream_data2.color_matrix();
    if (debug_print) {
        std::cout << "width2=" << width2 << '\n';
        std::cout << "height2=" << height2 << '\n';
        std::cout << "num_channels2=" << static_cast<uint32_t>(num_channels2) << '\n';
        std::cout << "color_matrix2:\n"
                  << " row0="
                  << color_matrix2.m00 << ","
                  << color_matrix2.m01 << ","
                  << color_matrix2.m02 << ","
                  << color_matrix2.m03 << '\n'
                  << " row1="
                  << color_matrix2.m10 << ","
                  << color_matrix2.m11 << ","
                  << color_matrix2.m12 << ","
                  << color_matrix2.m13 << '\n'
                  << " row2="
                  << color_matrix2.m20 << ","
                  << color_matrix2.m21 << ","
                  << color_matrix2.m22 << ","
                  << color_matrix2.m23 << '\n'
                  << " row3="
                  << color_matrix2.m30 << ","
                  << color_matrix2.m31 << ","
                  << color_matrix2.m32 << ","
                  << color_matrix2.m33 << '\n'
                  << '\n';
    }

    // Execute 3. The image should appear lighter.
    if (debug_print) {
        std::cout << "Execute #3 =========================================" << '\n';
    }
    auto status3 = graph.execute(write3_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    assert(status3 == ocg::ExecuteStatus::kSuccess);
    auto stream_data3 = graph.output_stream();
    auto pixel_buffer3 = stream_data3.pixel_buffer();
    if (debug_print) {
        std::cout << "pixel_buffer3"
                  << " data=" << &pixel_buffer3
                  << " size=" << pixel_buffer3.size() << '\n';
    }
    auto width3 = stream_data3.pixel_width();
    auto height3 = stream_data3.pixel_height();
    auto num_channels3 = stream_data3.pixel_num_channels();
    auto color_matrix3 = stream_data3.color_matrix();
    if (debug_print) {
        std::cout << "width3=" << width3 << '\n';
        std::cout << "height3=" << height3 << '\n';
        std::cout << "num_channels3=" << static_cast<uint32_t>(num_channels3) << '\n';
        std::cout << "color_matrix3:\n"
                  << " row0="
                  << color_matrix3.m00 << ","
                  << color_matrix3.m01 << ","
                  << color_matrix3.m02 << ","
                  << color_matrix3.m03 << '\n'
                  << " row1="
                  << color_matrix3.m10 << ","
                  << color_matrix3.m11 << ","
                  << color_matrix3.m12 << ","
                  << color_matrix3.m13 << '\n'
                  << " row2="
                  << color_matrix3.m20 << ","
                  << color_matrix3.m21 << ","
                  << color_matrix3.m22 << ","
                  << color_matrix3.m23 << '\n'
                  << " row3="
                  << color_matrix3.m30 << ","
                  << color_matrix3.m31 << ","
                  << color_matrix3.m32 << ","
                  << color_matrix3.m33 << '\n'
                  << '\n';

        bench.stop();
        bench.print("Test F:");
    }

    return 0;
}
