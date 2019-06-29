/*
 * Example usage of opencompgraph.h
 */

#include "jpeg.h"  // Used as pseudo-code, this header doesn't exist.
#include "opencompgraph.h"

namespace ocg = opencompgraph;

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


void main() {

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

    MergeOperation mergeOp();
    mergeOp.setInput(0, readOpA);
    mergeOp.setInput(1, readOpB);
    mergeOp.setMode("over");

    ocg::compile(mergeOp, cacheList);
}
