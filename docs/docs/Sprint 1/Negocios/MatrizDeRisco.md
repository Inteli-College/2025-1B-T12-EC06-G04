---
id: matriz-de-risco
title: Matriz de Risco
sidebar_label: Matriz de Risco
slug: /Sprint-1/Negocios/matriz-de-risco
---
# Matriz de Risco e Oportunidade

&emsp;A Matriz de Risco é uma ferramenta utilizada para identificar e avaliar os riscos associados a um projeto. Ela ajuda a priorizar os riscos com base em sua probabilidade de ocorrência e impacto no projeto. Riscos na área verde podem ser considerados de baixa prioridade, seja por terem baixa probabilidade de ocorrência ou por terem um impacto baixo no projeto. Já os riscos na área vermelha são considerados de alta prioridade, pois têm alta probabilidade de ocorrência e/ou alto impacto no projeto. Os riscos na área amarela são considerados de prioridade média.

&emsp;De forma semelhante, a Matriz de Oportunidade é uma ferramenta que ajuda a identificar e avaliar as oportunidades que podem surgir durante o projeto. Ela também ajuda a priorizar as oportunidades com base em sua probabilidade de ocorrência e impacto no projeto. As oportunidades na área vermelha são mais interessantes para o projeto, 

&emsp;Neste projeto, utilizamos esta matriz para classificar os riscos com base em sua probabilidade de ocorrência e impacto potencial:
* **Área Verde (Baixa Prioridade):** Baixa probabilidade e/ou baixo impacto.
* **Área Amarela (Prioridade Média):** Média probabilidade e/ou impacto.
* **Área Vermelha (Alta Prioridade):** Alta probabilidade e/ou alto impacto.

&emsp;De forma análoga, para as oportunidades, aquelas que caem na **área vermelha** são consideradas as mais estratégicas e com maior potencial de benefício para o projeto, devido à sua alta probabilidade e impacto.

<br />

![Matriz de Risco e Oportunidade](/img/matrizRisco14bis.png)
*(Representação visual conceitual da Matriz de Risco e Oportunidade)*

---


## 1. Riscos do Projeto

A seguir, detalhamos os riscos identificados, sua avaliação e as estratégias de mitigação.

**Legenda de Avaliação (Riscos):**
* **Probabilidade:** Muito Baixa (MB), Baixa (B), Média (M), Alta (A), Muito Alta (MA).
* **Impacto:** Muito Baixo (MB), Baixo (B), Médio (M), Alto (A), Muito Alto (MA).

<br />

![Matriz de Risco](/img/matRis.png)

*(Representação visual conceitual da Matriz de Risco)*

---
**R1: Tempo de Treinamento de Modelos Excessivo**
* **Categoria:** Técnica / Cronograma
* **Probabilidade:** Alta
* **Impacto:** Alto
* **Estratégias de Mitigação:**
    * **Otimização de Recursos:** Alocar recursos computacionais adequados para o treinamento.
    * **Estratégia de Treino Iterativa:** Começar com uma quantidade reduzida de imagens e gradualmente aumentá-la para alcançar um modelo funcional.
    * **Monitoramento Ativo:** Monitorar de perto o tempo de treinamento (ex: se exceder 16h) para rever a estratégia antes que o atraso comprometa a entrega do projeto.
    * **Avaliação de Modelos Pré-treinados:** Explorar o uso de modelos pré-treinados para acelerar o processo inicial de desenvolvimento.

---
**R2: Dificuldade na Integração Rust-Python**
* **Categoria:** Técnica
* **Probabilidade:** Média
* **Impacto:** Muito Alto
* **Estratégias de Mitigação:**
    * **Interface de Comunicação Bem Definida:** Projetar e documentar APIs (Application Programming Interfaces) claras e estáveis para a comunicação entre os módulos Rust e Python.
    * **Gerenciamento de Ambientes:** Utilizar ambientes virtuais Python ou contêineres (e.g., Docker) para isolar as dependências e garantir a portabilidade do ambiente de execução do modelo Python.
    * **Testes de Integração Automatizados:** Desenvolver e executar testes automatizados para validar a comunicação e o fluxo de dados entre as duas linguagens.
    * **Abordagem de Comunicação Clara:** Decidir e documentar explicitamente como a comunicação será realizada (e.g., chamadas de processo, servidor local via HTTP, bibliotecas de binding).

