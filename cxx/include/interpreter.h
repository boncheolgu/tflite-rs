#pragma once

#include "tflite/src/interpreter/cxx.rs.h"
#include "tensorflow/lite/model.h"
//#include "tensorflow/lite/kernels/register.h"

using FlatBufferModel = tflite::FlatBufferModel;

std::unique_ptr<tflite::FlatBufferModel> flatbuffer_from_buffer(const rust::Slice<const uint8_t> buffer);