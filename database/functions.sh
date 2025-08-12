#! /bin/bash
export PYTHONPATH=${BUILD_DIR}/database:$PYTHONPATH

function build_all() {
    cd ${WORKDIR}/database/mysql_client
    cargo build
    cd ${WORKDIR}/database
    OUT=${BUILD_DIR}/database
    python3 -m grpc_tools.protoc -I ${WORKDIR}/interfaces --python_out=$OUT --pyi_out=$OUT --grpc_python_out=$OUT ${WORKDIR}/interfaces/hello.proto
}

function run_all_integration_tests() {
    cd ${WORKDIR}/database
    pytest -s .
}

function run_intgration_test() {
    local test_name=$1
    if [ -z "$test_name" ]; then
        echo "Test name is required"
        return 1
    fi

    cd ${WORKDIR}/database
    pytest -s -k "$test_name"
}