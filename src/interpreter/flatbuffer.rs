use std::fs::File;
use std::io::Read;
use std::path::Path;

use failure::Fallible;

use crate::bindings::tflite as bindings;

cpp! {{
    #include "tensorflow/lite/model.h"
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite;
}}

#[derive(Default)]
pub struct Model {
    pub(crate) handle: Box<bindings::FlatBufferModel>,
    model_buffer: Vec<u8>,
}

impl Drop for Model {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, Default::default());
        let handle = Box::into_raw(handle);

        #[allow(clippy::forget_copy, clippy::useless_transmute)]
        unsafe {
            cpp!([handle as "Model*"] {
                delete handle;
            });
        }
    }
}

impl Model {
    pub fn build_from_file<P: AsRef<Path>>(path: P) -> Fallible<Self> {
        let mut model_buffer = Vec::new();
        File::open(path.as_ref())?.read_to_end(&mut model_buffer)?;
        Self::build_from_buffer(model_buffer)
    }

    pub fn build_from_buffer(model_buffer: Vec<u8>) -> Fallible<Self> {
        let ptr = model_buffer.as_ptr();
        let size = model_buffer.len();

        #[allow(clippy::forget_copy)]
        let handle = unsafe {
            cpp!([ptr as "const char*", size as "size_t"]
                  -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::BuildFromBuffer(ptr, size).release();
            })
        };
        ensure!(!handle.is_null(), "Building Model failed.");
        let handle = unsafe { Box::from_raw(handle) };
        Ok(Model {
            handle,
            model_buffer,
        })
    }

    pub fn buffer(&self) -> &[u8] {
        &self.model_buffer
    }

    pub fn release_buffer(mut self) -> Vec<u8> {
        use std::mem;
        mem::replace(&mut self.model_buffer, Vec::new())
    }
}
