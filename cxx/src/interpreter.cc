
#include "tflite/cxx/include/interpreter.h"

class NotImplementedException : public std::logic_error
{
public:
    NotImplementedException () : std::logic_error{"Function not yet implemented."} {}
};

std::unique_ptr<FlatBufferModel> flatbuffer_from_slice(const rust::Slice<const uint8_t> buffer) {
    return FlatBufferModel::BuildFromBuffer(reinterpret_cast<const char*> (buffer.data()), buffer.size());
}

std::unique_ptr<BuiltinOpResolver> builtin_op_resolver() {
    return std::unique_ptr<BuiltinOpResolver>(new BuiltinOpResolver());
}

std::unique_ptr<InterpreterBuilder> interpreter_builder(const FlatBufferModel& model, const OpResolver& op_resolver) {
    return std::unique_ptr<InterpreterBuilder>(new InterpreterBuilder(model, op_resolver));
}

const OpResolver& builtin_op_resolver_to_op_resolver(const BuiltinOpResolver& builtin_op_resolver) {
    return builtin_op_resolver;
}

std::unique_ptr<Interpreter> build_model(InterpreterBuilder& builder) {
    std::unique_ptr<Interpreter> interpreter;
    (builder)(&interpreter);
    return interpreter;
}

std::unique_ptr<Interpreter> build_model_with_threads(InterpreterBuilder& builder, uint8_t threads) {
    std::unique_ptr<Interpreter> interpreter;
    (builder)(&interpreter, threads);
    return interpreter;
}

bool interpreter_allocate_tensors(Interpreter& interpreter) {
    return interpreter.AllocateTensors() == kTfLiteOk;
}

void interpreter_print_state(const Interpreter& interpreter) {
    PrintInterpreterState(const_cast<Interpreter*>(&interpreter));
}

bool interpreter_invoke(Interpreter& interpreter) {
    return interpreter.Invoke() == kTfLiteOk;
}

void interpreter_set_num_threads(Interpreter& interpreter, int32_t threads) {
    interpreter.SetNumThreads(threads);
}

rust::Slice<const int32_t> interpreter_inputs(const Interpreter& interpreter) {
    const auto& inputs = interpreter.inputs();
    return rust::Slice<const int32_t>(inputs.data(), inputs.size());
}

rust::Slice<const int32_t> interpreter_outputs(const Interpreter& interpreter) {
    const auto& outputs = interpreter.outputs();
    return rust::Slice<const int32_t>(outputs.data(), outputs.size());
}

rust::Slice<const int32_t> interpreter_variables(const Interpreter& interpreter) {
    const auto& variables = interpreter.variables();
    return rust::Slice<const int32_t>(variables.data(), variables.size());
}

size_t interpreter_tensors_size(const Interpreter& interpreter) {
    return interpreter.tensors_size();
}

size_t interpreter_nodes_size(const Interpreter& interpreter) {
    return interpreter.nodes_size();
}

int32_t interpreter_add_tensors(Interpreter& interpreter, size_t count) {
    int32_t index = 0;
    if (interpreter.AddTensors(count, &index) != kTfLiteOk) {
        throw std::logic_error{"Add tensors failed"};
    }
    return index;
}

bool interpreter_set_inputs(Interpreter& interpreter, rust::Slice<const int32_t> inputs) {
    std::vector<int> v(inputs.data(), inputs.data() + inputs.size());
    return interpreter.SetInputs(v) == kTfLiteOk;
}

bool interpreter_set_outputs(Interpreter& interpreter, rust::Slice<const int32_t> outputs) {
    std::vector<int> v(outputs.data(), outputs.data() + outputs.size());
        return interpreter.SetOutputs(v) == kTfLiteOk;
}

bool interpreter_set_variables(Interpreter& interpreter, rust::Slice<const int32_t> variables) {
    std::vector<int> v(variables.data(), variables.data() + variables.size());
        return interpreter.SetVariables(v) == kTfLiteOk;
}

bool interpreter_set_tensor_parameters_read_write(Interpreter& interpreter, int32_t tensor_index, const TfLiteType& tflite_type, rust::Str name, rust::Slice<const size_t> dims, const TfLiteQuantization& quantization, bool is_variable) {
    throw NotImplementedException();
}

rust::Slice<const uint8_t> interpreter_tensor_data(const Interpreter& interpreter, TfLiteType tflite_type, int32_t tensor_index) {
    const auto tensor = interpreter.tensor(tensor_index);
    if (!tensor || tensor->type != tflite_type) {
        throw std::logic_error{"asking for wrong type of data"};
    }
    return rust::Slice<const uint8_t>(tensor->data.uint8, tensor->bytes);

}

rust::Slice<uint8_t> interpreter_tensor_data_mut(Interpreter& interpreter, TfLiteType tflite_type, int32_t tensor_index) {
    auto tensor = interpreter.tensor(tensor_index);
    if (!tensor || tensor->type != tflite_type) {
        throw std::logic_error{"asking for wrong type of data"};
    }
    return rust::Slice<uint8_t>(tensor->data.uint8, tensor->bytes);
}

rust::Slice<const uint8_t> interpreter_tensor_buffer(const Interpreter& interpreter, int32_t tensor_index) {
    const auto tensor = interpreter.tensor(tensor_index);
    if (!tensor) {
        throw std::logic_error{"index out of bounds"};
    }
    return rust::Slice<const uint8_t>(tensor->data.uint8, tensor->bytes);
}

rust::Slice<uint8_t> interpreter_tensor_buffer_mut(Interpreter& interpreter, int32_t tensor_index) {
    auto tensor = interpreter.tensor(tensor_index);
    if (!tensor) {
        throw std::logic_error{"index out of bounds"};
    }
    return rust::Slice<uint8_t>(tensor->data.uint8, tensor->bytes);
}