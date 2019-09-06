
use std::{fmt, mem, slice};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use libc::size_t;

use super::memory::UniquePtr;
use super::vector::{Vector, VectorErase, VectorExtract, VectorInsert, VectorSlice};

cpp! {{
    #include <vector>
}}

impl VectorSlice for Vector<u8> {
    type Item = u8;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*"]
                  -> *const u8 as "const uint8_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<uint8_t>*"]
                  -> *mut u8 as "uint8_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<u8> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<u8> for Vector<u8> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", vref as "uint8_t*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<u8> for Vector<u8> {
    fn extract(&mut self, index: usize) -> u8 {
        assert!(index < self.size());
        let mut v: u8 = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", index as "size_t", vref as "uint8_t*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<u8>);


impl VectorSlice for Vector<i32> {
    type Item = i32;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"]
                  -> *const i32 as "const int32_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<int32_t>*"]
                  -> *mut i32 as "int32_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<i32> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<i32> for Vector<i32> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", vref as "int32_t*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<i32> for Vector<i32> {
    fn extract(&mut self, index: usize) -> i32 {
        assert!(index < self.size());
        let mut v: i32 = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<int32_t>*", index as "size_t", vref as "int32_t*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<i32>);


impl VectorSlice for Vector<i64> {
    type Item = i64;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<int64_t>*"]
                  -> *const i64 as "const int64_t*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<int64_t>*"]
                  -> *mut i64 as "int64_t*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<int64_t>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<i64> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<int64_t>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<i64> for Vector<i64> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<int64_t>*", vref as "int64_t*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<i64> for Vector<i64> {
    fn extract(&mut self, index: usize) -> i64 {
        assert!(index < self.size());
        let mut v: i64 = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<int64_t>*", index as "size_t", vref as "int64_t*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<i64>);


impl VectorSlice for Vector<f32> {
    type Item = f32;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<float>*"]
                  -> *const f32 as "const float*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<float>*"]
                  -> *mut f32 as "float*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<float>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<f32> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<float>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<f32> for Vector<f32> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<float>*", vref as "float*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<f32> for Vector<f32> {
    fn extract(&mut self, index: usize) -> f32 {
        assert!(index < self.size());
        let mut v: f32 = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<float>*", index as "size_t", vref as "float*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<f32>);


impl VectorSlice for Vector<UniquePtr<crate::model::OperatorCodeT>> {
    type Item = UniquePtr<crate::model::OperatorCodeT>;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<OperatorCodeT>>*"]
                  -> *const UniquePtr<crate::model::OperatorCodeT> as "const std::unique_ptr<OperatorCodeT>*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorCodeT>>*"]
                  -> *mut UniquePtr<crate::model::OperatorCodeT> as "std::unique_ptr<OperatorCodeT>*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<OperatorCodeT>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<UniquePtr<crate::model::OperatorCodeT>> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorCodeT>>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<UniquePtr<crate::model::OperatorCodeT>> for Vector<UniquePtr<crate::model::OperatorCodeT>> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorCodeT>>*", vref as "std::unique_ptr<OperatorCodeT>*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<UniquePtr<crate::model::OperatorCodeT>> for Vector<UniquePtr<crate::model::OperatorCodeT>> {
    fn extract(&mut self, index: usize) -> UniquePtr<crate::model::OperatorCodeT> {
        assert!(index < self.size());
        let mut v: UniquePtr<crate::model::OperatorCodeT> = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorCodeT>>*", index as "size_t", vref as "std::unique_ptr<OperatorCodeT>*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<UniquePtr<crate::model::OperatorCodeT>>);


impl VectorSlice for Vector<UniquePtr<crate::model::TensorT>> {
    type Item = UniquePtr<crate::model::TensorT>;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<TensorT>>*"]
                  -> *const UniquePtr<crate::model::TensorT> as "const std::unique_ptr<TensorT>*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<TensorT>>*"]
                  -> *mut UniquePtr<crate::model::TensorT> as "std::unique_ptr<TensorT>*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<TensorT>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<UniquePtr<crate::model::TensorT>> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<TensorT>>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<UniquePtr<crate::model::TensorT>> for Vector<UniquePtr<crate::model::TensorT>> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<TensorT>>*", vref as "std::unique_ptr<TensorT>*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<UniquePtr<crate::model::TensorT>> for Vector<UniquePtr<crate::model::TensorT>> {
    fn extract(&mut self, index: usize) -> UniquePtr<crate::model::TensorT> {
        assert!(index < self.size());
        let mut v: UniquePtr<crate::model::TensorT> = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<TensorT>>*", index as "size_t", vref as "std::unique_ptr<TensorT>*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<UniquePtr<crate::model::TensorT>>);


