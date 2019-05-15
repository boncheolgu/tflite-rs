use std::ffi::CString;
use std::path::Path;

use failure::Fallible;

use bindings;
use interpreter::Interpreter;
use op_resolver::OpResolver;

cpp! {{
    #include "tensorflow/contrib/lite/model.h"
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite;
}}

#[derive(Default)]
pub struct FlatBufferModel {
    handle: Box<bindings::FlatBufferModel>,
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
        let path_str = CString::new(path.as_ref().to_str().unwrap())?;
        let path = path_str.as_ptr();

        #[allow(clippy::forget_copy)]
        let handle = unsafe {
            cpp!([path as "const char*"] -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::VerifyAndBuildFromFile(path).release();
            })
        };
        ensure!(!handle.is_null(), "Building FlatBufferModel failed.");
        let handle = unsafe{Box::from_raw(handle)};
        Ok(FlatBufferModel { handle })
    }

    pub fn build_from_buffer(buffer: &[u8]) -> Fallible<Self> {
        let ptr = buffer.as_ptr();
        let size = buffer.len();

        #[allow(clippy::forget_copy)]
        let handle = unsafe {
            cpp!([ptr as "const char*", size as "size_t"]
                  -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::BuildFromBuffer(ptr, size).release();
            })
        };
        ensure!(!handle.is_null(), "Building FlatBufferModel failed.");
        let handle = unsafe{Box::from_raw(handle)};
        Ok(FlatBufferModel { handle })
    }
}

pub struct InterpreterBuilder<'a> {
    handle: Box<bindings::InterpreterBuilder>,
    _model: &'a FlatBufferModel,
    _resolver: &'a OpResolver,
}

impl<'a> Drop for InterpreterBuilder<'a> {
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

impl<'a> InterpreterBuilder<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(model: &'a FlatBufferModel, resolver: &'a OpResolver) -> Fallible<Self> {
        use std::ops::Deref;
        let model_handle = model.handle.deref();
        let resolver_handle = resolver.get_resolver_handle();

        #[allow(clippy::forget_copy)]
        let handle = unsafe {
            cpp!([model_handle as "const FlatBufferModel*",
                  resolver_handle as "const OpResolver*"
            ] -> *mut bindings::InterpreterBuilder as "InterpreterBuilder*" {
                return new InterpreterBuilder(*model_handle, *resolver_handle);
            })
        };
        ensure!(!handle.is_null(), "Creating InterpreterBuilder failed.");
        let handle = unsafe {Box::from_raw(handle)};
        Ok(Self {
            handle,
            _model: model,
            _resolver: resolver,
        })
    }

    pub fn build(mut self) -> Fallible<Interpreter<'a>> {

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
        let handle = unsafe {Box::from_raw(handle)};
        Ok(Interpreter::new(handle, self))
    }
}
