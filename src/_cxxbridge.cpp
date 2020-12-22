#include "rust/cxx.h"
#include "opencompgraph/cpp.h"
#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_PANIC
#define CXXBRIDGE1_PANIC
template <typename Exception>
void panic [[noreturn]] (const char *msg);
#endif // CXXBRIDGE1_PANIC

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

private:
  friend impl<Str>;
  const char *ptr;
  std::size_t len;
};

inline const char *Str::data() const noexcept { return this->ptr; }

inline std::size_t Str::size() const noexcept { return this->len; }

inline std::size_t Str::length() const noexcept { return this->len; }
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_RUST_BOX
#define CXXBRIDGE1_RUST_BOX
template <typename T>
class Box final {
public:
  using value_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box() = delete;
  Box(const Box &);
  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(const Box &);
  Box &operator=(Box &&) noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;

private:
  class uninit;
  class allocation;
  Box(uninit) noexcept;
  void drop() noexcept;
  T *ptr;
};

template <typename T>
class Box<T>::uninit {};

template <typename T>
class Box<T>::allocation {
  static T *alloc() noexcept;
  static void dealloc(T *) noexcept;

public:
  allocation() noexcept : ptr(alloc()) {}
  ~allocation() noexcept {
    if (this->ptr) {
      dealloc(this->ptr);
    }
  }
  T *ptr;
};

template <typename T>
Box<T>::Box(const Box &other) : Box(*other) {}

template <typename T>
Box<T>::Box(Box &&other) noexcept : ptr(other.ptr) {
  other.ptr = nullptr;
}

