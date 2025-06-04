---
sidebar_position: 1
slug: /sprint4/testes/requisitos
title: Teste de requisitos
---

# Objetivos dos testes de requisitos

&emsp;Para os testes de requisitos realizados na sprint 4, o foco foi revisar todos os requisitos elicitados na sprint 1 e corrigi-los, caso não fizessem mais sentido de acordo com o escopo delimitado na sprint 2. Além da revisão, o objetivo foi entender se o produto criado estava de acordo com os requisitos, visto que os RF e RNF foram elicitados levando em conta os aspectos que mais trariam valor à instituição parceira. Dessa forma, com os testes, o foco principal foi entender se a solução agrega valor e possui qualidade conforme o esperado.

# Revisão dos requisitos não funcionais

&emsp;Durante a preparação dos testes, foram encontradas imprecisões nos requisitos não funcionais. Dessa forma, foram revisados os requisitos de tal forma:

- RNF-SYS-002: foram deletados o 3º e o 4º critérios de aceitação do requisito e se tornaram critérios de aceitação teste do mesmo requisito. 
- RNF-SYS-003: foram revisados os critérios de aceitação com base na literatura e bibliografia encontrada em pesquisas e artigos científicos.

&emsp;Abaixo, encontra-se a versão final dos 2 requisitos não funcionais:

#### Usabilidade na Visualização e Gerenciamento de Imagens

- **ID:** RNF-SYS-002  
- **Descrição:** A interface do sistema deve ser intuitiva para técnicos e pesquisadores, permitindo visualização, seleção, marcação e descarte de imagens antes, durante e após o processamento.  
- **Critérios de Aceitação:**
  - Deve obter uma pontuação média no SUS (System Usability Scale) superior a 68.
  - A taxa de sucesso na conclusão das tarefas críticas (ex: selecionar imagens para processamento) deve ser de no mínimo 95%.
  - O sistema deve ser testado com usuários representativos em ambiente simulado, com tarefas reais e questionário de usabilidade.
  - Devem ser registrados tempo de execução, taxa de sucesso e feedbacks qualitativos.
  <br/><br/>

---

#### Precisão na Detecção de Fissuras

- **ID:** RNF-SYS-003  
- **Descrição:** O sistema deve detectar a presença e o contorno das fissuras com alta precisão, minimizando falsos positivos e principalmente falsos negativos, mesmo sob variações nas condições das imagens (iluminação, sujeira, texturas).  
- **Critérios de Aceitação:**
  - O sistema deve alcançar um F1-score de pelo menos 90% na detecção de fissuras.
  - O sistema deve atingir Recall de no mínimo 92% no conjunto de teste.
  - A variação (desvio padrão) do F1-score entre diferentes condições de imagem não deve exceder 0.03.
  - Devem ser calculadas métricas como Acurácia de Pixels, IoU, Precisão, Recall e F1-score com base em anotações manuais de referência.
  - O conjunto de teste deve incluir no mínimo 100 imagens com diversidade de fissuras e condições ambientais.
  <br/><br/>


# Requisitos fora dos testes

&emsp;Durante a etapa de revisão dos requisitos elicitados na Sprint 1, foram encontrados requisitos que foram definidos antes da construção total do escopo do projeto e que, por hoje estarem fora dos planos de desenvolvimento, foram descartados do conjunto de requisitos funcionais e não funcionais do projeto.

&emsp;Isso ocorreu com o requisito ```RF-SYS-002: Integração e Recepção de Imagens por Drone```, visto que é um requisito que necessita da integração entre o drone utilizado nos testes e a aplicação desenvolvida. Entretanto, a instituição parceira utiliza um drone com técnicas de captação e armazenamento de imagem que nos impossibilitaram de testar nosso projeto de forma que agregasse valor real à instituição e permitisse testes minimamente reais de acordo com a jornada dos usuários. Consequentemente, optamos por não continuar com a integração entre o drone e a aplicação, e este requisito não foi testado.

&emsp;Abaixo, encontra-se o detalhamento do requisito deletado:

#### Integração e Recepção de Imagens por Drone

- **ID:** RF-SYS-002
- **Descrição:** O sistema deve ser capaz de receber imagens automaticamente de dispositivos externos designados para inspeção através de uma interface de comunicação programática ou protocolo definido.
- **Critérios de Aceitação:**
  - O sistema deve estabelecer e manter a comunicação com o drone.
  - As imagens devem ser transferidas com sucesso dos dispositivo externo para o sistema.
  - A interface de comunicação deve suportar o protocolo e formato de dado exigido pelo dispositivo.
  - O sistema deve implementar mecanismos de tratamento de erros para falhas na comunicação ou transferência de dados, notificando o problema.
  - A recepção das imagens deve ocorrer sem perda de dados visuais relevante. <br/><br/>

---

# Planejamento dos testes

&emsp;Para preparar os testes, foram analisados os critérios de aceitação e criados os passos para o teste individual de cada requisito. Com essas informações, foi construída a tabela que se encontra na seção do relátorio de resultados e que permite visualizar cada teste, os critérios alcançados e o status do requisito (alcançado/não alcançado).

# Relatório de resultados e análise

|        | Critério 1 | Critério 2 | Critério 3 |
|--------|------------|------------|------------|
| Foi alcançado? |  Sim  |  Não  |  Sim  |
| Observações gerais | tal | tal | tal |