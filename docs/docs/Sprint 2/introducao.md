---
title: Introdução
sidebar_position: 0
---

# Introdução e Contextualização da Sprint 2

&emsp; Esta é a **segunda sprint** do projeto desenvolvido pelo grupo **14 BIS** em parceria com o IPT (Instituto de Pesquisas Tecnológicas) — mais especificamente com o Laboratório de Materiais para Produtos de Construção (LMPC). Neste ciclo, o foco principal foi estabelecer as **bases do desenvolvimento do produto**, tanto na inteligência artificial quanto na aplicação desktop.

&emsp; OS esforços foram dedicados à criação da **primeira versão do algoritmo de classificação de fissuras em C++** e ao **início da construção da interface do usuário em Rust**, incluindo a funcionalidade de organização de projetos e a visualização de relatórios. O **Wireframe** também foi um entregável chave, guiando o desenvolvimento da experiência do usuário.

---

## 1. Objetivo da Sprint

&emsp; O principal objetivo da Sprint 2 foi iniciar o desenvolvimento dos dois módulos centrais da solução: o **módulo de inteligência artificial** para detecção e classificação de fissuras, e a **aplicação desktop** que servirá como interface para os usuários do IPT. Buscamos validar os primeiros passos técnicos de cada componente e criar a estrutura inicial para futuras integrações.

---

## 2. Principais Entregas

&emsp; Durante esta sprint, foram realizadas as seguintes entregas e implementações:

### 2.1. Primeira Versão do Modelo de Classificação de Fissuras (Módulo C++)

* **Estrutura de Projeto Definida:** Organização de pastas para código (src, include), dados (images/treinamento, images/validacao) e construção (build), otimizando o fluxo de desenvolvimento.
* **Pipeline de Processamento de Imagens:** Implementação de fases de **pré-processamento** (redimensionamento, filtros, extração de máscara binária com Canny), **extração de features** (orientação, desvio padrão, comprimento, número e densidade de fissuras) e **classificação com SVM**.
* **Otimização do Modelo:** Utilização de busca em grade (grid search) para encontrar os melhores hiperparâmetros do SVM (Kernel: POLY, C: 0.5, Degree: 2, Coef0: 0, Gamma: 0.5), visando otimizar a acurácia.
* **Validação Inicial:** Avaliação da acurácia média em 60.625% por validação cruzada (k=10), identificando desafios como máscaras incorretas e *overfitting*.

### 2.2. Módulo em Rust: Desenvolvimento da Aplicação Desktop (Dioxus)

* **Criação de Pastas de Projetos:** Implementação da funcionalidade para que o usuário possa criar e organizar projetos em pastas locais, replicando uma estrutura familiar de sistema de arquivos e fornecendo uma macro-organização para a aplicação.
* **Visualização e Exportação de Relatórios:** Desenvolvimento da tela que permite a visualização de relatórios de projetos e a exportação em múltiplos formatos (Markdown, PDF, Docx), aumentando a flexibilidade para o usuário final.

### 2.3. UX: Desenvolvimento do Wireframe

* **Esboço Visual da Aplicação:** Criação de wireframes detalhados para as principais telas do aplicativo desktop (Tela Inicial, Criação de Novo Projeto, Pasta do Projeto, Relatório, Galeria de Imagens Originais, Pastas por Prédios/Fachadas, Galeria de Imagens Classificadas, Descrição de Imagem), servindo como guia para o design e desenvolvimento da interface.
* **Validação Estrutural:** O wireframe auxiliou na definição da disposição dos elementos, usabilidade e comunicação visual, sendo uma ferramenta para alinhamento com o IPT.

---

## 3. Importância desta Etapa

&emsp; A Sprint 2 foi fundamental por lançar as bases técnicas do projeto. Ao mesmo tempo em que iniciamos o desenvolvimento do **"cérebro" do sistema** (o algoritmo de classificação em C++), também começamos a dar **"corpo" à aplicação** com a interface em Rust. Embora desafios tenham surgido no módulo de IA, a identificação precoce desses pontos críticos permite um planejamento mais eficaz para as próximas fases. A criação das funcionalidades de organização de projetos e visualização de relatórios, guiada pelo wireframe, garantiu que a estrutura principal da aplicação estivesse pronta para receber a inteligência artificial, alinhando a arquitetura com a experiência do usuário.

---

## 4. Próximos Passos

&emsp; Com as primeiras versões do algoritmo de IA e da interface estabelecidas, as próximas sprints terão como objetivo:

* Melhorar significativamente o pré-processamento de imagens e explorar outros modelos de classificação para aprimorar a acurácia do módulo de IA.
* Implementar o upload de imagens ao criar pastas e refinar a navegação na aplicação desktop.
* Desenvolver o gerador automático de relatórios e adaptar o template conforme as necessidades específicas do IPT.
* Aprofundar a integração entre os módulos de IA e a aplicação desktop, conforme planejado para a Sprint 3.

_**OBS:** Por decisão da equipe de desenvolvimento do projeto, a partir da próxima sprint a linguagem de programação utilizada para desenvolvimento do modelo de Machine Learning e Visão Computacional será o Python, a fim de agilizar o processo de treinamento._