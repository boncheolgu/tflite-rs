use failure::Fallible;
use maybe_owned::MaybeOwned;

use super::op_resolver::OpResolver;
use super::FlatBufferModel;
use super::Interpreter;
use crate::bindings::tflite as bindings;

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
        Interpreter::new(handle, self)
    }

    pub fn build_with_threads(
        mut self,
        threads: std::os::raw::c_int,
    ) -> Fallible<Interpreter<'a, Op>> {
        #[allow(clippy::forget_copy)]
        let handle = {
            let builder = &mut *self.handle;
            unsafe {
                cpp!([builder as "InterpreterBuilder*", threads as "int"] -> *mut bindings::Interpreter as "Interpreter*" {
                    std::unique_ptr<Interpreter> interpreter;
                    (*builder)(&interpreter, threads);
                    return interpreter.release();
                })
            }
        };
        Interpreter::new(handle, self)
    }
}
