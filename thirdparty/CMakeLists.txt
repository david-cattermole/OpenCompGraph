#
# Copyright (C) 2020, 2021 David Cattermole.
#
# This file is part of OpenCompGraph.
#
# OpenCompGraph is free software: you can redistribute it and/or modify it
# under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# OpenCompGraph is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with OpenCompGraph.  If not, see <https:#www.gnu.org/licenses/>.
# ====================================================================
#
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
SET(BZIP2_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/bzip2)
SET(ZLIB_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/zlib)
SET(LIBJPEG_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libjpeg_turbo)
SET(OPENJPEG_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/openjpeg)
SET(LIBTIFF_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libtiff)
SET(LIBPNG_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/libpng)
SET(ILMBASE_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/ilmbase_openexr)
SET(OPENEXR_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/ilmbase_openexr)
SET(OPENCOLORIO_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/opencolorio)
SET(OPENIMAGEIO_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/openimageio)
SET(DLFCN_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/dlfcn)
SET(LDPK_INSTALL_PATH ${THIRDPARTY_INSTALL_PREFIX}/ldpk)

##################
# bzip2
##################
#
# bzip2 is required for FFMPEG, inside OpenImageIO.
ExternalProject_Add(bzip2
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/bzip2
    GIT_REPOSITORY "https://gitlab.com/federicomenaquintero/bzip2.git"
    GIT_TAG "6211b6500c8bec13a17707440d3a84ca8b34eed5"
    INSTALL_DIR ${BZIP2_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${BZIP2_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DENABLE_APP=ON
        -DENABLE_EXAMPLES=OFF
)

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
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
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
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
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
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DZLIB_ROOT=${ZLIB_INSTALL_PATH}
        -DBUILD_SHARED_LIBS=ON
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
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DPNG_BUILD_ZLIB=OFF
        -DPNG_EXECUTABLES=OFF
        -DPNG_TESTS=OFF
        -DPNG_SHARED=ON
        -DPNG_STATIC=OFF
)


##################
# GIF
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_gif.bash
#
# TODO: Add GIF support (on *NIX only (because Windows doesn't have
#    make and it will be hard to build it for Windows.)


##################
# OpenJPEG
##################
#
SET(OPENJPEG_C_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE}")
if(MSVC)
  # On Windows
  SET(OPENJPEG_C_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /O2 /arch:AVX2 /DNDEBUG")
else()
  # On Linux and MacOS (Clang and GCC)
  SET(OPENJPEG_C_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -mavx2 -DNDEBUG")
endif()

ExternalProject_Add(openjpeg
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/openjpeg
    GIT_REPOSITORY "https://github.com/uclouvain/openjpeg.git"
    # OpenImageIO v2.2.10.1 CMake files can only find up to v2.3
    # automatically, so we're currently limited.
    #
    # TODO: Fix the CMake files in OpenImageIO and submit a PR.
    GIT_TAG "v2.3.1"
    INSTALL_DIR ${OPENJPEG_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_C_FLAGS_RELEASE=${OPENJPEG_C_FLAGS_RELEASE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${OPENJPEG_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH="${CMAKE_PREFIX_PATH}"
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DBUILD_SHARED_LIBS:BOOL=ON
        -DBUILD_THIRDPARTY:BOOL=ON
)

##################
# WebP
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_webp.bash
#
# TODO: Add WebP support.


##################
# LibRaw
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_libraw.bash
#     https://github.com/LibRaw/LibRaw-cmake
#     https://github.com/LibRaw/LibRaw
#     https://github.com/LibRaw/LibRaw.git
#
# TODO: Add LibRaw support.
#


##################
# ILM Base (OpenEXR)
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
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
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
        -DCMAKE_PREFIX_PATH="${CMAKE_PREFIX_PATH};${ZLIB_INSTALL_PATH}"
        -DCMAKE_INCLUDE_PATH=${ZLIB_INSTALL_PATH}/include/
        -DCMAKE_LIBRARY_PATH=${ZLIB_INSTALL_PATH}/lib/
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DILMBASE_PACKAGE_PREFIX=${ILMBASE_INSTALL_PATH}
        -DCMAKE_CXX_STANDARD=11
        -DBUILD_SHARED_LIBS=ON
)


##################
# OpenColorIO
##################
#
# Reference:
#     https://github.com/OpenImageIO/oiio/blob/master/src/build-scripts/build_opencolorio.bash
#
ExternalProject_Add(opencolorio
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/opencolorio
    DEPENDS zlib
    GIT_REPOSITORY "https://github.com/AcademySoftwareFoundation/OpenColorIO.git"
    GIT_TAG "v1.1.1"
    INSTALL_DIR ${OPENCOLORIO_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${OPENCOLORIO_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH="${CMAKE_PREFIX_PATH};${ZLIB_INSTALL_PATH}"
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DOCIO_BUILD_APPS=OFF
        -DOCIO_BUILD_NUKE=OFF
        -DOCIO_BUILD_DOCS=OFF
        -DOCIO_BUILD_TESTS=OFF
        -DOCIO_BUILD_GPU_TESTS=OFF
        -DOCIO_BUILD_TRUELIGHT=OFF
        -DOCIO_BUILD_PYTHON=OFF
        -DOCIO_BUILD_PYGLUE=OFF
        -DOCIO_BUILD_JAVA=OFF
        -DOCIO_BUILD_JNIGLUE=OFF
        -DOCIO_BUILD_SHARED=ON
        -DOCIO_BUILD_STATIC=OFF
)


##################
# OpenImageIO
##################
ExternalProject_Add(openimageio
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/openimageio
    DEPENDS bzip2 zlib libjpeg_turbo libtiff libpng openjpeg ilmbase openexr opencolorio
    GIT_REPOSITORY "https://github.com/OpenImageIO/oiio.git"
    GIT_TAG "v2.2.10.1"
    INSTALL_DIR ${OPENIMAGEIO_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${OPENIMAGEIO_INSTALL_PATH}
        -DCMAKE_INCLUDE_PATH="${ZLIB_INSTALL_PATH}/include;${CMAKE_INCLUDE_PATH}"
        -DCMAKE_PREFIX_PATH="${ILMBASE_INSTALL_PATH};${BZIP2_INSTALL_PATH};${ZLIB_INSTALL_PATH};${OPENEXR_INSTALL_PATH};${LIBJPEG_INSTALL_PATH};${LIBTIFF_INSTALL_PATH};${LIBPNG_INSTALL_PATH};${OPENCOLORIO_INSTALL_PATH};${CMAKE_PREFIX_PATH}"
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DCMAKE_CXX_STANDARD=11
        -DZLIB_ROOT=${ZLIB_INSTALL_PATH}
        -DBZip2_ROOT=${BZIP2_INSTALL_PATH}
        -DUSE_JPEGTURBO=ON
        -DJPEGTurbo_ROOT=${LIBJPEG_INSTALL_PATH}
        -DJPEG_ROOT=${LIBJPEG_INSTALL_PATH}
        -DOpenJpeg_ROOT=${OPENJPEG_INSTALL_PATH}
        -DPNG_ROOT=${LIBPNG_INSTALL_PATH}
        -DTIFF_ROOT=${LIBTIFF_INSTALL_PATH}
        -DILMBASE_ROOT=${ILMBASE_INSTALL_PATH}
        -DOPENEXR_ROOT=${OPENEXR_INSTALL_PATH}
        -DUSE_OPENCOLORIO=ON
        -DUSE_OCIO=ON
        -DOpenColorIO_ROOT=${OPENCOLORIO_INSTALL_PATH}
        -DUSE_OPENCV=OFF
        -DUSE_TBB=OFF
        # -DHDF5_ROOT=?
        # -DOpenCV_ROOT=?
        # -DTBB_ROOT=?
        # -DDCMTK_ROOT=?
        # -DFFmpeg_ROOT=?
        # -DGIF_ROOT=?
        # -DLibheif_ROOT=?
        # -DLibRaw_ROOT=?
        # -DPTex_ROOT=?
        # -DWebP_ROOT=?
        # -DR3DSDK_ROOT=?
        # -DNuke_ROOT=?
        # -DLibsquish_ROOT=?
        -DVERBOSE=ON
        -DUSE_PYTHON=OFF
        -DUSE_OPENGL=OFF
        -DUSE_QT=OFF
        -DUSE_FREETYPE=OFF
        -DUSE_FIELD3D=OFF
        -DUSE_OPENVDB=OFF
        -DUSE_EXTERNAL_PUGIXML=OFF
        -DOIIO_BUILD_TESTS=OFF
        -DOIIO_BUILD_TOOLS=OFF
        -DLINKSTATIC=OFF
        -DBUILD_SHARED_LIBS=ON
        -DBUILD_DOCS=OFF
        -DBUILD_MISSING_DEPS=OFF
        -DBUILD_COIIO=OFF
        -DEMBEDPLUGINS=ON
        -DINSTALL_DOCS=OFF
        -DINSTALL_FONTS=OFF
        -DINSTALL_CMAKE_HELPER=ON
)


##################
# dlfcn
##################
#
# dlfcn is required for the LDPK on Windows only.
ExternalProject_Add(dlfcn
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/dlfcn
    GIT_REPOSITORY "https://github.com/dlfcn-win32/dlfcn-win32.git"
    GIT_TAG "v1.2.0"
    INSTALL_DIR ${DLFCN_INSTALL_PATH}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${DLFCN_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DBUILD_SHARED_LIBS=ON
        -DBUILD_TESTS=OFF
)


##################
# LDPK (Lens Distortion Plug-in Kit)
##################
#
# The LDPK from Science-d-Visions is used to deform images using lens
# models.
SET(LDPK_PATCH_DIR ${CMAKE_CURRENT_SOURCE_DIR}/patches/ldpk)
SET(LDPK_SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/ldpk/src/ldpk)
SET(LDPK_PATCH_CMAKELISTS_TXT ${LDPK_PATCH_DIR}/CMakeLists.txt)
SET(LDPK_PATCH_CONFIG_FILE ${LDPK_PATCH_DIR}/ldpk-config.cmake.in)
SET(LDPK_INSTALL_CMAKELISTS_TXT_DIR ${LDPK_SRC_DIR})
SET(LDPK_INSTALL_CONFIG_FILE_DIR ${LDPK_SRC_DIR})
ExternalProject_Add(ldpk
    PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/ldpk
    DEPENDS dlfcn
    URL "https://www.3dequalizer.com/user_daten/sections/tech_docs/archives/ldpk-2.4.tgz"
    INSTALL_DIR ${LDPK_INSTALL_PATH}
    PATCH_COMMAND ${CMAKE_COMMAND} -E copy ${LDPK_PATCH_CMAKELISTS_TXT} ${LDPK_SRC_DIR} && ${CMAKE_COMMAND} -E copy ${LDPK_PATCH_CONFIG_FILE} ${LDPK_SRC_DIR}
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}
        -DCMAKE_INSTALL_PREFIX=${LDPK_INSTALL_PATH}
        -DCMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
        -DCMAKE_IGNORE_PATH=${CMAKE_IGNORE_PATH}
        -DDLFCN_LOCATION=${DLFCN_INSTALL_PATH}
)
