use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::bindings::root::rust::*;

cpp! {{
    #include <memory>

    struct class_with_unique_ptr {
        std::string desc;
        std::unique_ptr<int32_t> value;

        class_with_unique_ptr(int32_t v, const char* s): desc(s), value(new int32_t(v)) {}
    };
}}

#[repr(C)]
pub struct UniquePtr<T>(unique_ptr_of_void, PhantomData<T>);

impl<T> fmt::Debug for UniquePtr<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        // this will leak memory
        panic!("Do not drop `UniquePtr`!");
    }
}

impl<T> Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<void>*"] -> *const c_void as "const void*" {
                return self->get();
            }) as *const Self::Target;

            ptr.as_ref().unwrap()
        }
    }
}

impl<T> DerefMut for UniquePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<void>*"] -> *mut c_void as "void*" {
                return self->get();
            }) as *mut Self::Target;

            ptr.as_mut().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::stl::string::String as StlString;

    #[repr(C)]
    struct ClassWithUniquePtr {
        desc: StlString,
        value: UniquePtr<i32>,
    }

    #[test]
    fn unittest_unique_ptr() {
        let x = unsafe {
            cpp!([] -> &mut ClassWithUniquePtr as "class_with_unique_ptr*" {
                static class_with_unique_ptr x(23, "hello");
                return &x;
            })
        };

        assert_eq!(x.value.deref(), &23);
        assert_eq!(x.desc.c_str().to_string_lossy(), "hello");
    }
}
