# OpenCompGraph
2D Image Compositing Framework.

This project is in early proto-typing stages!

## Goals

This tool will be a cross-platform library that can perform various 2D
operations on images and their metadata.

- Fast & Simple to use in a C++ project.
- Minimal dependencies for ease of building.
- Simple build instructions.
- Targeting CentOS 7.x and Windows
- 8, 16 and 32-bit depth colour support.
- Do all pixel operations in linear colour space.
- Multiple Formats Supported
  - EXR reader
  - JPEG reader
  - OpenImageIO
  - STB (JPEG, PNG, TIFF, TARGA)
- Real-time image reader for fast playback (60 fps)
- Dump data to disk in compressed format (buffer compression file(s)).
- Caching must be optional (for testing), the Node object should
  know nothing about the Cache, it must function without the Cache.
- Initial version of project will be single-threaded, but API design
  should be open for multi-threaded processing of the graph. Each
  operation may be multi-threaded.
- We should be able to use the composite graph with Python.


## Use Cases

Read an image:
- Get path
- Create Cache
- Read image
- Get pixel data

Read multiple images:
- Get sequence of file paths
- Load each file into memory

Apply Non-Linear Point Offset (lens distortion)
- Read image
- Apply Point Operations
- Deform Image by Point Ops

Read an image and write it out again:
- Read image
- Connect image to write node.

## Compositing Operations

This section lists the required operations that can be performed in an
image.

- Read Image File (image)
   - No inputs
- Write Image File (image)
  - Input 0; image
- Lens Distortion 3DE LDPK (point)
- Apply ST Map (pixels) 
  - Input 0; image
  - Input 1; point
- Reformat (image + point)
- Crop (image)
- 2D Transform (point)
- Grade (color)
- Exposure (color)
- Merge (composite)
  - Input 0; image
  - Input 1; image

## General Ideas

- Must allow a chain of ops to write to a final output without
  allocation of pixel data for each operation.
- Some intermediate computations can be discarded from the cache.
- Think about how to allow caching of a sequence without playing on the frame 
- Make sure to support texture IDs (for GPU resources)
- Make sure to separate metadata from image data
- Node based - bakes down to "operations" or ops.
- Use Hashing function 'Murmur2'
- All image ops must be encapsulated, so we can account for hashing.
- Hashing is the responsibility of the op.
- Need to create a graph of operations.
- May have transform ops, or point ops, or pixel ops, or colour ops.
- The cache should not be static/global - Allow multiple caches to
  exist or not exist.
- The entire tool should never know or care about frames or time. We
  just care about combinations of parameters.
- Caching images other than the final image should be off by
  default. Users should turn on caching of specific operations.
- Each operation may have a cache priority associated with it. Less
  important operations are lower priority and can be removed from the
  cache at any time.
- Allow cropping and transforms to be concatenated and live separately
  from image data, for optimal performance.
- Make sure there are Python Bindings for the library. We want to read
  the pixel data in Python for a Qt application, or NumPy/SciPy.
- There are different types operations depending on what is edited.
  - Image Operations - modifying any/all data.
  - Pixel Operations - modifying pixels
  - Color Operations - modifying colors only.
  - Transform Operations - 2D operation able to be condensed into a 2D matrix.
  - Point Operations - Manipulating 2D points (like lens distortion or grid-warps).
- The op graph should compile down all ops into a sequence of
  functions. Some operations can be performed in parallel, or
  sequentially.
- Use the concepts of functional programming. Pass callbacks to
  calculate operations.
- Each read file operation (by convention) should have a flag "proxy
  factor", to allow calculation on every Nth pixel of the input
  data. So we can preview a lower res image, then flick to full
  resolution for final. This is to avoid excess memory allocation, for
  example extremely large images or long image sequences.
- We should be able to use Python bindings or Maya nodes to
  dynamically create a auto-comp preview of render layers or
  playblasts.
- Rather than using "hashes" to connect operations together, we can
  use "handles", like file descriptors are handled by an operating
  system, or we can connect operations directly.
- We must be sure to support OpenEXR DWA compression.
- It would be ideal to support a background thread to write into the
  cache. This will allow image sequence reading while keeping the user
  from being blocked. The cache must therefore have a thread lock.
