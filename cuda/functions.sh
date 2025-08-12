#! /bin/bash

export CUDA_BUILD_DIR=${BUILD_DIR}/cuda

function cuda_clean() {
    echo "Cleaning CUDA build artifacts..."
    rm -rf ${CUDA_BUILD_DIR}
}

function cuda_cmake() {
    cuda_clean
    echo "Configuring CUDA build with CMake..."
    mkdir -p ${CUDA_BUILD_DIR}
    cmake -S ${WORKDIR}/cuda -B ${CUDA_BUILD_DIR}
}

function cuda_build() {
    echo "Building CUDA project..."
    cmake --build ${CUDA_BUILD_DIR}
}

function cuda_run() {
    ${CUDA_BUILD_DIR}/main
}

function cuda_format() {
    FILE_LIST="$(find "${WORKDIR}/cuda" | grep -E ".*(\.cu|\.cpp|\.h|\.hpp)$")"
    echo "Files to format: $FILE_LIST"
    clang-format -i $FILE_LIST
}

function cuda_build_protos() {
    OUT=${BUILD_DIR}/cuda
    python3 -m grpc_tools.protoc -I ${WORKDIR}/interfaces --python_out=$OUT --pyi_out=$OUT --grpc_python_out=$OUT ${WORKDIR}/interfaces/hello.proto
}