use op_resolver::OpResolver;

use bindings;

cpp! {{
    #include "tensorflow/contrib/lite/kernels/register.h"

    using namespace tflite::ops::builtin;
}}

pub struct Resolver {
    handle: Box<bindings::OpResolver>,
}

impl Drop for Resolver {
    #[allow(clippy::useless_transmute, clippy::forget_copy)]
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, Default::default());
        let handle = Box::into_raw(handle);
        unsafe {
            cpp!([handle as "BuiltinOpResolver*"] {
                delete handle;
            });
        }
    }
}

impl OpResolver for Resolver {
    fn get_resolver_handle(&self) -> &bindings::OpResolver {
        use std::ops::Deref;
        self.handle.deref()
    }
}

impl Default for Resolver {
    #[allow(clippy::forget_copy)]
    fn default() -> Self {
        let handle = unsafe {
            cpp!([] -> *mut bindings::OpResolver as "OpResolver*" {
                return new BuiltinOpResolver();
            })
        };
        let handle = unsafe{Box::from_raw(handle)};
        Self { handle }
    }
}
