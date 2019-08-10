use std::fs::File;
use std::io::Read;
use std::path::Path;

use failure::Fallible;

use crate::bindings;
use crate::interpreter::Interpreter;
use crate::op_resolver::OpResolver;
use maybe_owned::MaybeOwned;

cpp! {{
    #include "tensorflow/lite/model.h"
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite;
}}

#[derive(Default)]
pub struct FlatBufferModel {
    handle: Box<bindings::FlatBufferModel>,
    model_buffer: Vec<u8>,
}

impl Drop for FlatBufferModel {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, Default::default());
        let handle = Box::into_raw(handle);

        #[allow(clippy::forget_copy, clippy::useless_transmute)]
        unsafe {
            cpp!([handle as "FlatBufferModel*"] {
                delete handle;
            });
        }
    }
}

impl FlatBufferModel {
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
        ensure!(!handle.is_null(), "Building FlatBufferModel failed.");
        let handle = unsafe { Box::from_raw(handle) };
        Ok(FlatBufferModel {
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
        let handle = std::mem::replace(&mut self.handle, Default::default());
        let handle = Box::into_raw(handle);
        #[allow(clippy::forget_copy, clippy::useless_transmute)]
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
    pub fn new<M: Into<MaybeOwned<'a, FlatBufferModel>>>(model: M, resolver: Op) -> Fallible<Self> {
        use std::ops::Deref;
        let model = model.into();
        let handle = {
            let model_handle = model.as_ref().handle.deref();
            let resolver_handle = resolver.get_resolver_handle();

            #[allow(clippy::forget_copy)]
            unsafe {
                cpp!([model_handle as "const FlatBufferModel*",
                    resolver_handle as "const OpResolver*"
                ] -> *mut bindings::InterpreterBuilder as "InterpreterBuilder*" {
                    return new InterpreterBuilder(*model_handle, *resolver_handle);
                })
            }
        };
        ensure!(!handle.is_null(), "Creating InterpreterBuilder failed.");
        let handle = unsafe { Box::from_raw(handle) };
        Ok(Self {
            handle,
            _model: model,
            _resolver: resolver,
        })
    }

    pub fn build(mut self) -> Fallible<Interpreter<'a, Op>> {
        #[allow(clippy::forget_copy)]
        let handle = {
            let builder = &mut *self.handle;
            unsafe {
                cpp!([builder as "InterpreterBuilder*"] -> *mut bindings::Interpreter as "Interpreter*" {
                    std::unique_ptr<Interpreter> interpreter;
                    (*builder)(&interpreter);
                    return interpreter.release();
                })
            }
        };
        ensure!(!handle.is_null(), "Building Interpreter failed.");
        let handle = unsafe { Box::from_raw(handle) };
        Ok(Interpreter::new(handle, self))
    }
}
