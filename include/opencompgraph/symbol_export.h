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
