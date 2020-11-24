#ifndef OPENCOMPGRAPH_CPP_H
#define OPENCOMPGRAPH_CPP_H

#include <cxx.h>
#include <memory>
#include <string>


namespace opencompgraph {

    class ThingC {
    public:
        ThingC(std::string appname);

        ~ThingC();

        std::string appname;
    };

    struct SharedThing;

    std::unique_ptr <ThingC> make_demo(rust::Str appname);

    const std::string &get_name(const ThingC &thing);

    void do_thing(SharedThing state);

} // namespace opencompgraph

#endif // OPENCOMPGRAPH_CPP_H
