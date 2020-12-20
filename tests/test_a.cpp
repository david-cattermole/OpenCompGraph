#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_a() {
    std::cout << "====================================== test_a()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    int scene_id = 42;
    ocg::cbindgen::SceneGraph *scene_graph = ocg::cbindgen::scene_graph_new(scene_id);
    ocg::cbindgen::scene_graph_delete(scene_graph);

    bench.stop();
    bench.print("Test A:");

    return 0;
}
