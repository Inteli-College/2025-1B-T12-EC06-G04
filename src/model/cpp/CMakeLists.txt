
cmake_minimum_required(VERSION 3.10)
project(visaocomput)

set(CMAKE_CXX_STANDARD 17)

find_package(OpenCV REQUIRED)

include_directories(${OpenCV_INCLUDE_DIRS})
include_directories(include)


add_executable(main
src/main.cpp
src/preprocessamento.cpp
src/detectorfissura.cpp
src/extrator_features.cpp
)
target_link_libraries(main ${OpenCV_LIBS})

add_executable(treinamentoModelo
    src/treinamentoModelo.cpp
    src/preprocessamento.cpp
    src/detectorfissura.cpp
    src/extrator_features.cpp)
target_link_libraries(treinamentoModelo ${OpenCV_LIBS})

