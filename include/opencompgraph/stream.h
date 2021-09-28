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

#ifndef OPENCOMPGRAPH_STREAM_H
#define OPENCOMPGRAPH_STREAM_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class StreamData {
public:
    OCG_API_EXPORT
    StreamData() noexcept;

    OCG_API_EXPORT
    explicit StreamData(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OCG_API_EXPORT
    rust::Box<internal::StreamDataImplRc> get_box() noexcept;

    OCG_API_EXPORT
    void set_box(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OCG_API_EXPORT
    StreamDataState state() const noexcept;

    OCG_API_EXPORT
    uint8_t state_id() const noexcept;

    OCG_API_EXPORT
    uint64_t hash() const noexcept;

    OCG_API_EXPORT
    BBox2Di display_window() const noexcept;

    OCG_API_EXPORT
    BBox2Di data_window() const noexcept;

    OCG_API_EXPORT
    Matrix4 color_matrix() const noexcept;

    OCG_API_EXPORT
    internal::ImageSpec clone_image_spec() const noexcept;

    OCG_API_EXPORT
    Matrix4 transform_matrix() const noexcept;

    OCG_API_EXPORT
    size_t deformers_len() const noexcept;

    OCG_API_EXPORT
    void apply_deformers(rust::Slice<float> &buffer,
                         BBox2Df display_window,
                         BBox2Df data_window) noexcept;

    OCG_API_EXPORT
    rust::Slice<const uint8_t> pixel_buffer() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_width() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_height() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_num_channels() const noexcept;

    OCG_API_EXPORT
    DataType pixel_data_type() const noexcept;

private:
    internal::StreamDataImplShared inner;
};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
