#pragma once
#include "rust/cxx.h"
#include "opencompgraph/symbol_export.h"
#include "opencompgraph/internal/colorspace.h"
#include "opencompgraph/internal/ldpk_utils.h"
#include "opencompgraph/internal/imageio.h"
#include "opencompgraph/internal/systemmemory.h"
#include <algorithm>
#include <array>
#include <cassert>
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

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

template <typename T>
::std::size_t size_of();
template <typename T>
::std::size_t align_of();

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
  String(const char *, std::size_t);

  String &operator=(const String &) & noexcept;
  String &operator=(String &&) & noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;

  const char *c_str() noexcept;

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

  void swap(String &) noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  friend void swap(String &lhs, String &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
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
  Str(const char *, std::size_t);

  Str &operator=(const Str &) & noexcept = default;

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

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_RUST_SLICE
#define CXXBRIDGE1_RUST_SLICE
namespace detail {
template <bool>
struct copy_assignable_if {};

template <>
struct copy_assignable_if<false> {
  copy_assignable_if() noexcept = default;
  copy_assignable_if(const copy_assignable_if &) noexcept = default;
  copy_assignable_if &operator=(const copy_assignable_if &) & noexcept = delete;
  copy_assignable_if &operator=(copy_assignable_if &&) & noexcept = default;
};
} // namespace detail

template <typename T>
class Slice final
    : private detail::copy_assignable_if<std::is_const<T>::value> {
public:
  using value_type = T;

  Slice() noexcept;
  Slice(T *, std::size_t count) noexcept;

  Slice &operator=(const Slice<T> &) & noexcept = default;
  Slice &operator=(Slice<T> &&) & noexcept = default;

  T *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  T &operator[](std::size_t n) const noexcept;
  T &at(std::size_t n) const;
  T &front() const noexcept;
  T &back() const noexcept;

  Slice(const Slice<T> &) noexcept = default;
  ~Slice() noexcept = default;

  class iterator;
  iterator begin() const noexcept;
  iterator end() const noexcept;

  void swap(Slice &) noexcept;

private:
  class uninit;
  Slice(uninit) noexcept;
  friend impl<Slice>;
  friend void sliceInit(void *, const void *, std::size_t) noexcept;
  friend void *slicePtr(const void *) noexcept;
  friend std::size_t sliceLen(const void *) noexcept;

  std::array<std::uintptr_t, 2> repr;
};

template <typename T>
class Slice<T>::iterator final {
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
  bool operator<=(const iterator &) const noexcept;
  bool operator>(const iterator &) const noexcept;
  bool operator>=(const iterator &) const noexcept;

private:
  friend class Slice;
  void *pos;
  std::size_t stride;
};

template <typename T>
Slice<T>::Slice() noexcept {
  sliceInit(this, reinterpret_cast<void *>(align_of<T>()), 0);
}

template <typename T>
Slice<T>::Slice(T *s, std::size_t count) noexcept {
  sliceInit(this, const_cast<typename std::remove_const<T>::type *>(s), count);
}

template <typename T>
T *Slice<T>::data() const noexcept {
  return reinterpret_cast<T *>(slicePtr(this));
}

template <typename T>
std::size_t Slice<T>::size() const noexcept {
  return sliceLen(this);
}

template <typename T>
std::size_t Slice<T>::length() const noexcept {
  return this->size();
}

template <typename T>
bool Slice<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T &Slice<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto pos = static_cast<char *>(slicePtr(this)) + size_of<T>() * n;
  return *reinterpret_cast<T *>(pos);
}

