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

namespace open_comp_graph {

namespace shared {
    struct SharedThing;
} // namespace shared

namespace cpp {

    class ThingC {
    public:
        __declspec(dllexport) ThingC(std::string appname);

        __declspec(dllexport) ~ThingC();

        std::string appname;
    };

    __declspec(dllexport) std::unique_ptr <ThingC> make_thingc(rust::Str appname);

    __declspec(dllexport) const std::string &get_name(const ThingC &thing);

    __declspec(dllexport) void run_sharedthing(shared::SharedThing state);

} // namespace cpp
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_CPP_H
