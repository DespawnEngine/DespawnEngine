cmake_minimum_required(VERSION 3.31)
project(DespawnEngine)

find_package(Vulkan REQUIRED)
find_package(glfw3 REQUIRED)

set(CMAKE_CXX_STANDARD 20)

add_executable(DespawnEngine src/main.cpp)

target_link_libraries(DespawnEngine PRIVATE glfw3 Vulkan::Vulkan)