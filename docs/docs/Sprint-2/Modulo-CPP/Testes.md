---
sidebar_position: 2
title: Testes do Modelo
slug: /sprint-2/cpp/testes
---
# Testes do Modelo

&emsp;Esta página tem como objetivo apresentar como funcionam atualmente os testes do modelo classificador e o processo de desenvolvimento do mesmo.

### O que são os testes?

&emsp;Os testes são uma parte importante do desenvolvimento de software, pois ajudam a garantir que o código funcione corretamente e atenda aos requisitos especificados. Eles podem ser usados para verificar se o código está livre de erros, se atende aos requisitos funcionais e não funcionais, e se é fácil de manter e modificar.
&emsp;No nosso caso individual, os testes são utilizados para verificar a precisão do modelo classificador, ou seja, o quão bom o modelo é em classificar as rachaduras nos tipos corretos.

### Como funcionam nossos testes?

&emsp;Os testes são realizados pelo script `testeImagem.cpp`, que é responsável por carregar as imagens de teste e aplicar individualmente a classificação. Primeiramente, o local das duas pastas de imagens (separadas pelo tipo de rachadura) é definido, e a informação do tipo é armazenada em um vetor. Após isso, o sistema loopa pela pasta das imagens, carregando a rota e chamando a função `resultadoAnalise(std::string imagem, int esperado)`, que se comunica com o código principal do modelo. Para mais informações sobre o funcionamento do modelo, consulte a página do [modelo](./modelo)
