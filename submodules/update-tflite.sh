#!/bin/sh
set -e

downloads="$(cd $(dirname $BASH_SOURCE[0]) && pwd)/downloads"

cd /tmp
rm -rf tensorflow
git clone https://github.com/tensorflow/tensorflow --branch v2.4.1 --depth 1
cd tensorflow

tensorflow/lite/tools/make/download_dependencies.sh

rm -rf .git
fd --type l . . --exec rm -f {}

rm -rf ${downloads}/*
mkdir -p ${downloads}/tensorflow/core/kernels/
mkdir -p ${downloads}/tensorflow/lite
mkdir -p ${downloads}/third_party

cp -a tensorflow/core/public ${downloads}/tensorflow/core/
cp -a tensorflow/core/kernels/*.h ${downloads}/tensorflow/core/kernels/
cp -a tensorflow/lite/* ${downloads}/tensorflow/lite/
cp -a third_party/eigen3 ${downloads}/third_party/
cp -a third_party/fft2d ${downloads}/third_party/

tf_downloads="tensorflow/lite/tools/make/downloads"

rm -rf ${downloads}/${tf_downloads}/gemmlowp/meta/generators
rm -rf ${downloads}/${tf_downloads}/flatbuffers/net
rm -rf ${downloads}/${tf_downloads}/farmhash/dev
rm -rf ${downloads}/${tf_downloads}/farmhash/m4

# hack to get gcc 11 to work...
echo '#include <limits>' > /tmp/block_map.cc
cat ${downloads}/${tf_downloads}/ruy/ruy/block_map.cc >> /tmp/block_map.cc
mv /tmp/block_map.cc ${downloads}/${tf_downloads}/ruy/ruy/block_map.cc 

fd --type file --hidden '^.gitignore$' ${downloads} --exec rm -f

for language in python java go rust php lua grpc dart android js objc swift ios; do
  echo "removing directories matching ^${language}$"
  fd --type d '^'${language}'$' "${downloads}" --exec rm -rf {}
done

for cruft in ^test 'test$' ^doc 'doc$' ^sample ^demo ^bench '^ci$' ^todo ^example; do
  echo "removing directories matching ${cruft}"
  fd --type d ${cruft} "${downloads}" --exec rm -rf {}
done

for extension in py bzl java html md pbtxt; do
  echo "removing files with extension ${extension}"
  fd --type f --extension ${extension} . "${downloads}" --exec rm -rf {}
done

for file in LICENSE; do
  echo "removing files matching ${file}"
  fd --type f ${file} "${downloads}" --exec rm -rf {}
done
