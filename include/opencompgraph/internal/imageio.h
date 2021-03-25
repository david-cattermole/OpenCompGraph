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

#ifndef OPENCOMPGRAPH_IMAGEIO_H
#define OPENCOMPGRAPH_IMAGEIO_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {
namespace internal {

// Forward declare.
struct ImageShared;

OCG_API_EXPORT
bool oiio_get_thread_count(int &num_threads);

OCG_API_EXPORT
bool oiio_set_thread_count(int32_t num_threads);

OCG_API_EXPORT
bool oiio_read_image(const rust::String &file_path, ImageShared &image);

OCG_API_EXPORT
bool oiio_write_image(const rust::String &file_path, const ImageShared &image);

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_IMAGEIO_H
