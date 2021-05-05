mod builder;
pub mod context;
mod cxx;
mod fbmodel;
pub mod op_resolver;
pub mod ops;

use crate::{Error, Result};
pub use builder::InterpreterBuilder;
use context::{ElemKindOf, ElementKind};
pub use fbmodel::FlatBufferModel;
use op_resolver::OpResolver;
use std::pin::Pin;
use std::sync::Arc;
use std::{mem, slice};

pub type TensorIndex = i32;

pub struct Interpreter {
    handle: ::cxx::UniquePtr<cxx::Interpreter>,
    _model: Arc<FlatBufferModel>,
    _op_resolver: Arc<dyn OpResolver>,
}

impl Interpreter {
    #[track_caller]
    fn handle(&self) -> &cxx::Interpreter {
        self.handle.as_ref().unwrap()
    }
    #[track_caller]
    fn handle_mut(&mut self) -> Pin<&mut cxx::Interpreter> {
        self.handle.as_mut().unwrap()
    }
    pub(crate) fn new(
        handle: ::cxx::UniquePtr<cxx::Interpreter>,
        model: Arc<FlatBufferModel>,
        op_resolver: Arc<dyn OpResolver>,
    ) -> Result<Self> {
        if handle.is_null() {
            return Err(Error::internal_error("Invalid interpreter"));
        }
        let mut interpreter = Self { handle, _model: model, _op_resolver: op_resolver };
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
        cxx::interpreter_allocate_tensors(self.handle_mut())
            .then(|| ())
            .ok_or_else(|| Error::internal_error("failed to allocate tensors"))
    }

    /// Prints a dump of what tensors and what nodes are in the interpreter.
    pub fn print_state(&self) {
        cxx::interpreter_print_state(self.handle());
    }

    /// Invoke the interpreter (run the whole graph in dependency order).
    pub fn invoke(&mut self) -> Result<()> {
        cxx::interpreter_invoke(self.handle_mut())
            .then(|| ())
            .ok_or_else(|| Error::internal_error("failed to invoke interpreter"))
    }

    /// Sets the number of threads available to the interpreter
    /// `threads` should be >= -1
    /// Passing in a value of -1 will let the interpreter set the number
    /// of threads available to itself.
    ///
    /// Note that increasing the number of threads does not always speed up inference
    pub fn set_num_threads(&mut self, threads: i32) {
        cxx::interpreter_set_num_threads(self.handle_mut(), threads);
        // println!("Set num threads to {}", threads);
    }

    /// Read only access to list of inputs.
    pub fn inputs(&self) -> &[TensorIndex] {
        cxx::interpreter_inputs(self.handle())
    }

    /// Read only access to list of outputs.
    pub fn outputs(&self) -> &[TensorIndex] {
        cxx::interpreter_outputs(self.handle())
    }

    /// Read only access to list of variable tensors.
    pub fn variables(&self) -> &[TensorIndex] {
        cxx::interpreter_variables(self.handle())
    }

    /// Return the number of tensors in the model.
    pub fn tensors_size(&self) -> usize {
        cxx::interpreter_tensors_size(self.handle())
    }

    /// Return the number of ops in the model.
    pub fn nodes_size(&self) -> usize {
        cxx::interpreter_nodes_size(self.handle())
    }

    /// Adds `count` tensors, preserving pre-existing Tensor entries.
    /// Return the index of the first new tensor.
    pub fn add_tensors(&mut self, count: usize) -> Result<TensorIndex> {
        cxx::interpreter_add_tensors(self.handle_mut(), count)
            .map_err(|_| Error::internal_error("failed to add tensors"))
    }

    /// Provide a list of tensor indexes that are inputs to the model.
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_inputs(&mut self, inputs: &[TensorIndex]) -> Result<()> {
        cxx::interpreter_set_inputs(self.handle_mut(), inputs)
            .then(|| ())
            .ok_or_else(|| Error::internal_error("failed to set inputs"))
    }

