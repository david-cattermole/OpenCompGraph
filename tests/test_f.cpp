#include <iostream>
#include <cassert>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_f(std::shared_ptr<ocg::Cache> cache) {
    std::cout << "====================================== test_f()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

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

    graph.set_node_attr_str(read_node, "file_path", "tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_f32(grade_overexpose_node, "multiply", 2.0f);
    graph.set_node_attr_f32(grade_underexpose_node, "multiply", 0.5f);
    graph.set_node_attr_f32(grade_dark_node, "multiply", 0.25f);
    graph.set_node_attr_f32(grade_light_node, "multiply", 1.2f);
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
    auto status1 = graph.execute(write1_node, cache);
    assert(status1 == ocg::ExecuteStatus::kSuccess);
    auto stream_data1 = graph.output_stream();
    auto state_id1 = stream_data1.state_id();
    auto hash1 = stream_data1.hash();
    std::cout << "state_id1=" << state_id1 << '\n';
    std::cout << "hash1=" << hash1 << '\n';
    const ocg::internal::PixelBlock &pixel_block1 = stream_data1.pixel_block();
    auto pixel_buffer1 = stream_data1.pixel_buffer();
    std::cout << "pixel_buffer1"
              << " data=" << &pixel_buffer1
              << " size=" << pixel_buffer1.size() << '\n';
    auto width1 = stream_data1.pixel_width();
    auto height1 = stream_data1.pixel_height();
    auto num_channels1 = stream_data1.pixel_num_channels();
    std::cout << "width1=" << width1 << '\n';
    std::cout << "height1=" << height1 << '\n';
    std::cout << "num_channels1=" << static_cast<uint32_t>(num_channels1) << '\n';

    // Execute 2. The image should appear darker.
    auto status2 = graph.execute(write2_node, cache);
    assert(status2 == ocg::ExecuteStatus::kSuccess);
    auto stream_data2 = graph.output_stream();
    const ocg::internal::PixelBlock &pixel_block2 = stream_data2.pixel_block();
    auto pixel_buffer2 = stream_data2.pixel_buffer();
    std::cout << "pixel_buffer2"
              << " data=" << &pixel_buffer2
              << " size=" << pixel_buffer2.size() << '\n';
    auto width2 = stream_data2.pixel_width();
    auto height2 = stream_data2.pixel_height();
    auto num_channels2 = stream_data2.pixel_num_channels();
    std::cout << "width2=" << width2 << '\n';
    std::cout << "height2=" << height2 << '\n';
    std::cout << "num_channels2=" << static_cast<uint32_t>(num_channels2) << '\n';

    // Execute 3. The image should appear lighter.
    auto status3 = graph.execute(write3_node, cache);
    assert(status3 == ocg::ExecuteStatus::kSuccess);
    auto stream_data3 = graph.output_stream();
    const ocg::internal::PixelBlock &pixel_block3 = stream_data3.pixel_block();
    auto pixel_buffer3 = stream_data3.pixel_buffer();
    std::cout << "pixel_buffer3"
              << " data=" << &pixel_buffer3
              << " size=" << pixel_buffer3.size() << '\n';
    auto width3 = stream_data3.pixel_width();
    auto height3 = stream_data3.pixel_height();
    auto num_channels3 = stream_data3.pixel_num_channels();
    std::cout << "width3=" << width3 << '\n';
    std::cout << "height3=" << height3 << '\n';
    std::cout << "num_channels3=" << static_cast<uint32_t>(num_channels3) << '\n';

    bench.stop();
    bench.print("Test F:");

    return 0;
}
