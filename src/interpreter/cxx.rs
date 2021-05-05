pub use self::interpreter::*;

#[cxx::bridge]
mod interpreter {
    unsafe extern "C++" {
        include!("tflite/cxx/include/interpreter.h");
        pub type FlatBufferModel;
        pub fn flatbuffer_from_slice(buffer: &[u8]) -> UniquePtr<FlatBufferModel>;
        pub type OpResolver;
        pub type BuiltinOpResolver;
        pub fn builtin_op_resolver() -> UniquePtr<BuiltinOpResolver>;
        pub fn builtin_op_resolver_to_op_resolver(
            builtin_op_resolver: &BuiltinOpResolver,
        ) -> &OpResolver;
        pub type InterpreterBuilder;
        pub fn interpreter_builder(
            model: &FlatBufferModel,
            op_resolver: &OpResolver,
        ) -> UniquePtr<InterpreterBuilder>;
        pub type Interpreter;
        pub fn build_model(builder: Pin<&mut InterpreterBuilder>) -> UniquePtr<Interpreter>;
        pub fn build_model_with_threads(
            builder: Pin<&mut InterpreterBuilder>,
            threads: u8,
        ) -> UniquePtr<Interpreter>;
        pub fn interpreter_allocate_tensors(interpreter: Pin<&mut Interpreter>) -> bool;
        pub fn interpreter_print_state(interpreter: &Interpreter);
        pub fn interpreter_invoke(interpreter: Pin<&mut Interpreter>) -> bool;
        pub fn interpreter_set_num_threads(interpreter: Pin<&mut Interpreter>, threads: i32);
        pub fn interpreter_inputs(interpreter: &Interpreter) -> &[i32];
        pub fn interpreter_outputs(interpreter: &Interpreter) -> &[i32];
        pub fn interpreter_variables(interpreter: &Interpreter) -> &[i32];
        pub fn interpreter_tensors_size(interpreter: &Interpreter) -> usize;
        pub fn interpreter_nodes_size(interpreter: &Interpreter) -> usize;
        pub fn interpreter_add_tensors(
            interpreter: Pin<&mut Interpreter>,
            count: usize,
        ) -> Result<i32>;
        pub fn interpreter_set_inputs(interpreter: Pin<&mut Interpreter>, inputs: &[i32]) -> bool;
        pub fn interpreter_set_outputs(interpreter: Pin<&mut Interpreter>, outputs: &[i32])
            -> bool;
        pub fn interpreter_set_variables(
            interpreter: Pin<&mut Interpreter>,
            variables: &[i32],
        ) -> bool;
        pub type TfLiteType = crate::bindings::TfLiteType;
        pub type TfLiteQuantization = crate::bindings::TfLiteQuantization;
        pub fn interpreter_set_tensor_parameters_read_write(
            interpreter: Pin<&mut Interpreter>,
            tensor_index: i32,
            element_type: &TfLiteType,
            name: &str,
            dims: &[usize],
            quantization: &TfLiteQuantization,
            is_variable: bool,
        ) -> bool;
        pub fn interpreter_tensor_data(
            interpreter: &Interpreter,
            tflite_type: TfLiteType,
            tensor_index: i32,
        ) -> Result<&[u8]>;
        pub fn interpreter_tensor_data_mut(
            interpreter: Pin<&mut Interpreter>,
            tflite_type: TfLiteType,
            tensor_index: i32,
        ) -> Result<&mut [u8]>;
        pub fn interpreter_tensor_buffer(
            interpreter: &Interpreter,
            tensor_index: i32,
        ) -> Result<&[u8]>;
        pub fn interpreter_tensor_buffer_mut(
            interpreter: Pin<&mut Interpreter>,
            tensor_index: i32,
        ) -> Result<&mut [u8]>;
    }
}

unsafe impl cxx::ExternType for crate::bindings::TfLiteType {
    type Id = cxx::type_id!("TfLiteType");
    type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for crate::bindings::TfLiteQuantization {
    type Id = cxx::type_id!("TfLiteQuantization");
    type Kind = cxx::kind::Trivial;
}
