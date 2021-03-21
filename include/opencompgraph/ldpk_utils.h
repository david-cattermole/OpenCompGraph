#ifndef OPENCOMPGRAPH_LDPK_H
#define OPENCOMPGRAPH_LDPK_H

#include <memory>

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"


// Forward declare.
class tde4_ld_plugin;


namespace open_comp_graph {
namespace internal {


class OcgLdPluginBase {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT
    OcgLdPluginBase();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ~OcgLdPluginBase();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Str get_version_string() const;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_model_name(rust::Slice<uint8_t> model_name);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_num_parameters(int32_t &value);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_parameter_name(int32_t i, rust::Slice<uint8_t> identifier);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_parameter_type(rust::Str identifier, int32_t &param_type);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_parameter_default_value_f64(rust::Str identifier, double &value);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool get_parameter_range(rust::Str identifier,
                             double &min_value, double &max_value);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool set_parameter_value_f64(rust::Str identifier, double value);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool initialize_parameters();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool undistort(double x0, double y0, double &x1, double &y1);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool distort(double x0, double y0, double &x1, double &y1);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool distort_with_guess(double x0, double y0,
                            double x1_start, double y1_start,
                            double &x1, double &y1);

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool provides_parameter_derivatives();

    // Iterate around the specified box, undistort the points and
    // compute the bounding box.
    //
    // Good default values for nx and ny are 32 and 32.
    OPENCOMPGRAPH_SYMBOL_EXPORT
    void get_bounding_box_undistort(double xa_in, double ya_in,
                                    double xb_in, double yb_in,
                                    double &xa_out, double &ya_out,
                                    double &xb_out, double &yb_out,
                                    int nx, int ny);

    // Iterate around the specified box, distort the points and
    // compute the bounding box.
    //
    // Good default values for nx and ny are 32 and 32.
    OPENCOMPGRAPH_SYMBOL_EXPORT
    void get_bounding_box_distort(double xa_in, double ya_in,
                                  double xb_in, double yb_in,
                                  double &xa_out, double &ya_out,
                                  double &xb_out, double &yb_out,
                                  int nx, int ny);

private:

    tde4_ld_plugin *m_plugin{};
};

OPENCOMPGRAPH_SYMBOL_EXPORT
std::unique_ptr<OcgLdPluginBase> ldpk_new_plugin();

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_LDPK_H
