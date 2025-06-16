---
title: Teste de Usabilidade
sidebar_position: 1
---


---
## Teste de Usabilidade com o Cliente Real

&emsp;Nesta etapa do projeto, realizamos um **teste de usabilidade com o cliente real (IPT)** para validar a interface e o fluxo de trabalho da aplicação. O objetivo foi observar a interação dos usuários-alvo com o sistema e coletar feedback direto sobre a facilidade de uso, pontos de melhoria e funcionalidades essenciais. As observações foram cruciais para guiar os próximos passos de desenvolvimento, garantindo que o produto atenda efetivamente às necessidades e expectativas do IPT.

---

### Cenários Testados e Desempenho

&emsp;O teste focou nos fluxos de trabalho centrais da aplicação, avaliando se o cliente conseguia executar as tarefas propostas de forma intuitiva.

<div align="center">

  <sub>Figura 1 - Testes com cliente 2  </sub>

  ![Tela Inicial](../../../static/img/testeusuario1.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>


* **Criação de Novo Projeto:** O usuário **conseguiu criar um novo projeto** sem dificuldades, indicando que o ponto de entrada principal da aplicação é claro.
* **Preenchimento de Informações do Projeto:** O processo de **preenchimento das informações** de um novo projeto foi bem-sucedido, demonstrando que os campos são compreensíveis e a navegação intuitiva.
* **Seleção de Fotos da Pasta:** A funcionalidade de **seleção de fotos** a partir de uma pasta local funcionou conforme o esperado, permitindo ao usuário importar as imagens necessárias para análise.
* **Utilização da Barra de Pesquisa:** A barra de pesquisa foi utilizada com sucesso, indicando que a funcionalidade de **busca e filtragem** de conteúdo é acessível e eficaz.
* **Visualização de Projetos Anteriores e Gráficos:** O cliente **conseguiu visualizar um projeto anterior** e navegar pelos **gráficos analíticos** associados, confirmando a clareza na apresentação dos dados históricos.
* **Visualização de Relatórios:** A funcionalidade de **visualização do relatório** foi bem-sucedida, permitindo ao usuário acessar as informações detalhadas geradas pelo sistema.
* **Exportação de Relatórios (PDF/DOCX):** **Falha crítica**: O usuário **não conseguiu exportar o arquivo em PDF/DOCX**. Este ponto representa um bloqueio significativo na conclusão de uma tarefa essencial e exige correção prioritária.

<div align="center">

  <sub>Figura 2 - Testes com cliente 2 </sub>

  ![Tela Inicial](../../../static/img/testeusuario2.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>


---

### Feedback Qualitativo e Pontos de Melhoria

&emsp;Além do desempenho em cenários específicos, coletamos valiosos feedbacks qualitativos sobre a experiência geral do usuário:

* **Design e Usabilidade:** O cliente expressou **satisfação com o botão "+" para iniciar ações**, considerando-o intuitivo e bem posicionado.
* **Feedback de Processamento (Confirmação Visual):** Foi identificada a **falta de confirmação visual** de que as ações estão sendo processadas (por exemplo, ao salvar dados ou processar imagens). O usuário precisa de um retorno claro de que "as coisas estão acontecendo" em segundo plano.
* **Processamento Automático de Imagens:** O teste revelou que o sistema **falhou em separar e processar as imagens automaticamente**, conforme o fluxo ideal esperado. Este é um gargalo operacional que impacta a autonomia prometida.
* **Campos Obrigatórios:** Sugere-se a **indicação clara de campos obrigatórios** (com um asterisco ou similar) para orientar o preenchimento de informações.
* **Visualização de Gráficos (Por Fachada):** O cliente solicitou a funcionalidade de **visualizar os dados gráficos por fachada**, além da visualização já existente por prédio. Isso agrega granularidade e valor à análise.

---

### Próximos Passos e Prioridades

&emsp;Com base neste teste de usabilidade, as principais prioridades para as próximas iterações incluem:

* **Correção da Exportação de Relatórios:** Solucionar a falha na exportação para PDF/DOCX.
* **Implementação de Feedback Visual:** Adicionar indicadores de carregamento, sucesso ou erro para todas as ações do usuário.
* **Aprimoramento do Processamento de Imagens:** Investigar e corrigir a falha no processamento automático e separação de imagens.
* **Melhorias na Interface:** Adicionar indicadores de campos obrigatórios e desenvolver a funcionalidade de visualização de gráficos por fachada.

&emsp;Este feedback direto do cliente é fundamental para garantir que o desenvolvimento esteja alinhado com as necessidades reais do IPT, priorizando a usabilidade e a funcionalidade que trarão maior valor.