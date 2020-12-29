#include <iostream>
#include <cassert>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_f(std::shared_ptr<ocg::Cache> cache) {
    std::cout << "====================================== test_f()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << '\n';
    auto read_node = ocg::Node(ocg::NodeType::kReadImage, "readimage1");
    auto grade_overexpose_node = ocg::Node(ocg::NodeType::kGrade, "grade1");
    auto grade_underexpose_node = ocg::Node(ocg::NodeType::kGrade, "grade2");
    auto grade_dark_node = ocg::Node(ocg::NodeType::kGrade, "grade3");
    auto grade_light_node = ocg::Node(ocg::NodeType::kGrade, "grade4");
    auto null1_node = ocg::Node(ocg::NodeType::kNull, "null1");
    auto null2_node = ocg::Node(ocg::NodeType::kNull, "null2");
    auto write1_node = ocg::Node(ocg::NodeType::kWriteImage, "writeimage1");
    auto write2_node = ocg::Node(ocg::NodeType::kWriteImage, "writeimage2");
    auto write3_node = ocg::Node(ocg::NodeType::kWriteImage, "writeimage3");

    std::cout << "read_node=" << &read_node << '\n';
    std::cout << "grade_overexpose_node=" << &grade_overexpose_node << '\n';
    std::cout << "grade_underexpose_node=" << &grade_underexpose_node << '\n';
    std::cout << "grade_dark_node=" << &grade_dark_node << '\n';
    std::cout << "grade_light_node=" << &grade_light_node << '\n';
    std::cout << "null1_node=" << &null1_node << '\n';
    std::cout << "null2_node=" << &null2_node << '\n';
    std::cout << "write1_node=" << &write1_node << '\n';
    std::cout << "write2_node=" << &write2_node << '\n';
    std::cout << "write3_node=" << &write3_node << '\n';

    read_node.set_attr_str("file_path", "tests/data/oiio-images/tahoe-gps.jpg");
    grade_overexpose_node.set_attr_f32("multiply", 2.0f);
    grade_underexpose_node.set_attr_f32("multiply", 0.5f);
    grade_dark_node.set_attr_f32("multiply", 0.25f);
    grade_light_node.set_attr_f32("multiply", 1.2f);
    write1_node.set_attr_str("file_path", "./tests/data/out/test_f_out1.png");
    write2_node.set_attr_str("file_path", "./tests/data/out/test_f_out2.png");
    write3_node.set_attr_str("file_path", "./tests/data/out/test_f_out3.png");

    auto read_id = g.add_node(read_node);
    auto grade_overexpose_id = g.add_node(grade_overexpose_node);
    auto grade_underexpose_id = g.add_node(grade_underexpose_node);
    auto grade_dark_id = g.add_node(grade_dark_node);
    auto grade_light_id = g.add_node(grade_light_node);
    auto null1_id = g.add_node(null1_node);
    auto null2_id = g.add_node(null2_node);
    auto write1_id = g.add_node(write1_node);
    auto write2_id = g.add_node(write2_node);
    auto write3_id = g.add_node(write3_node);

    g.connect(read_id, grade_overexpose_id, 0);
    g.connect(grade_overexpose_id, grade_underexpose_id, 0);
    g.connect(grade_underexpose_id, null1_id, 0);

    // Write1 (unchanged)
    g.connect(null1_id, write1_id, 0);

    // Write2 (darker)
    g.connect(grade_underexpose_id, grade_dark_id, 0);
    g.connect(grade_dark_id, null2_id, 0);
    g.connect(null2_id, write2_id, 0);

    // Write3 (lighter)
    g.connect(null1_id, grade_light_id, 0);
    g.connect(grade_light_id, write3_id, 0);

    auto status1 = g.execute(write1_id, cache);
    assert(status1 == ocg::ExecuteStatus::kSuccess);
    auto stream_data1 = g.output_stream();
    auto state_id1 = stream_data1.state_id();
    auto hash1 = stream_data1.hash();
    std::cout << "state_id1=" << state_id1 << '\n';
    std::cout << "hash1=" << hash1 << '\n';
    const ocg::internal::PixelBlock &pixel_block1 = stream_data1.pixel_block();

    auto status2 = g.execute(write2_id, cache);
    assert(status2 == ocg::ExecuteStatus::kSuccess);
    auto stream_data2 = g.output_stream();
    const ocg::internal::PixelBlock &pixel_block2 = stream_data2.pixel_block();

    auto status3 = g.execute(write3_id, cache);
    assert(status3 == ocg::ExecuteStatus::kSuccess);
    auto stream_data3 = g.output_stream();
    const ocg::internal::PixelBlock &pixel_block3 = stream_data3.pixel_block();

    bench.stop();
    bench.print("Test F:");

    return 0;
}
