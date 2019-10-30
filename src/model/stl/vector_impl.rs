use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::{fmt, mem, slice};

use libc::size_t;

use super::memory::UniquePtr;
use super::vector::{VectorErase, VectorExtract, VectorInsert, VectorOfUniquePtr, VectorSlice};
use crate::model::stl::bindings::root::rust::dummy_vector;

cpp! {{
    #include <vector>
}}

#[repr(C)]
pub struct VectorOfU8(dummy_vector);

impl Default for VectorOfU8 {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<uint8_t>*"] {
                new (this_ref) const std::vector<uint8_t>;
            })
        }
        this
    }
}

impl Drop for VectorOfU8 {
    fn drop(&mut self) {
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*"] {
                self->~vector<uint8_t>();
            })
        }
    }
}

impl Clone for VectorOfU8 {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        unsafe {
            cpp!([self as "const std::vector<uint8_t>*", cloned_ref as "std::vector<uint8_t>*"] {
                new (cloned_ref) std::vector<uint8_t>(*self);
            });
        }
        cloned
    }
}

impl PartialEq for VectorOfU8 {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for VectorOfU8 {}

impl VectorSlice for VectorOfU8 {
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

impl VectorErase for VectorOfU8 {
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

impl VectorInsert<u8> for VectorOfU8 {
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

impl VectorExtract<u8> for VectorOfU8 {
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

add_impl!(VectorOfU8);

#[repr(C)]
pub struct VectorOfI32(dummy_vector);

impl Default for VectorOfI32 {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<int32_t>*"] {
                new (this_ref) const std::vector<int32_t>;
            })
        }
        this
    }
}

impl Drop for VectorOfI32 {
    fn drop(&mut self) {
        unsafe {
            cpp!([self as "const std::vector<int32_t>*"] {
                self->~vector<int32_t>();
            })
        }
    }
}

impl Clone for VectorOfI32 {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        unsafe {
            cpp!([self as "const std::vector<int32_t>*", cloned_ref as "std::vector<int32_t>*"] {
                new (cloned_ref) std::vector<int32_t>(*self);
            });
        }
        cloned
    }
}

impl PartialEq for VectorOfI32 {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for VectorOfI32 {}

impl VectorSlice for VectorOfI32 {
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

impl VectorErase for VectorOfI32 {
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

impl VectorInsert<i32> for VectorOfI32 {
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

impl VectorExtract<i32> for VectorOfI32 {
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

add_impl!(VectorOfI32);

#[repr(C)]
pub struct VectorOfI64(dummy_vector);

impl Default for VectorOfI64 {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<int64_t>*"] {
                new (this_ref) const std::vector<int64_t>;
            })
        }
        this
    }
}

impl Drop for VectorOfI64 {
    fn drop(&mut self) {
        unsafe {
            cpp!([self as "const std::vector<int64_t>*"] {
                self->~vector<int64_t>();
            })
        }
    }
}

impl Clone for VectorOfI64 {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        unsafe {
            cpp!([self as "const std::vector<int64_t>*", cloned_ref as "std::vector<int64_t>*"] {
                new (cloned_ref) std::vector<int64_t>(*self);
            });
        }
        cloned
    }
}

impl PartialEq for VectorOfI64 {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for VectorOfI64 {}

impl VectorSlice for VectorOfI64 {
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

impl VectorErase for VectorOfI64 {
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

impl VectorInsert<i64> for VectorOfI64 {
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

impl VectorExtract<i64> for VectorOfI64 {
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

add_impl!(VectorOfI64);

#[repr(C)]
pub struct VectorOfF32(dummy_vector);

impl Default for VectorOfF32 {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<float>*"] {
                new (this_ref) const std::vector<float>;
            })
        }
        this
    }
}

impl Drop for VectorOfF32 {
    fn drop(&mut self) {
        unsafe {
            cpp!([self as "const std::vector<float>*"] {
                self->~vector<float>();
            })
        }
    }
}

impl Clone for VectorOfF32 {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        unsafe {
            cpp!([self as "const std::vector<float>*", cloned_ref as "std::vector<float>*"] {
                new (cloned_ref) std::vector<float>(*self);
            });
        }
        cloned
    }
}