template <typename T>
T &Slice<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Slice index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Slice<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Slice<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
typename Slice<T>::iterator::reference
Slice<T>::iterator::operator*() const noexcept {
  return *static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::pointer
Slice<T>::iterator::operator->() const noexcept {
  return static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::reference Slice<T>::iterator::operator[](
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto pos = static_cast<char *>(this->pos) + this->stride * n;
  return *reinterpret_cast<T *>(pos);
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator++() noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator++(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator--() noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator--(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator+=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator-=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator+(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) + this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator-(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) - this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator::difference_type
Slice<T>::iterator::operator-(const iterator &other) const noexcept {
  auto diff = std::distance(static_cast<char *>(other.pos),
                            static_cast<char *>(this->pos));
  return diff / this->stride;
}

template <typename T>
bool Slice<T>::iterator::operator==(const iterator &other) const noexcept {
  return this->pos == other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator!=(const iterator &other) const noexcept {
  return this->pos != other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<(const iterator &other) const noexcept {
  return this->pos < other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<=(const iterator &other) const noexcept {
  return this->pos <= other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>(const iterator &other) const noexcept {
  return this->pos > other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>=(const iterator &other) const noexcept {
  return this->pos >= other.pos;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::begin() const noexcept {
  iterator it;
  it.pos = slicePtr(this);
  it.stride = size_of<T>();
  return it;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::end() const noexcept {
  iterator it = this->begin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
void Slice<T>::swap(Slice &rhs) noexcept {
  std::swap(*this, rhs);
}
#endif // CXXBRIDGE1_RUST_SLICE

#ifndef CXXBRIDGE1_RUST_BOX
#define CXXBRIDGE1_RUST_BOX
template <typename T>
class Box final {
public:
  using element_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box() = delete;
  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(Box &&) & noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  void swap(Box &) noexcept;

  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;

  /* Deprecated */ using value_type = element_type;

private:
  class uninit;
  class allocation;
  Box(uninit) noexcept;
  void drop() noexcept;

  friend void swap(Box &lhs, Box &rhs) noexcept { lhs.swap(rhs); }

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
Box<T> &Box<T>::operator=(Box &&other) & noexcept {
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
void Box<T>::swap(Box &rhs) noexcept {
  using std::swap;
  swap(this->ptr, rhs.ptr);
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

  Vec &operator=(Vec &&) & noexcept;
  Vec &operator=(const Vec &) &;

  std::size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  std::size_t capacity() const noexcept;

  const T &operator[](std::size_t n) const noexcept;
  const T &at(std::size_t n) const;
  const T &front() const noexcept;
  const T &back() const noexcept;

  T &operator[](std::size_t n) noexcept;
  T &at(std::size_t n);
  T &front() noexcept;
  T &back() noexcept;

  void reserve(std::size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&... args);

  using iterator = typename Slice<T>::iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = typename Slice<const T>::iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  void swap(Vec &) noexcept;

  Vec(unsafe_bitcopy_t, const Vec &) noexcept;

private:
  void reserve_total(std::size_t cap) noexcept;
  void set_len(std::size_t len) noexcept;
  void drop() noexcept;

  friend void swap(Vec &lhs, Vec &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
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
Vec<T> &Vec<T>::operator=(Vec &&other) & noexcept {
  this->drop();
  this->repr = other.repr;
  new (&other) Vec();
  return *this;
}

template <typename T>
Vec<T> &Vec<T>::operator=(const Vec &other) & {
  if (this != &other) {
    this->drop();
    new (this) Vec(other);
  }
  return *this;
}

template <typename T>
bool Vec<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T *Vec<T>::data() noexcept {
  return const_cast<T *>(const_cast<const Vec<T> *>(this)->data());
}

template <typename T>
const T &Vec<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * size_of<T>());
}

template <typename T>
const T &Vec<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
const T &Vec<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
const T &Vec<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
T &Vec<T>::operator[](std::size_t n) noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * size_of<T>());
}

template <typename T>
T &Vec<T>::at(std::size_t n) {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Vec<T>::front() noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Vec<T>::back() noexcept {
  assert(!this->empty());
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
                               size * size_of<T>()))
      T(std::forward<Args>(args)...);
  this->set_len(size + 1);
}

template <typename T>
typename Vec<T>::iterator Vec<T>::begin() noexcept {
  return Slice<T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::iterator Vec<T>::end() noexcept {
  return Slice<T>(this->data(), this->size()).end();
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
  return Slice<const T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cend() const noexcept {
  return Slice<const T>(this->data(), this->size()).end();
}

template <typename T>
void Vec<T>::swap(Vec &rhs) noexcept {
  using std::swap;
  swap(this->repr, rhs.repr);
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

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT
} // namespace cxxbridge1
} // namespace rust

namespace open_comp_graph {
  struct BBox2Df;
  struct BBox2Di;
  struct Matrix4;
  enum class GraphState : ::std::uint8_t;
  enum class ExecuteStatus : ::std::uint8_t;
  enum class NodeType : ::std::uint8_t;
  enum class PixelDataType : ::std::uint8_t;
  enum class NodeStatus : ::std::uint8_t;
  enum class AttrState : ::std::uint8_t;
  enum class StreamDataState : ::std::uint8_t;
  enum class DeformerDirection : ::std::uint8_t;
  enum class MergeImageMode : ::std::uint8_t;
  namespace internal {
    struct GraphImplShared;
    struct StreamDataImplShared;
    struct CacheImplShared;
    struct ConfigImplShared;
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

#ifndef CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageShared
#define CXXBRIDGE1_STRUCT_open_comp_graph$internal$ImageShared
struct ImageShared final {
  ::rust::Box<::open_comp_graph::internal::PixelBlock> pixel_block;
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

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$NodeType
#define CXXBRIDGE1_ENUM_open_comp_graph$NodeType
enum class NodeType : ::std::uint8_t {
  kNull = 0,
  kReadImage = 1,
  kWriteImage = 2,
  kTransform = 5,
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

#ifndef CXXBRIDGE1_ENUM_open_comp_graph$DeformerDirection
#define CXXBRIDGE1_ENUM_open_comp_graph$DeformerDirection
enum class DeformerDirection : ::std::uint8_t {
  kForward = 0,
  kBackward = 1,
  kUninitialized = 255,
};
#endif // CXXBRIDGE1_ENUM_open_comp_graph$DeformerDirection

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
  OCG_API_EXPORT ::open_comp_graph::Matrix4 transform_matrix() const noexcept;
  OCG_API_EXPORT ::std::size_t deformers_len() const noexcept;
  OCG_API_EXPORT void apply_deformers(::rust::Slice<float> buffer, ::open_comp_graph::BBox2Df image_window, ::open_comp_graph::DeformerDirection direction) const noexcept;
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
