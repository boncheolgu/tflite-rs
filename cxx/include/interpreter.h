#pragma once

#include "tensorflow/lite/model.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/optional_debug_tools.h"
#include "tensorflow/lite/c/common.h"

using FlatBufferModel = tflite::FlatBufferModel;
using BuiltinOpResolver = tflite::ops::builtin::BuiltinOpResolver;
using OpResolver = tflite::OpResolver;
using InterpreterBuilder = tflite::InterpreterBuilder;
using Interpreter = tflite::Interpreter;

#include "tflite/src/interpreter/cxx.rs.h"

std::unique_ptr<FlatBufferModel> flatbuffer_from_slice(const rust::Slice<const uint8_t> buffer);

std::unique_ptr<BuiltinOpResolver> builtin_op_resolver();

std::unique_ptr<InterpreterBuilder> interpreter_builder(const FlatBufferModel& model, const OpResolver& op_resolver);

const OpResolver& builtin_op_resolver_to_op_resolver(const BuiltinOpResolver& builtin_op_resolver);

std::unique_ptr<Interpreter> build_model(InterpreterBuilder& builder);

std::unique_ptr<Interpreter> build_model_with_threads(InterpreterBuilder& builder, uint8_t threads);

bool interpreter_allocate_tensors(Interpreter& interpreter);

void interpreter_print_state(const Interpreter& interpreter);

bool interpreter_invoke(Interpreter& interpreter);

void interpreter_set_num_threads(Interpreter& interpreter, int32_t threads);

rust::Slice<const int32_t> interpreter_inputs(const Interpreter& interpreter);

rust::Slice<const int32_t> interpreter_outputs(const Interpreter& interpreter);

rust::Slice<const int32_t> interpreter_variables(const Interpreter& interpreter);

size_t interpreter_tensors_size(const Interpreter& interpreter);

size_t interpreter_nodes_size(const Interpreter& interpreter);

int32_t interpreter_add_tensors(Interpreter& interpreter, size_t count);

bool interpreter_set_inputs(Interpreter& interpreter, rust::Slice<const int32_t> inputs);

bool interpreter_set_outputs(Interpreter& interpreter, rust::Slice<const int32_t> outputs);

bool interpreter_set_variables(Interpreter& interpreter, rust::Slice<const int32_t> variables);

bool interpreter_set_tensor_parameters_read_write(Interpreter& interpreter, int32_t tensor_index, const TfLiteType& tflite_type, rust::Str name, rust::Slice<const size_t> dims, const TfLiteQuantization& quantization, bool is_variable);

rust::Slice<const uint8_t> interpreter_tensor_data(const Interpreter& interpreter, TfLiteType tflite_type, int32_t tensor_index);

rust::Slice<uint8_t> interpreter_tensor_data_mut(Interpreter& interpreter, TfLiteType tflite_type, int32_t tensor_index);

rust::Slice<const uint8_t> interpreter_tensor_buffer(const Interpreter& interpreter, int32_t tensor_index);

rust::Slice<uint8_t> interpreter_tensor_buffer_mut(Interpreter& interpreter, int32_t tensor_index);
