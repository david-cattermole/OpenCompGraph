#pragma once
#include "opencompgraph/_cpp.h"
#include "opencompgraph.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

#ifndef CXXBRIDGE1_RUST_STRING
#define CXXBRIDGE1_RUST_STRING
class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  String(const std::string &);
  String(const char *);
  String(const char *, size_t);

  String &operator=(const String &) noexcept;
  String &operator=(String &&) noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  size_t size() const noexcept;
  size_t length() const noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  std::array<uintptr_t, 3> repr;
};
#endif // CXXBRIDGE1_RUST_STRING

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

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE
} // namespace cxxbridge1
} // namespace rust

namespace opencompgraph {
  struct SharedThing;
  enum class OperationType : uint8_t;
  enum class AttrState : uint8_t;
  enum class AttrDataType : uint8_t;
  enum class AttrId : uint8_t;
  using ThingC = ::opencompgraph::ThingC;
  struct ThingR;
  struct Operation;
}

namespace opencompgraph {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$SharedThing
#define CXXBRIDGE1_STRUCT_opencompgraph$SharedThing
struct SharedThing final {
  int32_t z;
  ::rust::Box<::opencompgraph::ThingR> y;
  ::std::unique_ptr<::opencompgraph::ThingC> x;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$SharedThing

#ifndef CXXBRIDGE1_ENUM_opencompgraph$OperationType
#define CXXBRIDGE1_ENUM_opencompgraph$OperationType
enum class OperationType : uint8_t {
  ReadImage = 0,
  WriteImage = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$OperationType

#ifndef CXXBRIDGE1_ENUM_opencompgraph$AttrState
#define CXXBRIDGE1_ENUM_opencompgraph$AttrState
enum class AttrState : uint8_t {
  Missing = 0,
  Exists = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$AttrState

#ifndef CXXBRIDGE1_ENUM_opencompgraph$AttrDataType
#define CXXBRIDGE1_ENUM_opencompgraph$AttrDataType
enum class AttrDataType : uint8_t {
  None = 0,
  UnsignedInteger8 = 1,
  UnsignedInteger16 = 2,
  UnsignedInteger32 = 3,
  UnsignedInteger64 = 4,
  SignedInteger8 = 5,
  SignedInteger16 = 6,
  SignedInteger32 = 7,
  SignedInteger64 = 8,
  Float32 = 9,
  Float64 = 10,
  String = 11,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$AttrDataType

#ifndef CXXBRIDGE1_ENUM_opencompgraph$AttrId
#define CXXBRIDGE1_ENUM_opencompgraph$AttrId
enum class AttrId : uint8_t {
  ReadImage_FilePath = 0,
  WriteImage_FilePath = 1,
};
#endif // CXXBRIDGE1_ENUM_opencompgraph$AttrId

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$Operation
#define CXXBRIDGE1_STRUCT_opencompgraph$Operation
struct Operation final : public ::rust::Opaque {
  size_t get_id() const noexcept;
  ::opencompgraph::OperationType get_op_type() const noexcept;
  uint8_t get_op_type_id() const noexcept;
  bool compute();
  ::opencompgraph::AttrState attr_exists(::opencompgraph::AttrId attr) const noexcept;
  ::rust::String get_attr_string(::opencompgraph::AttrId attr) const noexcept;
  void set_attr(::opencompgraph::AttrId attr, ::rust::String value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$Operation

void print_r(const ::opencompgraph::ThingR &r) noexcept;

::rust::Box<::opencompgraph::Operation> create_operation(size_t id, ::opencompgraph::OperationType op_type) noexcept;
} // namespace opencompgraph
