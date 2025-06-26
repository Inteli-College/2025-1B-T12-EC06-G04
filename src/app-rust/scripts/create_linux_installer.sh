#!/bin/bash

APP_NAME="14Bis"
OUTPUT_DIR="installer"
RELEASE_DIR="target/release"
INSTALLER_NAME="${APP_NAME}_Installer.run"

# Compile o projeto Rust em modo release
cargo build --release
if [ $? -ne 0 ]; then
    echo "Erro ao compilar o projeto Rust."
    exit 1
fi

# Certifique-se de que o diretório de saída existe
mkdir -p $OUTPUT_DIR

# Crie o instalador usando makeself
makeself --nox11 $RELEASE_DIR $OUTPUT_DIR/$INSTALLER_NAME "$APP_NAME Installer" ./Group_14_bis
if [ $? -ne 0 ]; then
    echo "Erro ao criar o instalador Linux."
    exit 1
fi

echo "Instalador Linux criado em: $OUTPUT_DIR/$INSTALLER_NAME"
