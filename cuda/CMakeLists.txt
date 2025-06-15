cmake_minimum_required(VERSION 3.28 FATAL_ERROR)
project(cuda LANGUAGES CXX CUDA)


add_executable(main main.cu)
set_property(TARGET main PROPERTY CUDA_ARCHITECTURES 61)
