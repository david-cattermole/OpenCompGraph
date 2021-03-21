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