impl PartialEq for VectorOfF32 {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for VectorOfF32 {}

impl VectorSlice for VectorOfF32 {
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

impl VectorErase for VectorOfF32 {
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

impl VectorInsert<f32> for VectorOfF32 {
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

impl VectorExtract<f32> for VectorOfF32 {
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

add_impl!(VectorOfF32);

impl Default for VectorOfUniquePtr<crate::model::OperatorCodeT> {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<std::unique_ptr<OperatorCodeT>>*"] {
                new (this_ref) const std::vector<std::unique_ptr<OperatorCodeT>>;
            })
        }
        this
    }
}

impl VectorSlice for VectorOfUniquePtr<crate::model::OperatorCodeT> {
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

impl VectorErase for VectorOfUniquePtr<crate::model::OperatorCodeT> {
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

impl VectorInsert<UniquePtr<crate::model::OperatorCodeT>>
    for VectorOfUniquePtr<crate::model::OperatorCodeT>
{
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

impl VectorExtract<UniquePtr<crate::model::OperatorCodeT>>
    for VectorOfUniquePtr<crate::model::OperatorCodeT>
{
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

add_impl!(VectorOfUniquePtr<crate::model::OperatorCodeT>);

impl Default for VectorOfUniquePtr<crate::model::TensorT> {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<std::unique_ptr<TensorT>>*"] {
                new (this_ref) const std::vector<std::unique_ptr<TensorT>>;
            })
        }
        this
    }
}

impl VectorSlice for VectorOfUniquePtr<crate::model::TensorT> {
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

impl VectorErase for VectorOfUniquePtr<crate::model::TensorT> {
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

impl VectorInsert<UniquePtr<crate::model::TensorT>> for VectorOfUniquePtr<crate::model::TensorT> {
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

impl VectorExtract<UniquePtr<crate::model::TensorT>> for VectorOfUniquePtr<crate::model::TensorT> {
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

add_impl!(VectorOfUniquePtr<crate::model::TensorT>);

impl Default for VectorOfUniquePtr<crate::model::OperatorT> {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<std::unique_ptr<OperatorT>>*"] {
                new (this_ref) const std::vector<std::unique_ptr<OperatorT>>;
            })
        }
        this
    }
}

impl VectorSlice for VectorOfUniquePtr<crate::model::OperatorT> {
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

impl VectorErase for VectorOfUniquePtr<crate::model::OperatorT> {
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

impl VectorInsert<UniquePtr<crate::model::OperatorT>>
    for VectorOfUniquePtr<crate::model::OperatorT>
{
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

impl VectorExtract<UniquePtr<crate::model::OperatorT>>
    for VectorOfUniquePtr<crate::model::OperatorT>
{
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

add_impl!(VectorOfUniquePtr<crate::model::OperatorT>);

impl Default for VectorOfUniquePtr<crate::model::SubGraphT> {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<std::unique_ptr<SubGraphT>>*"] {
                new (this_ref) const std::vector<std::unique_ptr<SubGraphT>>;
            })
        }
        this
    }
}

impl VectorSlice for VectorOfUniquePtr<crate::model::SubGraphT> {
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

impl VectorErase for VectorOfUniquePtr<crate::model::SubGraphT> {
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

impl VectorInsert<UniquePtr<crate::model::SubGraphT>>
    for VectorOfUniquePtr<crate::model::SubGraphT>
{
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

impl VectorExtract<UniquePtr<crate::model::SubGraphT>>
    for VectorOfUniquePtr<crate::model::SubGraphT>
{
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

add_impl!(VectorOfUniquePtr<crate::model::SubGraphT>);

impl Default for VectorOfUniquePtr<crate::model::BufferT> {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::vector<std::unique_ptr<BufferT>>*"] {
                new (this_ref) const std::vector<std::unique_ptr<BufferT>>;
            })
        }
        this
    }
}

impl VectorSlice for VectorOfUniquePtr<crate::model::BufferT> {
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

impl VectorErase for VectorOfUniquePtr<crate::model::BufferT> {
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

impl VectorInsert<UniquePtr<crate::model::BufferT>> for VectorOfUniquePtr<crate::model::BufferT> {
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

impl VectorExtract<UniquePtr<crate::model::BufferT>> for VectorOfUniquePtr<crate::model::BufferT> {
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

add_impl!(VectorOfUniquePtr<crate::model::BufferT>);
