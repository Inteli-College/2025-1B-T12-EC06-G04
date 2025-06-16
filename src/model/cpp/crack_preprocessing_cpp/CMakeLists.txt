cmake_minimum_required(VERSION 3.10)
project(CrackPreprocessing)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(OpenCV REQUIRED)

include_directories(include)
include_directories(${OpenCV_INCLUDE_DIRS})

file(GLOB SOURCES "src/*.cpp")

add_executable(crack_preprocessing ${SOURCES})
target_link_libraries(crack_preprocessing ${OpenCV_LIBS})
