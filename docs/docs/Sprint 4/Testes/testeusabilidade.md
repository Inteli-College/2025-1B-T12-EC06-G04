---
title: Teste de Usabilidade
sidebar_position: 1
---

---

## Teste de Usabilidade com o Cliente Real

&emsp;Nesta etapa crucial do projeto, realizamos um **teste de usabilidade com o cliente real (IPT)** para validar a interface e o fluxo de trabalho da aplicação. Nosso principal objetivo foi observar de perto a interação dos usuários-alvo com o sistema, coletando feedback direto sobre a **facilidade de uso**, **pontos de melhoria** e a **eficácia das funcionalidades essenciais**. As observações feitas durante esse processo são fundamentais para guiar nossos próximos passos de desenvolvimento, assegurando que o produto final atenda plenamente às necessidades e expectativas do IPT.

---

### Cenários Testados e Desempenho dos Requisitos

&emsp;O teste focou nos fluxos de trabalho mais importantes da aplicação, avaliando a capacidade do cliente de executar tarefas propostas de forma intuitiva e, principalmente, se os requisitos definidos estavam sendo cumpridos.

<div align="center">

  <sub>Figura 1 - Testes com Cliente (Parte 1)  </sub>

  ![Tela Inicial](../../../static/img/testeusuario1.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>

* **Criação e Gestão de Projetos (Relacionado a RNF-SYS-002 - Usabilidade):**
    * O usuário **conseguiu criar um novo projeto** e **preencher suas informações** sem dificuldades, o que indica que o ponto de entrada principal e o fluxo de cadastro são claros e intuitivos.
* **Recepção e Gerenciamento de Imagens (Relacionado a RF-SYS-001 - Recepção de Imagens por Armazenamento Externo e RNF-SYS-002 - Usabilidade):**
    * A **seleção de fotos** de uma pasta local funcionou como esperado, permitindo que o usuário importasse as imagens necessárias para análise.
    * **Ponto Crítico:** No entanto, o sistema **não conseguiu separar e processar as imagens automaticamente** conforme o fluxo ideal. Isso impacta diretamente a autonomia esperada pelo **RF-SYS-001** e a usabilidade para grandes volumes de dados.
* **Navegação e Pesquisa (Relacionado a RNF-SYS-002 - Usabilidade):**
    * A **barra de pesquisa** foi utilizada com sucesso, mostrando que a funcionalidade de busca e filtragem de conteúdo é acessível e eficaz.
* **Visualização de Resultados e Relatórios (Relacionado a RF-SYS-006 - Geração e Apresentação da Lista de Fissuras Analisadas e RNF-SYS-002 - Usabilidade):**
    * O cliente **conseguiu visualizar projetos anteriores** e navegar pelos **gráficos analíticos** associados, confirmando a clareza na apresentação de dados históricos.
    * A funcionalidade de **visualização do relatório** também foi bem-sucedida, permitindo o acesso às informações detalhadas geradas pelo sistema.
* **Exportação de Relatórios (Relacionado a RF-SYS-006 - Geração e Apresentação da Lista de Fissuras Analisadas):**
    * **Falha Crítica:** O usuário **não conseguiu exportar o arquivo em PDF/DOCX**. Este é um bloqueio significativo na conclusão de uma tarefa essencial e representa uma falha no atendimento integral ao critério de aceitação do **RF-SYS-006** (que exige a exportação em formato comum).

<div align="center">

  <sub>Figura 2 - Testes com Cliente (Parte 2) </sub>

  ![Tela Inicial](../../../static/img/testeusuario2.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>

---

### Feedback Qualitativo e Pontos de Melhoria (Foco na Usabilidade - RNF-SYS-002)

&emsp;Além da performance em cenários específicos, coletamos feedbacks qualitativos valiosos sobre a experiência geral do usuário, com foco direto no requisito **RNF-SYS-002 (Usabilidade)**:

* **Design e Intuitividade:** O cliente **aprovou o botão "+" para iniciar ações**, considerando-o intuitivo e bem posicionado, o que contribui positivamente para a usabilidade geral.
* **Feedback de Processamento:** Identificamos a **falta de confirmação visual** de que as ações estão sendo processadas (por exemplo, ao salvar dados ou processar imagens). O usuário precisa de um retorno claro de que "as coisas estão acontecendo" em segundo plano, impactando a percepção de controle e a fluidez da interação.
* **Campos Obrigatórios:** Sugerimos a **indicação clara de campos obrigatórios** (com um asterisco ou similar) para guiar o preenchimento de informações, aprimorando a clareza da interface.
* **Visualização de Gráficos por Fachada:** O cliente solicitou a funcionalidade de **visualizar os dados gráficos por fachada**, além da visualização por prédio. Isso agrega maior granularidade e valor à análise, enriquecendo a apresentação de resultados do **RF-SYS-006** e melhorando a usabilidade para cenários específicos do IPT.

---

### Tabela de Status dos Requisitos Avaliados no Teste de Usabilidade

&emsp;A tabela abaixo resume o status dos principais requisitos funcionais e não funcionais avaliados durante o teste de usabilidade. Requisitos não mencionados especificamente no feedback do cliente são listados como "Não Avaliado diretamente neste teste", indicando que seu status foi aferido por outras formas ou será em testes futuros.

| ID Requisito | Descrição                                             | Status                 | Observações Chave do Teste de Usabilidade                                                                                                                                                 |
| :----------- | :---------------------------------------------------- | :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **RF-SYS-001** | Recepção de Imagens de Inspeção por Armazenamento Externo | Parcialmente Cumprido  | Seleção de fotos da pasta funcionou. **Problema na separação/processamento automático** das imagens, que afeta a autonomia.                                                   |
| **RF-SYS-002** | Integração e Recepção de Imagens por Drone            | Não Avaliado neste teste | A funcionalidade de integração com drone não foi o foco deste teste de usabilidade com o usuário final.                                                                       |
| **RF-SYS-003** | Validação e Pré-processamento de Formato de Imagem   | Não Avaliado neste teste | O pré-processamento de imagens não foi um cenário de observação direta de usabilidade para o cliente final nesta fase.                                                         |
| **RF-SYS-004** | Análise Geométrica de Fissuras                        | Não Avaliado neste teste | A análise interna de fissuras por algoritmos não foi observada diretamente na interface pelo cliente final nesta fase.                                                         |
| **RF-SYS-005** | Classificação de Risco Estrutural de Fissuras         | Não Avaliado neste teste | A classificação de risco não foi um cenário de observação direta de usabilidade para o cliente final nesta fase, embora os resultados sejam visualizados.                       |
| **RF-SYS-006** | Geração e Apresentação da Lista de Fissuras Analisadas  | Parcialmente Cumprido  | Visualização de projetos, gráficos e relatórios OK. **Falha crítica na exportação (PDF/DOCX)**. Necessidade de visualização de gráficos por fachada.                        |
| **RF-SYS-007** | Armazenamento Local de Imagens Processadas e Resultados | Não Avaliado neste teste | O armazenamento local não foi um cenário de observação direta de usabilidade para o cliente final nesta fase.                                                                |
| **RNF-SYS-002** | Usabilidade na Visualização e Gerenciamento de Imagens | Parcialmente Cumprido  | Bom feedback sobre o botão "+". **Ausência de feedback visual** de processamento e necessidade de indicação de campos obrigatórios.                                            |
| **RNF-SYS-003** | Precisão na Detecção de Fissuras                      | Não Avaliado neste teste | A precisão do modelo é uma métrica técnica avaliada separadamente (como visto na documentação do modelo YOLO), não diretamente pela usabilidade do cliente.                   |


---

### Próximos Passos e Prioridades

&emsp;Com base nos resultados deste teste de usabilidade e na análise detalhada dos requisitos, definimos as seguintes prioridades para as próximas iterações de desenvolvimento:

* **Prioridade Alta: Correção da Exportação de Relatórios (RF-SYS-006):** Resolver urgentemente a falha na exportação para PDF/DOCX, pois é um bloqueador para a conclusão do ciclo de trabalho do usuário.
* **Prioridade Alta: Aprimoramento do Processamento de Imagens (RF-SYS-001):** Investigar e corrigir a falha no processamento automático e separação de imagens para otimizar a recepção e gestão de dados, impactando diretamente a eficiência do usuário.
* **Prioridade Média: Implementação de Feedback Visual (RNF-SYS-002):** Adicionar indicadores de carregamento, sucesso ou erro para todas as ações do usuário, melhorando a experiência e a percepção de controle.
* **Prioridade Média: Melhorias na Interface (RNF-SYS-002 e RF-SYS-006):** Implementar a indicação de campos obrigatórios e desenvolver a funcionalidade de visualização de gráficos por fachada, enriquecendo a apresentação e interação com os dados.

&emsp;Este feedback direto do cliente é vital para garantir que o desenvolvimento esteja sempre alinhado às necessidades reais do IPT, priorizando a usabilidade e as funcionalidades que trarão o maior valor, e nos permitindo atender de forma mais completa aos requisitos definidos.