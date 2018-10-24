use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

use failure::Fallible;

use interpreter::Interpreter;
use op_resolver::OpResolver;

cpp!{{
    #include "tensorflow/contrib/lite/model.h"
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite;
}}

pub struct FlatBufferModel {
    handle: *mut c_void,
}

impl Drop for FlatBufferModel {
    fn drop(&mut self) {
        let handle = self.handle;
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
        let handle = unsafe {
            cpp!([path as "const char*"] -> *mut c_void as "void*" {
                return FlatBufferModel::BuildFromFile(path).release();
            })
        };
        Ok(FlatBufferModel { handle })
    }
}

pub struct InterpreterBuilder<'a> {
    handle: *mut c_void,
    phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for InterpreterBuilder<'a> {
    fn drop(&mut self) {
        let handle = self.handle;
        unsafe {
            cpp!([handle as "InterpreterBuilder*"] {
                delete handle;
            });
        }
    }
}

impl<'a> InterpreterBuilder<'a> {
    pub fn new<T: OpResolver>(model: &'a FlatBufferModel, resolver: &'a T) -> Fallible<Self> {
        let model_handle = model.handle;
        let resolver_handle = resolver.get_resolver_handle();
        let handle = unsafe {
            cpp!([model_handle as "const FlatBufferModel*",
                  resolver_handle as "const OpResolver*"
            ] -> *mut c_void as "void*" {
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
        let handle = unsafe {
            cpp!([builder as "InterpreterBuilder*"] -> *mut c_void as "void*" {
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
