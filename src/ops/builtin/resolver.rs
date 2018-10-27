use op_resolver::OpResolver;

use bindings;

cpp!{{
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite::ops::builtin;
}}

pub struct Resolver {
    handle: *mut bindings::OpResolver,
}

impl Drop for Resolver {
    #[cfg_attr(
        feature = "cargo-clippy",
        allow(useless_transmute, forget_copy)
    )]
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
    fn get_resolver_handle(&self) -> *mut bindings::OpResolver {
        self.handle
    }
}

impl Default for Resolver {
    #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
    fn default() -> Self {
        let handle = unsafe {
            cpp!([] -> *mut bindings::OpResolver as "OpResolver*" {
                return new BuiltinOpResolver();
            })
        };
        Self { handle }
    }
}
