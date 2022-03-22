cmake_minimum_required(VERSION 3.10)
project(thalweg VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17 CACHE STRING "The C++ standard to use")
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_BUILD_TYPE RelWithDebInfo CACHE STRING "The build type")

include(CTest)

add_subdirectory(external)
add_subdirectory(src)
add_subdirectory(tests)
