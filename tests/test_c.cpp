/*
 * Example usage of opencompgraph.h
 */

#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
// #include "jpeg.h"  // Used as pseudo-code, this header doesn't exist.
#include <opencompgraph.h>

namespace ocg = opencompgraph;


int test_c() {
    std::cout << "====================================== test_c()" << std::endl;

    /*
      // Disk-based cache.
      ocg::DiskCache<ocg::Hash, ocg::BaseNodeResult> cacheB();
      cacheB.setPath("/tmp/openCompGraph");
      cacheB.setCapacity(10 * GIGABYTES_TO_BYTES);

      // RAM-based cache.
      ocg::MemoryCache<ocg::Hash, ocg::BaseNodeResult> cacheA();
      cacheA.setCapacity(1024 * MEGABYTES_TO_BYTES);

      std::vector<BaseCache> cacheList;
      cacheList.push_back(cacheA);
      cacheList.push_back(cacheB);
    */

    size_t id = 0;

    auto read_op = ocg::internal::create_node_box(
            ocg::NodeType::ReadImage, id++);
    std::cout << "read_op=" << &read_op << std::endl;

    auto read_id2 = read_op->get_id();
    std::cout << "read_id2=" << read_id2 << std::endl;

    auto read_node_type = read_op->get_node_type() == ocg::NodeType::ReadImage;
    std::cout << "read_node_type=" << read_node_type << std::endl;

    auto read_node_type_id = read_op->get_node_type_id();
    std::cout << "read_node_type_id=" << std::hex << read_node_type_id << std::endl;

    // Set string attribute
    auto read_attr = "file_path";
    if (read_op->attr_exists(read_attr) == ocg::AttrState::Exists) {
        auto read_path1 = read_op->get_attr_string(read_attr);
        std::cout << "read_path1=" << read_path1 << std::endl;

        read_op->set_attr(read_attr, "tests/data/checker_8bit_rgba_8x8.png");
        auto read_path2 = read_op->get_attr_string(read_attr);
        std::cout << "read_path2=" << read_path2 << std::endl;
    }

    // Set float attribute
    auto read_attr_x = "x";
    if (read_op->attr_exists(read_attr_x) == ocg::AttrState::Exists) {
        auto read_x = read_op->get_attr_f32(read_attr_x);
        std::cout << "read_x=" << read_x << std::endl;

        float x = 3.147;
        read_op->set_attr(read_attr_x, x);
        auto read_x2 = read_op->get_attr_f32(read_attr_x);
        std::cout << "read_x2=" << read_x2 << std::endl;
    }

    // Set integer attribute
    auto read_attr_y = "y";
    if (read_op->attr_exists(read_attr_y) == ocg::AttrState::Exists) {
        auto read_y = read_op->get_attr_f32(read_attr_y);
        std::cout << "read_y=" << read_y << std::endl;

        int32_t y = 42;
        read_op->set_attr(read_attr_y, y);
        auto read_y2 = read_op->get_attr_f32(read_attr_y);
        std::cout << "read_y2=" << read_y2 << std::endl;
    }

    auto read_op_inputs = ocg::internal::create_vec_stream_data_shared();
    auto read_op_hash = read_op->hash(read_op_inputs);
    std::cout << "read_op_hash=" << read_op_hash << std::endl;

    auto read_op_output = ocg::internal::create_stream_data_shared();
    read_op->compute(read_op_inputs, read_op_output);
    auto read_status = read_op->get_status_id();
    std::cout << "read_status=" << read_status << std::endl;

    // auto read_output = read_op->get_output();
    // auto read_hash = read_output->get_hash();
    // auto read_pixels = read_output->get_pixel_block();
    // auto read_bbox = read_output->get_bounding_box();
    // auto read_cmat = read_output->get_color_matrix();
    // auto read_tmat = read_output->get_transform_matrix();

    auto write_op = ocg::internal::create_node_box(
            ocg::NodeType::WriteImage, id++);
    std::cout << "write_op=" << &write_op << std::endl;

    auto write_id2 = write_op->get_id();
    std::cout << "write_id2=" << write_id2 << std::endl;

    auto write_node_type =
            write_op->get_node_type() == ocg::NodeType::ReadImage;
    std::cout << "write_node_type=" << write_node_type << std::endl;

    auto write_node_type_id = write_op->get_node_type_id();
    std::cout << "write_node_type_id=" << std::hex << write_node_type_id
              << std::endl;

    auto write_attr = "file_path";
    if (write_op->attr_exists(write_attr) == ocg::AttrState::Exists) {
        auto write_path1 = write_op->get_attr_string(write_attr);
        std::cout << "write_path1=" << write_path1 << std::endl;

        write_op->set_attr(write_attr, "./tests/data/out/image_out.png");
        auto write_path2 = write_op->get_attr_string(write_attr);
        std::cout << "write_path2=" << write_path2 << std::endl;
    }

    auto write_op_inputs = ocg::internal::create_vec_stream_data_shared();
    write_op_inputs.push_back(std::move(read_op_output));

    auto write_op_output = ocg::internal::create_stream_data_shared();
    write_op->compute(write_op_inputs, write_op_output);
    auto write_status = write_op->get_status_id();
    std::cout << "write_status=" << write_status << std::endl;
    // auto write_result = ocg::get_op_result(write_op);
    // auto write_hash = ocg::get_result_hash(read_result);
    return 0;
}
