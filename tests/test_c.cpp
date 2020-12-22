/*
 * Example usage of open_comp_graph.h
 */

#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;


int test_c() {
    std::cout << "====================================== test_c()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    size_t id = 0;

    auto read_node = ocg::internal::create_node_box(
            ocg::NodeType::ReadImage, id++);
    std::cout << "read_node=" << &read_node << '\n';

    auto read_id2 = read_node->get_id();
    std::cout << "read_id2=" << read_id2 << '\n';

    auto read_node_type = read_node->get_node_type() == ocg::NodeType::ReadImage;
    std::cout << "read_node_type=" << read_node_type << '\n';

    auto read_node_type_id = read_node->get_node_type_id();
    std::cout << "read_node_type_id=" << std::hex << read_node_type_id << '\n';

    // Set string attribute
    auto read_attr = "file_path";
    if (read_node->attr_exists(read_attr) == ocg::AttrState::Exists) {
        auto read_path1 = read_node->get_attr_str(read_attr);
        std::cout << "read_path1=" << read_path1 << '\n';

        read_node->set_attr_str(read_attr, "tests/data/checker_8bit_rgba_8x8.png");
        auto read_path2 = read_node->get_attr_str(read_attr);
        std::cout << "read_path2=" << read_path2 << '\n';
    }

    // Set float attribute
    auto read_attr_x = "x";
    if (read_node->attr_exists(read_attr_x) == ocg::AttrState::Exists) {
        auto read_x = read_node->get_attr_f32(read_attr_x);
        std::cout << "read_x=" << read_x << '\n';

        read_node->set_attr_f32(read_attr_x, 3.147);
        auto read_x2 = read_node->get_attr_f32(read_attr_x);
        std::cout << "read_x2=" << read_x2 << '\n';
    }

    // Set integer attribute
    auto read_attr_y = "y";
    if (read_node->attr_exists(read_attr_y) == ocg::AttrState::Exists) {
        auto read_y = read_node->get_attr_f32(read_attr_y);
        std::cout << "read_y=" << read_y << '\n';

        read_node->set_attr_i32(read_attr_y, 42);
        auto read_y2 = read_node->get_attr_f32(read_attr_y);
        std::cout << "read_y2=" << read_y2 << '\n';
    }

    auto read_node_inputs = ocg::internal::create_vec_stream_data_shared();
    auto read_node_hash = read_node->hash(read_node_inputs);
    std::cout << "read_node_hash=" << read_node_hash << '\n';

    auto read_node_output = ocg::internal::create_stream_data_shared();
    read_node->compute(read_node_inputs, read_node_output);
    auto read_status = read_node->get_status_id();
    std::cout << "read_status=" << read_status << '\n';

    // auto read_output = read_node->get_output();
    // auto read_hash = read_output->get_hash();
    // auto read_pixels = read_output->get_pixel_block();
    // auto read_bbox = read_output->get_bounding_box();
    // auto read_cmat = read_output->get_color_matrix();
    // auto read_tmat = read_output->get_transform_matrix();

    auto write_node = ocg::internal::create_node_box(
            ocg::NodeType::WriteImage, id++);
    std::cout << "write_node=" << &write_node << '\n';

    auto write_id2 = write_node->get_id();
    std::cout << "write_id2=" << write_id2 << '\n';

    auto write_node_type =
            write_node->get_node_type() == ocg::NodeType::ReadImage;
    std::cout << "write_node_type=" << write_node_type << '\n';

    auto write_node_type_id = write_node->get_node_type_id();
    std::cout << "write_node_type_id=" << std::hex << write_node_type_id
              << '\n';

    auto write_attr = "file_path";
    if (write_node->attr_exists(write_attr) == ocg::AttrState::Exists) {
        auto write_path1 = write_node->get_attr_str(write_attr);
        std::cout << "write_path1=" << write_path1 << '\n';

        write_node->set_attr_str(write_attr, "./tests/data/out/test_c_out.png");
        auto write_path2 = write_node->get_attr_str(write_attr);
        std::cout << "write_path2=" << write_path2 << '\n';
    }

    auto write_node_inputs = ocg::internal::create_vec_stream_data_shared();
    write_node_inputs.push_back(std::move(read_node_output));

    auto write_node_output = ocg::internal::create_stream_data_shared();
    write_node->compute(write_node_inputs, write_node_output);
    auto write_status = write_node->get_status_id();
    std::cout << "write_status=" << write_status << '\n';

    bench.stop();
    bench.print("Test C:");
    
    return 0;
}
