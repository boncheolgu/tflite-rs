use std::marker::PhantomData;
use std::{mem, slice};

use super::bindings::root::rust::*;
use super::memory::UniquePtr;
pub use super::vector_impl::{VectorOfF32, VectorOfI32, VectorOfI64, VectorOfU8};

#[repr(C)]
pub struct Vector<T>(dummy_vector, PhantomData<T>);

pub trait VectorSlice {
    type Item;

    fn get_ptr(&self) -> *const Self::Item {
        self.as_slice().as_ptr()
    }

    fn get_mut_ptr(&mut self) -> *mut Self::Item {
        self.as_mut_slice().as_mut_ptr()
    }

    fn size(&self) -> usize {
        self.as_slice().len()
    }

    fn as_slice(&self) -> &[Self::Item] {
        unsafe { slice::from_raw_parts(self.get_ptr(), self.size()) }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        unsafe { slice::from_raw_parts_mut(self.get_mut_ptr(), self.size()) }
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl fmt::Debug for $t
        where
            <$t as VectorSlice>::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.as_slice().iter()).finish()
            }
        }

        impl Deref for $t {
            type Target = [<$t as VectorSlice>::Item];

            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut_slice()
            }
        }

        impl Index<usize> for $t {
            type Output = <$t as VectorSlice>::Item;

            fn index(&self, index: usize) -> &Self::Output {
                &self.as_slice()[index]
            }
        }

        impl IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut_slice()[index]
            }
        }

        impl<'a> IntoIterator for &'a $t {
            type Item = &'a <$t as VectorSlice>::Item;
            type IntoIter = slice::Iter<'a, <$t as VectorSlice>::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $t {
            type Item = &'a mut <$t as VectorSlice>::Item;
            type IntoIter = slice::IterMut<'a, <$t as VectorSlice>::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }
    )*)
}

pub trait VectorErase: VectorSlice {
    fn erase_range(&mut self, offset: usize, len: usize) {
        for i in (offset..offset + len).rev() {
            self.erase(i);
        }
    }

    fn pop_back(&mut self) {
        assert!(self.size() > 0);
        self.erase(self.size() - 1);
    }

    fn erase(&mut self, index: usize) {
        self.erase_range(index, 1);
    }

    fn clear(&mut self) {
        self.erase_range(0, self.size());
    }

    fn retain<F>(&mut self, pred: F) -> usize
    where
        F: Fn(usize, &Self::Item) -> bool,
    {
        let removed: Vec<_> = self
            .as_slice()
            .iter()
            .enumerate()
            .filter_map(|(i, op)| if pred(i, op) { None } else { Some(i) })
            .collect();

        for &i in removed.iter().rev() {
            self.erase(i);
        }
        removed.len()
    }

    fn truncate(&mut self, size: usize) {
        assert!(size <= self.size());
        self.erase_range(size, self.size() - size);
    }
}

pub trait VectorInsert<T>: VectorErase {
    fn push_back(&mut self, v: T);

    fn assign<I: IntoIterator<Item = T>>(&mut self, vs: I) {
        self.clear();
        for v in vs {
            self.push_back(v);
        }
    }

    fn append<I: IntoIterator<Item = T>>(&mut self, items: I) {
        for item in items.into_iter() {
            self.push_back(item);
        }
    }
}

pub trait VectorExtract<T>: VectorErase {
    fn extract(&mut self, index: usize) -> T;

    fn extract_remove(&mut self, index: usize) -> T {
        let item = self.extract(index);
        self.erase(index);
        item
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VectorOfBool(vector_of_bool);

impl Default for VectorOfBool {
    fn default() -> Self {
        let mut this = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        #[allow(deprecated)]
        unsafe {
            cpp!([this_ref as "std::vector<bool>*"] {
                new (this_ref) const std::vector<bool>;
            })
        }
        this
    }
}

impl Drop for VectorOfBool {
    fn drop(&mut self) {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::vector<bool>*"] {
                self->~vector<bool>();
            })
        }
    }
}

impl Clone for VectorOfBool {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::vector<bool>*", cloned_ref as "std::vector<bool>*"] {
                new (cloned_ref) std::vector<bool>(*self);
            });
        }
        cloned
    }
}

impl PartialEq for VectorOfBool {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl Eq for VectorOfBool {}

impl VectorOfBool {
    pub fn get(&self, index: usize) -> bool {
        #[allow(deprecated, clippy::transmute_num_to_bytes)]
        unsafe {
            cpp!([self as "const std::vector<bool>*", index as "size_t"] -> bool as "bool" {
                return (*self)[index];
            })
        }
    }

    pub fn set(&mut self, index: usize, v: bool) {
        #[allow(deprecated, clippy::transmute_num_to_bytes)]
        unsafe {
            cpp!([self as "std::vector<bool>*", index as "size_t", v as "bool"] {
                (*self)[index] = v;
            })
        }
    }

    pub fn size(&self) -> usize {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::vector<bool>*"] -> usize as "size_t" {
                return self->size();
            })
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        (0..self.size()).map(move |i| self.get(i))
    }
}

#[repr(C)]
pub struct VectorOfUniquePtr<T>(dummy_vector, PhantomData<T>);

impl<T> PartialEq for VectorOfUniquePtr<T>
where
    Self: VectorSlice<Item = UniquePtr<T>>,
    UniquePtr<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T> Eq for VectorOfUniquePtr<T>
where
    Self: VectorSlice<Item = UniquePtr<T>>,
    UniquePtr<T>: Eq,
{
}

impl<T> Drop for VectorOfUniquePtr<T> {
    fn drop(&mut self) {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::vector<std::unique_ptr<flatbuffers::NativeTable>>*"] {
                self->~vector<std::unique_ptr<flatbuffers::NativeTable>>();
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::stl::memory::UniquePtr;
    use crate::model::BufferT;

    #[test]
    fn unittest_vector_default() {
        let mut vs = VectorOfU8::default();
        assert_eq!(vs.size(), 0);

        vs.push_back(9);
        vs.push_back(10);
        assert_eq!(vs.size(), 2);
        assert_eq!(vs.as_slice(), &[9u8, 10]);

        let mut vs: VectorOfUniquePtr<BufferT> = VectorOfUniquePtr::default();
        vs.push_back(UniquePtr::default());
        vs.push_back(UniquePtr::default());
        vs.push_back(UniquePtr::default());
        assert_eq!(vs.size(), 3);
    }

    #[test]
    fn unittest_vector_clone() {
        let mut vs = VectorOfU8::default();
        vs.assign(0u8..6);
        assert_eq!(vs.size(), 6);

        let cloned = vs.clone();
        assert_eq!(vs.as_slice(), cloned.as_slice());
    }
}
