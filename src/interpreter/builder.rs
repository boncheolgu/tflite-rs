use maybe_owned::MaybeOwned;

use super::op_resolver::OpResolver;
use super::FlatBufferModel;
use super::Interpreter;
use crate::bindings::tflite as bindings;
use crate::{Error, Result};

cpp! {{
    #include "tensorflow/lite/model.h"
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite;
}}

pub struct InterpreterBuilder<'a, Op>
where
    Op: OpResolver,
{
    handle: Box<bindings::InterpreterBuilder>,
    _model: MaybeOwned<'a, FlatBufferModel>,
    _resolver: Op,
}

impl<'a, Op> Drop for InterpreterBuilder<'a, Op>
where
    Op: OpResolver,
{
    fn drop(&mut self) {
        let handle = Box::into_raw(std::mem::take(&mut self.handle));
        #[allow(clippy::forgetting_copy_types, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([handle as "InterpreterBuilder*"] {
                delete handle;
            });
        }
    }
}

impl<'a, Op> InterpreterBuilder<'a, Op>
where
    Op: OpResolver,
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new<M: Into<MaybeOwned<'a, FlatBufferModel>>>(model: M, resolver: Op) -> Result<Self> {
        use std::ops::Deref;
        let model = model.into();
        let handle = {
            let model_handle = model.as_ref().handle.deref() as *const _;
            let resolver_handle = resolver.get_resolver_handle() as *const _;

            #[allow(clippy::forgetting_copy_types, deprecated)]
            unsafe {
                cpp!([model_handle as "const FlatBufferModel*",
                    resolver_handle as "const OpResolver*"
                ] -> *mut bindings::InterpreterBuilder as "InterpreterBuilder*" {
                    return new InterpreterBuilder(*model_handle, *resolver_handle);
                })
            }
        };
        if handle.is_null() {
            return Err(Error::InternalError("failed to create InterpreterBuilder".to_string()));
        }
        let handle = unsafe { Box::from_raw(handle) };
        Ok(Self { handle, _model: model, _resolver: resolver })
    }

    pub fn build(mut self) -> Result<Interpreter<'a, Op>> {
        #[allow(clippy::forgetting_copy_types, deprecated)]
        let handle = {
            let builder = (&mut *self.handle) as *mut _;
            unsafe {
                cpp!([builder as "InterpreterBuilder*"] -> *mut bindings::Interpreter as "Interpreter*" {
                    std::unique_ptr<Interpreter> interpreter;
                    (*builder)(&interpreter);
                    return interpreter.release();
                })
            }
        };
        if handle.is_null() {
            return Err(Error::InternalError("failed to build".to_string()));
        }
        Interpreter::new(handle, self)
    }

    pub fn build_with_threads(
        mut self,
        threads: std::os::raw::c_int,
    ) -> Result<Interpreter<'a, Op>> {
        #[allow(clippy::forgetting_copy_types, deprecated)]
        let handle = {
            let builder = (&mut *self.handle) as *mut _;
            #[allow(clippy::transmute_num_to_bytes)]
            unsafe {
                cpp!([builder as "InterpreterBuilder*", threads as "int"] -> *mut bindings::Interpreter as "Interpreter*" {
                    std::unique_ptr<Interpreter> interpreter;
                    (*builder)(&interpreter, threads);
                    return interpreter.release();
                })
            }
        };
        if handle.is_null() {
            return Err(Error::InternalError("failed to build with threads".to_string()));
        }
        Interpreter::new(handle, self)
    }
}
