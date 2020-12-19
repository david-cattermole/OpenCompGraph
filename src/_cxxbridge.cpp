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
  Str(const char *, size_t);

  Str &operator=(const Str &) noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  size_t size() const noexcept;
  size_t length() const noexcept;

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
  size_t len;
};

inline const char *Str::data() const noexcept { return this->ptr; }

inline size_t Str::size() const noexcept { return this->len; }

inline size_t Str::length() const noexcept { return this->len; }
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
  Box() noexcept;
  void uninit() noexcept;
  void drop() noexcept;
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
  this->uninit();
  ::new (this->ptr) T(val);
}

template <typename T>
Box<T>::Box(T &&val) {
  this->uninit();
  ::new (this->ptr) T(std::move(val));
}

template <typename T>
Box<T>::~Box() noexcept {
  if (this->ptr) {
    this->drop();
  }
}

template <typename T>
Box<T> &Box<T>::operator=(const Box &other) {
  if (this != &other) {
    if (this->ptr) {
      **this = *other;
    } else {
      this->uninit();
      ::new (this->ptr) T(*other);
    }
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
  Box box;
  box.uninit();
  ::new (box.ptr) T{std::forward<Fields>(fields)...};
  return box;
}

template <typename T>
Box<T> Box<T>::from_raw(T *raw) noexcept {
  Box box;
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
Box<T>::Box() noexcept = default;
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

  size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  size_t capacity() const noexcept;

  const T &operator[](size_t n) const noexcept;
  const T &at(size_t n) const;
  const T &front() const;
  const T &back() const;

  T &operator[](size_t n) noexcept;
  T &at(size_t n);
  T &front();
  T &back();

  void reserve(size_t new_cap);
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
  static size_t stride() noexcept;
  void reserve_total(size_t cap) noexcept;
  void set_len(size_t len) noexcept;
  void drop() noexcept;

  std::array<uintptr_t, 3> repr;
};

template <typename T>
class Vec<T>::iterator final {
public:
  using iterator_category = std::random_access_iterator_tag;
  using value_type = T;
  using difference_type = ptrdiff_t;
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
  size_t stride;
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
const T &Vec<T>::operator[](size_t n) const noexcept {
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * this->stride());
}

template <typename T>
const T &Vec<T>::at(size_t n) const {
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
T &Vec<T>::operator[](size_t n) noexcept {
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * this->stride());
}

template <typename T>
T &Vec<T>::at(size_t n) {
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
void Vec<T>::reserve(size_t new_cap) {
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
  void *operator()(size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
namespace repr {
struct PtrLen final {
  void *ptr;
  size_t len;
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

template <typename T, typename = size_t>
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
  enum class ExecuteStatus : uint8_t;
  enum class OperationType : uint8_t;
  enum class OperationStatus : uint8_t;
  enum class AttrState : uint8_t;
  enum class OutputState : uint8_t;
  namespace shared {
    struct SharedThing;
  }
  namespace internal {
    struct GraphImplShared;
    struct OperationImplShared;
    struct ThingR;
    struct PixelBlock;
    struct BoundingBox2D;
    struct Matrix4;
    struct Output;
    struct OperationImpl;
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
  int32_t z;
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

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImplShared
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImplShared
struct OperationImplShared final {
  ::rust::Box<::opencompgraph::internal::OperationImpl> inner;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImplShared
} // namespace internal

#ifndef CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus
#define CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus
enum class ExecuteStatus : uint8_t {
  Error = 0,
  Success = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$ExecuteStatus

#ifndef CXXBRIDGE1_ENUM_opencompgraph$OperationType
#define CXXBRIDGE1_ENUM_opencompgraph$OperationType
enum class OperationType : uint8_t {
  Null = 0,
  ReadImage = 1,
  WriteImage = 2,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$OperationType

#ifndef CXXBRIDGE1_ENUM_opencompgraph$OperationStatus
#define CXXBRIDGE1_ENUM_opencompgraph$OperationStatus
enum class OperationStatus : uint8_t {
  Error = 0,
  Warning = 1,
  Valid = 2,
  Uninitialized = 3,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$OperationStatus

#ifndef CXXBRIDGE1_ENUM_opencompgraph$AttrState
#define CXXBRIDGE1_ENUM_opencompgraph$AttrState
enum class AttrState : uint8_t {
  Missing = 0,
  Exists = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$AttrState

#ifndef CXXBRIDGE1_ENUM_opencompgraph$OutputState
#define CXXBRIDGE1_ENUM_opencompgraph$OutputState
enum class OutputState : uint8_t {
  Invalid = 0,
  Valid = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$OutputState

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$Output
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$Output
struct Output final : public ::rust::Opaque {
  size_t get_hash() const noexcept;
  const ::rust::Box<::opencompgraph::internal::PixelBlock> &get_pixel_block() const noexcept;
  const ::rust::Box<::opencompgraph::internal::BoundingBox2D> &get_bounding_box() const noexcept;
  const ::rust::Box<::opencompgraph::internal::Matrix4> &get_color_matrix() const noexcept;
  const ::rust::Box<::opencompgraph::internal::Matrix4> &get_transform_matrix() const noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$Output

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl
struct OperationImpl final : public ::rust::Opaque {
  uint64_t get_id() const noexcept;
  ::opencompgraph::OperationType get_op_type() const noexcept;
  uint8_t get_op_type_id() const noexcept;
  ::opencompgraph::OperationStatus get_status() const noexcept;
  uint8_t get_status_id() const noexcept;
  size_t hash(const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept;
  ::opencompgraph::OperationStatus compute(const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept;
  ::opencompgraph::AttrState attr_exists(::rust::Str name) const noexcept;
  ::rust::Str get_attr_string(::rust::Str name) const noexcept;
  void set_attr(::rust::Str name, ::rust::Str value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
struct GraphImpl final : public ::rust::Opaque {
  size_t add_op(::rust::Box<::opencompgraph::internal::OperationImpl> op_box) noexcept;
  void connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept;
  ::opencompgraph::ExecuteStatus execute(size_t start_index) noexcept;
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

size_t opencompgraph$internal$cxxbridge1$Output$get_hash(const ::opencompgraph::internal::Output &self) noexcept;

const ::rust::Box<::opencompgraph::internal::PixelBlock> *opencompgraph$internal$cxxbridge1$Output$get_pixel_block(const ::opencompgraph::internal::Output &self) noexcept;

const ::rust::Box<::opencompgraph::internal::BoundingBox2D> *opencompgraph$internal$cxxbridge1$Output$get_bounding_box(const ::opencompgraph::internal::Output &self) noexcept;

const ::rust::Box<::opencompgraph::internal::Matrix4> *opencompgraph$internal$cxxbridge1$Output$get_color_matrix(const ::opencompgraph::internal::Output &self) noexcept;

const ::rust::Box<::opencompgraph::internal::Matrix4> *opencompgraph$internal$cxxbridge1$Output$get_transform_matrix(const ::opencompgraph::internal::Output &self) noexcept;

uint64_t opencompgraph$internal$cxxbridge1$OperationImpl$get_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::OperationType opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type(const ::opencompgraph::internal::OperationImpl &self) noexcept;

uint8_t opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::OperationStatus opencompgraph$internal$cxxbridge1$OperationImpl$get_status(const ::opencompgraph::internal::OperationImpl &self) noexcept;

uint8_t opencompgraph$internal$cxxbridge1$OperationImpl$get_status_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

size_t opencompgraph$internal$cxxbridge1$OperationImpl$hash(::opencompgraph::internal::OperationImpl &self, const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept;

::opencompgraph::OperationStatus opencompgraph$internal$cxxbridge1$OperationImpl$compute(::opencompgraph::internal::OperationImpl &self, const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept;

::opencompgraph::AttrState opencompgraph$internal$cxxbridge1$OperationImpl$attr_exists(const ::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name) noexcept;

::rust::repr::PtrLen opencompgraph$internal$cxxbridge1$OperationImpl$get_attr_string(const ::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name) noexcept;

void opencompgraph$internal$cxxbridge1$OperationImpl$set_attr_string(::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name, ::rust::repr::PtrLen value) noexcept;

size_t opencompgraph$internal$cxxbridge1$GraphImpl$add_op(::opencompgraph::internal::GraphImpl &self, ::opencompgraph::internal::OperationImpl *op_box) noexcept;

void opencompgraph$internal$cxxbridge1$GraphImpl$connect(::opencompgraph::internal::GraphImpl &self, size_t src_index, size_t dst_index, uint8_t input_num) noexcept;

::opencompgraph::ExecuteStatus opencompgraph$internal$cxxbridge1$GraphImpl$execute(::opencompgraph::internal::GraphImpl &self, size_t start_index) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box(::opencompgraph::OperationType op_type) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box_with_name(::opencompgraph::OperationType op_type, ::rust::repr::PtrLen name) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box_with_id(::opencompgraph::OperationType op_type, uint64_t id) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared(::opencompgraph::OperationType op_type, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared_with_name(::opencompgraph::OperationType op_type, ::rust::repr::PtrLen name, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared_with_id(::opencompgraph::OperationType op_type, uint64_t id, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

::opencompgraph::internal::GraphImpl *opencompgraph$internal$cxxbridge1$create_graph_box() noexcept;

void opencompgraph$internal$cxxbridge1$create_graph_shared(::opencompgraph::internal::GraphImplShared *return$) noexcept;
} // extern "C"

void print_r(const ::opencompgraph::internal::ThingR &r) noexcept {
  opencompgraph$internal$cxxbridge1$print_r(r);
}

size_t Output::get_hash() const noexcept {
  return opencompgraph$internal$cxxbridge1$Output$get_hash(*this);
}

const ::rust::Box<::opencompgraph::internal::PixelBlock> &Output::get_pixel_block() const noexcept {
  return *opencompgraph$internal$cxxbridge1$Output$get_pixel_block(*this);
}

const ::rust::Box<::opencompgraph::internal::BoundingBox2D> &Output::get_bounding_box() const noexcept {
  return *opencompgraph$internal$cxxbridge1$Output$get_bounding_box(*this);
}

const ::rust::Box<::opencompgraph::internal::Matrix4> &Output::get_color_matrix() const noexcept {
  return *opencompgraph$internal$cxxbridge1$Output$get_color_matrix(*this);
}

const ::rust::Box<::opencompgraph::internal::Matrix4> &Output::get_transform_matrix() const noexcept {
  return *opencompgraph$internal$cxxbridge1$Output$get_transform_matrix(*this);
}

uint64_t OperationImpl::get_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$get_id(*this);
}

::opencompgraph::OperationType OperationImpl::get_op_type() const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type(*this);
}

uint8_t OperationImpl::get_op_type_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type_id(*this);
}

::opencompgraph::OperationStatus OperationImpl::get_status() const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$get_status(*this);
}

uint8_t OperationImpl::get_status_id() const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$get_status_id(*this);
}

size_t OperationImpl::hash(const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$hash(*this, inputs);
}

::opencompgraph::OperationStatus OperationImpl::compute(const ::rust::Vec<::opencompgraph::internal::Output> &inputs) noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$compute(*this, inputs);
}

::opencompgraph::AttrState OperationImpl::attr_exists(::rust::Str name) const noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$attr_exists(*this, ::rust::impl<::rust::Str>::repr(name));
}

::rust::Str OperationImpl::get_attr_string(::rust::Str name) const noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(opencompgraph$internal$cxxbridge1$OperationImpl$get_attr_string(*this, ::rust::impl<::rust::Str>::repr(name)));
}

void OperationImpl::set_attr(::rust::Str name, ::rust::Str value) noexcept {
  opencompgraph$internal$cxxbridge1$OperationImpl$set_attr_string(*this, ::rust::impl<::rust::Str>::repr(name), ::rust::impl<::rust::Str>::repr(value));
}

size_t GraphImpl::add_op(::rust::Box<::opencompgraph::internal::OperationImpl> op_box) noexcept {
  return opencompgraph$internal$cxxbridge1$GraphImpl$add_op(*this, op_box.into_raw());
}

void GraphImpl::connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept {
  opencompgraph$internal$cxxbridge1$GraphImpl$connect(*this, src_index, dst_index, input_num);
}

::opencompgraph::ExecuteStatus GraphImpl::execute(size_t start_index) noexcept {
  return opencompgraph$internal$cxxbridge1$GraphImpl$execute(*this, start_index);
}

::rust::Box<::opencompgraph::internal::OperationImpl> create_operation_box(::opencompgraph::OperationType op_type) noexcept {
  return ::rust::Box<::opencompgraph::internal::OperationImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_operation_box(op_type));
}

::rust::Box<::opencompgraph::internal::OperationImpl> create_operation_box(::opencompgraph::OperationType op_type, ::rust::Str name) noexcept {
  return ::rust::Box<::opencompgraph::internal::OperationImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_operation_box_with_name(op_type, ::rust::impl<::rust::Str>::repr(name)));
}

::rust::Box<::opencompgraph::internal::OperationImpl> create_operation_box(::opencompgraph::OperationType op_type, uint64_t id) noexcept {
  return ::rust::Box<::opencompgraph::internal::OperationImpl>::from_raw(opencompgraph$internal$cxxbridge1$create_operation_box_with_id(op_type, id));
}

::opencompgraph::internal::OperationImplShared create_operation_shared(::opencompgraph::OperationType op_type) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::OperationImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_operation_shared(op_type, &return$.value);
  return ::std::move(return$.value);
}

::opencompgraph::internal::OperationImplShared create_operation_shared(::opencompgraph::OperationType op_type, ::rust::Str name) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::OperationImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_operation_shared_with_name(op_type, ::rust::impl<::rust::Str>::repr(name), &return$.value);
  return ::std::move(return$.value);
}

::opencompgraph::internal::OperationImplShared create_operation_shared(::opencompgraph::OperationType op_type, uint64_t id) noexcept {
  ::rust::MaybeUninit<::opencompgraph::internal::OperationImplShared> return$;
  opencompgraph$internal$cxxbridge1$create_operation_shared_with_id(op_type, id, &return$.value);
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
} // namespace internal
} // namespace opencompgraph

extern "C" {
#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$ThingR
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$ThingR
void cxxbridge1$box$opencompgraph$internal$ThingR$uninit(::rust::Box<::opencompgraph::internal::ThingR> *ptr) noexcept;
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
void cxxbridge1$box$opencompgraph$internal$GraphImpl$uninit(::rust::Box<::opencompgraph::internal::GraphImpl> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$internal$GraphImpl$drop(::rust::Box<::opencompgraph::internal::GraphImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$GraphImpl

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$OperationImpl
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$OperationImpl
void cxxbridge1$box$opencompgraph$internal$OperationImpl$uninit(::rust::Box<::opencompgraph::internal::OperationImpl> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$internal$OperationImpl$drop(::rust::Box<::opencompgraph::internal::OperationImpl> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$OperationImpl

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock
void cxxbridge1$box$opencompgraph$internal$PixelBlock$uninit(::rust::Box<::opencompgraph::internal::PixelBlock> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$internal$PixelBlock$drop(::rust::Box<::opencompgraph::internal::PixelBlock> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$PixelBlock

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D
void cxxbridge1$box$opencompgraph$internal$BoundingBox2D$uninit(::rust::Box<::opencompgraph::internal::BoundingBox2D> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$internal$BoundingBox2D$drop(::rust::Box<::opencompgraph::internal::BoundingBox2D> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$BoundingBox2D

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4
#define CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4
void cxxbridge1$box$opencompgraph$internal$Matrix4$uninit(::rust::Box<::opencompgraph::internal::Matrix4> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$internal$Matrix4$drop(::rust::Box<::opencompgraph::internal::Matrix4> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$internal$Matrix4

#ifndef CXXBRIDGE1_RUST_VEC_opencompgraph$internal$Output
#define CXXBRIDGE1_RUST_VEC_opencompgraph$internal$Output
void cxxbridge1$rust_vec$opencompgraph$internal$Output$new(const ::rust::Vec<::opencompgraph::internal::Output> *ptr) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$Output$drop(::rust::Vec<::opencompgraph::internal::Output> *ptr) noexcept;
size_t cxxbridge1$rust_vec$opencompgraph$internal$Output$len(const ::rust::Vec<::opencompgraph::internal::Output> *ptr) noexcept;
size_t cxxbridge1$rust_vec$opencompgraph$internal$Output$capacity(const ::rust::Vec<::opencompgraph::internal::Output> *ptr) noexcept;
const ::opencompgraph::internal::Output *cxxbridge1$rust_vec$opencompgraph$internal$Output$data(const ::rust::Vec<::opencompgraph::internal::Output> *ptr) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$Output$reserve_total(::rust::Vec<::opencompgraph::internal::Output> *ptr, size_t cap) noexcept;
void cxxbridge1$rust_vec$opencompgraph$internal$Output$set_len(::rust::Vec<::opencompgraph::internal::Output> *ptr, size_t len) noexcept;
size_t cxxbridge1$rust_vec$opencompgraph$internal$Output$stride() noexcept;
#endif // CXXBRIDGE1_RUST_VEC_opencompgraph$internal$Output
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
void Box<::opencompgraph::internal::ThingR>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$ThingR$uninit(this);
}
template <>
void Box<::opencompgraph::internal::ThingR>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$ThingR$drop(this);
}
template <>
void Box<::opencompgraph::internal::GraphImpl>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$GraphImpl$uninit(this);
}
template <>
void Box<::opencompgraph::internal::GraphImpl>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$GraphImpl$drop(this);
}
template <>
void Box<::opencompgraph::internal::OperationImpl>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$OperationImpl$uninit(this);
}
template <>
void Box<::opencompgraph::internal::OperationImpl>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$OperationImpl$drop(this);
}
template <>
void Box<::opencompgraph::internal::PixelBlock>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$PixelBlock$uninit(this);
}
template <>
void Box<::opencompgraph::internal::PixelBlock>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$PixelBlock$drop(this);
}
template <>
void Box<::opencompgraph::internal::BoundingBox2D>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$BoundingBox2D$uninit(this);
}
template <>
void Box<::opencompgraph::internal::BoundingBox2D>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$BoundingBox2D$drop(this);
}
template <>
void Box<::opencompgraph::internal::Matrix4>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$internal$Matrix4$uninit(this);
}
template <>
void Box<::opencompgraph::internal::Matrix4>::drop() noexcept {
  cxxbridge1$box$opencompgraph$internal$Matrix4$drop(this);
}
template <>
Vec<::opencompgraph::internal::Output>::Vec() noexcept {
  cxxbridge1$rust_vec$opencompgraph$internal$Output$new(this);
}
template <>
void Vec<::opencompgraph::internal::Output>::drop() noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$drop(this);
}
template <>
size_t Vec<::opencompgraph::internal::Output>::size() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$len(this);
}
template <>
size_t Vec<::opencompgraph::internal::Output>::capacity() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$capacity(this);
}
template <>
const ::opencompgraph::internal::Output *Vec<::opencompgraph::internal::Output>::data() const noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$data(this);
}
template <>
void Vec<::opencompgraph::internal::Output>::reserve_total(size_t cap) noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$reserve_total(this, cap);
}
template <>
void Vec<::opencompgraph::internal::Output>::set_len(size_t len) noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$set_len(this, len);
}
template <>
size_t Vec<::opencompgraph::internal::Output>::stride() noexcept {
  return cxxbridge1$rust_vec$opencompgraph$internal$Output$stride();
}
} // namespace cxxbridge1
} // namespace rust
