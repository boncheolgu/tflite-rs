use maybe_owned::MaybeOwned;

use super::op_resolver::OpResolver;
use super::FlatBufferModel;
use super::Interpreter;
use crate::bindings::tflite as bindings;
use crate::{Error, Result};
use std::sync::Arc;

pub struct InterpreterBuilder {
    handle: cxx::UniquePtr<super::cxx::InterpreterBuilder>,
    _resolver: Arc<dyn OpResolver>,
    pub(super) model: Arc<FlatBufferModel>,
}

impl InterpreterBuilder {
    pub fn new(model: Arc<FlatBufferModel>, resolver: Arc<dyn OpResolver>) -> Result<Self> {
        let handle = super::cxx::interpreter_builder(
            model.cxx_flatbuffer_model(),
            resolver.get_resolver_handle(),
        );
        if handle.is_null() {
            return Err(Error::InternalError("failed to create InterpreterBuilder".into()));
        }
        Ok(Self { handle, model, _resolver: resolver })
    }

    pub fn build(mut self) -> Result<Interpreter> {
        let handle = super::cxx::build_model(self.handle.as_mut().unwrap());
        if handle.is_null() {
            return Err(Error::InternalError("failed to build".into()));
        }
        Interpreter::new(handle, self.model)
    }

    pub fn build_with_threads(mut self, threads: u8) -> Result<Interpreter> {
        let handle = super::cxx::build_model_with_threads(self.handle.as_mut().unwrap(), threads);
        if handle.is_null() {
            return Err(Error::InternalError("failed to build with threads".into()));
        }
        Interpreter::new(handle, self.model)
    }
}
