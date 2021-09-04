# Benchmarks

## Benchmark test_cache_read_image_seq

Testing "test_cache_read_image_seq.cpp".

Some benchmark numbers for this test to complete with different
image types.

Tests: We read the same image encoded into different formats. The images
are all 3840 x 2160 (UHD) with RGBA channels (if alpha is
supported, otherwise RGB). There are 50 images in the sequence that
is loaded.

Note this benchmark cares about file read and decode performance
mostly, disk-space is a secondary concern and it is assumed that the
files are written to a scatch/temporary (SSD) device with fast read
speeds, and enough space.

### Benchmark Results

- JPEG (RGB 8-bit) quality 100, subsampling 2x2 (lossy) = 10.3 seconds

- PNG (RGBA 8-bit) compression level 0 (lossless) = 38.8 seconds

- PNG (RGBA 8-bit) compression level 1 (lossless) = 25.7 seconds

- PNG (RGBA 8-bit) compression level 2 (lossless) = 23.6 seconds

- PNG (RGBA 8-bit) compression level 3 (lossless)= 23.3 seconds

- PNG (RGBA 8-bit) compression level 6 (lossless) = 27.3 seconds

- EXR (RGBA 16-bit half float) ZIPS (scanline) (lossess) = 159.2 to 173.4 seconds

- EXR (RGBA 16-bit half float) DWAA 45 (lossy) = 14.7 seconds

- EXR (RGBA 16-bit half float) DWAB 45 (lossy) = 17.8 seconds

The exact time should not be used, but relative numbers might be
assumed for similar hardware. Do not trust these numbers, do your
own tests to be sure.

Please note the speed of each file format compression technique,
but also the limitations of each file format, including the
bit-depth allowed and the channels (RGB vs RGBA).

### Software and Hardware

These benchmark numbers are based on hardware:

- Intel i7-7700HQ CPU @ 2.80GHz (4 Cores and 4 Logical Processors)
- 32GB RAM
- SSD PM981 NVMe Samsung 1024GB (formatted with NTFS file system)

With software:

- zlib v1.2.11
- libpng v1.6.35
- libjpeg-turbo v2.0.5
- OpenEXR v2.2.2
- OpenImageIO v2.2.10.1

Running on:

- Windows 10 (10.0.19043 Build 19043)

Compiled on:

- MSVC 19.0.24215.1

### Conclusions

- Overall, EXR DWAA compression is a good start, as long as you
  have enough RAM for 16-bit images.

- If you want fast playback and don't care about memory usage, use
  EXR with DWAA compression.

- If you care about memory usage, and need an alpha channel use
  PNG.

- If you care about memory usage, you do not care about color
  bit-depth, and you don't need an alpha channel, use JPEG.

- JPEG is fastest, but lacks 16-bit float and an alpha channel. If
  an alpha channel is needed, but you want to keep a low-memory
  footprint use PNG.

- PNG compression 3 is fastest for 8-bit with an alpha channel.
  PNG is good because it has low-memory usage and contains an alpha
  channel. PNG is bad compared to JPEG because it's roughly 2 times
  slower to read.

- EXR DWAA compression should be preferred for RGBA images needing
  color fidelity with minimal compression artifacts and a high
  playback speed. Do not use EXR with ZIP (lossless) compression
  unless you do not care about playback speed, as it is
  significantly slower in the tests.
