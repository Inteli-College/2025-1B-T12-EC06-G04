#!/bin/bash

APP_NAME="14Bis"
OUTPUT_DIR="installer"
RELEASE_DIR="target/release"
INSTALLER_NAME="${APP_NAME}_Installer.tar.gz"

# Compile o projeto Rust em modo release
cargo build --release
if [ $? -ne 0 ]; then
    echo "Erro ao compilar o projeto Rust."
    exit 1
fi

# Certifique-se de que o diretório de saída existe
mkdir -p $OUTPUT_DIR

# Copie o binário para o diretório de saída
cp $RELEASE_DIR/Group_14_bis $OUTPUT_DIR/

# Crie um arquivo README com instruções de instalação
echo "Para instalar, copie o arquivo 'Group_14_bis' para um diretório de sua preferência e execute-o." > $OUTPUT_DIR/README.txt

# Empacote os arquivos em um tar.gz
tar -czvf $OUTPUT_DIR/$INSTALLER_NAME -C $OUTPUT_DIR Group_14_bis README.txt
if [ $? -ne 0 ]; then
    echo "Erro ao criar o instalador Linux."
    exit 1
fi

echo "Instalador Linux criado em: $OUTPUT_DIR/$INSTALLER_NAME"
