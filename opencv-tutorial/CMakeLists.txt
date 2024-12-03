# https://docs.opencv.org/4.x/db/df5/tutorial_linux_gcc_cmake.html

cmake_minimum_required(VERSION 3.28)
project(opencv_tutorial)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF) 

find_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})

add_executable(hello hello.cpp)
target_link_libraries(hello ${OpenCV_LIBS})

add_executable(mat mat.cpp)
target_link_libraries(mat ${OpenCV_LIBS})

add_executable(scan scan.cpp)
target_link_libraries(scan ${OpenCV_LIBS})

