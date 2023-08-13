cmake_minimum_required(VERSION 3.4.0)

project(smart_socket_lib CXX)

set(SOURCE ${CMAKE_SOURCE_DIR}/src/smart_socket_lib.cpp)

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

add_library(smart_socket SHARED ${SOURCE})
