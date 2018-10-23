cmake_minimum_required(VERSION 3.5)

project(tflite_interpreter CXX)

add_executable(tflite_interpreter src/main.cpp src/dummy.cpp)

target_compile_options(tflite_interpreter PRIVATE -std=c++0x)

target_compile_definitions(tflite_interpreter PUBLIC EIGEN_MPL2_ONLY EIGEN_MAX_ALIGN_BYTES=64 FARMHASH_NO_CXX_STRING GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK)

target_include_directories(tflite_interpreter PRIVATE /home/comatose/workspace/tensorflow/)

set(TF_SRC_DIR "${CMAKE_SOURCE_DIR}/../tensorflow")
set(TFLITE_LIB_DIR "${TF_SRC_DIR}/bazel-bin/tensorflow/contrib/lite")

target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libframework.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libcontext.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libutil.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libarena_planner.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libsimple_memory_arena.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/libbuiltin_ops.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/libeigen_support.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/libgemm_support.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/libkernel_util.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/internal/libtensor_utils.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/internal/libkernel_utils.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/internal/libquantization_util.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/internal/libportable_tensor_utils.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/kernels/internal/libaudio_utils.a")
target_link_libraries(tflite_interpreter "${TFLITE_LIB_DIR}/libstring_util.a")
target_link_libraries(tflite_interpreter "${TF_SRC_DIR}/bazel-bin/external/fft2d/libfft2d.a") # bazel build -s @fft2d//:fft 
target_link_libraries(tflite_interpreter pthread)
