---
title: Introdução
sidebar_position: 0
---

# Introdução e Contextualização da Sprint 3

&emsp; Esta é a terceira sprint do projeto desenvolvido pelo grupo **14 BIS** em parceria com o IPT (Instituto de Pesquisas Tecnológicas) — mais especificamente com o Laboratório de Materiais para Produtos de Construção (LMPC). Neste ciclo, o foco principal foi consolidar a **integração entre os diferentes módulos da solução**, com destaque para o alinhamento entre as equipes de **machine learning (YOLO)** e **desenvolvimento da aplicação desktop (Rust)**. Também foram feitos avanços significativos na documentação técnica e na automação da geração de relatórios de inspeção.

## 1. Objetivo da Sprint

&emsp; O principal objetivo da Sprint 3 foi **unificar todas as partes da solução**: conectar os modelos de detecção e classificação de fissuras (implementados em Python usando YOLOv8) com a aplicação desktop em Rust, responsável por organizar os dados e gerar os relatórios finais. Isso permite que o fluxo de trabalho técnico — da captura de imagens até o laudo final — seja executado de forma automatizada e coesa.

## 2. Principais Entregas

&emsp; Durante esta sprint, foram realizadas as seguintes entregas e implementações:

### 2.1. Integração entre Python e Rust

- Conexão dos dados de saída dos modelos YOLO com os arquivos JSON utilizados pela aplicação Rust.
- Geração automática de relatórios a partir de templates Markdown utilizando a biblioteca `handlebars-rust`.
- Testes end-to-end validando o fluxo completo: **imagem → predição → JSON → relatório**.

### 2.2. Evoluções nos Modelos de IA

- Inclusão de novos experimentos com YOLOv8 para **classificação de fissuras** com diferentes tamanhos de imagem (224x224 e 640x640).
- Criação dos scripts de treino (`yolo_train.py`) e validação (`yolo_test.py`), com suporte à biblioteca `ultralytics`.
- Armazenamento dos resultados e configurações de cada experimento em arquivos `.yaml` e `.csv`.

### 2.3. Melhorias na Aplicação Desktop

- Adição de novos arquivos JSON contendo dados de inspeções reais e simuladas.
- Adaptação do sistema para aceitar arquivos unificados, centralizando os dados por projeto.
- Estruturação do template de relatório com campos dinâmicos preenchidos automaticamente.

### 2.4. Atualização da Documentação

- Documentação completa sobre a **geração de relatórios** com Handlebars (`docs/docs/Sprint-3/Aplicação-Desktop/gerar-relatorio.md`).
- Expansão do conteúdo introdutório do projeto e contexto técnico no LMPC.
- Organização da documentação em estrutura clara por sprints, facilitando o rastreamento do progresso.

## 3. Importância desta Etapa

&emsp; Esta sprint representa um marco no projeto, pois é a primeira em que **todas as frentes técnicas se conectam de maneira funcional**. A partir daqui, a equipe pode focar em ajustes finos, melhorias de usabilidade, refino dos modelos e testes em cenários reais. O produto começa a se configurar como uma ferramenta concreta e utilizável, próxima do ambiente de produção.

## 4. Próximos Passos

&emsp; Com a base integrada e funcional, as próximas sprints terão como objetivo:

- Melhorar a interface da aplicação desktop;
- Otimizar os modelos de IA com mais dados e ajustes de hiperparâmetros;
- Validar a ferramenta com casos reais fornecidos pelo IPT;