    /// Provide a list of tensor indexes that are outputs to the model
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_outputs(&mut self, outputs: &[TensorIndex]) -> Result<()> {
        cxx::interpreter_set_outputs(self.handle_mut(), outputs)
            .then(|| ())
            .ok_or_else(|| Error::internal_error("failed to set outputs"))
    }

    /// Provide a list of tensor indexes that are variable tensors.
    /// Each index is bound check and this modifies the consistent_ flag of the
    /// interpreter.
    pub fn set_variables(&mut self, variables: &[TensorIndex]) -> Result<()> {
        cxx::interpreter_set_variables(self.handle_mut(), variables)
            .then(|| ())
            .ok_or_else(|| Error::internal_error("failed to set variables"))
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn set_tensor_parameters_read_write(
        &mut self,
        tensor_index: TensorIndex,
        element_type: ElementKind,
        name: &str,
        dims: &[usize],
        quantization: crate::bindings::TfLiteQuantization,
        is_variable: bool,
    ) -> Result<()> {
        cxx::interpreter_set_tensor_parameters_read_write(
            self.handle_mut(),
            tensor_index,
            &element_type,
            name,
            dims,
            &quantization,
            is_variable,
        )
        .then(|| ())
        .ok_or_else(|| Error::internal_error("failed to set tensor parameters"))
    }

    // fn tensor_inner(&self, tensor_index: TensorIndex) -> Option<&bindings::TfLiteTensor> {
    //     let interpreter = self.handle();
    //
    //     #[allow(clippy::forget_copy, deprecated)]
    //     unsafe {
    //         cpp!([
    //             interpreter as "const Interpreter*",
    //             tensor_index as "int"
    //         ] -> *const bindings::TfLiteTensor as "const TfLiteTensor*" {
    //             return interpreter->tensor(tensor_index);
    //         })
    //         .as_ref()
    //     }
    // }

    // pub fn tensor_info(&self, tensor_index: TensorIndex) -> Option<TensorInfo> {
    //     Some(self.tensor_inner(tensor_index)?.into())
    // }

    pub fn tensor_data<T>(&self, tensor_index: TensorIndex) -> Result<&[T]>
    where
        T: ElemKindOf,
    {
        let bytes = cxx::interpreter_tensor_data(self.handle(), T::elem_kind_of(), tensor_index)?;

        assert_eq!(
            bytes.as_ptr() as usize % mem::align_of::<T>(),
            0,
            "Type is not properly aligned!"
        );

        Ok(unsafe {
            slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / mem::size_of::<T>())
        })
    }

    pub fn tensor_data_mut<T>(&mut self, tensor_index: TensorIndex) -> Result<&mut [T]>
    where
        T: ElemKindOf,
    {
        let bytes =
            cxx::interpreter_tensor_data_mut(self.handle_mut(), T::elem_kind_of(), tensor_index)?;

        assert_eq!(
            bytes.as_ptr() as usize % mem::align_of::<T>(),
            0,
            "Type is not properly aligned!"
        );

        Ok(unsafe {
            slice::from_raw_parts_mut(bytes.as_ptr() as *mut T, bytes.len() / mem::size_of::<T>())
        })
    }

    pub fn tensor_buffer(&self, tensor_index: TensorIndex) -> Option<&[u8]> {
        cxx::interpreter_tensor_buffer(self.handle(), tensor_index).ok()
    }

    pub fn tensor_buffer_mut(&mut self, tensor_index: TensorIndex) -> Option<&mut [u8]> {
        cxx::interpreter_tensor_buffer_mut(self.handle_mut(), tensor_index).ok()
    }
}

// #[allow(unused)]
// fn threadsafe_types() {
//     fn send_sync<T: Send + Sync>() {}
//     send_sync::<FlatBufferModel>();
//     send_sync::<BuiltinOpResolver>();
//     send_sync::<InterpreterBuilder<BuiltinOpResolver>>();
//     send_sync::<Interpreter<BuiltinOpResolver>>();
// }
