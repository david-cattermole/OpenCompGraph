# OpenCompGraph

The **OpenCompGraph** (or **OCG** pronounced *Ohh-See-Gee*) is a
cross-platform library Rust/C++ library to provide a small 2D Image
Compositing Graph framework with an emphisis on concurent and parallel
image caching, real-time image sequence playback, and efficent
techniques to transfer data within image processing nodes.

This is the core library used in
[Open Comp Graph Maya](https://github.com/david-cattermole/OpenCompGraphMaya/).

This project and API is not ready for production (yet). The function,
nodes, classes may change at any moment. See the Roadmap section
(below) and
[issues](https://github.com/david-cattermole/OpenCompGraphMaya/issues)
page for more details. If you have suggestions for features or would
like to test, please do.

## Road Map

Below is a brief description on some of the features planed and being
worked on.

| Feature Description                                                          |        Status |
|:-----------------------------------------------------------------------------|--------------:|
| MS Windows support (MSVC)                                                    |          Done |
| Linux support (GCC)                                                          | To be started |
| MacOS support (Clang)                                                        | To be started |
| Rust to C++ API                                                              |   In progress |
| Node connection frame work.                                                  |          Done |
| Node attribute getting/setting.                                              |          Done |
| Node image data streams.                                                     |          Done |
| Multi-frame Execution context.                                               |          Done |
| Logging infrastructure for simple debug.                                     |          Done |
| JPEG image reading support.                                                  |          Done |
| JPEG image writing support.                                                  |          Done |
| PNG image reading support. (8-bit only)                                      |          Done |
| PNG image writing support. (8-bit only)                                      |          Done |
| TARGA image reading support.                                                 |          Done |
| TARGA image writing support.                                                 |          Done |
| TIFF image reading support. (8-bit only)                                     |          Done |
| TIFF image writing support. (8-bit only)                                     |          Done |
| EXR image reading support.                                                   |          Done |
| EXR image writing support.                                                   |          Done |
| Generating polygon geometry (for 3D texture playback).                       |          Done |
| Applying deformers to polygon vertices.                                      |          Done |
| Convert Lens distortion (deformers) to ST-Maps                               | To be started |
| Transform Matrix node data concatenation                                     |          Done |
| Pixel Deformer node data concatenation (lens distortion)                     |          Done |
| Color Matrix node data concatenation (lens distortion)                       |          Done |
| Image Sequence caching and real-time playback.                               |          Done |
| Multi-threaded/asyncronous graph execution.                                  | To be started |
| Configuration with files and environment variables.                          |   In progress |
| Internal Unsigned Integer 8-bit Pixel Format.                                |          Done |
| Internal Unsigned Integer 16-bit Pixel Format.                               |          Done |
| Internal "Half" Floating point 16-bit Pixel Format.                          |          Done |
| Internal Floating Point 32-bit Pixel Format.                                 |          Done |
| Image metadata display window and data window support.                       |          Done |
| Standalone executable to read and executre node graphs.                      | To be started |
| LDPK integration for lens distortion and image deformation.                  |   In progress |
| OpenImageIO integration for image reading and image processing.              |   In progress |
| OpenColorIO integration for accurate and configurable color workflow.        |   In progress |
| Create a Python API to wrap the library.                                     | To be started |
| R&D - Decrease RAM usage while still working in floating-point linear color. | To be startad |
| Write out cached data as loadable files for improved read-performance.       | To be started |

These are the nodes that are planed or currently implemented.

| Node Name     | Description                                                        |        Status |
|:--------------|--------------------------------------------------------------------|--------------:|
| ReadImage     | Read an image file (JPEG, PNG, TIFF, TGA) as a stream.             |          Done |
| WriteImage    | Write an image data stream to disk (JPEG, PNG, TIFF, TGA).         |          Done |
| LensDistort   | Deforms the image pixels with a brownian lens distortion function. |          Done |
| (Color) Grade | Applies a color matrix (4x4) to the input image stream             |          Done |
| Transform     | Applies a 2D transform to the input image stream                   |          Done |
| Null          | A "no op" node, that does nothing.                                 |          Done |
| MergeImage    | Blend/Combine 1 or more images together into a single image.       |   In progress |
| CropImage     | Remove pixel data outside a square region.                         | To be started |
| ReformatImage | Change the resolution of the image data.                           | To be started |
| Exposure      | Change the colors of an image using exposure values (EV)           | To be started |
| ConvertSTMap  | Apply a deformer as pixel colours                                  | To be started |
| Keyer         | Calculate an alpha channel for a green/blue-screen image           | To be started |


## Building

To build (compile) the plug-in follow the steps in
[BUILD.md](https://github.com/david-cattermole/OpenCompGraph/blob/master/BUILD.md).

## License

*Open Comp Graph* (OCG) is licensed under the
[Lesser GNU Public License v3.0](https://github.com/david-cattermole/OpenCompGraph/blob/master/LICENSE)
or *LGPL-3.0* for short.
This means the project is Free Open Source Software, and will always
stay Free Open Source Software:
[TL;DR](https://www.tldrlegal.com/l/lgpl-3.0).

Please read the *LICENSE* (text) file for details.

## Bugs, Questions or Issues?

All issues are listed in the
[issues page](https://github.com/david-cattermole/OpenCompGraph/issues)
on the project page. If you have found a bug, please submit an issue and we will
try to address it as soon as possible.
