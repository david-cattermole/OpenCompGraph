# Build third party projects as part of OpenCompGraphMaya, but in
# their own build directories.

cmake_minimum_required(VERSION 3.12.0)

# Project configuration.
project(Project)

# Include the CMake "ExternalProject" tools, otherwise nothing will
# work.
include(ExternalProject)

# Configuration
set(THIRDPARTY_INSTALL_PREFIX "${CMAKE_CURRENT_SOURCE_DIR}/install")

##################
# zlib
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_zlib.bash
#
ExternalProject_Add(zlib
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/zlib
    GIT_REPOSITORY "https://github.com/madler/zlib.git"
    GIT_TAG "v1.2.11"
    INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}/zlib
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}/zlib
)


##################
# libjpeg
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_libjpeg-turbo.bash
#
ExternalProject_Add(libjpeg_turbo
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/libjpeg_turbo
    GIT_REPOSITORY "https://github.com/libjpeg-turbo/libjpeg-turbo.git"
    GIT_TAG "2.0.5"
    INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}/libjpeg_turbo
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}/libjpeg_turbo
)


##################
# libtiff
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_libtiff.bash
#
ExternalProject_Add(libtiff
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/libtiff
    GIT_REPOSITORY "https://gitlab.com/libtiff/libtiff.git"
    GIT_TAG "v4.1.0"
    INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}/libtiff
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}/libtiff
        -DZLIB_ROOT=${THIRDPARTY_INSTALL_PREFIX}/zlib
)

##################
# libpng
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_libpng.bash
#
ExternalProject_Add(libpng
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/libpng
    GIT_REPOSITORY "https://github.com/glennrp/libpng.git"
    GIT_TAG "v1.6.35"
    INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}/libpng
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}/libpng
        -DCMAKE_INCLUDE_PATH=${THIRDPARTY_INSTALL_PREFIX}/zlib/include/
        -DCMAKE_LIBRARY_PATH=${THIRDPARTY_INSTALL_PREFIX}/zlib/lib/
        -DPNG_BUILD_ZLIB=OFF
        -DPNG_EXECUTABLES=OFF
        -DPNG_TESTS=OFF
        -DPNG_SHARED=ON
        -DPNG_STATIC=ON
)

##################
# OpenColorIO
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_opencolorio.bash
#

##################
# OpenEXR.
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_openexr.bash
#
# # ExternalProject_Add(openexr
# #     PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/openexr
# #     GIT_REPOSITORY "https://github.com/AcademySoftwareFoundation/openexr.git"
# #     GIT_TAG "v2.2.2"
# #     INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}/openexr
# #     CMAKE_ARGS
# #         -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
# #         -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
# #         -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
# #         -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}
# # )

# # OpenImageIO - Used to read files using OpenImageIO library
# ExternalProject_Add(openimageio
#     PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/openimageio
#     GIT_REPOSITORY "https://github.com/OpenImageIO/oiio.git"
#     GIT_TAG "v2.2.10.1"
#     INSTALL_DIR ${THIRDPARTY_INSTALL_PREFIX}
#     CMAKE_ARGS
#         -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
#         -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
#         -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
#         -DCMAKE_INSTALL_PREFIX=${THIRDPARTY_INSTALL_PREFIX}
#         -DEMBEDPLUGINS=OFF
#         -DUSE_PYTHON=OFF
#         -DOIIO_BUILD_TESTS=OFF
#         -DOIIO_BUILD_TOOLS=OFF
#         -DUSE_OPENGL=OFF
#         -DUSE_QT=OFF
#         -DBUILD_SHARED_LIBS=OFF
#         -DLINKSTATIC=ON
#         -DBUILD_MISSING_DEPS=OFF
# )


