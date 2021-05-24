use super::op_resolver::OpResolver;
use super::FlatBufferModel;
use super::Interpreter;
use crate::{Error, Result};
use std::sync::Arc;

pub struct InterpreterBuilder {
    handle: cxx::UniquePtr<super::cxx::InterpreterBuilder>,
    resolver: Arc<dyn OpResolver>,
    model: Arc<FlatBufferModel>,
}

impl InterpreterBuilder {
    pub fn new(model: Arc<FlatBufferModel>, resolver: Arc<dyn OpResolver>) -> Result<Self> {
        let handle = super::cxx::interpreter_builder(
            model.cxx_flatbuffer_model(),
            resolver.get_resolver_handle(),
        );
        if handle.is_null() {
            return Err(Error::internal_error("failed to create InterpreterBuilder"));
        }
        Ok(Self { handle, model, resolver })
    }

    pub fn build(mut self) -> Result<Interpreter> {
        let handle = super::cxx::build_model(self.handle.as_mut().unwrap());
        if handle.is_null() {
            return Err(Error::internal_error("failed to build"));
        }
        Interpreter::new(handle, self.model, self.resolver)
    }

    pub fn build_with_threads(mut self, threads: u8) -> Result<Interpreter> {
        let handle = super::cxx::build_model_with_threads(self.handle.as_mut().unwrap(), threads);
        if handle.is_null() {
            return Err(Error::internal_error("failed to build with threads"));
        }
        Interpreter::new(handle, self.model, self.resolver)
    }
}
