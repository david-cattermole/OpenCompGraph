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

#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph.h>

namespace open_comp_graph {
namespace cpp {

// ThingC::ThingC(std::string appname) : appname(std::move(appname)) {
//     std::cout << "ThingC()" << std::endl;
// }

// ThingC::~ThingC() {
//     std::cout << "~ThingC()" << std::endl;
//     std::cout << "done with ThingC" << std::endl;
// }

// std::unique_ptr<ThingC> make_thingc(rust::Str appname) {
//     std::cout << "make_thingc()" << std::endl;
//     return std::unique_ptr<ThingC>(new ThingC(std::string(appname)));
// }

// const std::string &get_name(const ThingC &thing) {
//     std::cout << "get_name()" << std::endl;
//     return thing.appname;
// }

// void run_sharedthing(shared::SharedThing state) {
//     std::cout << "run_sharedthing()" << std::endl;
//     internal::print_r(*state.y);
// }

} // namespace cpp
} // namespace open_comp_graph
