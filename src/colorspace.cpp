#include <iostream>
#include <memory>
#include <rust/cxx.h>
#include <opencompgraph.h>
#include <opencompgraph/oiio_utils.h>
#include <OpenColorIO/OpenColorIO.h>

namespace OCIO = OCIO_NAMESPACE;

namespace open_comp_graph {
namespace internal {

bool ocio_print_color_spaces() {
    bool ok = false;
    try {
        OCIO::ConstConfigRcPtr config = OCIO::GetCurrentConfig();
        auto num_color_spaces = config->getNumColorSpaces();
        std::cout << "num_color_spaces: " << num_color_spaces << '\n';
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


bool ocio_color_convert_inplace(
        rust::Slice<float> pixel_data,
        int width, int height, int num_channels,
        const rust::String &src_color_space_name,
        const rust::String &dst_color_space_name) {
    std::cout << "SRC COLOR SPACE: " << src_color_space_name << '\n';
    std::cout << "DST COLOR SPACE: " << dst_color_space_name << '\n';

    bool ok = false;
    try {
        OCIO::ConstConfigRcPtr config = OCIO::GetCurrentConfig();

        // FIXME: These are hard-coded for now, this should be fixed.
        auto src_color_space_name_c = "Utility - sRGB - Texture";
        auto dst_color_space_name_c = "ACES - ACEScg";
        auto src_color_space = config->getColorSpace(src_color_space_name_c);
        auto dst_color_space = config->getColorSpace(dst_color_space_name_c);

        if (src_color_space && dst_color_space) {
            std::cout << "applying color conversion\n";
            OCIO::ConstProcessorRcPtr processor = config->getProcessor(
                OCIO::ROLE_COMPOSITING_LOG, OCIO::ROLE_SCENE_LINEAR);
            OCIO::PackedImageDesc img(pixel_data.data(), width, height, num_channels);
            processor->apply(img);
            ok = true;
        }
    } catch(OCIO::Exception & exception) {
        std::cerr << "OpenColorIO Error: " << exception.what() << std::endl;
        ok = false;
    }
    std::cout << "OK: " << ok << '\n';
    return ok;
}

} // namespace internal
} // namespace open_comp_graph
