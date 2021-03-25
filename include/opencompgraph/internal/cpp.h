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
