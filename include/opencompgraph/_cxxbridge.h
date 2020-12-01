#pragma once
#include "opencompgraph/_cpp.h"
#include "opencompgraph.h"
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
  using ThingC = ::opencompgraph::ThingC;
  struct ThingR;
  struct ReadImageOp;
  struct WriteImageOp;
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

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$ReadImageOp
#define CXXBRIDGE1_STRUCT_opencompgraph$ReadImageOp
struct ReadImageOp final : public ::rust::Opaque {
  size_t get_id() noexcept;
  ::opencompgraph::OperationType get_op_type() noexcept;
  uint8_t get_op_type_id() noexcept;
  bool compute();
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$ReadImageOp

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$WriteImageOp
#define CXXBRIDGE1_STRUCT_opencompgraph$WriteImageOp
struct WriteImageOp final : public ::rust::Opaque {
  size_t get_id() noexcept;
  ::opencompgraph::OperationType get_op_type() noexcept;
  uint8_t get_op_type_id() noexcept;
  bool compute();
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$WriteImageOp

void print_r(const ::opencompgraph::ThingR &r) noexcept;

::rust::Box<::opencompgraph::ReadImageOp> create_read_image_op(size_t id) noexcept;

::rust::Box<::opencompgraph::WriteImageOp> create_write_image_op(size_t id) noexcept;
} // namespace opencompgraph
