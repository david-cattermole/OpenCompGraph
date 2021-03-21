#ifndef OPENCOMPGRAPH_CPP_H
#define OPENCOMPGRAPH_CPP_H

#include <rust/cxx.h>
#include <memory>
#include <string>
#include <list>
#include <vector>
#include <map>
#include <functional>
#include <algorithm>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {

namespace shared {
    struct SharedThing;
} // namespace shared

namespace cpp {

// class ThingC {
// public:
//     OCG_API_EXPORT
//     ThingC(std::string appname);

//     OCG_API_EXPORT
//     ~ThingC();

//     std::string appname;
// };

// OCG_API_EXPORT
// std::unique_ptr <ThingC> make_thingc(rust::Str appname);

// OCG_API_EXPORT
// const std::string &get_name(const ThingC &thing);

// OCG_API_EXPORT
// void run_sharedthing(shared::SharedThing state);

} // namespace cpp
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_CPP_H
