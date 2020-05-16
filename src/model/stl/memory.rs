use std::marker::PhantomData;
use std::ops::Deref;

use super::bindings::root::rust::*;

#[repr(C)]
pub struct UniquePtr<T>(unique_ptr_of_void, PhantomData<T>);

unsafe impl<T> Sync for UniquePtr<T> {}
unsafe impl<T> Send for UniquePtr<T> {}

impl<T> PartialEq for UniquePtr<T>
where
    T: PartialEq,
    UniquePtr<T>: Deref<Target = T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Eq for UniquePtr<T>
where
    T: Eq,
    UniquePtr<T>: Deref<Target = T>,
{
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "std::unique_ptr<flatbuffers::NativeTable>*"] {
                self->reset();
            });
        }
    }
}

impl<T> UniquePtr<T> {
    pub fn is_valid(&self) -> bool {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::unique_ptr<flatbuffers::NativeTable>*"] -> bool as "bool" {
                return static_cast<bool>(*self);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::stl::vector::VectorExtract;
    use crate::model::Model;

    #[test]
    fn unittest_unique_ptr_drop() {
        let mut model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
        let _subgraph = model.subgraphs.extract(0);
    }
}
