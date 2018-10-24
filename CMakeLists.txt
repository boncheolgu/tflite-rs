cmake_minimum_required(VERSION 3.5)

project(tflite_interpreter CXX)

set(TF_SRC_DIR "${CMAKE_SOURCE_DIR}/../tensorflow")
set(TFLITE_LIB_DIR "${TF_SRC_DIR}/bazel-bin/tensorflow/contrib/lite")

set(tflite_libraries
  "${TFLITE_LIB_DIR}/libframework.a"
  "${TFLITE_LIB_DIR}/libcontext.a"
  "${TFLITE_LIB_DIR}/libframework.a"
  "${TFLITE_LIB_DIR}/libcontext.a"
  "${TFLITE_LIB_DIR}/libutil.a"
  "${TFLITE_LIB_DIR}/libarena_planner.a"
  "${TFLITE_LIB_DIR}/libsimple_memory_arena.a"
  "${TFLITE_LIB_DIR}/kernels/libbuiltin_ops.a"
  "${TFLITE_LIB_DIR}/kernels/libeigen_support.a"
  "${TFLITE_LIB_DIR}/kernels/libgemm_support.a"
  "${TFLITE_LIB_DIR}/kernels/libkernel_util.a"
  "${TFLITE_LIB_DIR}/kernels/internal/libtensor_utils.a"
  "${TFLITE_LIB_DIR}/kernels/internal/libkernel_utils.a"
  "${TFLITE_LIB_DIR}/kernels/internal/libquantization_util.a"
  "${TFLITE_LIB_DIR}/kernels/internal/libportable_tensor_utils.a"
  "${TFLITE_LIB_DIR}/kernels/internal/libaudio_utils.a"
  "${TFLITE_LIB_DIR}/libstring_util.a"
  "${TF_SRC_DIR}/bazel-bin/external/fft2d/libfft2d.a" # bazel build @fft2d//:fft
  "${TF_SRC_DIR}/bazel-bin/external/farmhash_archive/libfarmhash.a" # bazel build @farmhash_archive//:farmhash
  )

file(COPY ${tflite_libraries} DESTINATION ${CMAKE_SOURCE_DIR}/lib)

add_library(tflite_proxy INTERFACE)
target_include_directories(tflite_proxy INTERFACE include)
target_link_libraries(tflite_proxy INTERFACE ${tflite_libraries} pthread)

add_executable(minimal examples/minimal/minimal.cc)
target_link_libraries(minimal PRIVATE tflite_proxy)

add_executable(label_image examples/label_image/label_image.cc examples/label_image/bitmap_helpers.cc)
target_include_directories(label_image PRIVATE examples)
target_link_libraries(label_image PRIVATE tflite_proxy)
