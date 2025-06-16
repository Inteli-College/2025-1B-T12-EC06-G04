---
title: Integração entre Python e Rust
sidebar_position: 0
---

# Introdução

&emsp;Tendo sido desenvolvidas as duas grandes partes do projeto - modelo de visão computacional e aplicativo desktop -, foi necessário, durante a sprint 4, integrar ambas na mesma aplicação. Dessa forma, esta seção da documentação tem como objetivo delinear como foi a integração entre as duas partes do projeto e como ainda precisa ser melhorada para a entrega final.

# Fluxo de dados na aplicação

&emsp;A aplicação atual, como muitas outras, possui várias páginas que necessitam trocar informações entre si. Entretanto, visto que temos uma aplicação local e não utilizamos um banco de dados, foi necessário repensar como o fluxo de dados e informações entre páginas e entre as seções do aplicativo e do modelo de visão computacional seria construído.

&emsp;Optamos por utilizar páginas de template e arquivos json, de forma que sempre poderíamos ter visualizações diferentes com base nas informações do json que foi repassado. Diante disso, existem 2 arquivos json principais na aplicação:

1. project.json: arquivo criado com as informaçõs do projeto escritas pelo usuário no momento de criação da pasta;
2. detection_results.json: arquivo criado com os resultados da análise das fotos pelo modelo de visão computacional

&emsp;Dessa forma, ao criar um projeto, o usuário delimita informações como nome e ano da construção, o que gera um primeiro json que é utilizado na geração do relatório. Depois, ao fazer o upload das fotos, o usuário também aciona o script do modelod e visão computacional e gera o json com os resultados de detecção e classificação, o qual também é utilizado na geração do relatório.

# Conclusão 
&emsp;A primeira etapa de integração do projeto foi concluída durante a sprint 4, que consiste na integração do upload de fotos do usuário com as detecções e classificações do modelo de visão computacional. Durante a sprint 5, é esperado que a integração se torne mais robusta e que seja tenha tratamento a possíveis erros da aplicação, mas que também integre de forma total as páginas de estatísticas/gráficos de cada projeto e os relatórios.