template <typename T>
Box<T>::Box(const T &val) {
  allocation alloc;
  ::new (alloc.ptr) T(val);
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::Box(T &&val) {
  allocation alloc;
  ::new (alloc.ptr) T(std::move(val));
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::~Box() noexcept {
  if (this->ptr) {
    this->drop();
  }
}

template <typename T>
Box<T> &Box<T>::operator=(const Box &other) {
  if (this->ptr) {
    **this = *other;
  } else {
    allocation alloc;
    ::new (alloc.ptr) T(*other);
    this->ptr = alloc.ptr;
    alloc.ptr = nullptr;
  }
  return *this;
}

template <typename T>
Box<T> &Box<T>::operator=(Box &&other) noexcept {
  if (this->ptr) {
    this->drop();
  }
  this->ptr = other.ptr;
  other.ptr = nullptr;
  return *this;
}

template <typename T>
const T *Box<T>::operator->() const noexcept {
  return this->ptr;
}

template <typename T>
const T &Box<T>::operator*() const noexcept {
  return *this->ptr;
}

template <typename T>
T *Box<T>::operator->() noexcept {
  return this->ptr;
}

template <typename T>
T &Box<T>::operator*() noexcept {
  return *this->ptr;
}

template <typename T>
template <typename... Fields>
Box<T> Box<T>::in_place(Fields &&... fields) {
  allocation alloc;
  auto ptr = alloc.ptr;
  ::new (ptr) T{std::forward<Fields>(fields)...};
  alloc.ptr = nullptr;
  return from_raw(ptr);
}

template <typename T>
Box<T> Box<T>::from_raw(T *raw) noexcept {
  Box box = uninit{};
  box.ptr = raw;
  return box;
}

template <typename T>
T *Box<T>::into_raw() noexcept {
  T *raw = this->ptr;
  this->ptr = nullptr;
  return raw;
}

template <typename T>
Box<T>::Box(uninit) noexcept {}
#endif // CXXBRIDGE1_RUST_BOX

#ifndef CXXBRIDGE1_RUST_BITCOPY
#define CXXBRIDGE1_RUST_BITCOPY
struct unsafe_bitcopy_t final {
  explicit unsafe_bitcopy_t() = default;
};

constexpr unsafe_bitcopy_t unsafe_bitcopy{};
#endif // CXXBRIDGE1_RUST_BITCOPY

#ifndef CXXBRIDGE1_RUST_VEC
#define CXXBRIDGE1_RUST_VEC
template <typename T>
class Vec final {
public:
  using value_type = T;

  Vec() noexcept;
  Vec(std::initializer_list<T>);
  Vec(const Vec &);
  Vec(Vec &&) noexcept;
  ~Vec() noexcept;

  Vec &operator=(Vec &&) noexcept;
  Vec &operator=(const Vec &);

  std::size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  std::size_t capacity() const noexcept;

  const T &operator[](std::size_t n) const noexcept;
  const T &at(std::size_t n) const;
  const T &front() const;
  const T &back() const;

  T &operator[](std::size_t n) noexcept;
  T &at(std::size_t n);
  T &front();
  T &back();

  void reserve(std::size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&... args);

  class iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = typename Vec<const T>::iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  Vec(unsafe_bitcopy_t, const Vec &) noexcept;

private:
  static std::size_t stride() noexcept;
  void reserve_total(std::size_t cap) noexcept;
  void set_len(std::size_t len) noexcept;
  void drop() noexcept;

  std::array<std::uintptr_t, 3> repr;
};

template <typename T>
class Vec<T>::iterator final {
public:
  using iterator_category = std::random_access_iterator_tag;
  using value_type = T;
  using difference_type = std::ptrdiff_t;
  using pointer = typename std::add_pointer<T>::type;
  using reference = typename std::add_lvalue_reference<T>::type;

  reference operator*() const noexcept;
  pointer operator->() const noexcept;
  reference operator[](difference_type) const noexcept;

  iterator &operator++() noexcept;
  iterator operator++(int) noexcept;
  iterator &operator--() noexcept;
  iterator operator--(int) noexcept;

  iterator &operator+=(difference_type) noexcept;
  iterator &operator-=(difference_type) noexcept;
  iterator operator+(difference_type) const noexcept;
  iterator operator-(difference_type) const noexcept;
  difference_type operator-(const iterator &) const noexcept;

  bool operator==(const iterator &) const noexcept;
  bool operator!=(const iterator &) const noexcept;
  bool operator<(const iterator &) const noexcept;
  bool operator>(const iterator &) const noexcept;
  bool operator<=(const iterator &) const noexcept;
  bool operator>=(const iterator &) const noexcept;

private:
  friend class Vec;
  friend class Vec<typename std::remove_const<T>::type>;
  void *pos;
  std::size_t stride;
};

template <typename T>
Vec<T>::Vec(std::initializer_list<T> init) : Vec{} {
  this->reserve_total(init.size());
  std::move(init.begin(), init.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(const Vec &other) : Vec() {
  this->reserve_total(other.size());
  std::copy(other.begin(), other.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(Vec &&other) noexcept : repr(other.repr) {
  new (&other) Vec();
}

template <typename T>
Vec<T>::~Vec() noexcept {
  this->drop();
}

template <typename T>
Vec<T> &Vec<T>::operator=(Vec &&other) noexcept {
  if (this != &other) {
    this->drop();
    this->repr = other.repr;
    new (&other) Vec();
  }
  return *this;
}

template <typename T>
Vec<T> &Vec<T>::operator=(const Vec &other) {
  if (this != &other) {
    this->drop();
    new (this) Vec(other);
  }
  return *this;
}

template <typename T>
bool Vec<T>::empty() const noexcept {
  return size() == 0;
}

template <typename T>
T *Vec<T>::data() noexcept {
  return const_cast<T *>(const_cast<const Vec<T> *>(this)->data());
}

template <typename T>
const T &Vec<T>::operator[](std::size_t n) const noexcept {
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * this->stride());
}

template <typename T>
const T &Vec<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
const T &Vec<T>::front() const {
  return (*this)[0];
}

template <typename T>
const T &Vec<T>::back() const {
  return (*this)[this->size() - 1];
}

template <typename T>
T &Vec<T>::operator[](std::size_t n) noexcept {
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * this->stride());
}

template <typename T>
T &Vec<T>::at(std::size_t n) {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Vec<T>::front() {
  return (*this)[0];
}

template <typename T>
T &Vec<T>::back() {
  return (*this)[this->size() - 1];
}

template <typename T>
void Vec<T>::reserve(std::size_t new_cap) {
  this->reserve_total(new_cap);
}

template <typename T>
void Vec<T>::push_back(const T &value) {
  this->emplace_back(value);
}

template <typename T>
void Vec<T>::push_back(T &&value) {
  this->emplace_back(std::move(value));
}

template <typename T>
template <typename... Args>
void Vec<T>::emplace_back(Args &&... args) {
  auto size = this->size();
  this->reserve_total(size + 1);
  ::new (reinterpret_cast<T *>(reinterpret_cast<char *>(this->data()) +
                               size * this->stride()))
      T(std::forward<Args>(args)...);
  this->set_len(size + 1);
}

template <typename T>
typename Vec<T>::iterator::reference
Vec<T>::iterator::operator*() const noexcept {
  return *static_cast<T *>(this->pos);
}

template <typename T>
typename Vec<T>::iterator::pointer
Vec<T>::iterator::operator->() const noexcept {
  return static_cast<T *>(this->pos);
}

template <typename T>
typename Vec<T>::iterator::reference Vec<T>::iterator::operator[](
    typename Vec<T>::iterator::difference_type n) const noexcept {
  auto pos = static_cast<char *>(this->pos) + this->stride * n;
  return *static_cast<T *>(pos);
}

template <typename T>
typename Vec<T>::iterator &Vec<T>::iterator::operator++() noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return *this;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::iterator::operator++(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return ret;
}

template <typename T>
typename Vec<T>::iterator &Vec<T>::iterator::operator--() noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return *this;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::iterator::operator--(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return ret;
}

template <typename T>
typename Vec<T>::iterator &Vec<T>::iterator::operator+=(
    typename Vec<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride * n;
  return *this;
}

template <typename T>
typename Vec<T>::iterator &Vec<T>::iterator::operator-=(
    typename Vec<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride * n;
  return *this;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::iterator::operator+(
    typename Vec<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) + this->stride * n;
  return ret;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::iterator::operator-(
    typename Vec<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) - this->stride * n;
  return ret;
}

template <typename T>
typename Vec<T>::iterator::difference_type
Vec<T>::iterator::operator-(const iterator &other) const noexcept {
  auto diff = std::distance(static_cast<char *>(other.pos),
                            static_cast<char *>(this->pos));
  return diff / this->stride;
}

template <typename T>
bool Vec<T>::iterator::operator==(const iterator &other) const noexcept {
  return this->pos == other.pos;
}

template <typename T>
bool Vec<T>::iterator::operator!=(const iterator &other) const noexcept {
  return this->pos != other.pos;
}

template <typename T>
bool Vec<T>::iterator::operator>(const iterator &other) const noexcept {
  return this->pos > other.pos;
}

template <typename T>
bool Vec<T>::iterator::operator<(const iterator &other) const noexcept {
  return this->pos < other.pos;
}

template <typename T>
bool Vec<T>::iterator::operator>=(const iterator &other) const noexcept {
  return this->pos >= other.pos;
}

template <typename T>
bool Vec<T>::iterator::operator<=(const iterator &other) const noexcept {
  return this->pos <= other.pos;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::begin() noexcept {
  iterator it;
  it.pos = const_cast<typename std::remove_const<T>::type *>(this->data());
  it.stride = this->stride();
  return it;
}

template <typename T>
typename Vec<T>::iterator Vec<T>::end() noexcept {
  iterator it = this->begin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::begin() const noexcept {
  return this->cbegin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::end() const noexcept {
  return this->cend();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cbegin() const noexcept {
  const_iterator it;
  it.pos = const_cast<typename std::remove_const<T>::type *>(this->data());
  it.stride = this->stride();
  return it;
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cend() const noexcept {
  const_iterator it = this->cbegin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
Vec<T>::Vec(unsafe_bitcopy_t, const Vec &bits) noexcept : repr(bits.repr) {}
#endif // CXXBRIDGE1_RUST_VEC

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE

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
struct PtrLen final {
  void *ptr;
  ::std::size_t len;
};
} // namespace repr

template <>
class impl<Str> final {
public:
  static Str new_unchecked(repr::PtrLen repr) noexcept {
    Str str;
    str.ptr = static_cast<const char *>(repr.ptr);
    str.len = repr.len;
    return str;
  }
  static repr::PtrLen repr(Str str) noexcept {
    return repr::PtrLen{const_cast<char *>(str.ptr), str.len};
  }
};

template <typename T, typename = ::std::size_t>
struct is_complete : std::false_type {};

template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};

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
  enum class ExecuteStatus : ::std::uint8_t;
  enum class NodeType : ::std::uint8_t;
  enum class NodeStatus : ::std::uint8_t;
  enum class AttrState : ::std::uint8_t;
  enum class StreamDataState : ::std::uint8_t;
  namespace shared {
    struct SharedThing;
  }
  namespace internal {
    struct GraphImplShared;
    struct NodeImplShared;
    struct StreamDataImplShared;
    struct CacheImplShared;
    struct ThingR;
    struct PixelBlock;
    struct BoundingBox2D;
    struct Matrix4;
    struct StreamDataImpl;
    struct NodeImpl;
    struct CacheImpl;
    struct GraphImpl;
  }
  namespace cpp {
    using ThingC = ::open_comp_graph::cpp::ThingC;
  }
}

namespace open_comp_graph {
namespace shared {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$shared$SharedThing
#define CXXBRIDGE1_STRUCT_open_comp_graph$shared$SharedThing
struct SharedThing final {
  ::std::int32_t z;
  ::rust::Box<::open_comp_graph::internal::ThingR> y;
  ::std::unique_ptr<::open_comp_graph::cpp::ThingC> x;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$shared$SharedThing
} // namespace shared

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared
struct GraphImplShared final {
  ::rust::Box<::open_comp_graph::internal::GraphImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImplShared
struct NodeImplShared final {
  ::rust::Box<::open_comp_graph::internal::NodeImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImplShared

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImplShared
struct StreamDataImplShared final {
  ::rust::Box<::open_comp_graph::internal::StreamDataImpl> inner;

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
} // namespace internal

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus
#define CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus
enum class ExecuteStatus : ::std::uint8_t {
  Error = 0,
  Success = 1,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$ExecuteStatus

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$NodeType
#define CXXBRIDGE1_ENUM_open_comp_graph$NodeType
enum class NodeType : ::std::uint8_t {
  Null = 0,
  ReadImage = 1,
  WriteImage = 2,
  Grade = 3,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$NodeType

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus
#define CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus
enum class NodeStatus : ::std::uint8_t {
  Error = 0,
  Valid = 1,
  Uninitialized = 2,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$NodeStatus

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$AttrState
#define CXXBRIDGE1_ENUM_open_comp_graph$AttrState
enum class AttrState : ::std::uint8_t {
  Missing = 0,
  Exists = 1,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$AttrState

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState
#define CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState
enum class StreamDataState : ::std::uint8_t {
  Invalid = 0,
  Valid = 1,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$StreamDataState

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl
struct StreamDataImpl final : public ::rust::Opaque {
  ::std::uint64_t get_hash() const noexcept;
  const ::rust::Box<::open_comp_graph::internal::BoundingBox2D> &get_bounding_box() const noexcept;
  const ::rust::Box<::open_comp_graph::internal::Matrix4> &get_color_matrix() const noexcept;
  const ::rust::Box<::open_comp_graph::internal::Matrix4> &get_transform_matrix() const noexcept;
  const ::open_comp_graph::internal::PixelBlock &get_pixel_block() const noexcept;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$StreamDataImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl
struct NodeImpl final : public ::rust::Opaque {
  ::std::uint64_t get_id() const noexcept;
  ::open_comp_graph::NodeType get_node_type() const noexcept;
  ::std::uint8_t get_node_type_id() const noexcept;
  ::open_comp_graph::NodeStatus get_status() const noexcept;
  ::std::uint8_t get_status_id() const noexcept;
  ::std::uint64_t hash(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs) const noexcept;
  ::open_comp_graph::NodeStatus compute(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs, ::open_comp_graph::internal::StreamDataImplShared &output) noexcept;
  ::open_comp_graph::AttrState attr_exists(::rust::Str name) const noexcept;
  float get_attr_f32(::rust::Str name) const noexcept;
  ::std::int32_t get_attr_i32(::rust::Str name) const noexcept;
  ::rust::Str get_attr_str(::rust::Str name) const noexcept;
  void set_attr_f32(::rust::Str name, float value) noexcept;
  void set_attr_i32(::rust::Str name, ::std::int32_t value) noexcept;
  void set_attr_str(::rust::Str name, ::rust::Str value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$NodeImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl
struct CacheImpl final : public ::rust::Opaque {
  ::std::uint64_t get_id() const noexcept;
  ::std::size_t len() const noexcept;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$CacheImpl

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl
struct GraphImpl final : public ::rust::Opaque {
  ::std::size_t add_node(::rust::Box<::open_comp_graph::internal::NodeImpl> op_box) noexcept;
  void connect(::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept;
  ::open_comp_graph::ExecuteStatus execute(::std::size_t start_index, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_open_comp_graph$internal$GraphImpl

extern "C" {
::std::size_t open_comp_graph$internal$cxxbridge1$StreamDataImplShared$operator$hash(const StreamDataImplShared &) noexcept;
} // extern "C"
} // namespace internal

namespace cpp {
extern "C" {
__declspec(dllexport) ::open_comp_graph::cpp::ThingC *open_comp_graph$cpp$cxxbridge1$make_thingc(::rust::repr::PtrLen appname) noexcept {
  ::std::unique_ptr<::open_comp_graph::cpp::ThingC> (*make_thingc$)(::rust::Str) = ::open_comp_graph::cpp::make_thingc;
  return make_thingc$(::rust::impl<::rust::Str>::new_unchecked(appname)).release();
}

__declspec(dllexport) const ::std::string *open_comp_graph$cpp$cxxbridge1$get_name(const ::open_comp_graph::cpp::ThingC &thing) noexcept {
  const ::std::string &(*get_name$)(const ::open_comp_graph::cpp::ThingC &) = ::open_comp_graph::cpp::get_name;
  return &get_name$(thing);
}

__declspec(dllexport) void open_comp_graph$cpp$cxxbridge1$run_sharedthing(::open_comp_graph::shared::SharedThing *state) noexcept {
  void (*run_sharedthing$)(::open_comp_graph::shared::SharedThing) = ::open_comp_graph::cpp::run_sharedthing;
  run_sharedthing$(::std::move(*state));
}
} // extern "C"
} // namespace cpp

namespace internal {
extern "C" {
void open_comp_graph$internal$cxxbridge1$print_r(const ::open_comp_graph::internal::ThingR &r) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_hash(const ::open_comp_graph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::open_comp_graph::internal::BoundingBox2D> *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_bounding_box(const ::open_comp_graph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::open_comp_graph::internal::Matrix4> *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_color_matrix(const ::open_comp_graph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::open_comp_graph::internal::Matrix4> *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_transform_matrix(const ::open_comp_graph::internal::StreamDataImpl &self) noexcept;

const ::open_comp_graph::internal::PixelBlock *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_pixel_block(const ::open_comp_graph::internal::StreamDataImpl &self) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$NodeImpl$get_id(const ::open_comp_graph::internal::NodeImpl &self) noexcept;

::open_comp_graph::NodeType open_comp_graph$internal$cxxbridge1$NodeImpl$get_node_type(const ::open_comp_graph::internal::NodeImpl &self) noexcept;

::std::uint8_t open_comp_graph$internal$cxxbridge1$NodeImpl$get_node_type_id(const ::open_comp_graph::internal::NodeImpl &self) noexcept;

::open_comp_graph::NodeStatus open_comp_graph$internal$cxxbridge1$NodeImpl$get_status(const ::open_comp_graph::internal::NodeImpl &self) noexcept;

::std::uint8_t open_comp_graph$internal$cxxbridge1$NodeImpl$get_status_id(const ::open_comp_graph::internal::NodeImpl &self) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$NodeImpl$hash(const ::open_comp_graph::internal::NodeImpl &self, const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs) noexcept;

::open_comp_graph::NodeStatus open_comp_graph$internal$cxxbridge1$NodeImpl$compute(::open_comp_graph::internal::NodeImpl &self, const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs, ::open_comp_graph::internal::StreamDataImplShared &output) noexcept;

::open_comp_graph::AttrState open_comp_graph$internal$cxxbridge1$NodeImpl$attr_exists(const ::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

float open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_f32(const ::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

::std::int32_t open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_i32(const ::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

::rust::repr::PtrLen open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_str(const ::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

void open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_f32(::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name, float value) noexcept;

void open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_i32(::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name, ::std::int32_t value) noexcept;

void open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_str(::open_comp_graph::internal::NodeImpl &self, ::rust::repr::PtrLen name, ::rust::repr::PtrLen value) noexcept;

::std::uint64_t open_comp_graph$internal$cxxbridge1$CacheImpl$get_id(const ::open_comp_graph::internal::CacheImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$CacheImpl$len(const ::open_comp_graph::internal::CacheImpl &self) noexcept;

::std::size_t open_comp_graph$internal$cxxbridge1$GraphImpl$add_node(::open_comp_graph::internal::GraphImpl &self, ::open_comp_graph::internal::NodeImpl *op_box) noexcept;

void open_comp_graph$internal$cxxbridge1$GraphImpl$connect(::open_comp_graph::internal::GraphImpl &self, ::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept;

::open_comp_graph::ExecuteStatus open_comp_graph$internal$cxxbridge1$GraphImpl$execute(::open_comp_graph::internal::GraphImpl &self, ::std::size_t start_index, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept;

::open_comp_graph::internal::NodeImpl *open_comp_graph$internal$cxxbridge1$create_node_box(::open_comp_graph::NodeType node_type) noexcept;

::open_comp_graph::internal::NodeImpl *open_comp_graph$internal$cxxbridge1$create_node_box_with_name(::open_comp_graph::NodeType node_type, ::rust::repr::PtrLen name) noexcept;

::open_comp_graph::internal::NodeImpl *open_comp_graph$internal$cxxbridge1$create_node_box_with_id(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept;

void open_comp_graph$internal$cxxbridge1$create_node_shared(::open_comp_graph::NodeType node_type, ::open_comp_graph::internal::NodeImplShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$create_node_shared_with_name(::open_comp_graph::NodeType node_type, ::rust::repr::PtrLen name, ::open_comp_graph::internal::NodeImplShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$create_node_shared_with_id(::open_comp_graph::NodeType node_type, ::std::uint64_t id, ::open_comp_graph::internal::NodeImplShared *return$) noexcept;

::open_comp_graph::internal::CacheImpl *open_comp_graph$internal$cxxbridge1$create_cache_box() noexcept;

void open_comp_graph$internal$cxxbridge1$create_cache_shared(::open_comp_graph::internal::CacheImplShared *return$) noexcept;

::open_comp_graph::internal::GraphImpl *open_comp_graph$internal$cxxbridge1$create_graph_box() noexcept;

void open_comp_graph$internal$cxxbridge1$create_graph_shared(::open_comp_graph::internal::GraphImplShared *return$) noexcept;

::open_comp_graph::internal::StreamDataImpl *open_comp_graph$internal$cxxbridge1$create_stream_data_box() noexcept;

void open_comp_graph$internal$cxxbridge1$create_stream_data_shared(::open_comp_graph::internal::StreamDataImplShared *return$) noexcept;

void open_comp_graph$internal$cxxbridge1$create_vec_stream_data_shared(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *return$) noexcept;
} // extern "C"
} // namespace internal
} // namespace open_comp_graph

namespace std {
template <> struct hash<::open_comp_graph::internal::StreamDataImplShared> {
  ::std::size_t operator()(const ::open_comp_graph::internal::StreamDataImplShared &self) const noexcept {
    return ::open_comp_graph::internal::open_comp_graph$internal$cxxbridge1$StreamDataImplShared$operator$hash(self);
  }
};
} // namespace std

namespace open_comp_graph {
namespace internal {
void print_r(const ::open_comp_graph::internal::ThingR &r) noexcept {
  open_comp_graph$internal$cxxbridge1$print_r(r);
}

::std::uint64_t StreamDataImpl::get_hash() const noexcept {
  return open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_hash(*this);
}

const ::rust::Box<::open_comp_graph::internal::BoundingBox2D> &StreamDataImpl::get_bounding_box() const noexcept {
  return *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_bounding_box(*this);
}

const ::rust::Box<::open_comp_graph::internal::Matrix4> &StreamDataImpl::get_color_matrix() const noexcept {
  return *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_color_matrix(*this);
}

const ::rust::Box<::open_comp_graph::internal::Matrix4> &StreamDataImpl::get_transform_matrix() const noexcept {
  return *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_transform_matrix(*this);
}

const ::open_comp_graph::internal::PixelBlock &StreamDataImpl::get_pixel_block() const noexcept {
  return *open_comp_graph$internal$cxxbridge1$StreamDataImpl$get_pixel_block(*this);
}

::std::uint64_t NodeImpl::get_id() const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_id(*this);
}

::open_comp_graph::NodeType NodeImpl::get_node_type() const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_node_type(*this);
}

::std::uint8_t NodeImpl::get_node_type_id() const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_node_type_id(*this);
}

::open_comp_graph::NodeStatus NodeImpl::get_status() const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_status(*this);
}

::std::uint8_t NodeImpl::get_status_id() const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_status_id(*this);
}

::std::uint64_t NodeImpl::hash(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs) const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$hash(*this, inputs);
}

::open_comp_graph::NodeStatus NodeImpl::compute(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> &inputs, ::open_comp_graph::internal::StreamDataImplShared &output) noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$compute(*this, inputs, output);
}

::open_comp_graph::AttrState NodeImpl::attr_exists(::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$attr_exists(*this, ::rust::impl<::rust::Str>::repr(name));
}

float NodeImpl::get_attr_f32(::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_f32(*this, ::rust::impl<::rust::Str>::repr(name));
}

::std::int32_t NodeImpl::get_attr_i32(::rust::Str name) const noexcept {
  return open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_i32(*this, ::rust::impl<::rust::Str>::repr(name));
}

::rust::Str NodeImpl::get_attr_str(::rust::Str name) const noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(open_comp_graph$internal$cxxbridge1$NodeImpl$get_attr_str(*this, ::rust::impl<::rust::Str>::repr(name)));
}

void NodeImpl::set_attr_f32(::rust::Str name, float value) noexcept {
  open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_f32(*this, ::rust::impl<::rust::Str>::repr(name), value);
}

void NodeImpl::set_attr_i32(::rust::Str name, ::std::int32_t value) noexcept {
  open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_i32(*this, ::rust::impl<::rust::Str>::repr(name), value);
}

void NodeImpl::set_attr_str(::rust::Str name, ::rust::Str value) noexcept {
  open_comp_graph$internal$cxxbridge1$NodeImpl$set_attr_str(*this, ::rust::impl<::rust::Str>::repr(name), ::rust::impl<::rust::Str>::repr(value));
}

::std::uint64_t CacheImpl::get_id() const noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$get_id(*this);
}

::std::size_t CacheImpl::len() const noexcept {
  return open_comp_graph$internal$cxxbridge1$CacheImpl$len(*this);
}

::std::size_t GraphImpl::add_node(::rust::Box<::open_comp_graph::internal::NodeImpl> op_box) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$add_node(*this, op_box.into_raw());
}

void GraphImpl::connect(::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept {
  open_comp_graph$internal$cxxbridge1$GraphImpl$connect(*this, src_index, dst_index, input_num);
}

::open_comp_graph::ExecuteStatus GraphImpl::execute(::std::size_t start_index, ::rust::Box<::open_comp_graph::internal::CacheImpl> &cache) noexcept {
  return open_comp_graph$internal$cxxbridge1$GraphImpl$execute(*this, start_index, cache);
}

::rust::Box<::open_comp_graph::internal::NodeImpl> create_node_box(::open_comp_graph::NodeType node_type) noexcept {
  return ::rust::Box<::open_comp_graph::internal::NodeImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_node_box(node_type));
}

::rust::Box<::open_comp_graph::internal::NodeImpl> create_node_box(::open_comp_graph::NodeType node_type, ::rust::Str name) noexcept {
  return ::rust::Box<::open_comp_graph::internal::NodeImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_node_box_with_name(node_type, ::rust::impl<::rust::Str>::repr(name)));
}

::rust::Box<::open_comp_graph::internal::NodeImpl> create_node_box(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept {
  return ::rust::Box<::open_comp_graph::internal::NodeImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_node_box_with_id(node_type, id));
}

::open_comp_graph::internal::NodeImplShared create_node_shared(::open_comp_graph::NodeType node_type) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::NodeImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_node_shared(node_type, &return$.value);
  return ::std::move(return$.value);
}

::open_comp_graph::internal::NodeImplShared create_node_shared(::open_comp_graph::NodeType node_type, ::rust::Str name) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::NodeImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_node_shared_with_name(node_type, ::rust::impl<::rust::Str>::repr(name), &return$.value);
  return ::std::move(return$.value);
}

::open_comp_graph::internal::NodeImplShared create_node_shared(::open_comp_graph::NodeType node_type, ::std::uint64_t id) noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::NodeImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_node_shared_with_id(node_type, id, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::open_comp_graph::internal::CacheImpl> create_cache_box() noexcept {
  return ::rust::Box<::open_comp_graph::internal::CacheImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_cache_box());
}

::open_comp_graph::internal::CacheImplShared create_cache_shared() noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::CacheImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_cache_shared(&return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::open_comp_graph::internal::GraphImpl> create_graph_box() noexcept {
  return ::rust::Box<::open_comp_graph::internal::GraphImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_graph_box());
}

::open_comp_graph::internal::GraphImplShared create_graph_shared() noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::GraphImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_graph_shared(&return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::open_comp_graph::internal::StreamDataImpl> create_stream_data_box() noexcept {
  return ::rust::Box<::open_comp_graph::internal::StreamDataImpl>::from_raw(open_comp_graph$internal$cxxbridge1$create_stream_data_box());
}

::open_comp_graph::internal::StreamDataImplShared create_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::open_comp_graph::internal::StreamDataImplShared> return$;
  open_comp_graph$internal$cxxbridge1$create_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> create_vec_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::rust::Vec<::open_comp_graph::internal::StreamDataImplShared>> return$;
  open_comp_graph$internal$cxxbridge1$create_vec_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}
} // namespace internal
} // namespace open_comp_graph

extern "C" {
#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$ThingR
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$ThingR
::open_comp_graph::internal::ThingR *cxxbridge1$box$open_comp_graph$internal$ThingR$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$ThingR$dealloc(::open_comp_graph::internal::ThingR *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$ThingR$drop(::rust::Box<::open_comp_graph::internal::ThingR> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$ThingR

#ifndef CXXBRIDGE1_UNIQUE_PTR_open_comp_graph$cpp$ThingC
#define CXXBRIDGE1_UNIQUE_PTR_open_comp_graph$cpp$ThingC
static_assert(::rust::is_complete<::open_comp_graph::cpp::ThingC>::value, "definition of ThingC is required");
static_assert(sizeof(::std::unique_ptr<::open_comp_graph::cpp::ThingC>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::open_comp_graph::cpp::ThingC>) == alignof(void *), "");
void cxxbridge1$unique_ptr$open_comp_graph$cpp$ThingC$null(::std::unique_ptr<::open_comp_graph::cpp::ThingC> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::open_comp_graph::cpp::ThingC>();
}
void cxxbridge1$unique_ptr$open_comp_graph$cpp$ThingC$raw(::std::unique_ptr<::open_comp_graph::cpp::ThingC> *ptr, ::open_comp_graph::cpp::ThingC *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::open_comp_graph::cpp::ThingC>(raw);
}
const ::open_comp_graph::cpp::ThingC *cxxbridge1$unique_ptr$open_comp_graph$cpp$ThingC$get(const ::std::unique_ptr<::open_comp_graph::cpp::ThingC>& ptr) noexcept {
  return ptr.get();
}
::open_comp_graph::cpp::ThingC *cxxbridge1$unique_ptr$open_comp_graph$cpp$ThingC$release(::std::unique_ptr<::open_comp_graph::cpp::ThingC>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$open_comp_graph$cpp$ThingC$drop(::std::unique_ptr<::open_comp_graph::cpp::ThingC> *ptr) noexcept {
  ::rust::deleter_if<::rust::is_complete<::open_comp_graph::cpp::ThingC>::value>{}(ptr);
}
#endif // CXXBRIDGE1_UNIQUE_PTR_open_comp_graph$cpp$ThingC

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$GraphImpl
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$GraphImpl
::open_comp_graph::internal::GraphImpl *cxxbridge1$box$open_comp_graph$internal$GraphImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$GraphImpl$dealloc(::open_comp_graph::internal::GraphImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$GraphImpl$drop(::rust::Box<::open_comp_graph::internal::GraphImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$GraphImpl

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$NodeImpl
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$NodeImpl
::open_comp_graph::internal::NodeImpl *cxxbridge1$box$open_comp_graph$internal$NodeImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$NodeImpl$dealloc(::open_comp_graph::internal::NodeImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$NodeImpl$drop(::rust::Box<::open_comp_graph::internal::NodeImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$NodeImpl

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$StreamDataImpl
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$StreamDataImpl
::open_comp_graph::internal::StreamDataImpl *cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$dealloc(::open_comp_graph::internal::StreamDataImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$drop(::rust::Box<::open_comp_graph::internal::StreamDataImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$StreamDataImpl

#ifndef CXXBRIDGE1_RUST_VEC_open_comp_graph$internal$StreamDataImplShared
#define CXXBRIDGE1_RUST_VEC_open_comp_graph$internal$StreamDataImplShared
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$new(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$drop(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$len(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$capacity(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
const ::open_comp_graph::internal::StreamDataImplShared *cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$data(const ::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$reserve_total(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr, ::std::size_t cap) noexcept;
void cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$set_len(::rust::Vec<::open_comp_graph::internal::StreamDataImplShared> *ptr, ::std::size_t len) noexcept;
::std::size_t cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$stride() noexcept;
#endif // CXXBRIDGE1_RUST_VEC_open_comp_graph$internal$StreamDataImplShared

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$CacheImpl
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$CacheImpl
::open_comp_graph::internal::CacheImpl *cxxbridge1$box$open_comp_graph$internal$CacheImpl$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$CacheImpl$dealloc(::open_comp_graph::internal::CacheImpl *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$CacheImpl$drop(::rust::Box<::open_comp_graph::internal::CacheImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$CacheImpl

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$BoundingBox2D
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$BoundingBox2D
::open_comp_graph::internal::BoundingBox2D *cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$dealloc(::open_comp_graph::internal::BoundingBox2D *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$drop(::rust::Box<::open_comp_graph::internal::BoundingBox2D> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$BoundingBox2D

#ifndef CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$Matrix4
#define CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$Matrix4
::open_comp_graph::internal::Matrix4 *cxxbridge1$box$open_comp_graph$internal$Matrix4$alloc() noexcept;
void cxxbridge1$box$open_comp_graph$internal$Matrix4$dealloc(::open_comp_graph::internal::Matrix4 *) noexcept;
void cxxbridge1$box$open_comp_graph$internal$Matrix4$drop(::rust::Box<::open_comp_graph::internal::Matrix4> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_open_comp_graph$internal$Matrix4
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::open_comp_graph::internal::ThingR *Box<::open_comp_graph::internal::ThingR>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$ThingR$alloc();
}
template <>
void Box<::open_comp_graph::internal::ThingR>::allocation::dealloc(::open_comp_graph::internal::ThingR *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$ThingR$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::ThingR>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$ThingR$drop(this);
}
template <>
::open_comp_graph::internal::GraphImpl *Box<::open_comp_graph::internal::GraphImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$GraphImpl$alloc();
}
template <>
void Box<::open_comp_graph::internal::GraphImpl>::allocation::dealloc(::open_comp_graph::internal::GraphImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$GraphImpl$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::GraphImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$GraphImpl$drop(this);
}
template <>
::open_comp_graph::internal::NodeImpl *Box<::open_comp_graph::internal::NodeImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$NodeImpl$alloc();
}
template <>
void Box<::open_comp_graph::internal::NodeImpl>::allocation::dealloc(::open_comp_graph::internal::NodeImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$NodeImpl$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::NodeImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$NodeImpl$drop(this);
}
template <>
::open_comp_graph::internal::StreamDataImpl *Box<::open_comp_graph::internal::StreamDataImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$alloc();
}
template <>
void Box<::open_comp_graph::internal::StreamDataImpl>::allocation::dealloc(::open_comp_graph::internal::StreamDataImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::StreamDataImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$StreamDataImpl$drop(this);
}
template <>
Vec<::open_comp_graph::internal::StreamDataImplShared>::Vec() noexcept {
  cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$new(this);
}
template <>
void Vec<::open_comp_graph::internal::StreamDataImplShared>::drop() noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$drop(this);
}
template <>
::std::size_t Vec<::open_comp_graph::internal::StreamDataImplShared>::size() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$len(this);
}
template <>
::std::size_t Vec<::open_comp_graph::internal::StreamDataImplShared>::capacity() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$capacity(this);
}
template <>
const ::open_comp_graph::internal::StreamDataImplShared *Vec<::open_comp_graph::internal::StreamDataImplShared>::data() const noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$data(this);
}
template <>
void Vec<::open_comp_graph::internal::StreamDataImplShared>::reserve_total(::std::size_t cap) noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$reserve_total(this, cap);
}
template <>
void Vec<::open_comp_graph::internal::StreamDataImplShared>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$set_len(this, len);
}
template <>
::std::size_t Vec<::open_comp_graph::internal::StreamDataImplShared>::stride() noexcept {
  return cxxbridge1$rust_vec$open_comp_graph$internal$StreamDataImplShared$stride();
}
template <>
::open_comp_graph::internal::CacheImpl *Box<::open_comp_graph::internal::CacheImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$CacheImpl$alloc();
}
template <>
void Box<::open_comp_graph::internal::CacheImpl>::allocation::dealloc(::open_comp_graph::internal::CacheImpl *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$CacheImpl$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::CacheImpl>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$CacheImpl$drop(this);
}
template <>
::open_comp_graph::internal::BoundingBox2D *Box<::open_comp_graph::internal::BoundingBox2D>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$alloc();
}
template <>
void Box<::open_comp_graph::internal::BoundingBox2D>::allocation::dealloc(::open_comp_graph::internal::BoundingBox2D *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::BoundingBox2D>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$BoundingBox2D$drop(this);
}
template <>
::open_comp_graph::internal::Matrix4 *Box<::open_comp_graph::internal::Matrix4>::allocation::alloc() noexcept {
  return cxxbridge1$box$open_comp_graph$internal$Matrix4$alloc();
}
template <>
void Box<::open_comp_graph::internal::Matrix4>::allocation::dealloc(::open_comp_graph::internal::Matrix4 *ptr) noexcept {
  cxxbridge1$box$open_comp_graph$internal$Matrix4$dealloc(ptr);
}
template <>
void Box<::open_comp_graph::internal::Matrix4>::drop() noexcept {
  cxxbridge1$box$open_comp_graph$internal$Matrix4$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
