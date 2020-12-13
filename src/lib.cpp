#include <iostream>
#include <opencompgraph.h>

namespace opencompgraph {

    void my_func() {
        std::cout << "my_func was called." << std::endl;
    };

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

void run_sharedthing(SharedThing state) {
    std::cout << "run_sharedthing()" << std::endl;
    print_r(*state.y);
}

} // namespace opencompgraph
