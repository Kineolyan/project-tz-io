cmake_minimum_required(VERSION 3.30)

project(tzk LANGUAGES CXX)

# Latest C++ standard
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
# For clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

add_executable(tzk src/main.cpp)
target_compile_options(tzk PRIVATE -Wall -Wextra -Wpedantic)

install(TARGETS tzk RUNTIME DESTINATION bin)
