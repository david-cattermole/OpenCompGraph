#ifndef OPENCOMPGRAPH_CPP_H
#define OPENCOMPGRAPH_CPP_H

#include <cxx.h>
#include <memory>
#include <string>
#include <list>
#include <vector>
#include <map>
#include <functional>
#include <algorithm>


namespace opencompgraph {

    class ThingC {
    public:
        ThingC(std::string appname);

        ~ThingC();

        std::string appname;
    };

    struct SharedThing;

    std::unique_ptr <ThingC> make_thingc(rust::Str appname);

    const std::string &get_name(const ThingC &thing);

    void run_sharedthing(SharedThing state);

/*

    typedef int OperationType;
    typedef signed long long int Hash;  // 64-bit
    typedef std::size_t Identifier;

    template<typename T>
    struct Point2 {
        T x;
        T y;
    };
    typedef Point2<float> Point2f;
    typedef Point2<double> Point2d;
    typedef std::function<Point2f> Point2fFunc;
    typedef std::function<Point2d> Point2dFunc;

    template <typename T>
    struct Point3 {
        T x;
        T y;
        T z;
    };
    typedef Point3<float> Point3f;
    typedef Point3<double> Point3d;

    template<typename T>
    struct BoundingBox2 {
        T minX;
        T minY;
        T maxX;
        T maxY;
    };
    typedef BoundingBox2<float> BoundingBox2f;
    typedef BoundingBox2<double> BoundingBox2d;

    template<typename T>
    class Matrix3 {
        T values[9];
    };

    template<typename T>
    class Matrix4 {
        T values[16];
    };

    typedef Matrix3<float> Matrix3f;
    typedef Matrix3<double> Matrix3d;
    typedef Matrix4<float> Matrix4f;
    typedef Matrix4<double> Matrix4d;

    typedef std::shared_ptr<Matrix3f> Matrix3fSPtr;
    typedef std::shared_ptr<Matrix3d> Matrix3dSPtr;
    typedef std::shared_ptr<Matrix4f> Matrix4fSPtr;
    typedef std::shared_ptr<Matrix4d> Matrix4dSPtr;

    ///////////////////////////////////////////////////////////////////////////

    enum BitDepth {
        Unsigned8 = 0, // char
        Unsigned16,    // ushort_t
        Float16,       // half-floating point.
        Float32,       // float
    };

    struct PixelBlock {
        int width;
        int height;
        int nchannels;
        BitDepth bitdepth;
        std::vector<char> data;
    };
    typedef std::shared_ptr<PixelBlock> PixelBlockSPtr;

    struct Image {
        BoundingBox2d displayWindow;
        PixelBlock pixelBlock;
    };
    typedef std::shared_ptr<Image> ImageSPtr;

    ///////////////////////////////////////////////////////////////////////////

    class BaseOperationResult {
        // Holds the output of an operation.
        virtual Hash getHash() const = 0;

        virtual Image getImage() const = 0;
        virtual void setImage(Image img) = 0;

        virtual Matrix3d getColorMatrix() const = 0;
        virtual void setColorMatrix(Matrix3d matrix) = 0;

        virtual Point2dFunc getPointCB() const = 0;
        virtual void setPointCB(Point2dFunc func) = 0;
    };
    typedef std::shared_ptr<BaseOperationResult> BaseOperationResultSPtr;

    class OperationPixelResult : BaseOperationResult {
        // Holds pixel operation result; a set of pixels.
    };

    class OperationPointResult : BaseOperationResult {
        // Holds point operation result;
    };

    class OperationColorResult : BaseOperationResult {
        // Holds color operation result
    };

    ///////////////////////////////////////////////////////////////////////////

    class BaseOperation;
    typedef std::shared_ptr<BaseOperation> OperationSPtr;

    class BaseOperation {
    public:
        virtual BaseOperation() = 0;

        // The operation ID, used to seed the hash.
        virtual Identifier id() const = 0;

        // Define the type of this operation; pixel or point, for example.
        virtual OperationType type() const = 0;

        virtual Hash hash() const = 0;
        // defined by user, computes a unique integer based on the
        // inputs of the operation and the type of operation. Each
        // operation must have a unique seed value.

        // Compute the Operation's result
        // - Get input data
        //
        // This method takes data from the input op stored
        // internally, computes it, and sets the internal data
        // structure to be returned with Get Output Data method.
        virtual BaseOperationResult compute() = 0;

        // Define the type of each input, number of inputs.
        virtual int numInputs() const = 0;

        // Operation may have more than one input Operation.
        virtual OperationSPtr getInput(uint index) const = 0;
        virtual void setInput(uint index, OperationSPtr operation) = 0;
        virtual std::vector<OperationSPtr> getNeededInputs() const = 0;

        // // forces a 'compute' of the operation.
        // virtual BaseOperationResult getOutput(BaseCache cache) const = 0;

    };

    ///////////////////////////////////////////////////////////////////////////

    template<typename K, typename V>
    class BaseCache {

    public:
        BaseCache() = default;
        virtual ~BaseCache() = default;

        // NOTE: If the cache is multi-threaded the user should not be
        // aware of the implementation of the threading, all
        // locking/guarding/unlocking/acquiring/releasing of threads or
        // resources should be handled internally.

        // TODO: Should we return an iterator?
        virtual Iterator find(K key) const = 0;
        virtual void insert(K key, V value,
                            std::size_t numBytes,
                            uint_t priority) = 0;

        // Remove a value from the cache.
        virtual void evict() = 0;
        virtual void evict(uint numberOfItems) = 0;
        virtual void evict(double ratioOfItems) = 0;
        virtual void erase(K key) = 0;
        virtual void clear() = 0;

        virtual std::size_t size() const = 0; // memory size (in bytes)
        virtual std::size_t count() const  = 0; // number of items in the cache.

        // Set the capacity of the
        virtual std::size_t getCapacity() const = 0;
        virtual void setCapacity(std::size_t numBytes) = 0;
    };

    template<typename K, typename V>
    class MemoryCache : public BaseCache<K, V> {
    public:
        // Allow the cache to be set to different modes;
        // Least-Recently-Used or Most-Recently-Used.
        int getStrategy() const;
        void setStrategy(int mode);

    private:
        std::size_t m_capacity;
        std::list<K> m_keys;
        std::map<K, std::pair<V, std::list<K>::iterator> > m_keysToValues;
        // TODO: How to store priority and 'least recently used'
        // stack?
        //
        // Rather than trying to model a least-recently-used cache
        // internally, we can generalise both priority and
        // 'age'/'recently used' into a single 'rank' number. For
        // example a low priority may simply add (or subtract) 100
        // from the rank, or a combination of data-structures and
        // numbers may be stored and converted to a rank using the
        // different strategy for the cache, this would allow
        // switching between different cache strategies very quickly.
        std::vector<K> m_keysPriority;  // use std::make_heap to turn this into a heap.
    };

    ////////////////////////////////////////////////////////////////////////////

    template<typename K, typename V>
    class DiskCache : public BaseCache<K, V> {
    public:
        // The directory to store data.
        std::string getDirectory() const;
        void setDirectory(std::string path);

    private:
        std::size_t m_capacity;

        // TODO: Add compression variables. Add ability to use
        // Google's 'Snappy' compression library to compress and
        // write data to disk.
        std::string m_path;
    };

    ///////////////////////////////////////////////////////////////////////////

    template<typename K, typename V>
    class MultiSubCache : public BaseCache<K, V> {
    public:
        // A list of sub-caches to be manipulated as a single cache.
        //
        // When querying values, we check the first, then second, then
        // third caches, etc.
        //
        // When inserting new values into the cache we add the value
        // to the first cache, until it is full, then we add it to the
        // second, and third, etc.
        std::vector<BaseCache<K, V> > getCacheList() const;
        void setCacheList(std::vector<BaseCache<K, V> > cacheList);

    private:
        std::vector<BaseCache<K, V>> m_cacheList;
    };

    ///////////////////////////////////////////////////////////////////////////

    class OperationHelper {
    public:

        // Defines common functions and behavior to be used by
        // default in a Operation object.
        static bool doStuff(int arg) {};

        static BaseOperationResult createEmptyResult() {
            // Create image of 1 pixel with no metadata.
            auto res = BaseOperationResult();
            return res;
        };

        static BaseOperationResult getOpResult(BaseOperation op,
                                               BaseCache cache) {
            // Get the result of the given operation.
            auto res = BaseOperationResult();
            auto h = op.hash();
            auto it = cache.find(h);
            if (it != cache.cend()) {
                res = &it;
            } else {
                res = op.compute();
            }
        };

    private:
        // No one can create an instance of this class, and that's the
        // way we want it.
        OperationHelper() {};
    };

    ///////////////////////////////////////////////////////////////////////////

    // TODO Write an iterator to loop over the graph of
    // Operations.
    //
    // Search the graph; breadth-first search. Allow searching
    // backwards (up) and forwards (down) through the graph.
    //
    class OperationIterator {
    public:
        OperationIterator(BaseOperation op);
        void begin();
        void end();
        void next();
    };

    ///////////////////////////////////////////////////////////////////////////

    typedef std::function<void()> Function;
    typedef std::list<Function> FunctionList;

    FunctionList compile(BaseOperationSPtr op,
                         std::vector<BaseCacheSPtr> cacheList) {
        // Given the end operation point, compile a list of functions
        // that must be run to compute the given operation.
        //
        std::vector<BaseOperationSPtr> opList {op};
        std::vector<BaseOperationSPtr> tempList;
        std::vector<BaseOperationSPtr> inputList;
        while (true) {
            tempList = opList;
            for (auto op : tempList) {
                inputList = op->getNeededInputs();
                for (auto inOp : inputList) {
                    opList.push_back(inOp);
                }
            }
            if (tempList.size() == opList.size()) {
                break;
            }
        }

        // TODO: Compute a list of lists of functions that can be run
        // on separate threads. The idea is to allow a sub-chain of
        // functions to be computed separately on a thread, while
        // another sub-chain is computed. However some sub-lists must
        // wait on previous sub-lists before starting.
        //
        // TODO: This loop must be reversed.
        FunctionList funcList;
        for (auto op : opList) {
            Function f = std::bind(&BaseOperationSPtr::compute, op);
            // f();  // run the function
            funcList.push_back(f);
        }

        return funcList;
    };

    void compute(FunctionList funcList) {
        // Run the functions given, sequentially.
    };

*/

} // namespace opencompgraph

#endif // OPENCOMPGRAPH_CPP_H
