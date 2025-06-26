#!/bin/bash

# Compile o projeto Rust em modo release
cargo build --release
if [ $? -ne 0 ]; then
    echo "Erro ao compilar o projeto Rust."
    exit 1
fi

# Gerar instalador Linux
./scripts/create_linux_installer.sh
if [ $? -ne 0 ]; then
    echo "Erro ao criar o instalador Linux."
    exit 1
fi

# Gerar instalador Windows usando NSIS
makensis installer.nsi
if [ $? -ne 0 ]; then
    echo "Erro ao criar o instalador Windows."
    exit 1
fi

echo "Instaladores gerados com sucesso!"
