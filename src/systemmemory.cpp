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

// STL
#include <cstdio>
#include <cstdint>

// System headers needed for each (supported) operating system.
#if defined __linux__ // Linux
#include <unistd.h>

#elif defined __APPLE__ // MacOS
#include <sys/types.h>
#include <sys/sysctl.h>

#elif defined _WIN32  // Windows
#include <windows.h>

#else // Unknown OS.
#error Unknown Operating System; Windows, MacOS and Linux are supported.
#endif

#include <opencompgraph/internal/systemmemory.h>

namespace open_comp_graph {
namespace internal {

size_t get_total_system_memory_as_bytes() {

#if defined __linux__ // Linux
    // Use 'sysconf'.
    long pages = sysconf(_SC_PHYS_PAGES);
    long page_size = sysconf(_SC_PAGE_SIZE);
    auto size = pages * page_size;

#elif defined _WIN32  // Windows
    // Use the Windows API.
    MEMORYSTATUSEX status;
    status.dwLength = sizeof(status);
    GlobalMemoryStatusEx(&status);
    auto bytes = status.ullTotalPhys;

#elif defined __APPLE__ // MacOS
    // Use 'sysctl'.
    int mib[2] = { CTL_HW, HW_MEMSIZE };
    auto namelen = sizeof(mib) / sizeof(mib[0]);
    size_t bytes = 0;
    size_t len = sizeof(size);
    auto error = sysctl(mib, namelen, &bytes, &len, NULL, 0);
    if (error < 0) {
        bytes = 0;
    }

#elseif // Unsupported OS
    auto bytes = 0;

#endif
    return bytes;
}

} // namespace cpp
} // namespace open_comp_graph
