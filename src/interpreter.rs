use std::slice;

use failure::Fallible;
use libc::{c_int, size_t};

use bindings;

cpp!{{
    #include "tensorflow/contrib/lite/interpreter.h"
    #include "tensorflow/contrib/lite/optional_debug_tools.h"

    using namespace tflite;
}}

type TensorIndex = c_int;

pub struct Interpreter<'a> {
    pub(crate) handle: *mut bindings::Interpreter,
    pub(crate) phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for Interpreter<'a> {
    fn drop(&mut self) {
        let handle = self.handle;
        unsafe {
            cpp!([handle as "Interpreter*"] {
                delete handle;
            });
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn allocate_tensors(&mut self) -> Fallible<()> {
        let interpreter = self.handle;
        let result = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->AllocateTensors() == kTfLiteOk;
            })
        };
        ensure!(result, "Interpreter::allocate_tensors failed");
        Ok(())
    }

    pub fn print_state(&self) {
        let interpreter = self.handle;
        unsafe {
            cpp!([interpreter as "Interpreter*"] {
                PrintInterpreterState(interpreter);
            })
        };
    }

    pub fn invoke(&mut self) -> Fallible<()> {
        let interpreter = self.handle;
        let result = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->Invoke() == kTfLiteOk;
            })
        };
        ensure!(result, "Interpreter::allocate_tensors failed");
        Ok(())
    }

    pub fn inputs(&self) -> &[TensorIndex] {
        let interpreter = self.handle;
        let mut count: size_t = 0;
        let ptr = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                mut count as "size_t"
            ] -> *const TensorIndex as "const int*" {
                const auto& inputs = interpreter->inputs();
                count = inputs.size();
                return inputs.data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, count) }
    }

    pub fn outputs(&self) -> &[TensorIndex] {
        let interpreter = self.handle;
        let mut count: size_t = 0;
        let ptr = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                mut count as "size_t"
            ] -> *const TensorIndex as "const int*" {
                const auto& outputs = interpreter->outputs();
                count = outputs.size();
                return outputs.data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, count) }
    }
}
