cmake_minimum_required(VERSION 3.5)

project(tflite_interpreter CXX)

add_library(tflite_proxy INTERFACE)
target_include_directories(tflite_proxy INTERFACE include)
target_link_libraries(tflite_proxy INTERFACE ${CMAKE_SOURCE_DIR}/lib/libtensorflow-lite.a pthread dl)

add_executable(minimal examples/minimal/minimal.cc)
target_link_libraries(minimal PRIVATE tflite_proxy)

add_executable(label_image examples/label_image/label_image.cc examples/label_image/bitmap_helpers.cc)
target_include_directories(label_image PRIVATE examples)
target_link_libraries(label_image PRIVATE tflite_proxy)
