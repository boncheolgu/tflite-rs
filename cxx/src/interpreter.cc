
#include "tflite/cxx/include/interpreter.h"

std::unique_ptr<tflite::FlatBufferModel> flatbuffer_from_buffer(const rust::Slice<const uint8_t> buffer) {
//    return FlatBufferModel::BuildFromBuffer(const_cast<int8_t*>(buffer.data()), buffer.size());
//    return FlatBufferModel::BuildFromBuffer(buffer.data(), buffer.size());
    return tflite::FlatBufferModel::BuildFromBuffer(reinterpret_cast<const char*> (buffer.data()), buffer.size());
}