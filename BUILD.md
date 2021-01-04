# Dependencies

You will need the Rust compiler and a C/C++ compiler.  Below are the
tested dependancy versions, for both Linux and Windows (MacOSX has not
been tested).

This demo targets the C++11 standard.

- Linux
  - GCC 6.3.1
- Windows 10
  - MSVC - Visual Studio 2015
- Linux or Windows
  - CMake 2.8.12+
  - Rust 1.48
    - cbindgen
    - cxx 1.0.1
    - libc
  - cxxbridge-cmd 1.0.1 (installed via Rust's Cargo package manager)

# Compile and Test (GNU Linux)

The build environment assumes to be running on Linux with GCC 6.3.1.

``` shell
$ cd /path/to/project/root/

$ cargo install cxxbridge-cmd --git "https://github.com/david-cattermole/cxx.git" --rev "363c18b2982329d6e4fbc2a6e5f324f4fef03661" --force

$ bash build_OpenCompGraph_linux.bash

# Run tests
$ ./install/bin/opencompgraph_tests
2 + 2 = 4
SCENEGRAPH: Add SceneGraph 42
SCENEGRAPH: Remove SceneGraph 0x96b5b0
my awesome demo
done with ThingC
```

Note: If you are using CentOS 7.8 and require the Red Hat Developer
Toolset 6 (GCC 6.3.1) for compatiblity with the VFX Reference Platform CY2018,
then you can use the following commands prior to running `build_OpenCompGraph_linux.bash`:
```
$ export CC=/opt/rh/devtoolset-6/root/usr/bin/gcc
$ export CXX=/opt/rh/devtoolset-6/root/usr/bin/g++
$ export _GLIBCXX_USE_CXX11_ABI=0  # Use the old std::string and std::list ABI.

# Enable Red Hat Developer Toolset 6
$ scl enable devtoolset-6 bash
```

# Compile and Test (Microsoft Windows)

This has been tested on Windows 10, with Visual Studio 2015.

Make sure the following commands are run in the Visual Studio 2015 enabled Command Prompt.
``` shell
> cd C:\path\to\project\root\

> build_OpenCompGraph_windows64.bat

> install\bin\opencompgraph_tests.exe
2 + 2 = 4
SCENEGRAPH: Add SceneGraph 42
SCENEGRAPH: Remove SceneGraph 0x1ffc8819410
my awesome demo
done with ThingC
```

