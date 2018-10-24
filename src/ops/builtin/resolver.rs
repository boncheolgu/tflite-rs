use std::os::raw::c_void;

use op_resolver::OpResolver;

cpp!{{
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite::ops::builtin;
}}

pub struct Resolver {
    handle: *mut c_void,
}

impl Drop for Resolver {
    fn drop(&mut self) {
        let handle = self.handle;
        unsafe {
            cpp!([handle as "BuiltinOpResolver*"] {
                delete handle;
            });
        }
    }
}

impl OpResolver for Resolver {
    fn get_resolver_handle(&self) -> *mut c_void {
        self.handle
    }
}

impl Resolver {
    pub fn new() -> Self {
        let handle = unsafe {
            cpp!([] -> *mut c_void as "void*" {
                return new BuiltinOpResolver();
            })
        };
        Self { handle }
    }
}
