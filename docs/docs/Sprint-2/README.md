---
sidebar_position: 1
slug: /sprint-2
title: Sprint 2
---

# Introdução

Nesta seção, serão explicadas as funcionalidades desenvolvidas para o módulo em Rust da aplicação, responsável pela criação do aplicativo desktop.

# Como executar:

Para executar o programa, é necessário instalar algumas linguagens e pacotes. Abaixo estão os comandos necessários para instalação em ambiente Linux (caso você utilize Windows, consulte a documentação de cada site): 

1. Instalar a linguagem rust
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

2. Instalar o Cargo-binstall
```curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash```

3. Instalar o framework dioxus
```cargo binstall dioxus-cli```

**Aqui, haverá uma separação de terminais. Abra um segundo terminal.**

## No primeiro terminal:

4. Entrar na pasta src/app rust
```cd src/app-rust```

5. Executar o aplicativo
```dx serve --platform desktop```

## No segundo terminal:

4. Abrir e ativar um venv
```python3 -m venv venv```
```source venv/bin/activate```

5. Instalar o requirements
```pip install -r requirements.tx```

6. Executar o programa de geração de imagens
```python3 gen_images.py```

