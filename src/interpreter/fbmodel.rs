use std::path::Path;
use std::{fs, mem};

use crate::bindings::tflite as bindings;
use crate::model::Model;
use crate::{Error, Result};

cpp! {{
    #include "tensorflow/lite/model.h"
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite;
}}

#[derive(Default)]
pub struct FlatBufferModel {
    pub(crate) handle: Box<bindings::FlatBufferModel>,
    model_buffer: Vec<u8>,
}

impl Drop for FlatBufferModel {
    fn drop(&mut self) {
        let handle = Box::into_raw(mem::take(&mut self.handle));

        #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([handle as "FlatBufferModel*"] {
                delete handle;
            });
        }
    }
}

impl FlatBufferModel {
    pub fn build_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::build_from_buffer(fs::read(path)?)
    }

    pub fn build_from_buffer(model_buffer: Vec<u8>) -> Result<Self> {
        let ptr = model_buffer.as_ptr();
        let size = model_buffer.len();

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let handle = unsafe {
            cpp!([ptr as "const char*", size as "size_t"]
                  -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::BuildFromBuffer(ptr, size).release();
            })
        };
        if handle.is_null() {
            return Err(Error::internal_error("failed to build model"));
        }
        let handle = unsafe { Box::from_raw(handle) };
        Ok(Self { handle, model_buffer })
    }

    pub fn build_from_model(model: &Model) -> Result<Self> {
        FlatBufferModel::build_from_buffer(model.to_buffer())
    }

    pub fn buffer(&self) -> &[u8] {
        &self.model_buffer
    }

    pub fn release_buffer(mut self) -> Vec<u8> {
        mem::take(&mut self.model_buffer)
    }
}
