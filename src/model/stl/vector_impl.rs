
use std::{fmt, slice};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use libc::size_t;

use super::memory::UniquePtr;
use super::vector::{Vector, VectorInsert, VectorRemove, VectorSlice};

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

impl VectorRemove for Vector<u8> {
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

add_impl!(Vector<u8>);

impl VectorInsert<u8> for Vector<u8> {
    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<uint8_t>*", v as "uint8_t"] {
                self->push_back(v);
            })
        }
    }
}

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

impl VectorRemove for Vector<i32> {
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

add_impl!(Vector<i32>);

impl VectorInsert<i32> for Vector<i32> {
    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<int32_t>*", v as "int32_t"] {
                self->push_back(v);
            })
        }
    }
}

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

impl VectorRemove for Vector<i64> {
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

add_impl!(Vector<i64>);

impl VectorInsert<i64> for Vector<i64> {
    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<int64_t>*", v as "int64_t"] {
                self->push_back(v);
            })
        }
    }
}

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

impl VectorRemove for Vector<f32> {
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

add_impl!(Vector<f32>);

impl VectorInsert<f32> for Vector<f32> {
    fn push_back(&mut self, v: Self::Item) {
        unsafe {
            cpp!([self as "std::vector<float>*", v as "float"] {
                self->push_back(v);
            })
        }
    }
}

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

impl VectorRemove for Vector<UniquePtr<crate::model::OperatorCodeT>> {
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

impl VectorRemove for Vector<UniquePtr<crate::model::TensorT>> {
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

impl VectorRemove for Vector<UniquePtr<crate::model::OperatorT>> {
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

impl VectorRemove for Vector<UniquePtr<crate::model::SubGraphT>> {
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

impl VectorRemove for Vector<UniquePtr<crate::model::BufferT>> {
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

add_impl!(Vector<UniquePtr<crate::model::BufferT>>);

