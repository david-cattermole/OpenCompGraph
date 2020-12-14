#include "rust/cxx.h"
#include "opencompgraph/cpp.h"
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
  enum class OperationType : uint8_t;
  enum class OperationStatus : uint8_t;
  enum class AttrState : uint8_t;
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

namespace internal {
#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl
struct OperationImpl final : public ::rust::Opaque {
  uint64_t get_id() const noexcept;
  ::opencompgraph::OperationType get_op_type() const noexcept;
  uint8_t get_op_type_id() const noexcept;
  ::opencompgraph::OperationStatus get_status() const noexcept;
  uint8_t get_status_id() const noexcept;
  size_t hash() noexcept;
  ::opencompgraph::OperationStatus compute() noexcept;
  ::opencompgraph::AttrState attr_exists(::rust::Str name) const noexcept;
  ::rust::Str get_attr_string(::rust::Str name) const noexcept;
  void set_attr(::rust::Str name, ::rust::Str value) noexcept;
};
#endif // CXXBRIDGE1_STRUCT_opencompgraph$internal$OperationImpl

#ifndef CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
#define CXXBRIDGE1_STRUCT_opencompgraph$internal$GraphImpl
struct GraphImpl final : public ::rust::Opaque {
  size_t add_op(::rust::Box<::opencompgraph::internal::OperationImpl> op_box) noexcept;
  void connect(uint64_t src_op_id, uint64_t dst_op_id) noexcept;
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

uint64_t opencompgraph$internal$cxxbridge1$OperationImpl$get_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::OperationType opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type(const ::opencompgraph::internal::OperationImpl &self) noexcept;

uint8_t opencompgraph$internal$cxxbridge1$OperationImpl$get_op_type_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::OperationStatus opencompgraph$internal$cxxbridge1$OperationImpl$get_status(const ::opencompgraph::internal::OperationImpl &self) noexcept;

uint8_t opencompgraph$internal$cxxbridge1$OperationImpl$get_status_id(const ::opencompgraph::internal::OperationImpl &self) noexcept;

size_t opencompgraph$internal$cxxbridge1$OperationImpl$hash(::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::OperationStatus opencompgraph$internal$cxxbridge1$OperationImpl$compute(::opencompgraph::internal::OperationImpl &self) noexcept;

::opencompgraph::AttrState opencompgraph$internal$cxxbridge1$OperationImpl$attr_exists(const ::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name) noexcept;

::rust::repr::PtrLen opencompgraph$internal$cxxbridge1$OperationImpl$get_attr_string(const ::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name) noexcept;

void opencompgraph$internal$cxxbridge1$OperationImpl$set_attr_string(::opencompgraph::internal::OperationImpl &self, ::rust::repr::PtrLen name, ::rust::repr::PtrLen value) noexcept;

size_t opencompgraph$internal$cxxbridge1$GraphImpl$add_op(::opencompgraph::internal::GraphImpl &self, ::opencompgraph::internal::OperationImpl *op_box) noexcept;

void opencompgraph$internal$cxxbridge1$GraphImpl$connect(::opencompgraph::internal::GraphImpl &self, uint64_t src_op_id, uint64_t dst_op_id) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box(::opencompgraph::OperationType op_type) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box_with_name(::opencompgraph::OperationType op_type, ::rust::repr::PtrLen name) noexcept;

::opencompgraph::internal::OperationImpl *opencompgraph$internal$cxxbridge1$create_operation_box_with_id(::opencompgraph::OperationType op_type, uint64_t id) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared(::opencompgraph::OperationType op_type, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared_with_name(::opencompgraph::OperationType op_type, ::rust::repr::PtrLen name, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

void opencompgraph$internal$cxxbridge1$create_operation_shared_with_id(::opencompgraph::OperationType op_type, uint64_t id, ::opencompgraph::internal::OperationImplShared *return$) noexcept;

::opencompgraph::internal::GraphImpl *opencompgraph$internal$cxxbridge1$create_graph_box() noexcept;

void opencompgraph$internal$cxxbridge1$create_graph_shared(::opencompgraph::internal::GraphImplShared *return$) noexcept;

uint64_t opencompgraph$internal$cxxbridge1$generate_random_id() noexcept;

uint64_t opencompgraph$internal$cxxbridge1$generate_id_from_name(::rust::repr::PtrLen name) noexcept;
} // extern "C"

void print_r(const ::opencompgraph::internal::ThingR &r) noexcept {
  opencompgraph$internal$cxxbridge1$print_r(r);
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

size_t OperationImpl::hash() noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$hash(*this);
}

::opencompgraph::OperationStatus OperationImpl::compute() noexcept {
  return opencompgraph$internal$cxxbridge1$OperationImpl$compute(*this);
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

void GraphImpl::connect(uint64_t src_op_id, uint64_t dst_op_id) noexcept {
  opencompgraph$internal$cxxbridge1$GraphImpl$connect(*this, src_op_id, dst_op_id);
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

uint64_t generate_random_id() noexcept {
  return opencompgraph$internal$cxxbridge1$generate_random_id();
}

uint64_t generate_id_from_name(::rust::Str name) noexcept {
  return opencompgraph$internal$cxxbridge1$generate_id_from_name(::rust::impl<::rust::Str>::repr(name));
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
} // namespace cxxbridge1
} // namespace rust
