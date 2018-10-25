use failure::Fallible;

use bindings;

cpp!{{
    #include "tensorflow/contrib/lite/interpreter.h"
    #include "tensorflow/contrib/lite/optional_debug_tools.h"

    using namespace tflite;
}}

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
}
