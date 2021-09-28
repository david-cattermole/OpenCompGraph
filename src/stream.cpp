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

#include <iostream>
#include <utility>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/stream.h>

namespace open_comp_graph {

StreamData::StreamData() noexcept
        : inner{internal::create_stream_data_shared()} {
    // std::cout << "StreamData()" << '\n';
}

StreamData::StreamData(rust::Box<internal::StreamDataImplRc> box) noexcept
        : inner{internal::create_stream_data_shared_box(std::move(box))} {
    // std::cout << "StreamData(box)" << '\n';
}

rust::Box<internal::StreamDataImplRc> StreamData::get_box() noexcept {
    return std::move(this->inner.inner);
}

void StreamData::set_box(rust::Box<internal::StreamDataImplRc> box) noexcept {
    this->inner.inner = std::move(box);
}

StreamDataState StreamData::state() const noexcept {
    return this->inner.inner->state();
}

uint8_t StreamData::state_id() const noexcept {
    return this->inner.inner->state_id();
}

uint64_t StreamData::hash() const noexcept {
    return this->inner.inner->hash();
}

BBox2Di StreamData::display_window() const noexcept {
    return this->inner.inner->display_window();
};

BBox2Di StreamData::data_window() const noexcept {
    return this->inner.inner->data_window();
};

Matrix4 StreamData::color_matrix() const noexcept {
    return this->inner.inner->color_matrix();
};

internal::ImageSpec StreamData::clone_image_spec() const noexcept {
    return this->inner.inner->clone_image_spec();
};

size_t StreamData::deformers_len() const noexcept {
    return this->inner.inner->deformers_len();
};

void StreamData::apply_deformers(rust::Slice<float> &buffer,
                                 BBox2Df display_window,
                                 BBox2Df data_window) noexcept {
    return this->inner.inner->apply_deformers(
        buffer,
        display_window,
        data_window);
};

rust::Slice<const uint8_t> StreamData::pixel_buffer() const noexcept {
    return this->inner.inner->pixel_buffer();
};

int32_t StreamData::pixel_width() const noexcept {
    return this->inner.inner->pixel_width();
};

int32_t StreamData::pixel_height() const noexcept {
    return this->inner.inner->pixel_height();
};

int32_t StreamData::pixel_num_channels() const noexcept {
    return this->inner.inner->pixel_num_channels();
};

DataType StreamData::pixel_data_type() const noexcept {
    return this->inner.inner->pixel_data_type();
};

} // namespace open_comp_graph
