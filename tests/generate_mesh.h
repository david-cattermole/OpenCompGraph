/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

#ifndef OPENCOMPGRAPH_TEST_GENERATE_MESH_H
#define OPENCOMPGRAPH_TEST_GENERATE_MESH_H

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

#endif //OPENCOMPGRAPH_TEST_GENERATE_MESH_H
