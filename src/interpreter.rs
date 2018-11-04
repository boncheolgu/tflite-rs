use std::mem::size_of;
use std::slice;

use failure::Fallible;
use libc::{c_int, size_t};

use bindings;
use context::{ElemKindOf, TensorInfo};

cpp!{{
    #include "tensorflow/contrib/lite/interpreter.h"
    #include "tensorflow/contrib/lite/optional_debug_tools.h"

    using namespace tflite;
}}

pub type TensorIndex = c_int;

pub struct Interpreter<'a> {
    pub(crate) handle: *mut bindings::Interpreter,
    pub(crate) phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for Interpreter<'a> {
    fn drop(&mut self) {
        let handle = self.handle;

        #[cfg_attr(
            feature = "cargo-clippy",
            allow(forget_copy, useless_transmute)
        )]
        unsafe {
            cpp!([handle as "Interpreter*"] {
                delete handle;
            });
        }
    }
}

impl<'a> Interpreter<'a> {
    /// Update allocations for all tensors. This will redim dependent tensors using
    /// the input tensor dimensionality as given. This is relatively expensive.
    /// If you know that your sizes are not changing, you need not call this.
    pub fn allocate_tensors(&mut self) -> Fallible<()> {
        let interpreter = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let result = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->AllocateTensors() == kTfLiteOk;
            })
        };
        ensure!(result, "Interpreter::allocate_tensors failed");
        Ok(())
    }

    /// Prints a dump of what tensors and what nodes are in the interpreter.
    pub fn print_state(&self) {
        let interpreter = self.handle;

        #[cfg_attr(
            feature = "cargo-clippy",
            allow(forget_copy, useless_transmute)
        )]
        unsafe {
            cpp!([interpreter as "Interpreter*"] {
                PrintInterpreterState(interpreter);
            })
        };
    }

    /// Invoke the interpreter (run the whole graph in dependency order).
    pub fn invoke(&mut self) -> Fallible<()> {
        let interpreter = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let result = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->Invoke() == kTfLiteOk;
            })
        };
        ensure!(result, "Interpreter::allocate_tensors failed");
        Ok(())
    }

    /// Read only access to list of inputs.
    pub fn inputs(&self) -> &[TensorIndex] {
        let interpreter = self.handle;
        let mut count: size_t = 0;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let ptr = unsafe {
            cpp!([
                interpreter as "const Interpreter*",
                mut count as "size_t"
            ] -> *const TensorIndex as "const int*" {
                const auto& inputs = interpreter->inputs();
                count = inputs.size();
                return inputs.data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, count) }
    }

    /// Read only access to list of outputs.
    pub fn outputs(&self) -> &[TensorIndex] {
        let interpreter = self.handle;
        let mut count: size_t = 0;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let ptr = unsafe {
            cpp!([
                interpreter as "const Interpreter*",
                mut count as "size_t"
            ] -> *const TensorIndex as "const int*" {
                const auto& outputs = interpreter->outputs();
                count = outputs.size();
                return outputs.data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, count) }
    }

    /// Read only access to list of variable tensors.
    pub fn variables(&self) -> &[TensorIndex] {
        let interpreter = self.handle;
        let mut count: size_t = 0;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let ptr = unsafe {
            cpp!([
                interpreter as "const Interpreter*",
                mut count as "size_t"
            ] -> *const TensorIndex as "const int*" {
                const auto& variables = interpreter->variables();
                count = variables.size();
                return variables.data();
            })
        };
        unsafe { slice::from_raw_parts(ptr, count) }
    }

    /// Return the number of tensors in the model.
    pub fn tensors_size(&self) -> size_t {
        let interpreter = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        unsafe {
            cpp!([interpreter as "const Interpreter*"] -> size_t as "size_t" {
                return interpreter->tensors_size();
            })
        }
    }

    /// Return the number of ops in the model.
    pub fn nodes_size(&self) -> size_t {
        let interpreter = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        unsafe {
            cpp!([interpreter as "const Interpreter*"] -> size_t as "size_t" {
                return interpreter->nodes_size();
            })
        }
    }

    fn tensor_inner(&self, tensor_index: TensorIndex) -> Fallible<&bindings::TfLiteTensor> {
        let interpreter = self.handle;

        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        let ptr = unsafe {
            cpp!([
                interpreter as "const Interpreter*",
                tensor_index as "int"
            ] -> *const bindings::TfLiteTensor as "const TfLiteTensor*" {
                return interpreter->tensor(tensor_index);
            })
        };
        ensure!(!ptr.is_null(), "Invalid tensor index");
        Ok(unsafe { &*ptr })
    }

    pub fn tensor_info(&self, tensor_index: TensorIndex) -> Fallible<TensorInfo> {
        Ok(self.tensor_inner(tensor_index)?.into())
    }

    pub fn tensor_data<T>(&self, tensor_index: TensorIndex) -> Fallible<&[T]>
    where
        T: ElemKindOf,
    {
        let inner = self.tensor_inner(tensor_index)?;
        let tensor_info: TensorInfo = inner.into();

        ensure!(
            tensor_info.element_kind == T::elem_kind_of(),
            "Invalid type reference of `{:?}` to the original type `{:?}`",
            T::elem_kind_of(),
            tensor_info.element_kind
        );

        Ok(unsafe {
            slice::from_raw_parts(
                inner.data.raw_const as *const T,
                inner.bytes / size_of::<T>(),
            )
        })
    }

    pub fn tensor_data_mut<T>(&mut self, tensor_index: TensorIndex) -> Fallible<&mut [T]>
    where
        T: ElemKindOf,
    {
        let inner = self.tensor_inner(tensor_index)?;
        let tensor_info: TensorInfo = inner.into();

        ensure!(
            tensor_info.element_kind == T::elem_kind_of(),
            "Invalid type reference of `{:?}` to the original type `{:?}`",
            T::elem_kind_of(),
            tensor_info.element_kind
        );

        Ok(unsafe {
            slice::from_raw_parts_mut(inner.data.raw as *mut T, inner.bytes / size_of::<T>())
        })
    }
}
