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

# Install Paths
SET(ZLIB_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/zlib)
SET(LIBJPEG_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libjpeg_turbo)
SET(LIBTIFF_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libtiff)
SET(LIBPNG_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libpng)
SET(ILMBASE_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/ilmbase)
SET(OPENEXR_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/openexr)
SET(OPENIMAGEIO_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/openimageio)

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
    INSTALL_DIR ${ZLIB_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${ZLIB_INSTALL_PATH}
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
    INSTALL_DIR ${LIBJPEG_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${LIBJPEG_INSTALL_PATH}
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
    DEPENDS zlib
    GIT_REPOSITORY "https://gitlab.com/libtiff/libtiff.git"
    GIT_TAG "v4.1.0"
    INSTALL_DIR ${LIBTIFF_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${LIBTIFF_INSTALL_PATH}
        -DZLIB_ROOT=${ZLIB_INSTALL_PATH}
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
    DEPENDS zlib
    GIT_REPOSITORY "https://github.com/glennrp/libpng.git"
    GIT_TAG "v1.6.35"
    INSTALL_DIR ${LIBPNG_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${LIBPNG_INSTALL_PATH}
        -DCMAKE_INCLUDE_PATH=${ZLIB_INSTALL_PATH}/include/
        -DCMAKE_LIBRARY_PATH=${ZLIB_INSTALL_PATH}/lib/
        -DPNG_BUILD_ZLIB=OFF
        -DPNG_EXECUTABLES=OFF
        -DPNG_TESTS=OFF
        -DPNG_SHARED=ON
        -DPNG_STATIC=ON
)


##################
# OpenEXR - ILM Base
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_openexr.bash
#
ExternalProject_Add(ilmbase
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/ilmbase
    DEPENDS zlib
    GIT_REPOSITORY "https://github.com/AcademySoftwareFoundation/openexr.git"
    GIT_TAG "v2.2.2"
    INSTALL_DIR ${ILMBASE_INSTALL_PATH}
    SOURCE_SUBDIR "IlmBase"
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${ILMBASE_INSTALL_PATH}
        -DCMAKE_CXX_STANDARD=11
        -DBUILD_SHARED_LIBS=ON
)


##################
# OpenEXR
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_openexr.bash
#
ExternalProject_Add(openexr
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/openexr
    DEPENDS ilmbase zlib
    GIT_REPOSITORY "https://github.com/AcademySoftwareFoundation/openexr.git"
    GIT_TAG "v2.2.2"
    INSTALL_DIR ${OPENEXR_INSTALL_PATH}
    SOURCE_SUBDIR "OpenEXR"
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${OPENEXR_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH=${ZLIB_INSTALL_PATH}
        -DCMAKE_INCLUDE_PATH=${ZLIB_INSTALL_PATH}/include/
        -DCMAKE_LIBRARY_PATH=${ZLIB_INSTALL_PATH}/lib/
        -DILMBASE_PACKAGE_PREFIX=${ILMBASE_INSTALL_PATH}
        -DCMAKE_CXX_STANDARD=11
        -DBUILD_SHARED_LIBS=ON
)
