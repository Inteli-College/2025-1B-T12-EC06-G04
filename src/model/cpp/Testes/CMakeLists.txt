cmake_minimum_required(VERSION 3.10)
project(AnaliseRachaduras)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Encontra OpenCV
find_package(OpenCV REQUIRED)

# Inclui diretórios de headers
include_directories(include ${OpenCV_INCLUDE_DIRS})

# Lista os arquivos-fonte
set(SOURCES
    ../src/validacao.cpp
    ../src/preprocessamento.cpp
    ../src/detectorfissura.cpp
    ../src/extrator_features.cpp
    ../Testes/testeImagem.cpp
)

# Cria executável
add_executable(testeImagem ${SOURCES})

# Linka as bibliotecas OpenCV
target_link_libraries(testeImagem ${OpenCV_LIBS})