impl VectorSlice for Vector<UniquePtr<crate::model::OperatorT>> {
    type Item = UniquePtr<crate::model::OperatorT>;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<OperatorT>>*"]
                  -> *const UniquePtr<crate::model::OperatorT> as "const std::unique_ptr<OperatorT>*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorT>>*"]
                  -> *mut UniquePtr<crate::model::OperatorT> as "std::unique_ptr<OperatorT>*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<OperatorT>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<UniquePtr<crate::model::OperatorT>> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorT>>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<UniquePtr<crate::model::OperatorT>> for Vector<UniquePtr<crate::model::OperatorT>> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorT>>*", vref as "std::unique_ptr<OperatorT>*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<UniquePtr<crate::model::OperatorT>> for Vector<UniquePtr<crate::model::OperatorT>> {
    fn extract(&mut self, index: usize) -> UniquePtr<crate::model::OperatorT> {
        assert!(index < self.size());
        let mut v: UniquePtr<crate::model::OperatorT> = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<OperatorT>>*", index as "size_t", vref as "std::unique_ptr<OperatorT>*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<UniquePtr<crate::model::OperatorT>>);


impl VectorSlice for Vector<UniquePtr<crate::model::SubGraphT>> {
    type Item = UniquePtr<crate::model::SubGraphT>;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<SubGraphT>>*"]
                  -> *const UniquePtr<crate::model::SubGraphT> as "const std::unique_ptr<SubGraphT>*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<SubGraphT>>*"]
                  -> *mut UniquePtr<crate::model::SubGraphT> as "std::unique_ptr<SubGraphT>*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<SubGraphT>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<UniquePtr<crate::model::SubGraphT>> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<SubGraphT>>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<UniquePtr<crate::model::SubGraphT>> for Vector<UniquePtr<crate::model::SubGraphT>> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<SubGraphT>>*", vref as "std::unique_ptr<SubGraphT>*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<UniquePtr<crate::model::SubGraphT>> for Vector<UniquePtr<crate::model::SubGraphT>> {
    fn extract(&mut self, index: usize) -> UniquePtr<crate::model::SubGraphT> {
        assert!(index < self.size());
        let mut v: UniquePtr<crate::model::SubGraphT> = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<SubGraphT>>*", index as "size_t", vref as "std::unique_ptr<SubGraphT>*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<UniquePtr<crate::model::SubGraphT>>);


impl VectorSlice for Vector<UniquePtr<crate::model::BufferT>> {
    type Item = UniquePtr<crate::model::BufferT>;

    fn get_ptr(&self) -> *const Self::Item {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<BufferT>>*"]
                  -> *const UniquePtr<crate::model::BufferT> as "const std::unique_ptr<BufferT>*" {
                return self->data();
            })
        }
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<BufferT>>*"]
                  -> *mut UniquePtr<crate::model::BufferT> as "std::unique_ptr<BufferT>*" {
                return self->data();
            })
        }
    }

    fn size(&self) -> usize {
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<BufferT>>*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }
}

impl VectorErase for Vector<UniquePtr<crate::model::BufferT>> {
    fn erase_range(&mut self, offset: usize, size: usize) {
        let begin = offset as size_t;
        let end = offset + size as size_t;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<BufferT>>*", begin as "size_t", end as "size_t"] {
                self->erase(self->begin() + begin, self->begin() + end);
            });
        }
    }
}

impl VectorInsert<UniquePtr<crate::model::BufferT>> for Vector<UniquePtr<crate::model::BufferT>> {
    fn push_back(&mut self, mut v: Self::Item) {
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<BufferT>>*", vref as "std::unique_ptr<BufferT>*"] {
                self->push_back(std::move(*vref));
            })
        }
        mem::forget(v);
    }
}

impl VectorExtract<UniquePtr<crate::model::BufferT>> for Vector<UniquePtr<crate::model::BufferT>> {
    fn extract(&mut self, index: usize) -> UniquePtr<crate::model::BufferT> {
        assert!(index < self.size());
        let mut v: UniquePtr<crate::model::BufferT> = unsafe { mem::zeroed() };
        let vref = &mut v;
        unsafe {
            cpp!([self as "std::vector<std::unique_ptr<BufferT>>*", index as "size_t", vref as "std::unique_ptr<BufferT>*"] {
                *vref = std::move((*self)[index]);
            })
        }
        v
    }
}

add_impl!(Vector<UniquePtr<crate::model::BufferT>>);


