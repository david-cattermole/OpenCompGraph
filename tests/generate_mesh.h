#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

void generate_mesh(
    bool debug_print,
    uint32_t divisions_x,
    uint32_t divisions_y,
    ocg::StreamData &stream_data,
    const char* file_path,
    size_t &pos_count,
    size_t &uv_count,
    size_t &tri_count);