#include <iostream>
#include <memory>
#include <algorithm>
#include <cstdlib>
#include <cassert>

#include <rust/cxx.h>
#include <opencompgraph.h>

// For loading DLLs at runtime on Unix (and there is a compatiblity
// later in 'dlfcn-win32') for Windows.
#include <dlfcn.h>

#include <ldpk/ldpk.h>
#include <ldpk/tde4_ld_plugin.h>
#include <ldpk/tde4_ldp_classic_3de_mixed.h>

namespace open_comp_graph {
namespace internal {

std::unique_ptr<OcgLdPluginBase> ldpk_new_plugin() {
    return std::unique_ptr<OcgLdPluginBase>(
        std::unique_ptr<OcgLdPluginBase>(new OcgLdPluginBase()));
}

OcgLdPluginBase::OcgLdPluginBase() {
    m_plugin = new tde4_ldp_classic_3de_mixed<ldpk::vec2d, ldpk::mat2d>();
}

OcgLdPluginBase::~OcgLdPluginBase() {
    delete m_plugin;
}

rust::Str OcgLdPluginBase::get_version_string() const {
    auto value = m_plugin->getVersionString();
    return value;
}

bool OcgLdPluginBase::get_model_name(rust::Slice<uint8_t> model_name) {
    assert((model_name.length() == 100));
    // Pretend our uint8_t pointer is actually a char array pointer,
    // as is required to pass to the LDPK method.
    char *name = reinterpret_cast<char*>(model_name.data());
    bool ok = m_plugin->getModelName(name);
    return ok;
}

bool OcgLdPluginBase::get_num_parameters(int32_t &value) {
    bool ok = m_plugin->getNumParameters(value);
    return ok;
}

bool OcgLdPluginBase::get_parameter_name(const int32_t i,
                                         rust::Slice<uint8_t> identifier) {
    assert((identifier.length() == 100));
    // Pretend our uint8_t pointer is actually a char array pointer,
    // as is required to pass to the LDPK method.
    char *name = reinterpret_cast<char*>(identifier.data());
    bool ok = m_plugin->getParameterName(i, name);
    return ok;
}

bool OcgLdPluginBase::get_parameter_type(rust::Str identifier,
                                         int32_t &param_type) {
    tde4_ldp_ptype ldpk_param_type = tde4_ldp_ptype::TDE4_LDP_DOUBLE;
    const char *name = identifier.data();
    bool ok = m_plugin->getParameterType(name, ldpk_param_type);
    param_type = static_cast<int32_t>(ldpk_param_type);
    return ok;
}

bool OcgLdPluginBase::get_parameter_default_value_f64(rust::Str identifier,
                                                      double &value) {
    const char *name = identifier.data();
    bool ok = m_plugin->getParameterDefaultValue(name, value);
    return ok;
}

bool OcgLdPluginBase::get_parameter_range(rust::Str identifier,
                                          double &min_value,
                                          double &max_value) {
    const char *name = identifier.data();
    bool ok = m_plugin->getParameterRange(name, min_value, max_value);
    return ok;
}

bool OcgLdPluginBase::set_parameter_value_f64(rust::Str identifier,
                                              double value) {
    const char *name = identifier.data();
    bool ok = m_plugin->setParameterValue(name, value);
    return ok;
}

bool OcgLdPluginBase::initialize_parameters() {
    bool ok = m_plugin->initializeParameters();
    return ok;
}

bool OcgLdPluginBase::undistort(double x0, double y0, double &x1, double &y1) {
    bool ok = m_plugin->undistort(x0, y0, x1, y1);
    return ok;
}

bool OcgLdPluginBase::distort(double x0, double y0, double &x1, double &y1) {
    bool ok = m_plugin->distort(x0, y0, x1, y1);
    return ok;
}

bool OcgLdPluginBase::distort_with_guess(double x0, double y0,
                                         double x1_start, double y1_start,
                                         double &x1, double &y1) {
    bool ok = m_plugin->distort(x0, y0, x1_start, y1_start, x1, y1);
    return ok;
}

bool OcgLdPluginBase::provides_parameter_derivatives() {
    bool ok = m_plugin->providesParameterDerivatives();
    return ok;
}

void OcgLdPluginBase::get_bounding_box_undistort(double xa_in, double ya_in,
                                                 double xb_in, double yb_in,
                                                 double &xa_out, double &ya_out,
                                                 double &xb_out, double &yb_out,
                                                 int nx, int ny) {
    m_plugin->getBoundingBoxUndistort(
        xa_in, ya_in,
        xb_in, yb_in,
        xa_out, ya_out,
        xb_out, yb_out,
        nx, ny);
}

void OcgLdPluginBase::get_bounding_box_distort(double xa_in, double ya_in,
                                               double xb_in, double yb_in,
                                               double &xa_out, double &ya_out,
                                               double &xb_out, double &yb_out,
                                               int nx, int ny) {
    m_plugin->getBoundingBoxDistort(
        xa_in, ya_in,
        xb_in, yb_in,
        xa_out, ya_out,
        xb_out, yb_out,
        nx, ny);
}


} // namespace internal
} // namespace open_comp_graph
