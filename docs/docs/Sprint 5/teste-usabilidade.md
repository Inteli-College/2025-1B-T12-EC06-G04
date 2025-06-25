# Documentação de Testes de Usabilidade — Sprint 5

## Teste de Usabilidade com o Colegas da faculdade

 Nesta etapa da Sprint 5, conduzimos um novo teste de usabilidade com colega de turma, focado em **validar as alterações implementadas na interface** após os feedbacks recebidos na Sprint anterior. O principal objetivo foi verificar se as melhorias no fluxo de uso e na apresentação das funcionalidades essenciais resultaram em uma **experiência mais fluida e satisfatória** para os usuários finais.

 O teste manteve como prioridade a observação da interação do cliente com o sistema em tarefas reais, buscando entender como as mudanças foram percebidas, quais pontos ainda geram fricção e quais requisitos estão efetivamente sendo cumpridos após as atualizações.

<div align="center">

  <sub>Figura 1 - Testes com Cliente (Parte 1)  </sub>

  ![Tela Inicial](../../../static/img/testeusuario1.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>

<div align="center">

  <sub>Figura 1 - Testes com Cliente (Parte 1)  </sub>

  ![Tela Inicial](../../../static/img/testeusuario1.jpeg)

  <sup>Fonte: Material produzido pelos autores (2025).</sup>

</div>

---

## Cenários Testados e Desempenho dos Requisitos

 O teste concentrou-se nos fluxos de trabalho principais da aplicação, especialmente aqueles que receberam modificações na interface e experiência do usuário.

### Criação e Gestão de Projetos  
**Relacionado a:** `RNF-SYS-002 – Usabilidade`  
- O novo layout do formulário de criação foi bem recebido.
- Os campos obrigatórios agora estão devidamente sinalizados, o que facilitou o preenchimento e reduziu dúvidas.
- O botão "+" para adicionar novos projetos continua sendo considerado intuitivo.

---

### Recepção e Gerenciamento de Imagens  
**Relacionado a:** `RF-SYS-001 – Recepção de Imagens por Armazenamento Externo` e `RNF-SYS-002 – Usabilidade`  
- A importação de imagens por pasta local seguiu funcionando normalmente.
- **Melhoria:** A nova pré-visualização de imagens ajudou o usuário a confirmar se o conteúdo estava correto antes do envio. Também, o sistema ja processa automaticamente as imagens após o upload, exigindo intervenção manual.

---

### Navegação e Pesquisa  
**Relacionado a:** `RNF-SYS-002 – Usabilidade`  
- A busca por nome de projeto e filtragem de imagens funcionou de forma eficiente.
- A interface de pesquisa foi elogiada pela simplicidade.

---

### Visualização de Resultados e Relatórios  
**Relacionado a:** `RF-SYS-006 – Geração e Apresentação da Lista de Fissuras Analisadas`  
- Os gráficos por projeto foram visualizados corretamente.
- **Nova Funcionalidade:** A visualização por fachada foi implementada e foi **bem recebida pelo teste**, que destacou sua importância para inspeções segmentadas.
- A apresentação geral dos dados foi considerada clara e bem organizada.


---

### Exportação de Relatórios  
**Relacionado a:** `RF-SYS-006 – Geração e Apresentação da Lista de Fissuras Analisadas`  
- **Falha Corrigida:** O sistema agora permite exportar relatórios em formato PDF e DOCX.
- A exportação foi testada com sucesso, representando um avanço significativo em relação à Sprint anterior.

---


---

## Tabela de Status dos Requisitos Avaliados

| ID Requisito   | Descrição                                                        | Status               
|----------------|------------------------------------------------------------------|------------------------|
| RF-SYS-001     | Recepção de Imagens de Inspeção por Armazenamento Externo       | **Cumprido**  
| RF-SYS-002     | Integração e Recepção de Imagens por Drone                      | **Parcialmente Cumprido** 
| RF-SYS-003     | Validação e Pré-processamento de Formato de Imagem              |**Cumprido**  
| RF-SYS-004     | Análise Geométrica de Fissuras                                  | **Cumprido** 
| RF-SYS-005     | Classificação de Risco Estrutural de Fissuras                   | **Cumprido** 
| RF-SYS-006     | Geração e Apresentação da Lista de Fissuras Analisadas          | **Cumprido**  
| RF-SYS-007     | Armazenamento Local de Imagens Processadas e Resultados         | **Cumprido** 
| RNF-SYS-002    | Usabilidade na Visualização e Gerenciamento de Imagens          | **Cumprido** 
| RNF-SYS-003    | Precisão na Detecção de Fissuras                                | **Cumprido** 
---


## Conclusão

 O teste de usabilidade da Sprint 5 evidenciou um avanço significativo em relação à experiência do usuário. As modificações na interface e o foco na clareza visual e funcional trouxeram retornos positivos por parte do IPT. Algumas limitações ainda permanecem, especialmente relacionadas ao processamento automático e à comunicação visual de ações em andamento, que serão o foco das próximas sprints.

 A escuta ativa do cliente, aliada a testes recorrentes e objetivos, tem se mostrado essencial para o alinhamento entre produto e necessidade real. O time seguirá priorizando os pontos apontados para garantir um produto funcional, intuitivo e completo ao final do ciclo.

---


