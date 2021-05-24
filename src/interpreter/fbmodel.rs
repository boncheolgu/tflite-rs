use std::path::Path;
use std::{fs, mem};

// use crate::model::Model;
use crate::{Error, Result};

pub struct FlatBufferModel {
    pub(crate) handle: mem::ManuallyDrop<cxx::UniquePtr<super::cxx::FlatBufferModel>>,
    model_buffer: mem::ManuallyDrop<std::borrow::Cow<'static, [u8]>>,
}

impl Drop for FlatBufferModel {
    fn drop(&mut self) {
        unsafe {
            mem::ManuallyDrop::drop(&mut self.handle);
            mem::ManuallyDrop::drop(&mut self.model_buffer);
        }
    }
}

impl FlatBufferModel {
    pub fn build_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::build_from_buffer(fs::read(path)?)
    }

    pub fn build_from_buffer(
        model_buffer: impl Into<std::borrow::Cow<'static, [u8]>>,
    ) -> Result<Self> {
        let model_buffer = model_buffer.into();

        let handle = mem::ManuallyDrop::new(super::cxx::flatbuffer_from_slice(&model_buffer));
        if handle.as_ref().is_none() {
            return Err(Error::internal_error("failed to build model"));
        }
        Ok(Self { handle, model_buffer: mem::ManuallyDrop::new(model_buffer) })
    }

    // pub fn build_from_model(model: &Model) -> Result<Self> {
    //     FlatBufferModel::build_from_buffer(model.to_buffer())
    // }

    pub fn buffer(&self) -> &[u8] {
        self.model_buffer.as_ref()
    }

    pub fn release_buffer(mut self) -> std::borrow::Cow<'static, [u8]> {
        mem::replace(&mut self.model_buffer, Vec::new().into())
    }

    pub fn cxx_flatbuffer_model(&self) -> &super::cxx::FlatBufferModel {
        self.handle.as_ref().unwrap()
    }
}
