#pragma once
#include "rust/cxx.h"
#include "opencompgraph/symbol_export.h"
#include "opencompgraph/internal/colorspace.h"
#include "opencompgraph/internal/ldpk_utils.h"
#include "opencompgraph/internal/imageio.h"
#include "opencompgraph/internal/systemmemory.h"
#include <memory>

namespace open_comp_graph {
  struct BBox2Df;
  struct BBox2Di;
  struct Vector4f32;
  struct Vector4i32;
  struct Matrix4;
  enum class GraphState : ::std::uint8_t;
  enum class ExecuteStatus : ::std::uint8_t;
  enum class BakeOption : ::std::uint8_t;
  enum class NodeType : ::std::uint8_t;
  enum class PixelDataType : ::std::uint8_t;
  enum class NodeStatus : ::std::uint8_t;
  enum class AttrState : ::std::uint8_t;
  enum class StreamDataState : ::std::uint8_t;
  enum class LensDistortDirection : ::std::uint8_t;
  enum class MergeImageMode : ::std::uint8_t;
  enum class ImageOrientation : ::std::uint8_t;
  namespace internal {
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

namespace internal {
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
  kMergeImage = 6,
  kGrade = 3,
  kLensDistort = 4,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$NodeType

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$PixelDataType
#define CXXBRIDGE1_ENUM_open_comp_graph$PixelDataType
enum class PixelDataType : ::std::uint8_t {
  kFloat32 = 0,
  kHalf16 = 1,
  kUInt8 = 2,
  kUInt16 = 3,
  kUnknown = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$PixelDataType

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

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$PixelBlock
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$PixelBlock
struct PixelBlock final : public ::rust::Opaque {
  OCG_API_EXPORT ::std::int32_t width() const noexcept;
  OCG_API_EXPORT ::std::int32_t height() const noexcept;
  OCG_API_EXPORT ::std::int32_t num_channels() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::PixelDataType pixel_data_type() const noexcept;
  OCG_API_EXPORT ::rust::Slice<const float> as_slice() const noexcept;
  OCG_API_EXPORT ::rust::Slice<float> as_slice_mut() noexcept;
  OCG_API_EXPORT void data_resize(::std::int32_t width, ::std::int32_t height, ::std::int32_t num_channels, ::open_comp_graph::PixelDataType pixel_data_type) noexcept;
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
  OCG_API_EXPORT ::rust::Slice<const float> pixel_buffer() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_width() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_height() const noexcept;
  OCG_API_EXPORT ::std::int32_t pixel_num_channels() const noexcept;
  OCG_API_EXPORT ::open_comp_graph::PixelDataType pixel_data_type() const noexcept;
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
  OCG_API_EXPORT void connect(::std::uint64_t src_node_id, ::std::uint64_t dst_node_id, ::std::uint8_t input_num) noexcept;
  OCG_API_EXPORT ::open_comp_graph::ExecuteStatus execute(::std::uint64_t node_id, ::rust::Slice<const ::std::int32_t> frames, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;
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

OCG_API_EXPORT ::std::size_t stride_num_channels(::std::int32_t num_channels, ::open_comp_graph::PixelDataType pixel_data_type) noexcept;

OCG_API_EXPORT ::std::size_t channel_size_bytes(::open_comp_graph::PixelDataType pixel_data_type) noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::StreamDataImplRc> create_stream_data_box_rc() noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::StreamDataImpl> create_stream_data_box() noexcept;

OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared create_stream_data_shared() noexcept;

OCG_API_EXPORT ::open_comp_graph::internal::StreamDataImplShared create_stream_data_shared_box(::rust::Box<::open_comp_graph::internal::StreamDataImplRc> data) noexcept;

OCG_API_EXPORT ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> create_vec_stream_data_shared() noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::NodeImpl> create_node_box(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::CacheImpl> create_cache_box_with_capacity(::std::size_t capacity_bytes) noexcept;

OCG_API_EXPORT ::open_comp_graph::internal::CacheImplShared create_cache_shared_with_capacity(::std::size_t capacity_bytes) noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::GraphImpl> create_graph_box() noexcept;

OCG_API_EXPORT ::open_comp_graph::internal::GraphImplShared create_graph_shared() noexcept;

OCG_API_EXPORT ::rust::Box<::open_comp_graph::internal::GeometryPlaneImpl> create_geometry_plane_box(float center_x, float center_y, float size_x, float size_y, ::std::uint32_t divisions_x, ::std::uint32_t divisions_y) noexcept;

OCG_API_EXPORT ::open_comp_graph::internal::ImageShared get_color_transform_3dlut(::rust::Str from_color_space, ::rust::Str to_color_space, ::std::int32_t cube_size, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;

OCG_API_EXPORT void export_mesh(::rust::Slice<const float> buffer_vertex_positions, ::rust::Slice<const float> buffer_vertex_uvs, ::rust::Slice<const ::std::uint32_t> buffer_index_tris, ::rust::Str file_path) noexcept;
} // namespace internal

namespace log {
OCG_API_EXPORT bool initialize() noexcept;
} // namespace log

namespace internal {
OCG_API_EXPORT ::open_comp_graph::internal::ConfigImplShared get_config(::rust::Str file_name) noexcept;

OCG_API_EXPORT ::std::uint64_t generate_random_id() noexcept;

OCG_API_EXPORT ::std::uint64_t generate_id_from_name(::rust::Str name) noexcept;
} // namespace internal
} // namespace open_comp_graph
