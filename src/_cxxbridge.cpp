#include "rust/cxx.h"
#include "opencompgraph/symbol_export.h"
#include "opencompgraph/internal/colorspace.h"
#include "opencompgraph/internal/imageresample.h"
#include "opencompgraph/internal/ldpk_utils.h"
#include "opencompgraph/internal/imageio.h"
#include "opencompgraph/internal/systemmemory.h"
#include <functional>
#include <memory>

namespace rust {
inline namespace cxxbridge1 {
class Str::uninit {};
inline Str::Str(uninit) noexcept {}

template <typename T>
class Slice<T>::uninit {};
template <typename T>
inline Slice<T>::Slice(uninit) noexcept {}

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
namespace repr {
using Fat = ::std::array<::std::uintptr_t, 2>;
} // namespace repr

template <>
class impl<Str> final {
public:
  static Str new_unchecked(repr::Fat repr) noexcept {
    Str str = Str::uninit{};
    str.repr = repr;
    return str;
  }
  static repr::Fat repr(Str str) noexcept {
    return str.repr;
  }
};

template <typename T>
class impl<Slice<T>> final {
public:
  static Slice<T> slice(repr::Fat repr) noexcept {
    Slice<T> slice = typename Slice<T>::uninit{};
    slice.repr = repr;
    return slice;
  }
};

template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

namespace open_comp_graph {
  struct BBox2Df;
  struct BBox2Di;
  struct Vector4f32;
  struct Vector4i32;
  struct Matrix4;
  struct BlockSize;
  enum class GraphState : ::std::uint8_t;
  enum class ExecuteStatus : ::std::uint8_t;
  enum class BakeOption : ::std::uint8_t;
  enum class NodeType : ::std::uint8_t;
  enum class DataType : ::std::uint8_t;
  enum class NodeStatus : ::std::uint8_t;
  enum class AttrState : ::std::uint8_t;
  enum class StreamDataState : ::std::uint8_t;
  enum class LensDistortDirection : ::std::uint8_t;
  enum class MergeImageMode : ::std::uint8_t;
  enum class ImageOrientation : ::std::uint8_t;
  enum class DiskCacheImageType : ::std::uint8_t;
  enum class CropOnWrite : ::std::uint8_t;
  enum class JpegChromaSubSampling : ::std::uint8_t;
  enum class ExrCompression : ::std::uint8_t;
  namespace internal {
    struct ImageCompression;
    struct GraphImplShared;
    struct StreamDataImplShared;
    struct CacheImplShared;
    struct ConfigImplShared;
    struct ImageSpec;
    struct ImageShared;
    enum class ParameterType : ::std::int32_t;
    using OcgLdPluginBase = ::open_comp_graph::internal::OcgLdPluginBase;
    struct PixelBlock;
    struct StreamDataImplRc;
    struct StreamDataImpl;
    struct NodeImpl;
    struct CacheImpl;
    struct GraphImpl;
    struct GeometryPlaneImpl;
    struct ConfigImpl;
  }
}

namespace open_comp_graph {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Df
#define CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Df
struct BBox2Df final {
  float min_x;
  float min_y;
  float max_x;
  float max_y;

  bool operator==(const BBox2Df &) const noexcept;
  bool operator!=(const BBox2Df &) const noexcept;
  bool operator<(const BBox2Df &) const noexcept;
  bool operator<=(const BBox2Df &) const noexcept;
  bool operator>(const BBox2Df &) const noexcept;
  bool operator>=(const BBox2Df &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Df

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Di
#define CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Di
struct BBox2Di final {
  ::std::int32_t min_x;
  ::std::int32_t min_y;
  ::std::int32_t max_x;
  ::std::int32_t max_y;

  bool operator==(const BBox2Di &) const noexcept;
  bool operator!=(const BBox2Di &) const noexcept;
  bool operator<(const BBox2Di &) const noexcept;
  bool operator<=(const BBox2Di &) const noexcept;
  bool operator>(const BBox2Di &) const noexcept;
  bool operator>=(const BBox2Di &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$BBox2Di

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$Vector4f32
#define CXXBRIDGE1_STRUCT_open_comp_graph$Vector4f32
struct Vector4f32 final {
  float x;
  float y;
  float z;
  float w;

  bool operator==(const Vector4f32 &) const noexcept;
  bool operator!=(const Vector4f32 &) const noexcept;
  bool operator<(const Vector4f32 &) const noexcept;
  bool operator<=(const Vector4f32 &) const noexcept;
  bool operator>(const Vector4f32 &) const noexcept;
  bool operator>=(const Vector4f32 &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$Vector4f32

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$Vector4i32
#define CXXBRIDGE1_STRUCT_open_comp_graph$Vector4i32
struct Vector4i32 final {
  ::std::int32_t x;
  ::std::int32_t y;
  ::std::int32_t z;
  ::std::int32_t w;

  bool operator==(const Vector4i32 &) const noexcept;
  bool operator!=(const Vector4i32 &) const noexcept;
  bool operator<(const Vector4i32 &) const noexcept;
  bool operator<=(const Vector4i32 &) const noexcept;
  bool operator>(const Vector4i32 &) const noexcept;
  bool operator>=(const Vector4i32 &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$Vector4i32

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$Matrix4
#define CXXBRIDGE1_STRUCT_open_comp_graph$Matrix4
struct Matrix4 final {
  float m00;
  float m01;
  float m02;
  float m03;
  float m10;
  float m11;
  float m12;
  float m13;
  float m20;
  float m21;
  float m22;
  float m23;
  float m30;
  float m31;
  float m32;
  float m33;

  bool operator==(const Matrix4 &) const noexcept;
  bool operator!=(const Matrix4 &) const noexcept;
  bool operator<(const Matrix4 &) const noexcept;
  bool operator<=(const Matrix4 &) const noexcept;
  bool operator>(const Matrix4 &) const noexcept;
  bool operator>=(const Matrix4 &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$Matrix4

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$BlockSize
#define CXXBRIDGE1_STRUCT_open_comp_graph$BlockSize
struct BlockSize final {
  ::std::int32_t width;
  ::std::int32_t height;
  ::std::int32_t num_channels;

  bool operator==(const BlockSize &) const noexcept;
  bool operator!=(const BlockSize &) const noexcept;
  bool operator<(const BlockSize &) const noexcept;
  bool operator<=(const BlockSize &) const noexcept;
  bool operator>(const BlockSize &) const noexcept;
  bool operator>=(const BlockSize &) const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$BlockSize

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageCompression
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageCompression
struct ImageCompression final {
  ::open_comp_graph::ExrCompression exr_compression;
  ::std::int32_t exr_dwa_compression_level;
  ::std::int32_t png_compression_level;
  ::std::int32_t jpeg_compression_level;
  ::open_comp_graph::JpegChromaSubSampling jpeg_subsampling;
  bool jpeg_progressive;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageCompression

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared
struct GraphImplShared final {
  ::rust::Box<::open_comp_graph::internal::GraphImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplShared
struct StreamDataImplShared final {
  ::rust::Box<::open_comp_graph::internal::StreamDataImplRc> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImplShared
struct CacheImplShared final {
  ::rust::Box<::open_comp_graph::internal::CacheImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImplShared
struct ConfigImplShared final {
  ::rust::Box<::open_comp_graph::internal::ConfigImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageSpec
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageSpec
struct ImageSpec final {
  ::rust::String color_space;
  float gamma;
  float pixel_aspect;
  ::open_comp_graph::ImageOrientation orientation;
  bool unassociated_alpha;
  ::std::int32_t dither;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageSpec

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageShared
struct ImageShared final {
  ::rust::Box<::open_comp_graph::internal::PixelBlock> pixel_block;
  ::open_comp_graph::internal::ImageSpec spec;
  ::open_comp_graph::BBox2Di display_window;
  ::open_comp_graph::BBox2Di data_window;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageShared
} // namespace internal

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$GraphState
#define CXXBRIDGE1_ENUM_open_comp_graph$GraphState
enum class GraphState : ::std::uint8_t {
  kDirty = 0,
  kClean = 1,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$GraphState

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus
#define CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus
enum class ExecuteStatus : ::std::uint8_t {
  kError = 0,
  kSuccess = 1,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$BakeOption
#define CXXBRIDGE1_ENUM_open_comp_graph$BakeOption
enum class BakeOption : ::std::uint8_t {
  kNothing = 0,
  kColorSpace = 1,
  kColorSpaceAndGrade = 2,
  kAll = 3,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$BakeOption

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$NodeType
#define CXXBRIDGE1_ENUM_open_comp_graph$NodeType
enum class NodeType : ::std::uint8_t {
  kNull = 0,
  kReadImage = 1,
  kWriteImage = 2,
  kViewer = 8,
  kTransform = 5,
  kCropImage = 7,
  kResampleImage = 9,
  kMergeImage = 6,
  kGrade = 3,
  kLensDistort = 4,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$NodeType

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$DataType
#define CXXBRIDGE1_ENUM_open_comp_graph$DataType
enum class DataType : ::std::uint8_t {
  kFloat32 = 0,
  kHalf16 = 1,
  kUInt8 = 2,
  kUInt16 = 3,
  kUnknown = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$DataType

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus
#define CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus
enum class NodeStatus : ::std::uint8_t {
  kError = 0,
  kValid = 1,
  kWarning = 2,
  kNonExistent = 3,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$AttrState
#define CXXBRIDGE1_ENUM_open_comp_graph$AttrState
enum class AttrState : ::std::uint8_t {
  kMissing = 0,
  kExists = 1,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$AttrState

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState
#define CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState
enum class StreamDataState : ::std::uint8_t {
  kInvalid = 0,
  kValid = 1,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$LensDistortDirection
#define CXXBRIDGE1_ENUM_open_comp_graph$LensDistortDirection
enum class LensDistortDirection : ::std::uint8_t {
  kUndistort = 0,
  kDistort = 1,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$LensDistortDirection

namespace internal {
#ifndef CXXBRIDGE1_ENUM_open_comp_graph$internal$ParameterType
#define CXXBRIDGE1_ENUM_open_comp_graph$internal$ParameterType
enum class ParameterType : ::std::int32_t {
  kString = 0,
  kDouble = 1,
  kInt = 2,
  kFile = 3,
  kBoolean = 4,
  kAdjustableDouble = 5,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$internal$ParameterType
} // namespace internal

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$MergeImageMode
#define CXXBRIDGE1_ENUM_open_comp_graph$MergeImageMode
enum class MergeImageMode : ::std::uint8_t {
  kAdd = 0,
  kOver = 1,
  kMultiply = 2,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$MergeImageMode

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$ImageOrientation
#define CXXBRIDGE1_ENUM_open_comp_graph$ImageOrientation
enum class ImageOrientation : ::std::uint8_t {
  kNormal = 0,
  kFlippedHorizontally = 1,
  kRotated180 = 2,
  kFlippedVertically = 3,
  kTransposed = 4,
  kRotated90Clockwise = 5,
  kTransverse = 6,
  kRotated90CounterClockwise = 7,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$ImageOrientation

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$DiskCacheImageType
#define CXXBRIDGE1_ENUM_open_comp_graph$DiskCacheImageType
enum class DiskCacheImageType : ::std::uint8_t {
  kJpegLossyUInt8 = 0,
  kJpeg2000LossyUInt8 = 1,
  kJpeg2000LosslessUInt8 = 2,
  kExrLossyHalf16 = 3,
  kExrLosslessHalf16 = 4,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$DiskCacheImageType

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$CropOnWrite
#define CXXBRIDGE1_ENUM_open_comp_graph$CropOnWrite
enum class CropOnWrite : ::std::uint8_t {
  kDisable = 0,
  kEnable = 1,
  kAuto = 2,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$CropOnWrite

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$JpegChromaSubSampling
#define CXXBRIDGE1_ENUM_open_comp_graph$JpegChromaSubSampling
enum class JpegChromaSubSampling : ::std::uint8_t {
  kDefault = 0,
  kNone444 = 1,
  kSample422 = 2,
  kSample420 = 3,
  kSample421 = 4,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$JpegChromaSubSampling

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$ExrCompression
#define CXXBRIDGE1_ENUM_open_comp_graph$ExrCompression
enum class ExrCompression : ::std::uint8_t {
  kDefault = 0,
  kNone = 1,
  kRle = 2,
  kZip = 3,
  kZipScanline = 4,
  kPiz = 5,
  kPxr24 = 6,
  kB44 = 7,
  kB44a = 8,
  kDwaa = 9,
  kDwab = 10,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$ExrCompression

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$PixelBlock
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$PixelBlock
struct PixelBlock final : public ::rust::Opaque {
  OCG_API_EXPORT ::std::int32_t width() const noexcept;
  OCG_API_EXPORT ::std::int32_t height() const noexcept;
  OCG_API_EXPORT ::std::int32_t num_channels() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::DataType data_type() const noexcept;
  OCG_API_EXPORT ::rust::Slice<const float> as_slice_f32() const noexcept;

  // CXX does not support f16 primative data type like we need,
  // so we pretend it's u16.
  OCG_API_EXPORT ::rust::Slice<const ::std::uint16_t> as_slice_f16_fake() const noexcept;

  OCG_API_EXPORT ::rust::Slice<const ::std::uint16_t> as_slice_u16() const noexcept;
  OCG_API_EXPORT ::rust::Slice<const ::std::uint8_t> as_slice_u8() const noexcept;
  OCG_API_EXPORT ::rust::Slice<float> as_mut_slice_f32() noexcept;

  // CXX does not support f16 primative data type like we need,
  // so we pretend it's u16.
  OCG_API_EXPORT ::rust::Slice<::std::uint16_t> as_mut_slice_f16_fake() noexcept;

  OCG_API_EXPORT ::rust::Slice<::std::uint16_t> as_mut_slice_u16() noexcept;
  OCG_API_EXPORT ::rust::Slice<::std::uint8_t> as_mut_slice_u8() noexcept;
  OCG_API_EXPORT void data_resize(::open_comp_graph::BlockSize blocksize, ::open_comp_graph::DataType data_type) noexcept;
  ~PixelBlock() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$PixelBlock

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplRc
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplRc
struct StreamDataImplRc final : public ::rust::Opaque {
  OCG_API_EXPORT ::open_comp_graph::StreamDataState state() const noexcept;
  OCG_API_EXPORT ::std::uint8_t state_id() const noexcept;
  OCG_API_EXPORT ::std::uint64_t hash() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::BBox2Di display_window() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::BBox2Di data_window() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::Matrix4 color_matrix() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::internal::ImageSpec clone_image_spec() const noexcept;
  OCG_API_EXPORT ::std::size_t deformers_len() const noexcept;
  OCG_API_EXPORT void apply_deformers(::rust::Slice<float> buffer, ::open_comp_graph::BBox2Df display_window, ::open_comp_graph::BBox2Df data_window) const noexcept;
  OCG_API_EXPORT ::rust::Slice<const ::std::uint8_t> pixel_buffer() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_width() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_height() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_num_channels() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::DataType pixel_data_type() const noexcept;
  ~StreamDataImplRc() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplRc

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl
struct StreamDataImpl final : public ::rust::Opaque {
  ~StreamDataImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl
struct NodeImpl final : public ::rust::Opaque {
  ~NodeImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl
struct CacheImpl final : public ::rust::Opaque {
  OCG_API_EXPORT ::std::size_t len() const noexcept;
  OCG_API_EXPORT ::std::size_t used_bytes() const noexcept;
  OCG_API_EXPORT ::std::size_t capacity_bytes() const noexcept;
  OCG_API_EXPORT void set_capacity_bytes(::std::size_t value) noexcept;
  OCG_API_EXPORT ::rust::String data_debug_string() const noexcept;
  ~CacheImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl
struct GraphImpl final : public ::rust::Opaque {
  OCG_API_EXPORT ::open_comp_graph::GraphState state() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::ExecuteStatus execute_status() const noexcept;
  OCG_API_EXPORT ::std::size_t add_node(::rust::Box<::open_comp_graph::internal::NodeImpl> op_box) noexcept;
  OCG_API_EXPORT bool remove_node(::std::uint64_t node_id) noexcept;
  OCG_API_EXPORT ::open_comp_graph::AttrState node_attr_exists(::std::uint64_t node_id, ::rust::Str name) const noexcept;
  OCG_API_EXPORT ::open_comp_graph::NodeStatus node_status(::std::uint64_t node_id) const noexcept;
  OCG_API_EXPORT float get_node_attr_f32(::std::uint64_t node_id, ::rust::Str name) const noexcept;
  OCG_API_EXPORT ::std::int32_t get_node_attr_i32(::std::uint64_t node_id, ::rust::Str name) const noexcept;
  OCG_API_EXPORT ::rust::Str get_node_attr_str(::std::uint64_t node_id, ::rust::Str name) const noexcept;
  OCG_API_EXPORT void set_node_attr_f32(::std::uint64_t node_id, ::rust::Str name, float value) noexcept;
  OCG_API_EXPORT void set_node_attr_i32(::std::uint64_t node_id, ::rust::Str name, ::std::int32_t value) noexcept;
  OCG_API_EXPORT void set_node_attr_str(::std::uint64_t node_id, ::rust::Str name, ::rust::Str value) noexcept;
  OCG_API_EXPORT bool node_exists(::std::uint64_t node_id) noexcept;
  OCG_API_EXPORT void disconnect_input(::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept;
  OCG_API_EXPORT void connect(::std::uint64_t src_node_id, ::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept;
  OCG_API_EXPORT ::open_comp_graph::ExecuteStatus execute(::std::uint64_t node_id, ::rust::Slice<const double> frames, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;
  OCG_API_EXPORT ::rust::String data_debug_string() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared output_stream() const noexcept;
  ~GraphImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$GeometryPlaneImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$GeometryPlaneImpl
struct GeometryPlaneImpl final : public ::rust::Opaque {
  OCG_API_EXPORT ::std::uint32_t divisions_x() const noexcept;
  OCG_API_EXPORT ::std::uint32_t divisions_y() const noexcept;
  OCG_API_EXPORT void set_divisions_x(::std::uint32_t value) noexcept;
  OCG_API_EXPORT void set_divisions_y(::std::uint32_t value) noexcept;
  OCG_API_EXPORT ::std::size_t calc_count_vertex_positions() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_count_vertex_uvs() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_buffer_size_vertex_positions() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_buffer_size_vertex_uvs() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_buffer_size_index_tris() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_buffer_size_index_border_lines() const noexcept;
  OCG_API_EXPORT ::std::size_t calc_buffer_size_index_wire_lines() const noexcept;
  OCG_API_EXPORT bool fill_buffer_vertex_positions(::rust::Slice<float> buffer_vertex_positions) const noexcept;
  OCG_API_EXPORT bool fill_buffer_vertex_uvs(::rust::Slice<float> buffer_vertex_uvs) const noexcept;
  OCG_API_EXPORT bool fill_buffer_index_tris(::rust::Slice<::std::uint32_t> buffer_index_tris) const noexcept;
  OCG_API_EXPORT bool fill_buffer_index_border_lines(::rust::Slice<::std::uint32_t> buffer_index_border_lines) const noexcept;
  OCG_API_EXPORT bool fill_buffer_index_wire_lines(::rust::Slice<::std::uint32_t> buffer_index_wire_lines) const noexcept;
  ~GeometryPlaneImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$GeometryPlaneImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImpl
struct ConfigImpl final : public ::rust::Opaque {
  OCG_API_EXPORT ::std::size_t cache_ram_capacity_bytes() const noexcept;
  OCG_API_EXPORT float cache_ram_capacity_percent() const noexcept;
  OCG_API_EXPORT ::rust::String data_debug_string() const noexcept;
  ~ConfigImpl() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$ConfigImpl
} // namespace internal

extern "C" {
bool open_comp_graph$cxxbridge1$BBox2Df$operator$eq(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Df$operator$ne(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Df$operator$lt(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Df$operator$le(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Df$operator$gt(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Df$operator$ge(const BBox2Df &, const BBox2Df &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$eq(const BBox2Di &, const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$ne(const BBox2Di &, const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$lt(const BBox2Di &, const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$le(const BBox2Di &, const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$gt(const BBox2Di &, const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$BBox2Di$operator$ge(const BBox2Di &, const BBox2Di &) noexcept;
::std::size_t open_comp_graph$cxxbridge1$BBox2Di$operator$hash(const BBox2Di &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$eq(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$ne(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$lt(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$le(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$gt(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4f32$operator$ge(const Vector4f32 &, const Vector4f32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$eq(const Vector4i32 &, const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$ne(const Vector4i32 &, const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$lt(const Vector4i32 &, const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$le(const Vector4i32 &, const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$gt(const Vector4i32 &, const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Vector4i32$operator$ge(const Vector4i32 &, const Vector4i32 &) noexcept;
::std::size_t open_comp_graph$cxxbridge1$Vector4i32$operator$hash(const Vector4i32 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$eq(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$ne(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$lt(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$le(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$gt(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$Matrix4$operator$ge(const Matrix4 &, const Matrix4 &) noexcept;
bool open_comp_graph$cxxbridge1$BlockSize$operator$eq(const BlockSize &, const BlockSize &) noexcept;
bool open_comp_graph$cxxbridge1$BlockSize$operator$lt(const BlockSize &, const BlockSize &) noexcept;
bool open_comp_graph$cxxbridge1$BlockSize$operator$le(const BlockSize &, const BlockSize &) noexcept;
::std::size_t open_comp_graph$cxxbridge1$BlockSize$operator$hash(const BlockSize &) noexcept;
} // extern "C"

namespace internal {
extern "C" {
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImplShared$operator$hash(const StreamDataImplShared &) noexcept;

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$ocio_print_color_spaces() noexcept {
  bool (*ocio_print_color_spaces$)() = ::open_comp_graph::internal::ocio_print_color_spaces;
  return ocio_print_color_spaces$();
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_color_convert_inplace(::rust::Slice<float> pixel_data, ::std::int32_t width, ::std::int32_t height, ::std::int32_t num_channels, ::std::int32_t alpha_channel, bool unassociated_alpha, ::rust::Str src_color_space, ::rust::Str dst_color_space) noexcept {
  bool (*oiio_color_convert_inplace$)(::rust::Slice<float>, ::std::int32_t, ::std::int32_t, ::std::int32_t, ::std::int32_t, bool, ::rust::Str, ::rust::Str) = ::open_comp_graph::internal::oiio_color_convert_inplace;
  return oiio_color_convert_inplace$(pixel_data, width, height, num_channels, alpha_channel, unassociated_alpha, src_color_space, dst_color_space);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_image_resample(::open_comp_graph::internal::ImageShared &src_image, ::open_comp_graph::internal::ImageShared &dst_image, ::std::int32_t factor_num, bool interpolate) noexcept {
  bool (*oiio_image_resample$)(::open_comp_graph::internal::ImageShared &, ::open_comp_graph::internal::ImageShared &, ::std::int32_t, bool) = ::open_comp_graph::internal::oiio_image_resample;
  return oiio_image_resample$(src_image, dst_image, factor_num, interpolate);
}

OCG_API_EXPORT ::rust::repr::Fat open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_version_string(const ::open_comp_graph::internal::OcgLdPluginBase &self) noexcept {
  ::rust::Str (::open_comp_graph::internal::OcgLdPluginBase::*get_version_string$)() const = &::open_comp_graph::internal::OcgLdPluginBase::get_version_string;
  return ::rust::impl<::rust::Str>::repr((self.*get_version_string$)());
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_model_name(::open_comp_graph::internal::OcgLdPluginBase &self, ::rust::Slice<::std::uint8_t> model_name) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_model_name$)(::rust::Slice<::std::uint8_t>) = &::open_comp_graph::internal::OcgLdPluginBase::get_model_name;
  return (self.*get_model_name$)(model_name);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_num_parameters(::open_comp_graph::internal::OcgLdPluginBase &self, ::std::int32_t &value) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_num_parameters$)(::std::int32_t &) = &::open_comp_graph::internal::OcgLdPluginBase::get_num_parameters;
  return (self.*get_num_parameters$)(value);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_parameter_name(::open_comp_graph::internal::OcgLdPluginBase &self, ::std::int32_t value, ::rust::Slice<::std::uint8_t> identifier) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_parameter_name$)(::std::int32_t, ::rust::Slice<::std::uint8_t>) = &::open_comp_graph::internal::OcgLdPluginBase::get_parameter_name;
  return (self.*get_parameter_name$)(value, identifier);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_parameter_type(::open_comp_graph::internal::OcgLdPluginBase &self, ::rust::Str identifier, ::std::int32_t &value) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_parameter_type$)(::rust::Str, ::std::int32_t &) = &::open_comp_graph::internal::OcgLdPluginBase::get_parameter_type;
  return (self.*get_parameter_type$)(identifier, value);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_parameter_default_value_f64(::open_comp_graph::internal::OcgLdPluginBase &self, ::rust::Str identifier, double &value) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_parameter_default_value_f64$)(::rust::Str, double &) = &::open_comp_graph::internal::OcgLdPluginBase::get_parameter_default_value_f64;
  return (self.*get_parameter_default_value_f64$)(identifier, value);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_parameter_range(::open_comp_graph::internal::OcgLdPluginBase &self, ::rust::Str identifier, double &min_value, double &max_value) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*get_parameter_range$)(::rust::Str, double &, double &) = &::open_comp_graph::internal::OcgLdPluginBase::get_parameter_range;
  return (self.*get_parameter_range$)(identifier, min_value, max_value);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$set_parameter_value_f64(::open_comp_graph::internal::OcgLdPluginBase &self, ::rust::Str identifier, double value) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*set_parameter_value_f64$)(::rust::Str, double) = &::open_comp_graph::internal::OcgLdPluginBase::set_parameter_value_f64;
  return (self.*set_parameter_value_f64$)(identifier, value);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$initialize_parameters(::open_comp_graph::internal::OcgLdPluginBase &self) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*initialize_parameters$)() = &::open_comp_graph::internal::OcgLdPluginBase::initialize_parameters;
  return (self.*initialize_parameters$)();
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$undistort(::open_comp_graph::internal::OcgLdPluginBase &self, double x0, double y0, double &x1, double &y1) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*undistort$)(double, double, double &, double &) = &::open_comp_graph::internal::OcgLdPluginBase::undistort;
  return (self.*undistort$)(x0, y0, x1, y1);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$distort(::open_comp_graph::internal::OcgLdPluginBase &self, double x0, double y0, double &x1, double &y1) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*distort$)(double, double, double &, double &) = &::open_comp_graph::internal::OcgLdPluginBase::distort;
  return (self.*distort$)(x0, y0, x1, y1);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$distort_with_guess(::open_comp_graph::internal::OcgLdPluginBase &self, double x0, double y0, double x1_start, double y1_start, double &x1, double &y1) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*distort_with_guess$)(double, double, double, double, double &, double &) = &::open_comp_graph::internal::OcgLdPluginBase::distort_with_guess;
  return (self.*distort_with_guess$)(x0, y0, x1_start, y1_start, x1, y1);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$provides_parameter_derivatives(::open_comp_graph::internal::OcgLdPluginBase &self) noexcept {
  bool (::open_comp_graph::internal::OcgLdPluginBase::*provides_parameter_derivatives$)() = &::open_comp_graph::internal::OcgLdPluginBase::provides_parameter_derivatives;
  return (self.*provides_parameter_derivatives$)();
}

OCG_API_EXPORT void open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_bounding_box_undistort(::open_comp_graph::internal::OcgLdPluginBase &self, double xa_in, double ya_in, double xb_in, double yb_in, double &xa_out, double &ya_out, double &xb_out, double &yb_out, ::std::int32_t nx, ::std::int32_t ny) noexcept {
  void (::open_comp_graph::internal::OcgLdPluginBase::*get_bounding_box_undistort$)(double, double, double, double, double &, double &, double &, double &, ::std::int32_t, ::std::int32_t) = &::open_comp_graph::internal::OcgLdPluginBase::get_bounding_box_undistort;
  (self.*get_bounding_box_undistort$)(xa_in, ya_in, xb_in, yb_in, xa_out, ya_out, xb_out, yb_out, nx, ny);
}

OCG_API_EXPORT void open_comp_graph$internal$cxxbridge1$OcgLdPluginBase$get_bounding_box_distort(::open_comp_graph::internal::OcgLdPluginBase &self, double xa_in, double ya_in, double xb_in, double yb_in, double &xa_out, double &ya_out, double &xb_out, double &yb_out, ::std::int32_t nx, ::std::int32_t ny) noexcept {
  void (::open_comp_graph::internal::OcgLdPluginBase::*get_bounding_box_distort$)(double, double, double, double, double &, double &, double &, double &, ::std::int32_t, ::std::int32_t) = &::open_comp_graph::internal::OcgLdPluginBase::get_bounding_box_distort;
  (self.*get_bounding_box_distort$)(xa_in, ya_in, xb_in, yb_in, xa_out, ya_out, xb_out, yb_out, nx, ny);
}

OCG_API_EXPORT ::open_comp_graph::internal::OcgLdPluginBase *open_comp_graph$internal$cxxbridge1$ldpk_new_plugin() noexcept {
  ::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase> (*ldpk_new_plugin$)() = ::open_comp_graph::internal::ldpk_new_plugin;
  return ldpk_new_plugin$().release();
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_get_thread_count(::std::int32_t &num_threads) noexcept {
  bool (*oiio_get_thread_count$)(::std::int32_t &) = ::open_comp_graph::internal::oiio_get_thread_count;
  return oiio_get_thread_count$(num_threads);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_set_thread_count(::std::int32_t num_threads) noexcept {
  bool (*oiio_set_thread_count$)(::std::int32_t) = ::open_comp_graph::internal::oiio_set_thread_count;
  return oiio_set_thread_count$(num_threads);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_read_image(const ::rust::String &file_path, ::open_comp_graph::internal::ImageShared &image) noexcept {
  bool (*oiio_read_image$)(const ::rust::String &, ::open_comp_graph::internal::ImageShared &) = ::open_comp_graph::internal::oiio_read_image;
  return oiio_read_image$(file_path, image);
}

OCG_API_EXPORT bool open_comp_graph$internal$cxxbridge1$oiio_write_image(const ::rust::String &file_path, const ::open_comp_graph::internal::ImageShared &image, const ::open_comp_graph::internal::ImageCompression &compress) noexcept {
  bool (*oiio_write_image$)(const ::rust::String &, const ::open_comp_graph::internal::ImageShared &, const ::open_comp_graph::internal::ImageCompression &) = ::open_comp_graph::internal::oiio_write_image;
  return oiio_write_image$(file_path, image, compress);
}

OCG_API_EXPORT ::std::size_t open_comp_graph$internal$cxxbridge1$get_total_system_memory_as_bytes() noexcept {
  ::std::size_t (*get_total_system_memory_as_bytes$)() = ::open_comp_graph::internal::get_total_system_memory_as_bytes;
  return get_total_system_memory_as_bytes$();
}
::std::size_t open_comp_graph$internal$cxxbridge1$PixelBlock$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$PixelBlock$operator$alignof() noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$PixelBlock$width(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$PixelBlock$height(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$PixelBlock$num_channels(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::open_comp_graph::DataType open_comp_graph$internal$cxxbridge1$PixelBlock$data_type(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_f32(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_f16_fake(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_u16(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_u8(const ::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_f32(::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_f16_fake(::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_u16(::open_comp_graph::internal::PixelBlock &self) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_u8(::open_comp_graph::internal::PixelBlock &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$stride_num_channels(::std::int32_t num_channels, ::open_comp_graph::DataType data_type) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$channel_size_bytes(::open_comp_graph::DataType data_type) noexcept;

void open_comp_graph$internal$cxxbridge1$PixelBlock$data_resize(::open_comp_graph::internal::PixelBlock &self, ::open_comp_graph::BlockSize blocksize, ::open_comp_graph::DataType data_type) noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$operator$alignof() noexcept;

::open_comp_graph::StreamDataState open_comp_graph$internal$cxxbridge1$StreamDataImplRc$state(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::std::uint8_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$state_id(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$hash(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::open_comp_graph::BBox2Di open_comp_graph$internal$cxxbridge1$StreamDataImplRc$display_window(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::open_comp_graph::BBox2Di open_comp_graph$internal$cxxbridge1$StreamDataImplRc$data_window(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::open_comp_graph::Matrix4 open_comp_graph$internal$cxxbridge1$StreamDataImplRc$color_matrix(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

void open_comp_graph$internal$cxxbridge1$StreamDataImplRc$clone_image_spec(const ::open_comp_graph::internal::StreamDataImplRc &self, ::open_comp_graph::internal::ImageSpec *return$) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$deformers_len(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

void open_comp_graph$internal$cxxbridge1$StreamDataImplRc$apply_deformers(const ::open_comp_graph::internal::StreamDataImplRc &self, ::rust::Slice<float> buffer, ::open_comp_graph::BBox2Df display_window, ::open_comp_graph::BBox2Df data_window) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_buffer(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_width(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_height(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_num_channels(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::open_comp_graph::DataType open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_data_type(const ::open_comp_graph::internal::StreamDataImplRc &self) noexcept;

::open_comp_graph::internal::StreamDataImplRc *open_comp_graph$internal$cxxbridge1$create_stream_data_box_rc() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImpl$operator$alignof() noexcept;

::open_comp_graph::internal::StreamDataImpl *open_comp_graph$internal$cxxbridge1$create_stream_data_box() noexcept;

void open_comp_graph$internal$cxxbridge1$create_stream_data_shared(::open_comp_graph::internal::StreamDataImplShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$create_stream_data_shared_box(::open_comp_graph::internal::StreamDataImplRc *data, ::open_comp_graph::internal::StreamDataImplShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$create_vec_stream_data_shared(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *return$) noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$NodeImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$NodeImpl$operator$alignof() noexcept;

::open_comp_graph::internal::NodeImpl *open_comp_graph$internal$cxxbridge1$create_node_box_with_id(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$operator$alignof() noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$len(const ::open_comp_graph::internal::CacheImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$used_bytes(const ::open_comp_graph::internal::CacheImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$capacity_bytes(const ::open_comp_graph::internal::CacheImpl &self) noexcept;

void open_comp_graph$internal$cxxbridge1$CacheImpl$set_capacity_bytes(::open_comp_graph::internal::CacheImpl &self, ::std::size_t value) noexcept;

void open_comp_graph$internal$cxxbridge1$CacheImpl$data_debug_string(const ::open_comp_graph::internal::CacheImpl &self, ::rust::String *return$) noexcept;

::open_comp_graph::internal::CacheImpl *open_comp_graph$internal$cxxbridge1$create_cache_box_with_capacity(::std::size_t capacity_bytes) noexcept;

void open_comp_graph$internal$cxxbridge1$create_cache_shared_with_capacity(::std::size_t capacity_bytes, ::open_comp_graph::internal::CacheImplShared *return$) noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$GraphImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$GraphImpl$operator$alignof() noexcept;

::open_comp_graph::GraphState open_comp_graph$internal$cxxbridge1$GraphImpl$state(const ::open_comp_graph::internal::GraphImpl &self) noexcept;

::open_comp_graph::ExecuteStatus open_comp_graph$internal$cxxbridge1$GraphImpl$execute_status(const ::open_comp_graph::internal::GraphImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GraphImpl$add_node(::open_comp_graph::internal::GraphImpl &self, ::open_comp_graph::internal::NodeImpl *op_box) noexcept;

bool open_comp_graph$internal$cxxbridge1$GraphImpl$remove_node(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id) noexcept;

::open_comp_graph::AttrState open_comp_graph$internal$cxxbridge1$GraphImpl$node_attr_exists(const ::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name) noexcept;

::open_comp_graph::NodeStatus open_comp_graph$internal$cxxbridge1$GraphImpl$node_status(const ::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id) noexcept;

float open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_f32(const ::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_i32(const ::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name) noexcept;

::rust::repr::Fat open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_str(const ::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_f32(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name, float value) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_i32(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name, ::std::int32_t value) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_str(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Str name, ::rust::Str value) noexcept;

bool open_comp_graph$internal$cxxbridge1$GraphImpl$node_exists(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$disconnect_input(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$connect(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t src_node_id, ::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept;

::open_comp_graph::ExecuteStatus open_comp_graph$internal$cxxbridge1$GraphImpl$execute(::open_comp_graph::internal::GraphImpl &self, ::std::uint64_t node_id, ::rust::Slice<const double> frames, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$data_debug_string(const ::open_comp_graph::internal::GraphImpl &self, ::rust::String *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$output_stream(const ::open_comp_graph::internal::GraphImpl &self, ::open_comp_graph::internal::StreamDataImplShared *return$) noexcept;

::open_comp_graph::internal::GraphImpl *open_comp_graph$internal$cxxbridge1$create_graph_box() noexcept;

void open_comp_graph$internal$cxxbridge1$create_graph_shared(::open_comp_graph::internal::GraphImplShared *return$) noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$operator$alignof() noexcept;

::std::uint32_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$divisions_x(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::uint32_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$divisions_y(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

void open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$set_divisions_x(::open_comp_graph::internal::GeometryPlaneImpl &self, ::std::uint32_t value) noexcept;

void open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$set_divisions_y(::open_comp_graph::internal::GeometryPlaneImpl &self, ::std::uint32_t value) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_count_vertex_positions(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_count_vertex_uvs(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_vertex_positions(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_vertex_uvs(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_tris(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_border_lines(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_wire_lines(const ::open_comp_graph::internal::GeometryPlaneImpl &self) noexcept;

bool open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_vertex_positions(const ::open_comp_graph::internal::GeometryPlaneImpl &self, ::rust::Slice<float> buffer_vertex_positions) noexcept;

bool open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_vertex_uvs(const ::open_comp_graph::internal::GeometryPlaneImpl &self, ::rust::Slice<float> buffer_vertex_uvs) noexcept;

bool open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_tris(const ::open_comp_graph::internal::GeometryPlaneImpl &self, ::rust::Slice<::std::uint32_t> buffer_index_tris) noexcept;

bool open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_border_lines(const ::open_comp_graph::internal::GeometryPlaneImpl &self, ::rust::Slice<::std::uint32_t> buffer_index_border_lines) noexcept;

bool open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_wire_lines(const ::open_comp_graph::internal::GeometryPlaneImpl &self, ::rust::Slice<::std::uint32_t> buffer_index_wire_lines) noexcept;

::open_comp_graph::internal::GeometryPlaneImpl *open_comp_graph$internal$cxxbridge1$create_geometry_plane_box(float center_x, float center_y, float size_x, float size_y, ::std::uint32_t divisions_x, ::std::uint32_t divisions_y) noexcept;

void open_comp_graph$internal$cxxbridge1$get_color_transform_3dlut(::rust::Str from_color_space, ::rust::Str to_color_space, ::std::int32_t cube_size, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache, ::open_comp_graph::internal::ImageShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$export_mesh(::rust::Slice<const float> buffer_vertex_positions, ::rust::Slice<const float> buffer_vertex_uvs, ::rust::Slice<const ::std::uint32_t> buffer_index_tris, ::rust::Str file_path) noexcept;
} // extern "C"
} // namespace internal

namespace log {
extern "C" {
bool open_comp_graph$log$cxxbridge1$initialize() noexcept;
} // extern "C"
} // namespace log

namespace internal {
extern "C" {
::std::size_t open_comp_graph$internal$cxxbridge1$ConfigImpl$operator$sizeof() noexcept;
::std::size_t open_comp_graph$internal$cxxbridge1$ConfigImpl$operator$alignof() noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$ConfigImpl$cache_ram_capacity_bytes(const ::open_comp_graph::internal::ConfigImpl &self) noexcept;

float open_comp_graph$internal$cxxbridge1$ConfigImpl$cache_ram_capacity_percent(const ::open_comp_graph::internal::ConfigImpl &self) noexcept;

void open_comp_graph$internal$cxxbridge1$ConfigImpl$data_debug_string(const ::open_comp_graph::internal::ConfigImpl &self, ::rust::String *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$get_config(::rust::Str file_name, ::open_comp_graph::internal::ConfigImplShared *return$) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$generate_random_id() noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$generate_id_from_name(::rust::Str name) noexcept;
} // extern "C"
} // namespace internal
} // namespace open_comp_graph

namespace std {
template <> struct hash<::open_comp_graph::BBox2Di> {
  ::std::size_t operator()(const ::open_comp_graph::BBox2Di &self) const noexcept {
    return ::open_comp_graph::open_comp_graph$cxxbridge1$BBox2Di$operator$hash(self);
  }
};

template <> struct hash<::open_comp_graph::Vector4i32> {
  ::std::size_t operator()(const ::open_comp_graph::Vector4i32 &self) const noexcept {
    return ::open_comp_graph::open_comp_graph$cxxbridge1$Vector4i32$operator$hash(self);
  }
};

template <> struct hash<::open_comp_graph::BlockSize> {
  ::std::size_t operator()(const ::open_comp_graph::BlockSize &self) const noexcept {
    return ::open_comp_graph::open_comp_graph$cxxbridge1$BlockSize$operator$hash(self);
  }
};

template <> struct hash<::open_comp_graph::internal::StreamDataImplShared> {
  ::std::size_t operator()(const ::open_comp_graph::internal::StreamDataImplShared &self) const noexcept {
    return ::open_comp_graph::internal::open_comp_graph$internal$cxxbridge1$StreamDataImplShared$operator$hash(self);
  }
};
} // namespace std

namespace open_comp_graph {
bool BBox2Df::operator==(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$eq(*this, rhs);
}

bool BBox2Df::operator!=(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$ne(*this, rhs);
}

bool BBox2Df::operator<(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$lt(*this, rhs);
}

bool BBox2Df::operator<=(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$le(*this, rhs);
}

bool BBox2Df::operator>(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$gt(*this, rhs);
}

bool BBox2Df::operator>=(const BBox2Df &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Df$operator$ge(*this, rhs);
}

bool BBox2Di::operator==(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$eq(*this, rhs);
}

bool BBox2Di::operator!=(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$ne(*this, rhs);
}

bool BBox2Di::operator<(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$lt(*this, rhs);
}

bool BBox2Di::operator<=(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$le(*this, rhs);
}

bool BBox2Di::operator>(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$gt(*this, rhs);
}

bool BBox2Di::operator>=(const BBox2Di &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BBox2Di$operator$ge(*this, rhs);
}

bool Vector4f32::operator==(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$eq(*this, rhs);
}

bool Vector4f32::operator!=(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$ne(*this, rhs);
}

bool Vector4f32::operator<(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$lt(*this, rhs);
}

bool Vector4f32::operator<=(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$le(*this, rhs);
}

bool Vector4f32::operator>(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$gt(*this, rhs);
}

bool Vector4f32::operator>=(const Vector4f32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4f32$operator$ge(*this, rhs);
}

bool Vector4i32::operator==(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$eq(*this, rhs);
}

bool Vector4i32::operator!=(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$ne(*this, rhs);
}

bool Vector4i32::operator<(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$lt(*this, rhs);
}

bool Vector4i32::operator<=(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$le(*this, rhs);
}

bool Vector4i32::operator>(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$gt(*this, rhs);
}

bool Vector4i32::operator>=(const Vector4i32 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Vector4i32$operator$ge(*this, rhs);
}

bool Matrix4::operator==(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$eq(*this, rhs);
}

bool Matrix4::operator!=(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$ne(*this, rhs);
}

bool Matrix4::operator<(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$lt(*this, rhs);
}

bool Matrix4::operator<=(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$le(*this, rhs);
}

bool Matrix4::operator>(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$gt(*this, rhs);
}

bool Matrix4::operator>=(const Matrix4 &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$Matrix4$operator$ge(*this, rhs);
}

bool BlockSize::operator==(const BlockSize &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BlockSize$operator$eq(*this, rhs);
}

bool BlockSize::operator!=(const BlockSize &rhs) const noexcept {
  return !(*this == rhs);
}

bool BlockSize::operator<(const BlockSize &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BlockSize$operator$lt(*this, rhs);
}

bool BlockSize::operator<=(const BlockSize &rhs) const noexcept {
  return open_comp_graph$cxxbridge1$BlockSize$operator$le(*this, rhs);
}

bool BlockSize::operator>(const BlockSize &rhs) const noexcept {
  return !(*this <= rhs);
}

bool BlockSize::operator>=(const BlockSize &rhs) const noexcept {
  return !(*this < rhs);
}

namespace internal {
::std::size_t PixelBlock::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$operator$sizeof();
}

::std::size_t PixelBlock::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$operator$alignof();
}

OCG_API_EXPORT ::std::int32_t PixelBlock::width() const noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$width(*this);
}

OCG_API_EXPORT ::std::int32_t PixelBlock::height() const noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$height(*this);
}

OCG_API_EXPORT ::std::int32_t PixelBlock::num_channels() const noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$num_channels(*this);
}

OCG_API_EXPORT ::open_comp_graph::DataType PixelBlock::data_type() const noexcept {
  return open_comp_graph$internal$cxxbridge1$PixelBlock$data_type(*this);
}

OCG_API_EXPORT ::rust::Slice<const float> PixelBlock::as_slice_f32() const noexcept {
  return ::rust::impl<::rust::Slice<const float>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_f32(*this));
}

OCG_API_EXPORT ::rust::Slice<const ::std::uint16_t> PixelBlock::as_slice_f16_fake() const noexcept {
  return ::rust::impl<::rust::Slice<const ::std::uint16_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_f16_fake(*this));
}

OCG_API_EXPORT ::rust::Slice<const ::std::uint16_t> PixelBlock::as_slice_u16() const noexcept {
  return ::rust::impl<::rust::Slice<const ::std::uint16_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_u16(*this));
}

OCG_API_EXPORT ::rust::Slice<const ::std::uint8_t> PixelBlock::as_slice_u8() const noexcept {
  return ::rust::impl<::rust::Slice<const ::std::uint8_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_slice_u8(*this));
}

OCG_API_EXPORT ::rust::Slice<float> PixelBlock::as_mut_slice_f32() noexcept {
  return ::rust::impl<::rust::Slice<float>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_f32(*this));
}

OCG_API_EXPORT ::rust::Slice<::std::uint16_t> PixelBlock::as_mut_slice_f16_fake() noexcept {
  return ::rust::impl<::rust::Slice<::std::uint16_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_f16_fake(*this));
}

OCG_API_EXPORT ::rust::Slice<::std::uint16_t> PixelBlock::as_mut_slice_u16() noexcept {
  return ::rust::impl<::rust::Slice<::std::uint16_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_u16(*this));
}

OCG_API_EXPORT ::rust::Slice<::std::uint8_t> PixelBlock::as_mut_slice_u8() noexcept {
  return ::rust::impl<::rust::Slice<::std::uint8_t>>::slice(open_comp_graph$internal$cxxbridge1$PixelBlock$as_mut_slice_u8(*this));
}

OCG_API_EXPORT ::std::size_t stride_num_channels(::std::int32_t num_channels, ::open_comp_graph::DataType data_type) noexcept {
  return open_comp_graph$internal$cxxbridge1$stride_num_channels(num_channels, data_type);
}

OCG_API_EXPORT ::std::size_t channel_size_bytes(::open_comp_graph::DataType data_type) noexcept {
  return open_comp_graph$internal$cxxbridge1$channel_size_bytes(data_type);
}

OCG_API_EXPORT void PixelBlock::data_resize(::open_comp_graph::BlockSize blocksize, ::open_comp_graph::DataType data_type) noexcept {
  open_comp_graph$internal$cxxbridge1$PixelBlock$data_resize(*this, blocksize, data_type);
}

::std::size_t StreamDataImplRc::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$operator$sizeof();
}

::std::size_t StreamDataImplRc::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$operator$alignof();
}

OCG_API_EXPORT ::open_comp_graph::StreamDataState StreamDataImplRc::state() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$state(*this);
}

OCG_API_EXPORT ::std::uint8_t StreamDataImplRc::state_id() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$state_id(*this);
}

OCG_API_EXPORT ::std::uint64_t StreamDataImplRc::hash() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$hash(*this);
}

OCG_API_EXPORT ::open_comp_graph::BBox2Di StreamDataImplRc::display_window() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$display_window(*this);
}

OCG_API_EXPORT ::open_comp_graph::BBox2Di StreamDataImplRc::data_window() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$data_window(*this);
}

OCG_API_EXPORT ::open_comp_graph::Matrix4 StreamDataImplRc::color_matrix() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$color_matrix(*this);
}

OCG_API_EXPORT ::open_comp_graph::internal::ImageSpec StreamDataImplRc::clone_image_spec() const noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::ImageSpec> return$;
  open_comp_graph$internal$cxxbridge1$StreamDataImplRc$clone_image_spec(*this, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::std::size_t StreamDataImplRc::deformers_len() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$deformers_len(*this);
}

OCG_API_EXPORT void StreamDataImplRc::apply_deformers(::rust::Slice<float> buffer, ::open_comp_graph::BBox2Df display_window, ::open_comp_graph::BBox2Df data_window) const noexcept {
  open_comp_graph$internal$cxxbridge1$StreamDataImplRc$apply_deformers(*this, buffer, display_window, data_window);
}

OCG_API_EXPORT ::rust::Slice<const ::std::uint8_t> StreamDataImplRc::pixel_buffer() const noexcept {
  return ::rust::impl<::rust::Slice<const ::std::uint8_t>>::slice(open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_buffer(*this));
}

OCG_API_EXPORT ::std::int32_t StreamDataImplRc::pixel_width() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_width(*this);
}

OCG_API_EXPORT ::std::int32_t StreamDataImplRc::pixel_height() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_height(*this);
}

OCG_API_EXPORT ::std::int32_t StreamDataImplRc::pixel_num_channels() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_num_channels(*this);
}

OCG_API_EXPORT ::open_comp_graph::DataType StreamDataImplRc::pixel_data_type() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImplRc$pixel_data_type(*this);
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::StreamDataImplRc> create_stream_data_box_rc() noexcept {
  return ::rust::Box<::open_comp_graph::internal::StreamDataImplRc>::from_raw(open_comp_graph$internal$cxxbridge1$create_stream_data_box_rc());
}

::std::size_t StreamDataImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImpl$operator$sizeof();
}

::std::size_t StreamDataImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImpl$operator$alignof();
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::StreamDataImpl> create_stream_data_box() noexcept {
  return ::rust::Box<::open_comp_graph::internal::StreamDataImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_stream_data_box());
}

OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared create_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::StreamDataImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared create_stream_data_shared_box(::rust::Box<::open_comp_graph::internal::StreamDataImplRc> data) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::StreamDataImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_stream_data_shared_box(data.into_raw(), &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> create_vec_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::rust::Vec<::open_comp_graph::internal::StreamDataImplShared>> return$;
  open_comp_graph$internal$cxxbridge1$create_vec_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}

::std::size_t NodeImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$operator$sizeof();
}

::std::size_t NodeImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$operator$alignof();
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::NodeImpl> create_node_box(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept {
  return ::rust::Box<::open_comp_graph::internal::NodeImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_node_box_with_id(node_type, id));
}

::std::size_t CacheImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$operator$sizeof();
}

::std::size_t CacheImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$operator$alignof();
}

OCG_API_EXPORT ::std::size_t CacheImpl::len() const noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$len(*this);
}

OCG_API_EXPORT ::std::size_t CacheImpl::used_bytes() const noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$used_bytes(*this);
}

OCG_API_EXPORT ::std::size_t CacheImpl::capacity_bytes() const noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$capacity_bytes(*this);
}

OCG_API_EXPORT void CacheImpl::set_capacity_bytes(::std::size_t value) noexcept {
  open_comp_graph$internal$cxxbridge1$CacheImpl$set_capacity_bytes(*this, value);
}

OCG_API_EXPORT ::rust::String CacheImpl::data_debug_string() const noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  open_comp_graph$internal$cxxbridge1$CacheImpl$data_debug_string(*this, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::CacheImpl> create_cache_box_with_capacity(::std::size_t capacity_bytes) noexcept {
  return ::rust::Box<::open_comp_graph::internal::CacheImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_cache_box_with_capacity(capacity_bytes));
}

OCG_API_EXPORT ::open_comp_graph::internal::CacheImplShared create_cache_shared_with_capacity(::std::size_t capacity_bytes) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::CacheImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_cache_shared_with_capacity(capacity_bytes, &return$.value);
  return ::std::move(return$.value);
}

::std::size_t GraphImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$operator$sizeof();
}

::std::size_t GraphImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$operator$alignof();
}

OCG_API_EXPORT ::open_comp_graph::GraphState GraphImpl::state() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$state(*this);
}

OCG_API_EXPORT ::open_comp_graph::ExecuteStatus GraphImpl::execute_status() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$execute_status(*this);
}

OCG_API_EXPORT ::std::size_t GraphImpl::add_node(::rust::Box<::open_comp_graph::internal::NodeImpl> op_box) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$add_node(*this, op_box.into_raw());
}

OCG_API_EXPORT bool GraphImpl::remove_node(::std::uint64_t node_id) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$remove_node(*this, node_id);
}

OCG_API_EXPORT ::open_comp_graph::AttrState GraphImpl::node_attr_exists(::std::uint64_t node_id, ::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$node_attr_exists(*this, node_id, name);
}

OCG_API_EXPORT ::open_comp_graph::NodeStatus GraphImpl::node_status(::std::uint64_t node_id) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$node_status(*this, node_id);
}

OCG_API_EXPORT float GraphImpl::get_node_attr_f32(::std::uint64_t node_id, ::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_f32(*this, node_id, name);
}

OCG_API_EXPORT ::std::int32_t GraphImpl::get_node_attr_i32(::std::uint64_t node_id, ::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_i32(*this, node_id, name);
}

OCG_API_EXPORT ::rust::Str GraphImpl::get_node_attr_str(::std::uint64_t node_id, ::rust::Str name) const noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(open_comp_graph$internal$cxxbridge1$GraphImpl$get_node_attr_str(*this, node_id, name));
}

OCG_API_EXPORT void GraphImpl::set_node_attr_f32(::std::uint64_t node_id, ::rust::Str name, float value) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_f32(*this, node_id, name, value);
}

OCG_API_EXPORT void GraphImpl::set_node_attr_i32(::std::uint64_t node_id, ::rust::Str name, ::std::int32_t value) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_i32(*this, node_id, name, value);
}

OCG_API_EXPORT void GraphImpl::set_node_attr_str(::std::uint64_t node_id, ::rust::Str name, ::rust::Str value) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$set_node_attr_str(*this, node_id, name, value);
}

OCG_API_EXPORT bool GraphImpl::node_exists(::std::uint64_t node_id) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$node_exists(*this, node_id);
}

OCG_API_EXPORT void GraphImpl::disconnect_input(::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$disconnect_input(*this, dst_node_id, input_num);
}

OCG_API_EXPORT void GraphImpl::connect(::std::uint64_t src_node_id, ::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$connect(*this, src_node_id, dst_node_id, input_num);
}

OCG_API_EXPORT ::open_comp_graph::ExecuteStatus GraphImpl::execute(::std::uint64_t node_id, ::rust::Slice<const double> frames, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$execute(*this, node_id, frames, cache);
}

OCG_API_EXPORT ::rust::String GraphImpl::data_debug_string() const noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  open_comp_graph$internal$cxxbridge1$GraphImpl$data_debug_string(*this, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared GraphImpl::output_stream() const noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::StreamDataImplShared> return$;
  open_comp_graph$internal$cxxbridge1$GraphImpl$output_stream(*this, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::GraphImpl> create_graph_box() noexcept {
  return ::rust::Box<::open_comp_graph::internal::GraphImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_graph_box());
}

OCG_API_EXPORT ::open_comp_graph::internal::GraphImplShared create_graph_shared() noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::GraphImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_graph_shared(&return$.value);
  return ::std::move(return$.value);
}

::std::size_t GeometryPlaneImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$operator$sizeof();
}

::std::size_t GeometryPlaneImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$operator$alignof();
}

OCG_API_EXPORT ::std::uint32_t GeometryPlaneImpl::divisions_x() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$divisions_x(*this);
}

OCG_API_EXPORT ::std::uint32_t GeometryPlaneImpl::divisions_y() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$divisions_y(*this);
}

OCG_API_EXPORT void GeometryPlaneImpl::set_divisions_x(::std::uint32_t value) noexcept {
  open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$set_divisions_x(*this, value);
}

OCG_API_EXPORT void GeometryPlaneImpl::set_divisions_y(::std::uint32_t value) noexcept {
  open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$set_divisions_y(*this, value);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_count_vertex_positions() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_count_vertex_positions(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_count_vertex_uvs() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_count_vertex_uvs(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_buffer_size_vertex_positions() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_vertex_positions(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_buffer_size_vertex_uvs() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_vertex_uvs(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_buffer_size_index_tris() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_tris(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_buffer_size_index_border_lines() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_border_lines(*this);
}

OCG_API_EXPORT ::std::size_t GeometryPlaneImpl::calc_buffer_size_index_wire_lines() const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$calc_buffer_size_index_wire_lines(*this);
}

OCG_API_EXPORT bool GeometryPlaneImpl::fill_buffer_vertex_positions(::rust::Slice<float> buffer_vertex_positions) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_vertex_positions(*this, buffer_vertex_positions);
}

OCG_API_EXPORT bool GeometryPlaneImpl::fill_buffer_vertex_uvs(::rust::Slice<float> buffer_vertex_uvs) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_vertex_uvs(*this, buffer_vertex_uvs);
}

OCG_API_EXPORT bool GeometryPlaneImpl::fill_buffer_index_tris(::rust::Slice<::std::uint32_t> buffer_index_tris) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_tris(*this, buffer_index_tris);
}

OCG_API_EXPORT bool GeometryPlaneImpl::fill_buffer_index_border_lines(::rust::Slice<::std::uint32_t> buffer_index_border_lines) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_border_lines(*this, buffer_index_border_lines);
}

OCG_API_EXPORT bool GeometryPlaneImpl::fill_buffer_index_wire_lines(::rust::Slice<::std::uint32_t> buffer_index_wire_lines) const noexcept {
  return open_comp_graph$internal$cxxbridge1$GeometryPlaneImpl$fill_buffer_index_wire_lines(*this, buffer_index_wire_lines);
}

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::GeometryPlaneImpl> create_geometry_plane_box(float center_x, float center_y, float size_x, float size_y, ::std::uint32_t divisions_x, ::std::uint32_t divisions_y) noexcept {
  return ::rust::Box<::open_comp_graph::internal::GeometryPlaneImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_geometry_plane_box(center_x, center_y, size_x, size_y, divisions_x, divisions_y));
}

OCG_API_EXPORT ::open_comp_graph::internal::ImageShared get_color_transform_3dlut(::rust::Str from_color_space, ::rust::Str to_color_space, ::std::int32_t cube_size, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::ImageShared> return$;
  open_comp_graph$internal$cxxbridge1$get_color_transform_3dlut(from_color_space, to_color_space, cube_size, cache, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT void export_mesh(::rust::Slice<const float> buffer_vertex_positions, ::rust::Slice<const float> buffer_vertex_uvs, ::rust::Slice<const ::std::uint32_t> buffer_index_tris, ::rust::Str file_path) noexcept {
  open_comp_graph$internal$cxxbridge1$export_mesh(buffer_vertex_positions, buffer_vertex_uvs, buffer_index_tris, file_path);
}
} // namespace internal

namespace log {
OCG_API_EXPORT bool initialize() noexcept {
  return open_comp_graph$log$cxxbridge1$initialize();
}
} // namespace log

namespace internal {
::std::size_t ConfigImpl::layout::size() noexcept {
  return open_comp_graph$internal$cxxbridge1$ConfigImpl$operator$sizeof();
}

::std::size_t ConfigImpl::layout::align() noexcept {
  return open_comp_graph$internal$cxxbridge1$ConfigImpl$operator$alignof();
}

OCG_API_EXPORT ::std::size_t ConfigImpl::cache_ram_capacity_bytes() const noexcept {
  return open_comp_graph$internal$cxxbridge1$ConfigImpl$cache_ram_capacity_bytes(*this);
}

OCG_API_EXPORT float ConfigImpl::cache_ram_capacity_percent() const noexcept {
  return open_comp_graph$internal$cxxbridge1$ConfigImpl$cache_ram_capacity_percent(*this);
}

OCG_API_EXPORT ::rust::String ConfigImpl::data_debug_string() const noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  open_comp_graph$internal$cxxbridge1$ConfigImpl$data_debug_string(*this, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::open_comp_graph::internal::ConfigImplShared get_config(::rust::Str file_name) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::ConfigImplShared> return$;
  open_comp_graph$internal$cxxbridge1$get_config(file_name, &return$.value);
  return ::std::move(return$.value);
}

OCG_API_EXPORT ::std::uint64_t generate_random_id() noexcept {
  return open_comp_graph$internal$cxxbridge1$generate_random_id();
}

OCG_API_EXPORT ::std::uint64_t generate_id_from_name(::rust::Str name) noexcept {
  return open_comp_graph$internal$cxxbridge1$generate_id_from_name(name);
}
} // namespace internal
} // namespace open_comp_graph

extern "C" {
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$new(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$drop(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$len(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$capacity(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
const ::open_comp_graph::internal::StreamDataImplShared *cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$data(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$reserve_total(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$set_len(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr, ::std::size_t len) noexcept;

::open_comp_graph::internal::GraphImpl *cxxbridge1$box$open_comp_graph$internal$GraphImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$GraphImpl$dealloc(::open_comp_graph::internal::GraphImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$GraphImpl$drop(::rust::Box<::open_comp_graph::internal::GraphImpl> *ptr) noexcept;

::open_comp_graph::internal::StreamDataImplRc *cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$dealloc(::open_comp_graph::internal::StreamDataImplRc *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$drop(::rust::Box<::open_comp_graph::internal::StreamDataImplRc> *ptr) noexcept;

::open_comp_graph::internal::CacheImpl *cxxbridge1$box$open_comp_graph$internal$CacheImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$CacheImpl$dealloc(::open_comp_graph::internal::CacheImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$CacheImpl$drop(::rust::Box<::open_comp_graph::internal::CacheImpl> *ptr) noexcept;

::open_comp_graph::internal::ConfigImpl *cxxbridge1$box$open_comp_graph$internal$ConfigImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$ConfigImpl$dealloc(::open_comp_graph::internal::ConfigImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$ConfigImpl$drop(::rust::Box<::open_comp_graph::internal::ConfigImpl> *ptr) noexcept;

::open_comp_graph::internal::PixelBlock *cxxbridge1$box$open_comp_graph$internal$PixelBlock$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$PixelBlock$dealloc(::open_comp_graph::internal::PixelBlock *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$PixelBlock$drop(::rust::Box<::open_comp_graph::internal::PixelBlock> *ptr) noexcept;

static_assert(::rust::detail::is_complete<::open_comp_graph::internal::OcgLdPluginBase>::value, "definition of OcgLdPluginBase is required");
static_assert(sizeof(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>) == alignof(void *), "");
void cxxbridge1$unique_ptr$open_comp_graph$internal$OcgLdPluginBase$null(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>();
}
void cxxbridge1$unique_ptr$open_comp_graph$internal$OcgLdPluginBase$raw(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase> *ptr, ::open_comp_graph::internal::OcgLdPluginBase *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>(raw);
}
const ::open_comp_graph::internal::OcgLdPluginBase *cxxbridge1$unique_ptr$open_comp_graph$internal$OcgLdPluginBase$get(const ::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>& ptr) noexcept {
  return ptr.get();
}
::open_comp_graph::internal::OcgLdPluginBase *cxxbridge1$unique_ptr$open_comp_graph$internal$OcgLdPluginBase$release(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$open_comp_graph$internal$OcgLdPluginBase$drop(::std::unique_ptr<::open_comp_graph::internal::OcgLdPluginBase> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::open_comp_graph::internal::OcgLdPluginBase>::value>{}(ptr);
}

::open_comp_graph::internal::StreamDataImpl *cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$dealloc(::open_comp_graph::internal::StreamDataImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$drop(::rust::Box<::open_comp_graph::internal::StreamDataImpl> *ptr) noexcept;

::open_comp_graph::internal::NodeImpl *cxxbridge1$box$open_comp_graph$internal$NodeImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$NodeImpl$dealloc(::open_comp_graph::internal::NodeImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$NodeImpl$drop(::rust::Box<::open_comp_graph::internal::NodeImpl> *ptr) noexcept;

::open_comp_graph::internal::GeometryPlaneImpl *cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$dealloc(::open_comp_graph::internal::GeometryPlaneImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$drop(::rust::Box<::open_comp_graph::internal::GeometryPlaneImpl> *ptr) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
OCG_API_EXPORT Vec<::open_comp_graph::internal::StreamDataImplShared>::Vec() noexcept {
  cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$new(this);
}
template <>
OCG_API_EXPORT void Vec<::open_comp_graph::internal::StreamDataImplShared>::drop() noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$drop(this);
}
template <>
OCG_API_EXPORT ::std::size_t Vec<::open_comp_graph::internal::StreamDataImplShared>::size() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$len(this);
}
template <>
OCG_API_EXPORT ::std::size_t Vec<::open_comp_graph::internal::StreamDataImplShared>::capacity() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$capacity(this);
}
template <>
OCG_API_EXPORT const ::open_comp_graph::internal::StreamDataImplShared *Vec<::open_comp_graph::internal::StreamDataImplShared>::data() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$data(this);
}
template <>
OCG_API_EXPORT void Vec<::open_comp_graph::internal::StreamDataImplShared>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$reserve_total(this, new_cap);
}
template <>
OCG_API_EXPORT void Vec<::open_comp_graph::internal::StreamDataImplShared>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$set_len(this, len);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::GraphImpl *Box<::open_comp_graph::internal::GraphImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$GraphImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::GraphImpl>::allocation::dealloc(::open_comp_graph::internal::GraphImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$GraphImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::GraphImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$GraphImpl$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplRc *Box<::open_comp_graph::internal::StreamDataImplRc>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::StreamDataImplRc>::allocation::dealloc(::open_comp_graph::internal::StreamDataImplRc *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::StreamDataImplRc>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImplRc$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::CacheImpl *Box<::open_comp_graph::internal::CacheImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$CacheImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::CacheImpl>::allocation::dealloc(::open_comp_graph::internal::CacheImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$CacheImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::CacheImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$CacheImpl$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::ConfigImpl *Box<::open_comp_graph::internal::ConfigImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$ConfigImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::ConfigImpl>::allocation::dealloc(::open_comp_graph::internal::ConfigImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$ConfigImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::ConfigImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$ConfigImpl$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::PixelBlock *Box<::open_comp_graph::internal::PixelBlock>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$PixelBlock$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::PixelBlock>::allocation::dealloc(::open_comp_graph::internal::PixelBlock *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$PixelBlock$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::PixelBlock>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$PixelBlock$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImpl *Box<::open_comp_graph::internal::StreamDataImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::StreamDataImpl>::allocation::dealloc(::open_comp_graph::internal::StreamDataImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::StreamDataImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::NodeImpl *Box<::open_comp_graph::internal::NodeImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$NodeImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::NodeImpl>::allocation::dealloc(::open_comp_graph::internal::NodeImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$NodeImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::NodeImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$NodeImpl$drop(this);
}
template <>
OCG_API_EXPORT ::open_comp_graph::internal::GeometryPlaneImpl *Box<::open_comp_graph::internal::GeometryPlaneImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$alloc();
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::GeometryPlaneImpl>::allocation::dealloc(::open_comp_graph::internal::GeometryPlaneImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$dealloc(ptr);
}
template <>
OCG_API_EXPORT void Box<::open_comp_graph::internal::GeometryPlaneImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$GeometryPlaneImpl$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
