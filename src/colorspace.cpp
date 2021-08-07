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

// STL
#include <iostream>
#include <memory>
#include <string>

// OpenCompGraph
#include <rust/cxx.h>
#include <opencompgraph.h>
#include <opencompgraph/internal/oiio_utils.h>
#include <opencompgraph/internal/colorspace.h>

#include <OpenImageIO/imagebufalgo.h>
#include <OpenImageIO/color.h>

// OCIO
#include <OpenColorIO/OpenColorIO.h>
namespace OCIO = OCIO_NAMESPACE;

namespace open_comp_graph {
namespace internal {

bool ocio_print_color_spaces() {
    bool ok = false;
    try {
        OCIO::ConstConfigRcPtr config = OCIO::GetCurrentConfig();
        auto num_color_spaces = config->getNumColorSpaces();
        for (auto i = 0; i < num_color_spaces; i++) {
            auto color_space_name = config->getColorSpaceNameByIndex(i);
            std::cout << i << ": " << color_space_name << '\n';
        }
    } catch(OCIO::Exception & exception) {
        std::cerr << "OpenColorIO Error: " << exception.what() << std::endl;
        ok = false;
    }
    return ok;
}

bool oiio_color_convert_inplace(
        rust::Slice<float> pixel_data,
        int width, int height, int num_channels,
        int alpha_channel_index,
        bool unassociated_alpha,
        const rust::String &src_color_space_name,
        const rust::String &dst_color_space_name) {
    auto src_color_space_name_str = std::string(src_color_space_name);
    auto dst_color_space_name_str = std::string(dst_color_space_name);

    auto spec = OIIO::ImageSpec(
        width, height, num_channels,
        OIIO::TypeDesc::FLOAT);
    spec.alpha_channel = alpha_channel_index;
    void *data = static_cast<void*>(pixel_data.data());
    auto image_buffer = OIIO::ImageBuf::ImageBuf(spec, data);

    OIIO::ColorConfig colorConfig;
    auto processor = colorConfig.createColorProcessor(
        src_color_space_name_str,
        dst_color_space_name_str);
    if (!processor) {
        std::cerr << "Color Processor is invalid\n";
        return false;
    }

    auto roi = OIIO::get_roi_full(spec);
    const int32_t num_threads = 0;
    bool unpremult = unassociated_alpha;

    bool ok = OIIO::ImageBufAlgo::colorconvert(
        image_buffer, image_buffer,
        &*processor,
        unpremult, roi, num_threads);
    return ok;
}

} // namespace internal
} // namespace open_comp_graph
