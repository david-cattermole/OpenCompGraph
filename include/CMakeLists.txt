
# # Include public header files for installation
# set(header_files
#   # C++ public header files.
#   include/opencompgraph.h

#   # Include internal generated headers, using 'cbindgen' and 'cxx'.
#   include/cxx.h
#   include/opencompgraph/_cpp.h
#   include/opencompgraph/_cbindgen.h
#   include/opencompgraph/_cxxbridge.h
#   )


# Install
install(DIRECTORY
  include/
  DESTINATION "include"
  FILES_MATCHING PATTERN "*.h")
