/*
 * Example usage of opencompgraph.h
 */

#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
// #include "jpeg.h"  // Used as pseudo-code, this header doesn't exist.
#include "opencompgraph.h"

namespace ocg = opencompgraph;

/*

class ReadImageOperation : ocg::BaseOperation {

public:

    virtual ReadImageOperation() {};

    virtual const Identifier id() {
        return 42;
    };

    virtual const OperationType type() {
        return 123;
    };

    virtual const Hash hash() {
        return this->id * this->type * 123456789;
    };

    virtual void compute() {
        // This is the fallback operation output.
        Result res = ocg::OperationHelper::createEmptyResult();

        // Read Image here.
        if (m_path.find(".jpg") != -1) {
            int image_width = 0;
            int image_height = 0;
            void *data = jpeg::read_file(
                m_path.c_str(),
                image_width,
                image_height);

            PixelBlock pixels;
            pixels.data = data;

            BoundingBox displayBox;
            displayBox.min.x = 0;
            displayBox.min.y = 0;
            displayBox.max.x = image_width;
            displayBox.max.y = image_height;

            Image img;
            img.pixelBlock = pixels;
            img.displayWindow = displayBox;

            m_result.setImage(img);
        }
    };

    virtual const OperationSPtr getInput(uint index) {
        OperationSPtr result;
        return result;
    };

    virtual void setInput(uint index, OperationSPtr operation) {
        // Read function does not need an input.
        return;
    };

    virtual const int numInputs() {
        return 0;
    };

    virtual const BaseOperationResult getOutput() {
        return m_result;
    };


private:

    OperationPixelResult m_result;
    std::string m_path;
};


class MergeImageOperation : ocg::BaseOperation {

public:

    virtual MergeImageOperation() {};

    virtual const Identifier id() {
        return 43;
    };

    virtual const OperationType type() {
        return 231;
    };

    virtual const Hash hash() {
        return this->id * this->type * 234567891;
    };

    virtual void compute() {
        // TODO: Combine both input images.
    };

    virtual const OperationSPtr getInput(uint index) {
        OperationSPtr result;
        return result;
    };

    virtual void setInput(uint index, OperationSPtr operation) {
        return;
    };

    virtual const int numInputs() {
        return 2;
    };

    virtual const BaseOperationResult getOutput() {
        return m_result;
    };


private:

    OperationPixelResult m_result;
};


void test_c() {
    // Disk-based cache.
    ocg::DiskCache<ocg::Hash, ocg::BaseOperationResult> cacheB();
    cacheB.setPath("/tmp/openCompGraph");
    cacheB.setCapacity(10 * GIGABYTES_TO_BYTES);

    // RAM-based cache.
    ocg::MemoryCache<ocg::Hash, ocg::BaseOperationResult> cacheA();
    cacheA.setCapacity(1024 * MEGABYTES_TO_BYTES);

    std::vector<BaseCache> cacheList;
    cacheList.push_back(cacheA);
    cacheList.push_back(cacheB);

    ReadImageOperation readOpA();
    readOpA.setPath("/my/path/to/fileA.1001.jpg");

    ReadImageOperation readOpB();
    readOpB.setPath("/my/path/to/fileB.1001.jpg");

    MergeImageOperation mergeOp();
    mergeOp.setInput(0, readOpA);
    mergeOp.setInput(1, readOpB);
    mergeOp.setMode("over");

    // ocg::compile(mergeOp, cacheList);

    OperationList opList {mergeOp};
    while (true) {
        OperationList tempList = opList;
        for (const auto op : tempList) {
            OperationList inputs = op->getNeededInputs();
            for (const auto inOp : input) {
                opList.push_back();
            }
        }
        if (tempList.size() == opList.size()) {
            // Nothing added
            break;
        }
    }
    // for (auto op : opList) {
    for (auto it = opList.rbegin(); it != opList.rend(); ++it) {
        Operation op = &it;
        // op.getOutput(cacheList);
        BaseOperationResult res = ocg::OperationHelper::getOpResult(op, cache);
    }
}

*/

