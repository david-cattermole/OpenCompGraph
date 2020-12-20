#include "rust/cxx.h"
#include "opencompgraph/cpp.h"
#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
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

namespace opencompgraph {
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
    struct ThingR;
    struct PixelBlock;
    struct BoundingBox2D;
    struct Matrix4;
    struct StreamDataImpl;
    struct NodeImpl;
    struct GraphImpl;
  }
  namespace cpp {
    using ThingC = ::opencompgraph::cpp::ThingC;
  }
}

namespace opencompgraph {
namespace shared {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$shared$SharedThing
#define CXXBRIDGE1_STRUCT_opencompgraph$shared$SharedThing
struct SharedThing final {
  ::std::int32_t z;
  ::rust::Box<::opencompgraph::internal::ThingR> y;
  ::std::unique_ptr<::opencompgraph::cpp::ThingC> x;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$shared$SharedThing
} // namespace shared

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImplShared
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImplShared
struct GraphImplShared final {
  ::rust::Box<::opencompgraph::internal::GraphImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImplShared

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImplShared
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImplShared
struct NodeImplShared final {
  ::rust::Box<::opencompgraph::internal::NodeImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImplShared

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImplShared
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImplShared
struct StreamDataImplShared final {
  ::rust::Box<::opencompgraph::internal::StreamDataImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImplShared
} // namespace internal

#ifndef CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus
#define CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus
enum class ExecuteStatus : ::std::uint8_t {
  Error = 0,
  Success = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus

#ifndef CXXBRIDGE1_ENUM_opencompgraph$NodeType
#define CXXBRIDGE1_ENUM_opencompgraph$NodeType
enum class NodeType : ::std::uint8_t {
  Null = 0,
  ReadImage = 1,
  WriteImage = 2,
  Grade = 3,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$NodeType

#ifndef CXXBRIDGE1_ENUM_opencompgraph$NodeStatus
#define CXXBRIDGE1_ENUM_opencompgraph$NodeStatus
enum class NodeStatus : ::std::uint8_t {
  Error = 0,
  Warning = 1,
  Valid = 2,
  Uninitialized = 3,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$NodeStatus

#ifndef CXXBRIDGE1_ENUM_opencompgraph$AttrState
#define CXXBRIDGE1_ENUM_opencompgraph$AttrState
enum class AttrState : ::std::uint8_t {
  Missing = 0,
  Exists = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$AttrState

#ifndef CXXBRIDGE1_ENUM_opencompgraph$StreamDataState
#define CXXBRIDGE1_ENUM_opencompgraph$StreamDataState
enum class StreamDataState : ::std::uint8_t {
  Invalid = 0,
  Valid = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$StreamDataState

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImpl
struct StreamDataImpl final : public ::rust::Opaque {
  ::std::size_t get_hash() const noexcept;
  const ::rust::Box<::opencompgraph::internal::PixelBlock> &get_pixel_block() const noexcept;
  const ::rust::Box<::opencompgraph::internal::BoundingBox2D> &get_bounding_box() const noexcept;
  const ::rust::Box<::opencompgraph::internal::Matrix4> &get_color_matrix() const noexcept;
  const ::rust::Box<::opencompgraph::internal::Matrix4> &get_transform_matrix() const noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$StreamDataImpl

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImpl
struct NodeImpl final : public ::rust::Opaque {
  ::std::uint64_t get_id() const noexcept;
  ::opencompgraph::NodeType get_node_type() const noexcept;
  ::std::uint8_t get_node_type_id() const noexcept;
  ::opencompgraph::NodeStatus get_status() const noexcept;
  ::std::uint8_t get_status_id() const noexcept;
  ::std::size_t hash(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs) noexcept;
  ::opencompgraph::NodeStatus compute(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs, ::opencompgraph::internal::StreamDataImplShared &output) noexcept;
  ::opencompgraph::AttrState attr_exists(::rust::Str name) const noexcept;
  float get_attr_f32(::rust::Str name) const noexcept;
  ::std::int32_t get_attr_i32(::rust::Str name) const noexcept;
  ::rust::Str get_attr_string(::rust::Str name) const noexcept;
  void set_attr(::rust::Str name, float value) noexcept;
  void set_attr(::rust::Str name, ::std::int32_t value) noexcept;
  void set_attr(::rust::Str name, ::rust::Str value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$NodeImpl

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
struct GraphImpl final : public ::rust::Opaque {
  ::std::size_t add_op(::rust::Box<::opencompgraph::internal::NodeImpl> op_box) noexcept;
  void connect(::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept;
  ::opencompgraph::ExecuteStatus execute(::std::size_t start_index) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
} // namespace internal

namespace cpp {
extern "C" {
__declspec(dllexport) ::opencompgraph::cpp::ThingC *opencompgraph$cpp$cxxbridge1$make_thingc(::rust::repr::PtrLen appname) noexcept {
  ::std::unique_ptr<::opencompgraph::cpp::ThingC> (*make_thingc$)(::rust::Str) = ::opencompgraph::cpp::make_thingc;
  return make_thingc$(::rust::impl<::rust::Str>::new_unchecked(appname)).release();
}

__declspec(dllexport) const ::std::string *opencompgraph$cpp$cxxbridge1$get_name(const ::opencompgraph::cpp::ThingC &thing) noexcept {
  const ::std::string &(*get_name$)(const ::opencompgraph::cpp::ThingC &) = ::opencompgraph::cpp::get_name;
  return &get_name$(thing);
}

__declspec(dllexport) void opencompgraph$cpp$cxxbridge1$run_sharedthing(::opencompgraph::shared::SharedThing *state) noexcept {
  void (*run_sharedthing$)(::opencompgraph::shared::SharedThing) = ::opencompgraph::cpp::run_sharedthing;
  run_sharedthing$(::std::move(*state));
}
} // extern "C"
} // namespace cpp

namespace internal {
extern "C" {
void opencompgraph$internal$cxxbridge1$print_r(const ::opencompgraph::internal::ThingR &r) noexcept;

::std::size_t opencompgraph$internal$cxxbridge1$StreamDataImpl$get_hash(const ::opencompgraph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::opencompgraph::internal::PixelBlock> *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_pixel_block(const ::opencompgraph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::opencompgraph::internal::BoundingBox2D> *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_bounding_box(const ::opencompgraph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::opencompgraph::internal::Matrix4> *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_color_matrix(const ::opencompgraph::internal::StreamDataImpl &self) noexcept;

const ::rust::Box<::opencompgraph::internal::Matrix4> *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_transform_matrix(const ::opencompgraph::internal::StreamDataImpl &self) noexcept;

::std::uint64_t opencompgraph$internal$cxxbridge1$NodeImpl$get_id(const ::opencompgraph::internal::NodeImpl &self) noexcept;

::opencompgraph::NodeType opencompgraph$internal$cxxbridge1$NodeImpl$get_node_type(const ::opencompgraph::internal::NodeImpl &self) noexcept;

::std::uint8_t opencompgraph$internal$cxxbridge1$NodeImpl$get_node_type_id(const ::opencompgraph::internal::NodeImpl &self) noexcept;

::opencompgraph::NodeStatus opencompgraph$internal$cxxbridge1$NodeImpl$get_status(const ::opencompgraph::internal::NodeImpl &self) noexcept;

::std::uint8_t opencompgraph$internal$cxxbridge1$NodeImpl$get_status_id(const ::opencompgraph::internal::NodeImpl &self) noexcept;

::std::size_t opencompgraph$internal$cxxbridge1$NodeImpl$hash(::opencompgraph::internal::NodeImpl &self, const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs) noexcept;

::opencompgraph::NodeStatus opencompgraph$internal$cxxbridge1$NodeImpl$compute(::opencompgraph::internal::NodeImpl &self, const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs, ::opencompgraph::internal::StreamDataImplShared &output) noexcept;

::opencompgraph::AttrState opencompgraph$internal$cxxbridge1$NodeImpl$attr_exists(const ::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

float opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_f32(const ::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

::std::int32_t opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_i32(const ::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

::rust::repr::PtrLen opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_string(const ::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name) noexcept;

void opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_f32(::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name, float value) noexcept;

void opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_i32(::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name, ::std::int32_t value) noexcept;

void opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_string(::opencompgraph::internal::NodeImpl &self, ::rust::repr::PtrLen name, ::rust::repr::PtrLen value) noexcept;

::std::size_t opencompgraph$internal$cxxbridge1$GraphImpl$add_op(::opencompgraph::internal::GraphImpl &self, ::opencompgraph::internal::NodeImpl *op_box) noexcept;

void opencompgraph$internal$cxxbridge1$GraphImpl$connect(::opencompgraph::internal::GraphImpl &self, ::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept;

::opencompgraph::ExecuteStatus opencompgraph$internal$cxxbridge1$GraphImpl$execute(::opencompgraph::internal::GraphImpl &self, ::std::size_t start_index) noexcept;

::opencompgraph::internal::NodeImpl *opencompgraph$internal$cxxbridge1$create_node_box(::opencompgraph::NodeType node_type) noexcept;

::opencompgraph::internal::NodeImpl *opencompgraph$internal$cxxbridge1$create_node_box_with_name(::opencompgraph::NodeType node_type, ::rust::repr::PtrLen name) noexcept;

::opencompgraph::internal::NodeImpl *opencompgraph$internal$cxxbridge1$create_node_box_with_id(::opencompgraph::NodeType node_type, ::std::uint64_t id) noexcept;

void opencompgraph$internal$cxxbridge1$create_node_shared(::opencompgraph::NodeType node_type, ::opencompgraph::internal::NodeImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_node_shared_with_name(::opencompgraph::NodeType node_type, ::rust::repr::PtrLen name, ::opencompgraph::internal::NodeImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_node_shared_with_id(::opencompgraph::NodeType node_type, ::std::uint64_t id, ::opencompgraph::internal::NodeImplShared *return$) noexcept;

::opencompgraph::internal::GraphImpl *opencompgraph$internal$cxxbridge1$create_graph_box() noexcept;

void opencompgraph$internal$cxxbridge1$create_graph_shared(::opencompgraph::internal::GraphImplShared *return$) noexcept;

::opencompgraph::internal::StreamDataImpl *opencompgraph$internal$cxxbridge1$create_stream_data_box() noexcept;

void opencompgraph$internal$cxxbridge1$create_stream_data_shared(::opencompgraph::internal::StreamDataImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_vec_stream_data_shared(::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *return$) noexcept;
} // extern "C"

void print_r(const ::opencompgraph::internal::ThingR &r) noexcept {
  opencompgraph$internal$cxxbridge1$print_r(r);
}

::std::size_t StreamDataImpl::get_hash() const noexcept {
  return opencompgraph$internal$cxxbridge1$StreamDataImpl$get_hash(*this);
}

const ::rust::Box<::opencompgraph::internal::PixelBlock> &StreamDataImpl::get_pixel_block() const noexcept {
  return *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_pixel_block(*this);
}

const ::rust::Box<::opencompgraph::internal::BoundingBox2D> &StreamDataImpl::get_bounding_box() const noexcept {
  return *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_bounding_box(*this);
}

const ::rust::Box<::opencompgraph::internal::Matrix4> &StreamDataImpl::get_color_matrix() const noexcept {
  return *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_color_matrix(*this);
}

const ::rust::Box<::opencompgraph::internal::Matrix4> &StreamDataImpl::get_transform_matrix() const noexcept {
  return *opencompgraph$internal$cxxbridge1$StreamDataImpl$get_transform_matrix(*this);
}

::std::uint64_t NodeImpl::get_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_id(*this);
}

::opencompgraph::NodeType NodeImpl::get_node_type() const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_node_type(*this);
}

::std::uint8_t NodeImpl::get_node_type_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_node_type_id(*this);
}

::opencompgraph::NodeStatus NodeImpl::get_status() const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_status(*this);
}

::std::uint8_t NodeImpl::get_status_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_status_id(*this);
}

::std::size_t NodeImpl::hash(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs) noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$hash(*this, inputs);
}

::opencompgraph::NodeStatus NodeImpl::compute(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> &inputs, ::opencompgraph::internal::StreamDataImplShared &output) noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$compute(*this, inputs, output);
}

::opencompgraph::AttrState NodeImpl::attr_exists(::rust::Str name) const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$attr_exists(*this, ::rust::impl<::rust::Str>::repr(name));
}

float NodeImpl::get_attr_f32(::rust::Str name) const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_f32(*this, ::rust::impl<::rust::Str>::repr(name));
}

::std::int32_t NodeImpl::get_attr_i32(::rust::Str name) const noexcept {
  return opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_i32(*this, ::rust::impl<::rust::Str>::repr(name));
}

::rust::Str NodeImpl::get_attr_string(::rust::Str name) const noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(opencompgraph$internal$cxxbridge1$NodeImpl$get_attr_string(*this, ::rust::impl<::rust::Str>::repr(name)));
}

void NodeImpl::set_attr(::rust::Str name, float value) noexcept {
  opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_f32(*this, ::rust::impl<::rust::Str>::repr(name), value);
}

void NodeImpl::set_attr(::rust::Str name, ::std::int32_t value) noexcept {
  opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_i32(*this, ::rust::impl<::rust::Str>::repr(name), value);
}

void NodeImpl::set_attr(::rust::Str name, ::rust::Str value) noexcept {
  opencompgraph$internal$cxxbridge1$NodeImpl$set_attr_string(*this, ::rust::impl<::rust::Str>::repr(name), ::rust::impl<::rust::Str>::repr(value));
}

::std::size_t GraphImpl::add_op(::rust::Box<::opencompgraph::internal::NodeImpl> op_box) noexcept {
  return opencompgraph$internal$cxxbridge1$GraphImpl$add_op(*this, op_box.into_raw());
}

void GraphImpl::connect(::std::size_t src_index, ::std::size_t dst_index, ::std::uint8_t input_num) noexcept {
  opencompgraph$internal$cxxbridge1$GraphImpl$connect(*this, src_index, dst_index, input_num);
}

::opencompgraph::ExecuteStatus GraphImpl::execute(::std::size_t start_index) noexcept {
  return opencompgraph$internal$cxxbridge1$GraphImpl$execute(*this, start_index);
}

::rust::Box<::opencompgraph::internal::NodeImpl> create_node_box(::opencompgraph::NodeType node_type) noexcept {
  return ::rust::Box<::opencompgraph::internal::NodeImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_node_box(node_type));
}

::rust::Box<::opencompgraph::internal::NodeImpl> create_node_box(::opencompgraph::NodeType node_type, ::rust::Str name) noexcept {
  return ::rust::Box<::opencompgraph::internal::NodeImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_node_box_with_name(node_type, ::rust::impl<::rust::Str>::repr(name)));
}

::rust::Box<::opencompgraph::internal::NodeImpl> create_node_box(::opencompgraph::NodeType node_type, ::std::uint64_t id) noexcept {
  return ::rust::Box<::opencompgraph::internal::NodeImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_node_box_with_id(node_type, id));
}

::opencompgraph::internal::NodeImplShared create_node_shared(::opencompgraph::NodeType node_type) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::NodeImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_node_shared(node_type, &return$.value);
  return ::std::move(return$.value);
}

::opencompgraph::internal::NodeImplShared create_node_shared(::opencompgraph::NodeType node_type, ::rust::Str name) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::NodeImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_node_shared_with_name(node_type, ::rust::impl<::rust::Str>::repr(name), &return$.value);
  return ::std::move(return$.value);
}

::opencompgraph::internal::NodeImplShared create_node_shared(::opencompgraph::NodeType node_type, ::std::uint64_t id) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::NodeImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_node_shared_with_id(node_type, id, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::opencompgraph::internal::GraphImpl> create_graph_box() noexcept {
  return ::rust::Box<::opencompgraph::internal::GraphImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_graph_box());
}

::opencompgraph::internal::GraphImplShared create_graph_shared() noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::GraphImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_graph_shared(&return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::opencompgraph::internal::StreamDataImpl> create_stream_data_box() noexcept {
  return ::rust::Box<::opencompgraph::internal::StreamDataImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_stream_data_box());
}

::opencompgraph::internal::StreamDataImplShared create_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::StreamDataImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::opencompgraph::internal::StreamDataImplShared> create_vec_stream_data_shared() noexcept {
  ::rust::MaybeUninit<::rust::Vec<::opencompgraph::internal::StreamDataImplShared>> return$;
  opencompgraph$internal$cxxbridge1$create_vec_stream_data_shared(&return$.value);
  return ::std::move(return$.value);
}
} // namespace internal
} // namespace opencompgraph

extern "C" {
#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$ThingR
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$ThingR
::opencompgraph::internal::ThingR *cxxbridge1$box$opencompgraph$internal$ThingR$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$ThingR$dealloc(::opencompgraph::internal::ThingR *) noexcept;
void cxxbridge1$box$opencompgraph$internal$ThingR$drop(::rust::Box<::opencompgraph::internal::ThingR> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$ThingR

#ifndef CXXBRIDGE1_UNIQUE_PTR_opencompgraph$cpp$ThingC
#define CXXBRIDGE1_UNIQUE_PTR_opencompgraph$cpp$ThingC
static_assert(::rust::is_complete<::opencompgraph::cpp::ThingC>::value, "definition of ThingC is required");
static_assert(sizeof(::std::unique_ptr<::opencompgraph::cpp::ThingC>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::opencompgraph::cpp::ThingC>) == alignof(void *), "");
void cxxbridge1$unique_ptr$opencompgraph$cpp$ThingC$null(::std::unique_ptr<::opencompgraph::cpp::ThingC> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::opencompgraph::cpp::ThingC>();
}
void cxxbridge1$unique_ptr$opencompgraph$cpp$ThingC$raw(::std::unique_ptr<::opencompgraph::cpp::ThingC> *ptr, ::opencompgraph::cpp::ThingC *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::opencompgraph::cpp::ThingC>(raw);
}
const ::opencompgraph::cpp::ThingC *cxxbridge1$unique_ptr$opencompgraph$cpp$ThingC$get(const ::std::unique_ptr<::opencompgraph::cpp::ThingC>& ptr) noexcept {
  return ptr.get();
}
::opencompgraph::cpp::ThingC *cxxbridge1$unique_ptr$opencompgraph$cpp$ThingC$release(::std::unique_ptr<::opencompgraph::cpp::ThingC>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$opencompgraph$cpp$ThingC$drop(::std::unique_ptr<::opencompgraph::cpp::ThingC> *ptr) noexcept {
  ::rust::deleter_if<::rust::is_complete<::opencompgraph::cpp::ThingC>::value>{}(ptr);
}
#endif // CXXBRIDGE1_UNIQUE_PTR_opencompgraph$cpp$ThingC

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$GraphImpl
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$GraphImpl
::opencompgraph::internal::GraphImpl *cxxbridge1$box$opencompgraph$internal$GraphImpl$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$GraphImpl$dealloc(::opencompgraph::internal::GraphImpl *) noexcept;
void cxxbridge1$box$opencompgraph$internal$GraphImpl$drop(::rust::Box<::opencompgraph::internal::GraphImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$GraphImpl

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$NodeImpl
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$NodeImpl
::opencompgraph::internal::NodeImpl *cxxbridge1$box$opencompgraph$internal$NodeImpl$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$NodeImpl$dealloc(::opencompgraph::internal::NodeImpl *) noexcept;
void cxxbridge1$box$opencompgraph$internal$NodeImpl$drop(::rust::Box<::opencompgraph::internal::NodeImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$NodeImpl

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$StreamDataImpl
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$StreamDataImpl
::opencompgraph::internal::StreamDataImpl *cxxbridge1$box$opencompgraph$internal$StreamDataImpl$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$StreamDataImpl$dealloc(::opencompgraph::internal::StreamDataImpl *) noexcept;
void cxxbridge1$box$opencompgraph$internal$StreamDataImpl$drop(::rust::Box<::opencompgraph::internal::StreamDataImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$StreamDataImpl

#ifndef CXXBRIDGE1_RUST_VEC_opencompgraph$internal$StreamDataImplShared
#define CXXBRIDGE1_RUST_VEC_opencompgraph$internal$StreamDataImplShared
void cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$new(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$drop(::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$len(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$capacity(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr) noexcept;
const ::opencompgraph::internal::StreamDataImplShared *cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$data(const ::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$reserve_total(::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr, ::std::size_t cap) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$set_len(::rust::Vec<::opencompgraph::internal::StreamDataImplShared> *ptr, ::std::size_t len) noexcept;
::std::size_t cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$stride() noexcept;
#endif // CXXBRIDGE1_RUST_VEC_opencompgraph$internal$StreamDataImplShared

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock
::opencompgraph::internal::PixelBlock *cxxbridge1$box$opencompgraph$internal$PixelBlock$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$PixelBlock$dealloc(::opencompgraph::internal::PixelBlock *) noexcept;
void cxxbridge1$box$opencompgraph$internal$PixelBlock$drop(::rust::Box<::opencompgraph::internal::PixelBlock> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D
::opencompgraph::internal::BoundingBox2D *cxxbridge1$box$opencompgraph$internal$BoundingBox2D$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$BoundingBox2D$dealloc(::opencompgraph::internal::BoundingBox2D *) noexcept;
void cxxbridge1$box$opencompgraph$internal$BoundingBox2D$drop(::rust::Box<::opencompgraph::internal::BoundingBox2D> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4
::opencompgraph::internal::Matrix4 *cxxbridge1$box$opencompgraph$internal$Matrix4$alloc() noexcept;
void cxxbridge1$box$opencompgraph$internal$Matrix4$dealloc(::opencompgraph::internal::Matrix4 *) noexcept;
void cxxbridge1$box$opencompgraph$internal$Matrix4$drop(::rust::Box<::opencompgraph::internal::Matrix4> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::opencompgraph::internal::ThingR *Box<::opencompgraph::internal::ThingR>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$ThingR$alloc();
}
template <>
void Box<::opencompgraph::internal::ThingR>::allocation::dealloc(::opencompgraph::internal::ThingR *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$ThingR$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::ThingR>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$ThingR$drop(this);
}
template <>
::opencompgraph::internal::GraphImpl *Box<::opencompgraph::internal::GraphImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$GraphImpl$alloc();
}
template <>
void Box<::opencompgraph::internal::GraphImpl>::allocation::dealloc(::opencompgraph::internal::GraphImpl *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$GraphImpl$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::GraphImpl>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$GraphImpl$drop(this);
}
template <>
::opencompgraph::internal::NodeImpl *Box<::opencompgraph::internal::NodeImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$NodeImpl$alloc();
}
template <>
void Box<::opencompgraph::internal::NodeImpl>::allocation::dealloc(::opencompgraph::internal::NodeImpl *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$NodeImpl$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::NodeImpl>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$NodeImpl$drop(this);
}
template <>
::opencompgraph::internal::StreamDataImpl *Box<::opencompgraph::internal::StreamDataImpl>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$StreamDataImpl$alloc();
}
template <>
void Box<::opencompgraph::internal::StreamDataImpl>::allocation::dealloc(::opencompgraph::internal::StreamDataImpl *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$StreamDataImpl$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::StreamDataImpl>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$StreamDataImpl$drop(this);
}
template <>
Vec<::opencompgraph::internal::StreamDataImplShared>::Vec() noexcept {
  cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$new(this);
}
template <>
void Vec<::opencompgraph::internal::StreamDataImplShared>::drop() noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$drop(this);
}
template <>
::std::size_t Vec<::opencompgraph::internal::StreamDataImplShared>::size() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$len(this);
}
template <>
::std::size_t Vec<::opencompgraph::internal::StreamDataImplShared>::capacity() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$capacity(this);
}
template <>
const ::opencompgraph::internal::StreamDataImplShared *Vec<::opencompgraph::internal::StreamDataImplShared>::data() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$data(this);
}
template <>
void Vec<::opencompgraph::internal::StreamDataImplShared>::reserve_total(::std::size_t cap) noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$reserve_total(this, cap);
}
template <>
void Vec<::opencompgraph::internal::StreamDataImplShared>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$set_len(this, len);
}
template <>
::std::size_t Vec<::opencompgraph::internal::StreamDataImplShared>::stride() noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$StreamDataImplShared$stride();
}
template <>
::opencompgraph::internal::PixelBlock *Box<::opencompgraph::internal::PixelBlock>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$PixelBlock$alloc();
}
template <>
void Box<::opencompgraph::internal::PixelBlock>::allocation::dealloc(::opencompgraph::internal::PixelBlock *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$PixelBlock$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::PixelBlock>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$PixelBlock$drop(this);
}
template <>
::opencompgraph::internal::BoundingBox2D *Box<::opencompgraph::internal::BoundingBox2D>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$BoundingBox2D$alloc();
}
template <>
void Box<::opencompgraph::internal::BoundingBox2D>::allocation::dealloc(::opencompgraph::internal::BoundingBox2D *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$BoundingBox2D$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::BoundingBox2D>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$BoundingBox2D$drop(this);
}
template <>
::opencompgraph::internal::Matrix4 *Box<::opencompgraph::internal::Matrix4>::allocation::alloc() noexcept {
  return cxxbridge1$box$opencompgraph$internal$Matrix4$alloc();
}
template <>
void Box<::opencompgraph::internal::Matrix4>::allocation::dealloc(::opencompgraph::internal::Matrix4 *ptr) noexcept {
  cxxbridge1$box$opencompgraph$internal$Matrix4$dealloc(ptr);
}
template <>
void Box<::opencompgraph::internal::Matrix4>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$Matrix4$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
