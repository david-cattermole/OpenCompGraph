# Dependencies

You will need the Rust compiler and a C/C++ compiler.  Below are the
tested dependancy versions, for both Linux and Windows (MacOSX has not
been tested).

This demo targets the C++11 standard.

- Linux
  - GCC 6.3.1
- Windows 10
  - MSVC - Visual Studio 2015 - 2017
- [CMake 2.8.12+](https://cmake.org/)
- [Rust 1.48+](https://www.rust-lang.org/)
  - [cxx and cxxbridge-cmd 1.0.33](https://cxx.rs/) (automatically installed)
  - cbindgen
  - ... see Cargo.toml for full list.
- [Boost](https://www.boost.org/users/download/) (required for OpenImageIO)
- Automatically built with helper scripts.
  - [bzip2](https://gitlab.com/federicomenaquintero/bzip2)
  - [zlib](https://github.com/madler/zlib)
  - [Science-D-Visions 3DEqualizer4 LDPK](https://www.3dequalizer.com/?site=tech_docs&id=110216_01) (Lens Distortion Plug-in Kit)
  - [libjpeg_turbo](https://github.com/libjpeg-turbo/libjpeg-turbo)
  - [libpng](https://github.com/glennrp/libpng)
  - [libtiff](https://gitlab.com/libtiff/libtiff)
  - [dlfcn](https://github.com/dlfcn-win32/dlfcn-win32) (Windows only)
  - [ILM Base and OpenEXR](https://github.com/AcademySoftwareFoundation/openexr)
  - [OpenImageIO](https://github.com/OpenImageIO/oiio)
  - [OpenColorIO](https://github.com/AcademySoftwareFoundation/OpenColorIO)

For specific configurations, the [VFX
Platform](https://vfxplatform.com/) can be helpful if you are
targeting specific third-party software versions (such as Autodesk
Maya).

# Compile and Test (GNU Linux)

The build environment assumes to be running on Linux with GCC 6.3.1.

``` shell
$ cd /path/to/project/root/

$ cargo install cxxbridge-cmd --git "https://github.com/david-cattermole/cxx.git" --rev "363c18b2982329d6e4fbc2a6e5f324f4fef03661" --force

$ bash build_OpenCompGraph_linux.bash

# Run tests
$ ./install/bin/opencompgraph_tests
```

Note: If you are using CentOS 7.8 and require the Red Hat Developer
Toolset 6 (GCC 6.3.1) for compatibility with the
[VFX Reference Platform](https://vfxplatform.com/) CY2018,
then you can use the following commands prior to running `build_OpenCompGraph_linux.bash`:
```
$ export CC=/opt/rh/devtoolset-6/root/usr/bin/gcc
$ export CXX=/opt/rh/devtoolset-6/root/usr/bin/g++
$ export _GLIBCXX_USE_CXX11_ABI=0  # Use the old std::string and std::list ABI.

# Enable Red Hat Developer Toolset 6
$ scl enable devtoolset-6 bash
```

# Compile and Test (Microsoft Windows)

This has been tested on Windows 10, with Visual Studio 2015 and 2017.

Make sure the following commands are run in the Visual Studio 2015 enabled Command Prompt.
``` shell
> CHDIR C:\path\to\project\root\

> CHDIR thirdparty
> build_thirdparty_windows64.bat

> CHDIR ..
> build_OpenCompGraph_windows64.bat

> install\bin\opencompgraph_tests.exe
```

