#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_a() {
    int scene_id = 42;
    ocg::SceneGraph *scene_graph = ocg::scene_graph_new(scene_id);
    ocg::scene_graph_delete(scene_graph);
    return 0;
}
