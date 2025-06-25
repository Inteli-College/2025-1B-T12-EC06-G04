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
- **Melhoria:** A nova pré-visualização de imagens ajudou o usuário a confirmar se o conteúdo estava correto antes do envio.
- **Ponto Crítico Persistente:** O sistema ainda não processa automaticamente as imagens após o upload, exigindo intervenção manual.

---

### Navegação e Pesquisa  
**Relacionado a:** `RNF-SYS-002 – Usabilidade`  
- A busca por nome de projeto e filtragem de imagens funcionou de forma eficiente.
- A interface de pesquisa foi elogiada pela simplicidade e desempenho.

---

### Visualização de Resultados e Relatórios  
**Relacionado a:** `RF-SYS-006 – Geração e Apresentação da Lista de Fissuras Analisadas`  
- Os gráficos por projeto foram visualizados corretamente.
- **Nova Funcionalidade:** A visualização por fachada foi implementada e foi **bem recebida pelo cliente**, que destacou sua importância para inspeções segmentadas.
- A apresentação geral dos dados foi considerada clara e bem organizada.

---

### Exportação de Relatórios  
**Relacionado a:** `RF-SYS-006 – Geração e Apresentação da Lista de Fissuras Analisadas`  
- **Falha Corrigida:** O sistema agora permite exportar relatórios em formato PDF e DOCX.
- A exportação foi testada com sucesso, representando um avanço significativo em relação à Sprint anterior.

---

## Feedback Qualitativo e Pontos de Melhoria

Com foco contínuo no requisito `RNF-SYS-002 – Usabilidade`, o feedback do cliente destacou os seguintes pontos:

### Aspectos Positivos:
- Melhor organização visual e hierarquia de informações;
- Uso de cores mais consistente e útil para destacar ações importantes;
- Campos obrigatórios bem sinalizados;
- Implementação da visualização por fachada agregou valor analítico.

### Sugestões e Melhorias Restantes:
- Incluir feedback visual (ex: barra de carregamento) em mais pontos do sistema;
- Possibilidade de editar dados de projetos já cadastrados de forma mais acessível;
- Pequenos ajustes em mensagens de erro ainda genéricas.

---

## Tabela de Status dos Requisitos Avaliados

| ID Requisito   | Descrição                                                        | Status                | Observações Sprint 5                                                          |
|----------------|------------------------------------------------------------------|------------------------|--------------------------------------------------------------------------------|
| RF-SYS-001     | Recepção de Imagens de Inspeção por Armazenamento Externo       | Parcialmente Cumprido | Importação funcional, mas sem processamento automático após upload.          |
| RF-SYS-002     | Integração e Recepção de Imagens por Drone                      | Não Avaliado           | Não testado com cliente nesta sprint.                                         |
| RF-SYS-003     | Validação e Pré-processamento de Formato de Imagem              | Não Avaliado           | Não observado diretamente pelo cliente.                                       |
| RF-SYS-004     | Análise Geométrica de Fissuras                                  | Não Avaliado           | Processo interno, não avaliado pela interface.                                |
| RF-SYS-005     | Classificação de Risco Estrutural de Fissuras                   | Não Avaliado           | Métrica técnica, não avaliada em teste de usabilidade.                        |
| RF-SYS-006     | Geração e Apresentação da Lista de Fissuras Analisadas          | **Cumprido**           | Visualização e exportação funcionando; visualização por fachada implementada. |
| RF-SYS-007     | Armazenamento Local de Imagens Processadas e Resultados         | Não Avaliado           | Armazenamento não foi foco do teste de usabilidade.                           |
| RNF-SYS-002    | Usabilidade na Visualização e Gerenciamento de Imagens          | **Parcialmente Cumprido** | Interface aprimorada, mas ainda requer feedback visual de ações em andamento. |
| RNF-SYS-003    | Precisão na Detecção de Fissuras                                | Não Avaliado           | Avaliado em testes técnicos, não via cliente final.                           |

---

## Próximos Passos e Prioridades

Com base nos resultados do teste de usabilidade da Sprint 5, definimos as seguintes prioridades para as próximas entregas:

### Prioridade Alta:
- **Automatizar o processamento de imagens após o upload** (`RF-SYS-001`);
- **Aprimorar feedback visual durante ações críticas** (ex: carregamento, salvamento) (`RNF-SYS-002`).

### Prioridade Média:
- **Permitir edição facilitada de projetos existentes**;
- **Refinar mensagens de erro e validações**.

### Priorização Concluída:
- **Exportação de relatórios em PDF/DOCX** corrigida com sucesso (`RF-SYS-006`);
- **Visualização gráfica por fachada** implementada conforme sugestão do cliente (`RF-SYS-006`).

---

## Conclusão

 O teste de usabilidade da Sprint 5 evidenciou um avanço significativo em relação à experiência do usuário. As modificações na interface e o foco na clareza visual e funcional trouxeram retornos positivos por parte do IPT. Algumas limitações ainda permanecem, especialmente relacionadas ao processamento automático e à comunicação visual de ações em andamento, que serão o foco das próximas sprints.

 A escuta ativa do cliente, aliada a testes recorrentes e objetivos, tem se mostrado essencial para o alinhamento entre produto e necessidade real. O time seguirá priorizando os pontos apontados para garantir um produto funcional, intuitivo e completo ao final do ciclo.

---

## Anexos

- Capturas de tela das interações com o cliente;
- Vídeos de navegação (se aplicável);
- Registro dos feedbacks recebidos;
- Lista de tarefas testadas.