---
**R3: Desempenho Insuficiente no Processamento de Imagens**
* **Categoria:** Técnica / Operacional
* **Probabilidade:** Média
* **Impacto:** Alto
* **Estratégias de Mitigação:**
    * **Otimização de Algoritmos:** Realizar profiling e otimização dos algoritmos de IA e de processamento de imagem para melhorar a eficiência e reduzir o tempo de execução.
    * **Benchmarking de Desempenho:** Realizar testes de desempenho da aplicação em diferentes configurações de hardware e com volumes variados de dados para identificar gargalos.
    * **Processamento em Lotes (Batch Processing):** Implementar a capacidade de processar múltiplas imagens em lote para otimizar o uso de recursos e o tempo total de análise.
    * **Análise de Requisitos:** Validar com o LMPC o "tempo razoável" esperado para o processamento e ajustar as otimizações a esse limite.

---
**R4: Falhas no Armazenamento Local e Integridade dos Dados**
* **Categoria:** Operacional / Técnica
* **Probabilidade:** Baixa
* **Impacto:** Médio
* **Estratégias de Mitigação:**
    * **Mecanismos de Backup:** Implementar rotinas de backup local automáticas ou opções claras para o usuário realizar backups de seus projetos e dados.
    * **Validação de Integridade:** Incluir verificações de integridade dos arquivos (e.g., checksums) no momento do armazenamento e da recuperação para detectar corrupção de dados.
    * **Mensagens de Erro Claras:** Fornecer feedback explícito e compreensível ao usuário em caso de falha no armazenamento, com orientações sobre como proceder.
    * **Uso de Bibliotecas Robustas:** Utilizar bibliotecas padrão e bem testadas para manipulação de arquivos e gerenciamento de caminhos.

---
**R5: Dependência de Ferramenta Externa (Pandoc) para Exportação**
* **Categoria:** Técnica / Operacional
* **Probabilidade:** Média
* **Impacto:** Médio
* **Estratégias de Mitigação:**
    * **Instruções de Instalação Claras:** Fornecer um guia detalhado e simples para a instalação do Pandoc como um pré-requisito da aplicação.
    * **Empacotamento Simplificado:** Se as licenças permitirem e a ferramenta de empacotamento do Dioxus/Rust suportar, explorar a possibilidade de incluir o Pandoc no pacote de instalação da aplicação para facilitar a implantação.
    * **Avaliação de Alternativas Nativas:** No longo prazo, avaliar a viabilidade de desenvolver funcionalidades de exportação diretamente em Rust para reduzir ou eliminar a dependência de ferramentas externas.

---

## 2. Oportunidades do Projeto

A seguir, apresentamos as oportunidades identificadas, sua avaliação e como o projeto pode capitalizá-las.

**Legenda de Avaliação (Oportunidades):**
* **Probabilidade:** Muito Baixa (MB), Baixa (B), Média (M), Alta (A), Muito Alta (MA).
* **Impacto:** Muito Baixo (MB), Baixo (B), Médio (M), Alto (A), Muito Alto (MA).

<br />

![Matriz de Oportunidade](/img/matOp.png)

*(Representação visual conceitual da Matriz de Oportunidade)*

---
**O1: Acesso a Sistemas de Drone (ou Similar)**
* **Categoria:** Técnica / Mercado
* **Probabilidade:** Alta
* **Impacto:** Muito Alto
* **Estratégias de Exploração:**
    * **Parceria Ativa:** Manter e estreitar a colaboração com o IPT para explorar o acesso a drones ou sistemas de coleta de dados similares, visando a integração direta.
    * **Estudo de Caso:** Utilizar o acesso a esses sistemas para aprimorar o modelo de negócio, adaptando a aplicação para um cenário de uso real com drones.
    * **Desenvolvimento Orientado:** Priorizar o desenvolvimento de funcionalidades que suportem a entrada de dados via drone, como o requisito RF-SYS-002, para capitalizar essa oportunidade e diferenciar o produto.

