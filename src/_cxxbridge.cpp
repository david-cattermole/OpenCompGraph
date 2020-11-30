#include "opencompgraph/_cpp.h"
#include "opencompgraph.h"
#include <cstddef>
#include <cstdint>
#include <exception>
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

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
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

#ifndef CXXBRIDGE1_RUST_ERROR
#define CXXBRIDGE1_RUST_ERROR
class Error final : public std::exception {
public:
  Error(const Error &);
  Error(Error &&) noexcept;
  ~Error() noexcept override;

  Error &operator=(const Error &);
  Error &operator=(Error &&) noexcept;

  const char *what() const noexcept override;

private:
  Error() noexcept = default;
  friend impl<Error>;
  const char *msg;
  size_t len;
};
#endif // CXXBRIDGE1_RUST_ERROR

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE

template <typename T>
union MaybeUninit {
  T value;
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
};

template <>
class impl<Error> final {
public:
  static Error error(repr::PtrLen repr) noexcept {
    Error error;
    error.msg = static_cast<const char *>(repr.ptr);
    error.len = repr.len;
    return error;
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

extern "C" {
__declspec(dllexport) ::opencompgraph::ThingC *opencompgraph$cxxbridge1$make_thingc(::rust::repr::PtrLen appname) noexcept {
  ::std::unique_ptr<::opencompgraph::ThingC> (*make_thingc$)(::rust::Str) = ::opencompgraph::make_thingc;
  return make_thingc$(::rust::impl<::rust::Str>::new_unchecked(appname)).release();
}

__declspec(dllexport) const ::std::string *opencompgraph$cxxbridge1$get_name(const ::opencompgraph::ThingC &thing) noexcept {
  const ::std::string &(*get_name$)(const ::opencompgraph::ThingC &) = ::opencompgraph::get_name;
  return &get_name$(thing);
}

__declspec(dllexport) void opencompgraph$cxxbridge1$run_sharedthing(::opencompgraph::SharedThing *state) noexcept {
  void (*run_sharedthing$)(::opencompgraph::SharedThing) = ::opencompgraph::run_sharedthing;
  run_sharedthing$(::std::move(*state));
}

void opencompgraph$cxxbridge1$print_r(const ::opencompgraph::ThingR &r) noexcept;

size_t opencompgraph$cxxbridge1$ReadImageOp$get_id(::opencompgraph::ReadImageOp &self) noexcept;

::opencompgraph::OperationType opencompgraph$cxxbridge1$ReadImageOp$get_op_type(::opencompgraph::ReadImageOp &self) noexcept;

uint8_t opencompgraph$cxxbridge1$ReadImageOp$get_op_type_id(::opencompgraph::ReadImageOp &self) noexcept;

::rust::repr::PtrLen opencompgraph$cxxbridge1$ReadImageOp$compute(::opencompgraph::ReadImageOp &self, bool *return$) noexcept;

::opencompgraph::ReadImageOp *opencompgraph$cxxbridge1$create_read_image_op(size_t id) noexcept;

size_t opencompgraph$cxxbridge1$WriteImageOp$get_id(::opencompgraph::WriteImageOp &self) noexcept;

::opencompgraph::OperationType opencompgraph$cxxbridge1$WriteImageOp$get_op_type(::opencompgraph::WriteImageOp &self) noexcept;

uint8_t opencompgraph$cxxbridge1$WriteImageOp$get_op_type_id(::opencompgraph::WriteImageOp &self) noexcept;

::rust::repr::PtrLen opencompgraph$cxxbridge1$WriteImageOp$compute(::opencompgraph::WriteImageOp &self, bool *return$) noexcept;

::opencompgraph::WriteImageOp *opencompgraph$cxxbridge1$create_write_image_op(size_t id) noexcept;
} // extern "C"

void print_r(const ::opencompgraph::ThingR &r) noexcept {
  opencompgraph$cxxbridge1$print_r(r);
}

size_t ReadImageOp::get_id() noexcept {
  return opencompgraph$cxxbridge1$ReadImageOp$get_id(*this);
}

::opencompgraph::OperationType ReadImageOp::get_op_type() noexcept {
  return opencompgraph$cxxbridge1$ReadImageOp$get_op_type(*this);
}

uint8_t ReadImageOp::get_op_type_id() noexcept {
  return opencompgraph$cxxbridge1$ReadImageOp$get_op_type_id(*this);
}

bool ReadImageOp::compute() {
  ::rust::MaybeUninit<bool> return$;
  ::rust::repr::PtrLen error$ = opencompgraph$cxxbridge1$ReadImageOp$compute(*this, &return$.value);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
  return ::std::move(return$.value);
}

::rust::Box<::opencompgraph::ReadImageOp> create_read_image_op(size_t id) noexcept {
  return ::rust::Box<::opencompgraph::ReadImageOp>::from_raw(opencompgraph$cxxbridge1$create_read_image_op(id));
}

size_t WriteImageOp::get_id() noexcept {
  return opencompgraph$cxxbridge1$WriteImageOp$get_id(*this);
}

::opencompgraph::OperationType WriteImageOp::get_op_type() noexcept {
  return opencompgraph$cxxbridge1$WriteImageOp$get_op_type(*this);
}

uint8_t WriteImageOp::get_op_type_id() noexcept {
  return opencompgraph$cxxbridge1$WriteImageOp$get_op_type_id(*this);
}

bool WriteImageOp::compute() {
  ::rust::MaybeUninit<bool> return$;
  ::rust::repr::PtrLen error$ = opencompgraph$cxxbridge1$WriteImageOp$compute(*this, &return$.value);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
  return ::std::move(return$.value);
}

::rust::Box<::opencompgraph::WriteImageOp> create_write_image_op(size_t id) noexcept {
  return ::rust::Box<::opencompgraph::WriteImageOp>::from_raw(opencompgraph$cxxbridge1$create_write_image_op(id));
}
} // namespace opencompgraph

extern "C" {
#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$ThingR
#define CXXBRIDGE1_RUST_BOX_opencompgraph$ThingR
void cxxbridge1$box$opencompgraph$ThingR$uninit(::rust::Box<::opencompgraph::ThingR> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$ThingR$drop(::rust::Box<::opencompgraph::ThingR> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$ThingR

#ifndef CXXBRIDGE1_UNIQUE_PTR_opencompgraph$ThingC
#define CXXBRIDGE1_UNIQUE_PTR_opencompgraph$ThingC
static_assert(::rust::is_complete<::opencompgraph::ThingC>::value, "definition of ThingC is required");
static_assert(sizeof(::std::unique_ptr<::opencompgraph::ThingC>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::opencompgraph::ThingC>) == alignof(void *), "");
void cxxbridge1$unique_ptr$opencompgraph$ThingC$null(::std::unique_ptr<::opencompgraph::ThingC> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<::opencompgraph::ThingC>();
}
void cxxbridge1$unique_ptr$opencompgraph$ThingC$raw(::std::unique_ptr<::opencompgraph::ThingC> *ptr, ::opencompgraph::ThingC *raw) noexcept {
  new (ptr) ::std::unique_ptr<::opencompgraph::ThingC>(raw);
}
const ::opencompgraph::ThingC *cxxbridge1$unique_ptr$opencompgraph$ThingC$get(const ::std::unique_ptr<::opencompgraph::ThingC>& ptr) noexcept {
  return ptr.get();
}
::opencompgraph::ThingC *cxxbridge1$unique_ptr$opencompgraph$ThingC$release(::std::unique_ptr<::opencompgraph::ThingC>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$opencompgraph$ThingC$drop(::std::unique_ptr<::opencompgraph::ThingC> *ptr) noexcept {
  ::rust::deleter_if<::rust::is_complete<::opencompgraph::ThingC>::value>{}(ptr);
}
#endif // CXXBRIDGE1_UNIQUE_PTR_opencompgraph$ThingC

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$ReadImageOp
#define CXXBRIDGE1_RUST_BOX_opencompgraph$ReadImageOp
void cxxbridge1$box$opencompgraph$ReadImageOp$uninit(::rust::Box<::opencompgraph::ReadImageOp> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$ReadImageOp$drop(::rust::Box<::opencompgraph::ReadImageOp> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$ReadImageOp

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$WriteImageOp
#define CXXBRIDGE1_RUST_BOX_opencompgraph$WriteImageOp
void cxxbridge1$box$opencompgraph$WriteImageOp$uninit(::rust::Box<::opencompgraph::WriteImageOp> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$WriteImageOp$drop(::rust::Box<::opencompgraph::WriteImageOp> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$WriteImageOp
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
void Box<::opencompgraph::ThingR>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$ThingR$uninit(this);
}
template <>
void Box<::opencompgraph::ThingR>::drop() noexcept {
  cxxbridge1$box$opencompgraph$ThingR$drop(this);
}
template <>
void Box<::opencompgraph::ReadImageOp>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$ReadImageOp$uninit(this);
}
template <>
void Box<::opencompgraph::ReadImageOp>::drop() noexcept {
  cxxbridge1$box$opencompgraph$ReadImageOp$drop(this);
}
template <>
void Box<::opencompgraph::WriteImageOp>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$WriteImageOp$uninit(this);
}
template <>
void Box<::opencompgraph::WriteImageOp>::drop() noexcept {
  cxxbridge1$box$opencompgraph$WriteImageOp$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
