use std::{alloc::Layout, fmt::Display, ptr::NonNull, usize};

pub struct Vector<T>{
    inner: RawVec<T>,
    pub size: usize,
}


impl<T> Display for Vector<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Vector(size: {}, cap: {})--[", self.size, self.inner.cap))?;
        let mut size = self.size;
        let mut ptr = self.inner.ptr.as_ptr();
        while size > 0 {
            let data = if std::mem::size_of::<T>() != 0{
                unsafe {
                    ptr.as_ref().unwrap()  // 一定有值
                }
            } else {
                unsafe {
                    std::ptr::read(NonNull::dangling().as_ptr())
                }
            };
            unsafe {
                ptr = ptr.add(1);
            }
            f.write_str(&format!("{}, ", data))?;
            size -= 1;
        }

        f.write_str("]")?;
        Ok(())
    }
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self { inner: RawVec::new(), size: 0 }
    }

    pub fn push(&mut self, data: T) {
        if self.size == self.inner.cap {
            self.inner.grow();
        }
        unsafe {
            self.inner.ptr.as_ptr().add(self.size).write(data);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        } else {
            self.size -= 1;
            unsafe {
                Some(self.inner.ptr.add(self.size).read())
            }
        }
    }

    pub fn insert(&mut self, index: usize, data: T) {
        assert!(index <= self.size, "Index Out of Bounds");
        if self.size == self.inner.cap {
            self.inner.grow();
        }

        unsafe {
            let ptr = self.inner.ptr.as_ptr();
            std::ptr::copy(ptr.add(index) as * const T, ptr.add(index + 1), self.size - index);
            std::ptr::write(ptr.add(index), data);
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size - 1, "Index Out of Bounds");
        let ptr = self.inner.ptr.as_ptr();
        unsafe {
            let data = ptr.add(index).read();
            self.size -= 1;
            std::ptr::copy(ptr.add(index + 1), ptr.add(index), self.size - index);
            data
        }
    }
}



struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}



impl<T> RawVec<T> {

    fn new() -> Self {
        let cap = if std::mem::size_of::<T>() == 0 { usize::MAX } else { 0 };
        Self { ptr: NonNull::dangling(), cap: cap }
    }

    fn grow(&mut self) {
        assert!(std::mem::size_of::<T>() != 0, "error");
        let (new_cap, new_layout) = if self.cap == 0 {
            (1usize, Layout::new::<T>())
        } else {
             (self.cap * 2, Layout::array::<T>(self.cap * 2).unwrap()) 
        };
        tracing::debug!("从{}扩容到{}", self.cap, new_cap);
        let new_ptr= if self.cap == 0 {
            unsafe {
                std::alloc::alloc(new_layout)
            }
        } else {
            let old_ptr = self.ptr.as_ptr() as * mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };
        self.ptr = NonNull::new(new_ptr as *mut T).unwrap();
        self.cap = new_cap;
    }

}


impl<T> Drop for RawVec<T> {

    fn drop(&mut self) {
        if std::mem::size_of::<T>() != 0 && self.cap > 0 {
            unsafe {
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap());
            }
        }
    }   

}

pub struct Iter<T> {
    raw_iter: RawIter<T>,
}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        if std::mem::size_of::<T>() != 0{
            for _ in &mut self.raw_iter {}  // 释放掉未被迭代出来的数据
            // 有问题
        }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_iter.size_hint()
    }
}

pub struct RawIter<T> {
    start_ptr: *mut T,
    end_ptr: *mut T,
    size: usize,
    cap: usize
}

impl<T> Drop for RawIter<T> {

    fn drop(&mut self) {
        if std::mem::size_of::<T>() != 0 {
            unsafe {
                let start_ptr = self.end_ptr.sub(self.size);
                std::alloc::dealloc(start_ptr as * mut u8, Layout::array::<T>(self.cap).unwrap());
            }    
        }
    }
}


impl <T> Iterator for RawIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let elem_size = std::mem::size_of::<T>();
        if self.end_ptr == self.start_ptr {
            return None;
        }
        if elem_size == 0 {
            self.start_ptr = (self.start_ptr as usize + 1) as *mut T;
            unsafe {
                return Some(std::ptr::read(NonNull::dangling().as_ptr()));
            }
        }
        let data = unsafe {
            tracing::debug!("读取数据");
            self.start_ptr.read()   
        };
        self.start_ptr = unsafe {
            self.start_ptr.add(1)
        };
        Some(data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = std::mem::size_of::<T>();
        if elem_size == 0 {
            (usize::MAX, Some(usize::MAX))
        } else {
            let gap = self.end_ptr as usize - self.start_ptr as usize;
            let size = gap / elem_size;
            (size, Some(size))
        }
    }
}

impl<T> IntoIterator for Vector<T> {
    type IntoIter = Iter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        let elem_size = std::mem::size_of::<T>();
        let start_ptr = self.inner.ptr.as_ptr();
        let end_ptr = if elem_size != 0{ 
            unsafe {
                start_ptr.add(self.size)   
            }
        } else {
            (start_ptr as usize + self.size) as *mut T
        };
        
        let raw_iter = RawIter {
            start_ptr,
            end_ptr,
            size: self.size,
            cap: self.inner.cap,
        };
        std::mem::forget(self); // 防止释放有数据的地方
        Iter{
            raw_iter,
        }
    }
}