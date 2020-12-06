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
  static repr::PtrLen repr(Str str) noexcept {
    return repr::PtrLen{const_cast<char *>(str.ptr), str.len};
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
  enum class AttrState : uint8_t;
  enum class AttrDataType : uint8_t;
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

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$Operation
#define CXXBRIDGE1_STRUCT_opencompgraph$Operation
struct Operation final : public ::rust::Opaque {
  size_t get_id() const noexcept;
  ::opencompgraph::OperationType get_op_type() const noexcept;
  uint8_t get_op_type_id() const noexcept;
  bool compute();
  ::opencompgraph::AttrState attr_exists(::rust::Str name) const noexcept;
  ::rust::Str get_attr_string(::rust::Str name) const noexcept;
  void set_attr(::rust::Str name, ::rust::Str value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$Operation

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

size_t opencompgraph$cxxbridge1$Operation$get_id(const ::opencompgraph::Operation &self) noexcept;

::opencompgraph::OperationType opencompgraph$cxxbridge1$Operation$get_op_type(const ::opencompgraph::Operation &self) noexcept;

uint8_t opencompgraph$cxxbridge1$Operation$get_op_type_id(const ::opencompgraph::Operation &self) noexcept;

::rust::repr::PtrLen opencompgraph$cxxbridge1$Operation$compute(::opencompgraph::Operation &self, bool *return$) noexcept;

::opencompgraph::AttrState opencompgraph$cxxbridge1$Operation$attr_exists(const ::opencompgraph::Operation &self, ::rust::repr::PtrLen name) noexcept;

::rust::repr::PtrLen opencompgraph$cxxbridge1$Operation$get_attr_string(const ::opencompgraph::Operation &self, ::rust::repr::PtrLen name) noexcept;

void opencompgraph$cxxbridge1$Operation$set_attr_string(::opencompgraph::Operation &self, ::rust::repr::PtrLen name, ::rust::repr::PtrLen value) noexcept;

::opencompgraph::Operation *opencompgraph$cxxbridge1$create_operation(size_t id, ::opencompgraph::OperationType op_type) noexcept;
} // extern "C"

void print_r(const ::opencompgraph::ThingR &r) noexcept {
  opencompgraph$cxxbridge1$print_r(r);
}

size_t Operation::get_id() const noexcept {
  return opencompgraph$cxxbridge1$Operation$get_id(*this);
}

::opencompgraph::OperationType Operation::get_op_type() const noexcept {
  return opencompgraph$cxxbridge1$Operation$get_op_type(*this);
}

uint8_t Operation::get_op_type_id() const noexcept {
  return opencompgraph$cxxbridge1$Operation$get_op_type_id(*this);
}

bool Operation::compute() {
  ::rust::MaybeUninit<bool> return$;
  ::rust::repr::PtrLen error$ = opencompgraph$cxxbridge1$Operation$compute(*this, &return$.value);
  if (error$.ptr) {
    throw ::rust::impl<::rust::Error>::error(error$);
  }
  return ::std::move(return$.value);
}

::opencompgraph::AttrState Operation::attr_exists(::rust::Str name) const noexcept {
  return opencompgraph$cxxbridge1$Operation$attr_exists(*this, ::rust::impl<::rust::Str>::repr(name));
}

::rust::Str Operation::get_attr_string(::rust::Str name) const noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(opencompgraph$cxxbridge1$Operation$get_attr_string(*this, ::rust::impl<::rust::Str>::repr(name)));
}

void Operation::set_attr(::rust::Str name, ::rust::Str value) noexcept {
  opencompgraph$cxxbridge1$Operation$set_attr_string(*this, ::rust::impl<::rust::Str>::repr(name), ::rust::impl<::rust::Str>::repr(value));
}

::rust::Box<::opencompgraph::Operation> create_operation(size_t id, ::opencompgraph::OperationType op_type) noexcept {
  return ::rust::Box<::opencompgraph::Operation>::from_raw(opencompgraph$cxxbridge1$create_operation(id, op_type));
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

#ifndef CXXBRIDGE1_RUST_BOX_opencompgraph$Operation
#define CXXBRIDGE1_RUST_BOX_opencompgraph$Operation
void cxxbridge1$box$opencompgraph$Operation$uninit(::rust::Box<::opencompgraph::Operation> *ptr) noexcept;
void cxxbridge1$box$opencompgraph$Operation$drop(::rust::Box<::opencompgraph::Operation> *ptr) noexcept;
#endif // CXXBRIDGE1_RUST_BOX_opencompgraph$Operation
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
void Box<::opencompgraph::Operation>::uninit() noexcept {
  cxxbridge1$box$opencompgraph$Operation$uninit(this);
}
template <>
void Box<::opencompgraph::Operation>::drop() noexcept {
  cxxbridge1$box$opencompgraph$Operation$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
