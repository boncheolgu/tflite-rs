mod builder;
pub mod context;
mod fbmodel;
pub mod op_resolver;
pub mod ops;

use std::mem;
use std::slice;

use libc::{c_int, size_t};

use crate::{bindings, Error, Result};
pub use builder::InterpreterBuilder;
use context::{ElemKindOf, ElementKind, QuantizationParams, TensorInfo};
pub use fbmodel::FlatBufferModel;
use op_resolver::OpResolver;

cpp! {{
    #include "tensorflow/lite/interpreter.h"
    #include "tensorflow/lite/optional_debug_tools.h"

    using namespace tflite;
}}

pub type TensorIndex = c_int;

pub struct Interpreter<'a, Op>
where
    Op: OpResolver,
{
    handle: Box<bindings::tflite::Interpreter>,
    _builder: InterpreterBuilder<'a, Op>,
}

impl<'a, Op> Drop for Interpreter<'a, Op>
where
    Op: OpResolver,
{
    fn drop(&mut self) {
        let handle = Box::into_raw(mem::take(&mut self.handle));
        #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([handle as "Interpreter*"] {
                delete handle;
            });
        }
    }
}

impl<'a, Op> Interpreter<'a, Op>
where
    Op: OpResolver,
{
    fn handle(&self) -> &bindings::tflite::Interpreter {
        use std::ops::Deref;
        self.handle.deref()
    }
    fn handle_mut(&mut self) -> &mut bindings::tflite::Interpreter {
        use std::ops::DerefMut;
        self.handle.deref_mut()
    }
    pub(crate) fn new(
        handle: *mut bindings::tflite::Interpreter,
        builder: InterpreterBuilder<'a, Op>,
    ) -> Result<Self> {
        if handle.is_null() {
            return Err(Error::internal_error("failed to create interpreter"));
        }
        let handle = unsafe { Box::from_raw(handle) };
        let mut interpreter = Self { handle, _builder: builder };
        // # Safety
        // Always allocate tensors so we don't get into a state
        // where we try to read from or write to unallocated memory
        // without doing this it is possible to have undefined behavior
        // outside of an unsafe block
        interpreter.allocate_tensors()?;
        Ok(interpreter)
    }
    /// Update allocations for all tensors. This will redim dependent tensors using
    /// the input tensor dimensionality as given. This is relatively expensive.
    /// If you know that your sizes are not changing, you need not call this.
    pub fn allocate_tensors(&mut self) -> Result<()> {
        let interpreter = self.handle_mut();

        #[allow(clippy::forget_copy, deprecated)]
        let r = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->AllocateTensors() == kTfLiteOk;
            })
        };
        if r {
            Ok(())
        } else {
            Err(Error::internal_error("failed to allocate tensors"))
        }
    }

    /// Prints a dump of what tensors and what nodes are in the interpreter.
    pub fn print_state(&self) {
        let interpreter = self.handle();

        #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([interpreter as "Interpreter*"] {
                PrintInterpreterState(interpreter);
            })
        };
    }

    /// Invoke the interpreter (run the whole graph in dependency order).
    pub fn invoke(&mut self) -> Result<()> {
        let interpreter = self.handle_mut();

        #[allow(deprecated)]
        let r = unsafe {
            cpp!([interpreter as "Interpreter*"] -> bool as "bool" {
                return interpreter->Invoke() == kTfLiteOk;
            })
        };
        if r {
            Ok(())
        } else {
            Err(Error::internal_error("failed to invoke interpreter"))
        }
    }

    /// Sets the number of threads available to the interpreter
    /// `threads` should be >= -1
    /// Passing in a value of -1 will let the interpreter set the number
    /// of threads available to itself.
    ///
    /// Note that increasing the number of threads does not always speed up inference
    pub fn set_num_threads(&mut self, threads: c_int) {
        let interpreter = self.handle_mut();

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        unsafe {
            cpp!([interpreter as "Interpreter*", threads as "int"] {
                interpreter->SetNumThreads(threads);
            })
        };
        println!("Set num threads to {}", threads);
    }

    /// Read only access to list of inputs.
    pub fn inputs(&self) -> &[TensorIndex] {
        let interpreter = self.handle();
        let mut count: size_t = 0;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
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
        let interpreter = self.handle();
        let mut count: size_t = 0;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
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
        let interpreter = self.handle();
        let mut count: size_t = 0;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
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
        let interpreter = self.handle();

        #[allow(clippy::forget_copy, deprecated)]
        unsafe {
            cpp!([interpreter as "const Interpreter*"] -> size_t as "size_t" {
                return interpreter->tensors_size();
            })
        }
    }

    /// Return the number of ops in the model.
    pub fn nodes_size(&self) -> size_t {
        let interpreter = self.handle();

        #[allow(clippy::forget_copy, deprecated)]
        unsafe {
            cpp!([interpreter as "const Interpreter*"] -> size_t as "size_t" {
                return interpreter->nodes_size();
            })
        }
    }

    /// Adds `count` tensors, preserving pre-existing Tensor entries.
    /// Return the index of the first new tensor.
    pub fn add_tensors(&mut self, count: size_t) -> Result<TensorIndex> {
        let interpreter = self.handle();
        let mut index: TensorIndex = 0;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let result = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                count as "size_t",
                mut index as "int"
            ] -> bindings::TfLiteStatus as "TfLiteStatus" {
                return interpreter->AddTensors(count, &index);
            })
        };
        if result == bindings::TfLiteStatus::kTfLiteOk {
            Ok(index)
        } else {
            Err(Error::internal_error("failed to add tensors"))
        }
    }

    /// Provide a list of tensor indexes that are inputs to the model.
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_inputs(&mut self, inputs: &[TensorIndex]) -> Result<()> {
        let interpreter = self.handle_mut();
        let ptr = inputs.as_ptr();
        let len = inputs.len() as size_t;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let result = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                ptr as "const int*",
                len as "size_t"
            ] -> bindings::TfLiteStatus as "TfLiteStatus" {
                std::vector<int> inputs(ptr, ptr + len);
                return interpreter->SetInputs(inputs);
            })
        };
        if result == bindings::TfLiteStatus::kTfLiteOk {
            Ok(())
        } else {
            Err(Error::internal_error("failed to set inputs"))
        }
    }

    /// Provide a list of tensor indexes that are outputs to the model
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_outputs(&mut self, outputs: &[TensorIndex]) -> Result<()> {
        let interpreter = self.handle_mut();
        let ptr = outputs.as_ptr();
        let len = outputs.len() as size_t;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let result = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                ptr as "const int*",
                len as "size_t"
            ] -> bindings::TfLiteStatus as "TfLiteStatus" {
                std::vector<int> outputs(ptr, ptr + len);
                return interpreter->SetOutputs(outputs);
            })
        };
        if result == bindings::TfLiteStatus::kTfLiteOk {
            Ok(())
        } else {
            Err(Error::internal_error("failed to set outputs"))
        }
    }

    /// Provide a list of tensor indexes that are variable tensors.
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_variables(&mut self, variables: &[TensorIndex]) -> Result<()> {
        let interpreter = self.handle_mut();
        let ptr = variables.as_ptr();
        let len = variables.len() as size_t;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let result = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                ptr as "const int*",
                len as "size_t"
            ] -> bindings::TfLiteStatus as "TfLiteStatus" {
                std::vector<int> variables(ptr, ptr + len);
                return interpreter->SetVariables(variables);
            })
        };
        if result == bindings::TfLiteStatus::kTfLiteOk {
            Ok(())
        } else {
            Err(Error::internal_error("failed to set variables"))
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn set_tensor_parameters_read_write(
        &mut self,
        tensor_index: TensorIndex,
        element_type: ElementKind,
        name: &str,
        dims: &[usize],
        quantization: QuantizationParams,
        is_variable: bool,
    ) -> Result<()> {
        let interpreter = self.handle_mut();

        let name_ptr = name.as_ptr();
        let name_len = name.len() as size_t;

        let dims: Vec<i32> = dims.iter().map(|x| *x as i32).collect();
        let dims_ptr = dims.as_ptr();
        let dims_len = dims.len() as size_t;

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let result = unsafe {
            cpp!([
                interpreter as "Interpreter*",
                tensor_index as "int",
                element_type as "TfLiteType",
                name_ptr as "const char*",
                name_len as "size_t",
                dims_ptr as "const int*",
                dims_len as "size_t",
                quantization as "TfLiteQuantizationParams",
                is_variable as "bool"
            ] -> bindings::TfLiteStatus as "TfLiteStatus" {
                return interpreter->SetTensorParametersReadWrite(
                    tensor_index, element_type, std::string(name_ptr, name_len).c_str(),
                    dims_len, dims_ptr, quantization, is_variable);
            })
        };
        if result == bindings::TfLiteStatus::kTfLiteOk {
            Ok(())
        } else {
            Err(Error::internal_error("failed to set tensor parameters"))
        }
    }

    fn tensor_inner(&self, tensor_index: TensorIndex) -> Option<&bindings::TfLiteTensor> {
        let interpreter = self.handle();

        #[allow(clippy::forget_copy, deprecated, clippy::transmute_num_to_bytes)]
        let ptr = unsafe {
            cpp!([
                interpreter as "const Interpreter*",
                tensor_index as "int"
            ] -> *const bindings::TfLiteTensor as "const TfLiteTensor*" {
                return interpreter->tensor(tensor_index);
            })
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    pub fn tensor_info(&self, tensor_index: TensorIndex) -> Option<TensorInfo> {
        Some(self.tensor_inner(tensor_index)?.into())
    }

    pub fn tensor_data<T>(&self, tensor_index: TensorIndex) -> Result<&[T]>
    where
        T: ElemKindOf,
    {
        let inner = self
            .tensor_inner(tensor_index)
            .ok_or_else(|| Error::internal_error("invalid tensor index"))?;
        let tensor_info: TensorInfo = inner.into();

        if tensor_info.element_kind != T::elem_kind_of() {
            return Err(Error::InternalError(format!(
                "Invalid type reference of `{:?}` to the original type `{:?}`",
                T::elem_kind_of(),
                tensor_info.element_kind
            )));
        }

        Ok(unsafe {
            slice::from_raw_parts(
                inner.data.raw_const as *const T,
                inner.bytes / mem::size_of::<T>(),
            )
        })
    }

    pub fn tensor_data_mut<T>(&mut self, tensor_index: TensorIndex) -> Result<&mut [T]>
    where
        T: ElemKindOf,
    {
        let inner = self
            .tensor_inner(tensor_index)
            .ok_or_else(|| Error::internal_error("invalid tensor index"))?;
        let tensor_info: TensorInfo = inner.into();

        if tensor_info.element_kind != T::elem_kind_of() {
            return Err(Error::InternalError(format!(
                "Invalid type reference of `{:?}` to the original type `{:?}`",
                T::elem_kind_of(),
                tensor_info.element_kind
            )));
        }

        Ok(unsafe {
            slice::from_raw_parts_mut(inner.data.raw as *mut T, inner.bytes / mem::size_of::<T>())
        })
    }

    pub fn tensor_buffer(&self, tensor_index: TensorIndex) -> Option<&[u8]> {
        let inner = self.tensor_inner(tensor_index)?;

        Some(unsafe { slice::from_raw_parts(inner.data.raw_const as *mut u8, inner.bytes) })
    }

    pub fn tensor_buffer_mut(&mut self, tensor_index: TensorIndex) -> Option<&mut [u8]> {
        let inner = self.tensor_inner(tensor_index)?;

        Some(unsafe { slice::from_raw_parts_mut(inner.data.raw as *mut u8, inner.bytes) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::ops::builtin::BuiltinOpResolver;

    #[test]
    fn threadsafe_types() {
        fn send_sync<T: Send + Sync>(_t: &T) {}
        let model = FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite")
            .expect("Unable to build flatbuffer model");
        send_sync(&model);
        let resolver = Arc::new(BuiltinOpResolver::default());
        send_sync(&resolver);
        let builder = InterpreterBuilder::new(model, resolver).expect("Not able to build builder");
        send_sync(&builder);
        let interpreter = builder.build().expect("Not able to build model");
        send_sync(&interpreter);
    }
}
