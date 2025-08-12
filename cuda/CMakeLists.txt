cmake_minimum_required(VERSION 3.28 FATAL_ERROR)
project(cuda LANGUAGES CXX CUDA)

set(CMAKE_CUDA_STANDARD 17)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)

find_package(Protobuf CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
get_target_property(grpc_cpp_plugin_location gRPC::grpc_cpp_plugin LOCATION)
find_package(GTest REQUIRED)

message(STATUS "Protobuf version: ${Protobuf_VERSION}")
message(STATUS "gRPC version: ${gRPC_VERSION}")

message(STATUS "Protobuf include directories: ${Protobuf_INCLUDE_DIRS}")
message(STATUS "Protobuf directory: ${Protobuf_DIR}")
message(STATUS "CMake prefix path: ${CMAKE_PREFIX_PATH}")

set(PROTO_FILES_DIR /workspace/interfaces)
set(PROTOS_BUILD_DIR /workspace/build/cuda/interfaces)
make_directory(${PROTOS_BUILD_DIR})

set(PROTO_FILES
    ${PROTO_FILES_DIR}/hello.proto
)

add_library(hello_proto ${PROTO_FILES})
target_link_libraries(hello_proto 
    PUBLIC
        protobuf::libprotobuf
        gRPC::grpc
        gRPC::grpc++)

target_include_directories(hello_proto PUBLIC ${CMAKE_CURRENT_BINARY_DIR})

protobuf_generate(
    TARGET hello_proto
    IMPORT_DIRS ${PROTO_FILES_DIR}
    LANGUAGE cpp
    OUT_VAR ${GENERATED_PROTOS}
    PROTOS
      ${PROTO_FILES}
)

protobuf_generate(
     TARGET hello_proto 
     LANGUAGE grpc 
     IMPORT_DIRS ${PROTO_FILES_DIR}
     OUT_VAR ${GENERATED_PROTOS}
     PROTOS
      ${PROTO_FILES}
     GENERATE_EXTENSIONS .grpc.pb.h .grpc.pb.cc 
     PLUGIN "protoc-gen-grpc=${grpc_cpp_plugin_location}")


add_executable(main main.cpp app.cpp)

add_library(kernels kernels.cu)
set_target_properties(kernels PROPERTIES CUDA_SEPARABLE_COMPILATION ON)
set_property(TARGET kernels PROPERTY CUDA_ARCHITECTURES 61)

target_link_libraries(main PUBLIC gRPC::grpc++ protobuf::libprotobuf hello_proto kernels)

add_executable(kernels_test kernels_tests.cpp)
target_link_libraries(kernels_test PUBLIC GTest::gtest_main kernels)

add_dependencies(main hello_proto)

