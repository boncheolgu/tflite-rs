cmake_minimum_required(VERSION 3.5)

project(tflite CXX)

set(TENSORFLOW_SRC_DIR "${CMAKE_SOURCE_DIR}/external/tensorflow")

add_library(tflite INTERFACE)
target_include_directories(tflite INTERFACE ${TENSORFLOW_SRC_DIR})
target_link_libraries(tflite INTERFACE "${TENSORFLOW_SRC_DIR}/tensorflow/contrib/lite/tools/make/gen/linux_x86_64/lib/libtensorflow-lite.a" pthread dl)

add_executable(minimal "${TENSORFLOW_SRC_DIR}/tensorflow/contrib/lite/examples/minimal/minimal.cc")
target_link_libraries(minimal PRIVATE tflite)

add_executable(label_image
  "${TENSORFLOW_SRC_DIR}/tensorflow/contrib/lite/examples/label_image/label_image.cc"
  "${TENSORFLOW_SRC_DIR}/tensorflow/contrib/lite/examples/label_image/bitmap_helpers.cc"
)
target_link_libraries(label_image PRIVATE tflite)
