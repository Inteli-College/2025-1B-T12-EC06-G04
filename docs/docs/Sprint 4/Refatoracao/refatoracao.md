---
title: Refatoração do código fonte
sidebar_position: 0
---

# Objetivos da Refatoração do Código

&emsp;Durante as 3 primeiras sprints, o grupo de desenvolvimento trabalhou com uma única pasta de código, sendo ela a ```src```. Entretanto, nela estavam agrupadas em subpastas todos os códigos oficiais e testes das 3 frentes de projeto: rust, c++ e python. Com o objetivo de gerar maior organização e facilidade em manipular as pastas e diretórios, foi realizada a refatoração.

# Refatoração do código

&emsp;A refatoração teve 3 fases: a decisão de subpastas dentro do ```src``` e as organização de pastas de cada frente do projeto.

1. No diretório ```src```, criamos duas pastas: model, que contém todo o código relacionado à construção do modelo de visão computacional; e app-rust, que contém toda a lógica para construção do aplicativo desktop.

2. Dentro de model, foram criadas duas pastas principais: cpp, que contém os primeiros códigos de utilização do algoritmo de visão computacional em C++; e Yolo, que contém os códigos em Python do algoritmo final, tanto de classificação quanto de detecção.

3. Dentro de app-rust, foram criadas duas pastas principais dentro do src do projeto DIoxus: utils, que contém as funções utilizadas em todas as páginas do projeto; e pages, que contém o front-end de cada página. 

&emsp;É importante ressaltar que a refatoração do app-rust ainda está em andamento, visto que esse diretório ainda possui mudanças a cada sprint que podem acarretar conflitos entre os imports. Sendo assim, é esperado que a refatoração dessa pasta seja concluída na sprint 5.

# Conclusão

&emsp;Com a refatoração em andamento, almejamos construir pastas e arquivos de códigos mais limpos e robustos, além de possibilitar a não repetição e criar um ambiente para reutilização de funções e códigos já criados. Dessa forma, espera-se que o processo de refatoração do código possa criar maior organização a qualquer pessoa que acesse o repositório do projeto.