int test_c() {
    std::cout << "test_c()" << std::endl;

    size_t id = 0;
    auto read_op = ocg::create_operation(id++, ocg::OperationType::ReadImage);
    std::cout << "read_op=" << &read_op << std::endl;

    auto read_id2 = read_op->get_id();
    std::cout << "read_id2=" << read_id2 << std::endl;

    auto read_op_type = read_op->get_op_type() == ocg::OperationType::ReadImage;
    std::cout << "read_op_type=" << read_op_type << std::endl;

    auto read_op_type_id = read_op->get_op_type_id();
    std::cout << "read_op_type_id=" << std::hex << read_op_type_id << std::endl;

    // Set string attribute
    auto read_attr = "file_path";
    if (read_op->attr_exists(read_attr) == ocg::AttrState::Exists) {
        auto read_path1 = read_op->get_attr_string(read_attr);
        std::cout << "read_path1=" << read_path1 << std::endl;

        read_op->set_attr(read_attr, "/path/to/image.exr");
        auto read_path2 = read_op->get_attr_string(read_attr);
        std::cout << "read_path2=" << read_path2 << std::endl;
    }

    // // Set float attribute
    // auto read_attr_x = "x";
    // if (read_op->attr_exists(read_attr_x) == ocg::AttrState::Exists) {
    //     auto read_x = read_op->get_attr_f32(read_attr_x);
    //     std::cout << "read_x=" << read_path1 << std::endl;

    //     read_op->set_attr(read_attr_x, 3.147);
    //     auto read_x2 = read_op->get_attr_f32(read_attr_x);
    //     std::cout << "read_x2=" << read_x2 << std::endl;
    // }

    // // Set integer attribute
    // auto read_attr_y = "y";
    // if (read_op->attr_exists(read_attr_y) == ocg::AttrState::Exists) {
    //     auto read_y = read_op->get_attr_f32(read_attr_y);
    //     std::cout << "read_y=" << read_path1 << std::endl;

    //     read_op->set_attr(read_attr_y, 42);
    //     auto read_y2 = read_op->get_attr_f32(read_attr_y);
    //     std::cout << "read_y2=" << read_y2 << std::endl;
    // }

    auto read_op_hash = read_op->hash();
    std::cout << "read_op_hash=" << read_op_hash << std::endl;

    read_op->compute();
    auto read_status = read_op->get_status_id();
    std::cout << "read_status=" << read_status << std::endl;

    // auto read_output = read_op->get_output();
    // auto read_hash = read_output->get_hash();
    // auto read_pixels = read_output->get_pixel_block();
    // auto read_bbox = read_output->get_bounding_box();
    // auto read_cmat = read_output->get_color_matrix();
    // auto read_tmat = read_output->get_transform_matrix();

    auto write_op = ocg::create_operation(id++, ocg::OperationType::WriteImage);
    std::cout << "write_op=" << &write_op << std::endl;

    // write_op->set_input(0, read_op);

    auto write_id2 = write_op->get_id();
    std::cout << "write_id2=" << write_id2 << std::endl;

    auto write_op_type = write_op->get_op_type() == ocg::OperationType::ReadImage;
    std::cout << "write_op_type=" << write_op_type << std::endl;

    auto write_op_type_id = write_op->get_op_type_id();
    std::cout << "write_op_type_id=" << std::hex << write_op_type_id << std::endl;

    auto write_attr = "file_path";
    if (write_op->attr_exists(write_attr) == ocg::AttrState::Exists) {
        auto write_path1 = write_op->get_attr_string(write_attr);
        std::cout << "write_path1=" << write_path1 << std::endl;

        write_op->set_attr(write_attr, "/path/to/image_out.jpg");
        auto write_path2 = write_op->get_attr_string(write_attr);
        std::cout << "write_path2=" << write_path2 << std::endl;
    }

    write_op->compute();
    auto write_status = write_op->get_status_id();
    std::cout << "write_status=" << write_status << std::endl;
    // auto write_result = ocg::get_op_result(write_op);
    // auto write_hash = ocg::get_result_hash(read_result);

    return 0;
}
