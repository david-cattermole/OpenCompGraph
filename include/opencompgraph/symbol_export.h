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

#ifndef OPENCOMPGRAPH_SYMBOL_EXPORT_H
#define OPENCOMPGRAPH_SYMBOL_EXPORT_H

// Cross-platform symbol visibility macro.
#if defined(_MSC_VER)
    #define OCG_API_EXPORT __declspec(dllexport)
#elif defined(__GNUC__)
    #define OCG_API_EXPORT __attribute__((visibility("default")))
#else
    #define OCG_API_EXPORT
#endif

#endif // OPENCOMPGRAPH_SYMBOL_EXPORT_H
