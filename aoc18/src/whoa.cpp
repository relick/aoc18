#include <memory>
#include <iostream>

namespace std
{
// A non-owning raw pointer wrapper with a semantic name.
template<typename T>
class observer_ptr
{
public:
	observer_ptr() = default;

	observer_ptr(T* const obj) : m_ptr{obj} {}
	observer_ptr(std::unique_ptr<T>& obj) : m_ptr{obj.get()} {}
	observer_ptr(observer_ptr<T> const& other) : m_ptr{other.m_ptr} {}
	observer_ptr(nullptr_t const n) : m_ptr{nullptr} {}

	void operator=(T* const obj)
	{
		m_ptr = obj;
	}
	void operator=(std::unique_ptr<T>& obj)
	{
		m_ptr = obj.get();
	}
	void operator=(observer_ptr const& other)
	{
		m_ptr = other.m_ptr;
	}
	void operator=(nullptr_t const n)
	{
		m_ptr = nullptr;
	}


	void reset() { m_ptr = nullptr; }
	void swap(observer_ptr<T>& other) { swap(m_ptr, other.m_ptr); }
	operator bool() const { return m_ptr != nullptr; }
	T* get() const { return m_ptr; }

	typename add_lvalue_reference<T>::type operator*() const
	{
		return *m_ptr;
	}
	typename add_lvalue_reference<T>::type operator->() const
	{
		return *m_ptr;
	}

	operator T*() const
	{
		return get();
	}
private:
	T* m_ptr {nullptr};
};

template<typename T>
class observer_ptr<T[]>
{
public:
	observer_ptr() = default;

	observer_ptr(T* const obj) : m_ptr{obj} {}
	observer_ptr(std::unique_ptr<T[]>& obj) : m_ptr{obj.get()} {}
	observer_ptr(observer_ptr<T[]> const& other) : m_ptr{other.m_ptr} {}
	observer_ptr(nullptr_t const n) : m_ptr{nullptr} {}

	void operator=(T* const obj)
	{
		m_ptr = obj;
	}
	void operator=(std::unique_ptr<T[]>& obj)
	{
		m_ptr = obj.get();
	}
	void operator=(observer_ptr const& other)
	{
		m_ptr = other.m_ptr;
	}
	void operator=(nullptr_t const n)
	{
		m_ptr = nullptr;
	}

	void reset() { m_ptr = nullptr; }
	void swap(observer_ptr<T>& other) { swap(m_ptr, other.m_ptr); }
	operator bool() const { return m_ptr != nullptr; }
	T* get() const { return m_ptr; }

	typename add_lvalue_reference<T>::type operator*() const
	{
		return *m_ptr;
	}
	typename add_lvalue_reference<T>::type operator->() const
	{
		return *m_ptr;
	}
	typename add_lvalue_reference<T>::type operator[](size_t const i) const
	{
		return m_ptr[i];
	}

	operator T*() const
	{
		return get();
	}
private:
	T* m_ptr {nullptr};
};
}


class kstring
{
public:
  kstring()
  {

  }

  kstring(const char* str) 
  {
    size_ = std::strlen(str);
    if (size_ + 1 > capacity_)
      reserve(capacity_ + 1);
    std::strcpy(data(), str);
  }

  void reserve(size_t capacity)
  {
    if (capacity <= capacity_ || capacity <= 24)
      return;
    char* alloc = new char[capacity];
    std::cout << "Alloc: " << static_cast<void*>(alloc) << std::endl;
    std::copy(data(), data() + size_, alloc);
    if (on_heap())
	{
	  std::cout << "Deleting: " << static_cast<void*>(data()) << std::endl;
      delete[] data();
	}
    std::memcpy(data_, &alloc, sizeof(char*));
    capacity_ = capacity;
  }

  char* data()
  {
    return capacity_ > 24 ? heap_ptr() : data_;
  }

  size_t size() const noexcept
  {
    return size_;
  }

  size_t capacity() const noexcept
  {
    return capacity_;
  }

  char& operator[](size_t n)
  {
    return data()[n];
  }

  ~kstring()
  {
    if (on_heap())
    {
      std::cout << "Deleting: " << static_cast<void*>(data()) << std::endl;
      delete[] data();
    }
  }

  bool on_heap()
  {
    return capacity_ > 24;
  }

  char* heap_ptr()
  {
    char* ptr = nullptr;
    std::memcpy(&ptr, data_, sizeof(char*));
    return ptr;
  }

  char data_[24] = {0};
private:
  size_t size_ = 0;
  size_t capacity_ = 24;
};

int main()
{
	kstring k{"012345678901234567890123"};

	std::cout << k.data() << std::endl;

	return 0;
}