---
**O2: Utilização de Modelos de Imagem Pré-existentes**
* **Categoria:** Técnica / Desenvolvimento
* **Probabilidade:** Alta
* **Impacto:** Médio
* **Estratégias de Exploração:**
    * **Pesquisa e Avaliação:** Realizar uma pesquisa aprofundada de modelos de visão computacional pré-existentes e frameworks de IA para identificar aqueles que podem ser adaptados e integrados ao projeto.
    * **Aceleração do Desenvolvimento:** Adotar modelos ou componentes já testados para agilizar o processo de produção da solução, garantindo um funcionamento preciso em menor tempo.
    * **Redução de Custos:** A utilização de modelos existentes pode reduzir o tempo e o custo de treinamento inicial do modelo.

---
**O3: Expansão para Novos Segmentos de Cliente**
* **Categoria:** Mercado / Estratégica
* **Probabilidade:** Média
* **Impacto:** Alto
* **Estratégias de Exploração:**
    * **Pesquisa de Mercado:** Realizar pesquisas para identificar outros setores (e.g., inspeção de infraestruturas, agricultura de precisão, segurança) que poderiam se beneficiar da tecnologia de análise de fissuras.
    * **Customização da Solução:** Desenvolver módulos ou funcionalidades customizáveis que atendam às necessidades específicas desses novos mercados.
    * **Parcerias Estratégicas:** Buscar parcerias com empresas ou associações atuantes em novos segmentos para facilitar a entrada no mercado.

---
**O4: Monetização de Dados Anônimos de Fissuras**
* **Categoria:** Negócio / Estratégica
* **Probabilidade:** Baixa
* **Impacto:** Médio
* **Estratégias de Exploração:**
    * **Política de Privacidade:** Estabelecer uma política de privacidade clara e transparente para a coleta de dados de fissuras (anonimizados e agregados) com o consentimento dos usuários.
    * **Desenvolvimento de Produtos de Dados:** Criar relatórios de tendências, benchmarks ou modelos de dados agregados que possam ser comercializados para empresas do setor.
    * **Análise de Mercado de Dados:** Avaliar a demanda e o valor potencial do mercado de dados de inspeção estrutural.

---
**O5: Integração com Plataformas de Gerenciamento de Ativos**
* **Categoria:** Técnica / Mercado
* **Probabilidade:** Baixa
* **Impacto:** Alto
* **Estratégias de Exploração:**
    * **Pesquisa de Mercado:** Identificar as principais plataformas de gerenciamento de ativos ou EAM (Enterprise Asset Management) utilizadas no setor de engenharia.
    * **Desenvolvimento de APIs/Plugins:** Criar interfaces de programação de aplicação (APIs) ou plugins que permitam a integração fluida dos resultados do 14-Bis com essas plataformas, facilitando o fluxo de trabalho dos clientes.
    * **Parcerias Tecnológicas:** Buscar parcerias com fornecedores de plataformas de gerenciamento de ativos para desenvolver integrações conjuntas.
---

## 3. Conclusão

&emsp;A aplicação da Matriz de Risco e Oportunidade demonstra que o projeto 14-Bis, embora promissor, enfrenta desafios significativos, principalmente relacionados à acurácia do modelo de IA e à sua usabilidade. No entanto, as oportunidades de integração com tecnologias emergentes como drones e o uso de modelos pré-existentes representam alavancas importantes para o sucesso.

&emsp;É importante ressaltar que a matriz de risco e oportunidade é uma ferramenta dinâmica, e deve ser atualizada conforme o projeto avança, novos riscos e oportunidades surgem, e o impacto e probabilidade dos itens já registrados são reavaliados. A gestão proativa desses fatores é fundamental para garantir a sustentabilidade e o sucesso do 14-Bis.