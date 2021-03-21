#include <opencompgraph/internal/oiio_utils.h>
#include <OpenImageIO/typedesc.h>

namespace open_comp_graph {
namespace internal {

PixelDataType oiio_format_to_ocg_format(const OIIO::TypeDesc oiio_type_desc) {
    auto pixel_data_type = PixelDataType::kUnknown;
    if (oiio_type_desc == OIIO::TypeDesc::UINT8) {
        pixel_data_type = PixelDataType::kUInt8;
    } else if (oiio_type_desc == OIIO::TypeDesc::HALF) {
        pixel_data_type = PixelDataType::kHalf16;
    } else if (oiio_type_desc == OIIO::TypeDesc::FLOAT) {
        pixel_data_type = PixelDataType::kFloat32;
    } else if (oiio_type_desc == OIIO::TypeDesc::UINT16) {
        pixel_data_type = PixelDataType::kUInt16;
    }
    return pixel_data_type;
}


OIIO::TypeDesc ocg_format_to_oiio_format(const PixelDataType ocg_pixel_data_type) {
    auto oiio_type_desc = OIIO::TypeDesc::UNKNOWN;
    if (ocg_pixel_data_type == PixelDataType::kUInt8) {
        oiio_type_desc = OIIO::TypeDesc::UINT8;
    } else if (ocg_pixel_data_type == PixelDataType::kHalf16) {
        oiio_type_desc = OIIO::TypeDesc::HALF;
    } else if (ocg_pixel_data_type == PixelDataType::kFloat32) {
        oiio_type_desc = OIIO::TypeDesc::FLOAT;
    } else if (ocg_pixel_data_type == PixelDataType::kUInt16) {
        oiio_type_desc = OIIO::TypeDesc::UINT16;
    }
    return oiio_type_desc;
}


} // namespace internal
} // namespace open_comp_graph
