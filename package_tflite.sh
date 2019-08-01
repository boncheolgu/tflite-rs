#!/usr/bin/env sh
set -e

TFLITE_ARCHIVE=$(basename -- "$1")
TFLITE_DIR="${TFLITE_ARCHIVE%.tar.gz}"
tar xvaf "${TFLITE_ARCHIVE}"
"${TFLITE_DIR}/tensorflow/lite/tools/make/download_dependencies.sh"
TFLITE_ARCHIVE_REPACKED="${TFLITE_DIR}_repacked.tar.gz"
tar cvzf "${TFLITE_ARCHIVE_REPACKED}" "${TFLITE_DIR}"
mv "${TFLITE_ARCHIVE_REPACKED}" data
