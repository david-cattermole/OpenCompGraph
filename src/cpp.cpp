#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph.h>

namespace open_comp_graph {
namespace cpp {

ThingC::ThingC(std::string appname) : appname(std::move(appname)) {
    std::cout << "ThingC()" << std::endl;
}

ThingC::~ThingC() {
    std::cout << "~ThingC()" << std::endl;
    std::cout << "done with ThingC" << std::endl;
}

std::unique_ptr<ThingC> make_thingc(rust::Str appname) {
    std::cout << "make_thingc()" << std::endl;
    return std::unique_ptr<ThingC>(new ThingC(std::string(appname)));
}

const std::string &get_name(const ThingC &thing) {
    std::cout << "get_name()" << std::endl;
    return thing.appname;
}

void run_sharedthing(shared::SharedThing state) {
    std::cout << "run_sharedthing()" << std::endl;
    internal::print_r(*state.y);
}

} // namespace cpp
} // namespace open_comp_graph