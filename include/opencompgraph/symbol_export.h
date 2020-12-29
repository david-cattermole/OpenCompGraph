#ifndef OPENCOMPGRAPH_SYMBOL_EXPORT_H
#define OPENCOMPGRAPH_SYMBOL_EXPORT_H

// Cross-platform symbol visibility macro.
#if defined(_MSC_VER)
    #define OPENCOMPGRAPH_SYMBOL_EXPORT __declspec(dllexport)
#elif defined(__GNUC__)
    #define OPENCOMPGRAPH_SYMBOL_EXPORT __attribute__((visibility("default")))
#else
    #define OPENCOMPGRAPH_SYMBOL_EXPORT
#endif

#endif // OPENCOMPGRAPH_SYMBOL_EXPORT_H
