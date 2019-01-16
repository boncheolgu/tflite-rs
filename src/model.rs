use std::ffi::CString;
use std::path::Path;

use failure::Fallible;

use bindings;
use interpreter::Interpreter;
use op_resolver::OpResolver;

cpp!{{
    #include "tensorflow/contrib/lite/model.h"
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite;
}}

pub struct FlatBufferModel {
    handle: *mut bindings::FlatBufferModel,
}

impl Drop for FlatBufferModel {
    fn drop(&mut self) {
        let handle = self.handle;

        #[cfg_attr(
            feature = "cargo-clippy",
            allow(clippy::forget_copy, clippy::useless_transmute)
        )]
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

        #[cfg_attr(feature = "cargo-clippy", allow(clippy::forget_copy))]
        let handle = unsafe {
            cpp!([path as "const char*"] -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::VerifyAndBuildFromFile(path).release();
            })
        };
        ensure!(!handle.is_null(), "Building FlatBufferModel failed.");
        Ok(FlatBufferModel { handle })
    }

    pub fn build_from_buffer(buffer: &[u8]) -> Fallible<Self> {
        let ptr = buffer.as_ptr();
        let size = buffer.len();

        #[cfg_attr(feature = "cargo-clippy", allow(clippy::forget_copy))]
        let handle = unsafe {
            cpp!([ptr as "const char*", size as "size_t"]
                  -> *mut bindings::FlatBufferModel as "FlatBufferModel*" {
                return FlatBufferModel::BuildFromBuffer(ptr, size).release();
            })
        };
        ensure!(!handle.is_null(), "Building FlatBufferModel failed.");
        Ok(FlatBufferModel { handle })
    }
}

pub struct InterpreterBuilder<'a> {
    handle: *mut bindings::InterpreterBuilder,
    phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for InterpreterBuilder<'a> {
    fn drop(&mut self) {
        let handle = self.handle;

        #[cfg_attr(
            feature = "cargo-clippy",
            allow(clippy::forget_copy, clippy::useless_transmute)
        )]
        unsafe {
            cpp!([handle as "InterpreterBuilder*"] {
                delete handle;
            });
        }
    }
}

impl<'a> InterpreterBuilder<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::new_ret_no_self))]
    pub fn new<T: OpResolver>(model: &'a FlatBufferModel, resolver: &'a T) -> Fallible<Self> {
        let model_handle = model.handle;
        let resolver_handle = resolver.get_resolver_handle();

        #[cfg_attr(feature = "cargo-clippy", allow(clippy::forget_copy))]
        let handle = unsafe {
            cpp!([model_handle as "const FlatBufferModel*",
                  resolver_handle as "const OpResolver*"
            ] -> *mut bindings::InterpreterBuilder as "InterpreterBuilder*" {
                return new InterpreterBuilder(*model_handle, *resolver_handle);
            })
        };
        ensure!(!handle.is_null(), "Creating InterpreterBuilder failed.");
        Ok(Self {
            handle,
            phantom: Default::default(),
        })
    }

    pub fn build(&self) -> Fallible<Interpreter> {
        let builder = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(clippy::forget_copy))]
        let handle = unsafe {
            cpp!([builder as "InterpreterBuilder*"] -> *mut bindings::Interpreter as "Interpreter*" {
                std::unique_ptr<Interpreter> interpreter;
                (*builder)(&interpreter);
                return interpreter.release();
            })
        };
        ensure!(!handle.is_null(), "Building Interpreter failed.");
        Ok(Interpreter {
            handle,
            phantom: Default::default(),
        })
    }
}
