#!/bin/sh

thisDir=$(cd $(dirname $0) && pwd)

rm -rf ${thisDir}/downloads/

${thisDir}/tensorflow/tensorflow/lite/tools/make/download_dependencies.sh

mv ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads ${thisDir}/

rm -rf ${thisDir}/downloads/googletest
rm -rf ${thisDir}/downloads/gemmlowp/todo
rm -rf ${thisDir}/downloads/gemmlowp/test
rm -rf ${thisDir}/downloads/gemmlowp/meta/generators
rm -rf ${thisDir}/downloads/gemmlowp/doc
rm -rf ${thisDir}/downloads/flatbuffers/tests
rm -rf ${thisDir}/downloads/flatbuffers/samples
rm -rf ${thisDir}/downloads/flatbuffers/rust
rm -rf ${thisDir}/downloads/flatbuffers/python
rm -rf ${thisDir}/downloads/flatbuffers/php
rm -rf ${thisDir}/downloads/flatbuffers/net
rm -rf ${thisDir}/downloads/flatbuffers/lua
rm -rf ${thisDir}/downloads/flatbuffers/java
rm -rf ${thisDir}/downloads/flatbuffers/grpc
rm -rf ${thisDir}/downloads/flatbuffers/go
rm -rf ${thisDir}/downloads/flatbuffers/docs
rm -rf ${thisDir}/downloads/flatbuffers/dart
rm -rf ${thisDir}/downloads/flatbuffers/android
rm -rf ${thisDir}/downloads/farmhash/dev
rm -rf ${thisDir}/downloads/farmhash/m4
rm -rf ${thisDir}/downloads/eigen/unsupported/test
rm -rf ${thisDir}/downloads/eigen/unsupported/doc
rm -rf ${thisDir}/downloads/eigen/test
rm -rf ${thisDir}/downloads/eigen/failtest
rm -rf ${thisDir}/downloads/eigen/doc
rm -rf ${thisDir}/downloads/eigen/demos
rm -rf ${thisDir}/downloads/eigen/bench
rm -rf ${thisDir}/downloads/absl/ci

