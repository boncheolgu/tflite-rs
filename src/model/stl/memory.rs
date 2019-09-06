use std::marker::PhantomData;

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

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        unsafe {
            cpp!([self as "std::unique_ptr<flatbuffers::NativeTable>*"] {
                self->reset();
            });
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
