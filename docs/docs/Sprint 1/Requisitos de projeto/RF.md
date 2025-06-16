---
sidebar_position: 1
title: Requisitos Funcionais
---

## Requisitos Funcionais do Sistema 

<br/>

### Recepção de Imagens de Inspeção por Armazenamento Externo

- **ID:** RF-SYS-001
- **Descrição:** O sistema deve permitir que um usuário (operador ou analista) faça o upload manual de arquivos de imagem contendo registros visuais de superfícies a serem inspecionadas em busca de rachaduras.
- **Critérios de Aceitação:**
  - O sistema deve fornecer uma interface para o upload de arquivos de imagem.
  - O sistema deve aceitar e processar o upload de um ou múltiplos arquivos de imagem.
  - O sistema não deve perder dados visuais relevantes durante o processo de upload.
  - Após o upload bem-sucedido, a imagem deve estar disponível para as próximas etapas de processamento. <br/><br/>

--- 

### Integração e Recepção de Imagens por Drone

- **ID:** RF-SYS-002
- **Descrição:** O sistema deve ser capaz de receber imagens automaticamente de dispositivos externos designados para inspeção através de uma interface de comunicação programática ou protocolo definido.
- **Critérios de Aceitação:**
  - O sistema deve estabelecer e manter a comunicação com o drone.
  - As imagens devem ser transferidas com sucesso dos dispositivo externo para o sistema.
  - A interface de comunicação deve suportar o protocolo e formato de dado exigido pelo dispositivo.
  - O sistema deve implementar mecanismos de tratamento de erros para falhas na comunicação ou transferência de dados, notificando o problema.
  - A recepção das imagens deve ocorrer sem perda de dados visuais relevante. <br/><br/>

---

### Validação e Pré-processamento de Formato de Imagem

- **ID:** RF-SYS-003
- **Descrição:** Após a recepção, o sistema deve validar o formato de arquivo da imagem e realizar pré-processamentos necessários para prepará-la para a análise dimensional. Imagens em formatos incompatíveis devem ser identificadas e tratadas adequadamente.
- **Critérios de Aceitação:**
  - O sistema deve identificar corretamente o formato de arquivo da imagem recebida (ex: JPEG, PNG, BMP - listar formatos suportados).
  - O sistema deve carregar e pré-processar imagens nos formatos especificados sem erros de leitura ou corrupção de dados.
  - O sistema deve detectar e rejeitar imagens que não estejam nos formatos especificados.
  - Ao rejeitar uma imagem, o sistema deve gerar uma notificação ou log indicando a imagem e o motivo da rejeição (formato inválido).<br/><br/>

---

### Análise Geométrica de Fissuras

- **ID:** RF-SYS-004
- **Descrição:** Utilizando algoritmos de processamento de imagem, o sistema deve analisar as imagens pré-processadas para detectar a presença de fissuras e extrair suas características geométricas.
- **Critérios de Aceitação:**
  - O sistema deve detectar as fissuras presentes nas imagens com uma taxa de sucesso [especificar percentual ou métrica]
  - O sistema deve identificar e classificar o formato geométrico predominante de cada fissura detectada (ex: linear, ramificada, irregular).
  - O sistema deve processar a imagem e gerar os resultados da análise em um tempo razoável [especificar tempo máximo].
  - Se nenhuma fissura for detectada na imagem, o sistema deve indicar explicitamente que nenhuma fissura foi encontrada. <br/><br/>

---

### Classificação de Risco Estrutural de Fissuras

- **ID:** RF-SYS-005
- **Descrição:** Com base nas características geométricas extraídas pela análise de imagem e regras de negócio definidas, o sistema deve atribuir um nível de risco estrutural a cada fissura detectada.
- **Critérios de Aceitação:**
  - O sistema deve atribuir um nível de risco (Leve, Moderado, ou Elevado) a _cada_ fissura detectada.
  - A atribuição do nível de risco deve ser consistentemente baseada nos dados de análise e nas regras de negócio/limiares definidos.
  - Se aplicável e definido pelas regras de negócio, o sistema deve classificar o tipo da rachadura.
  - A classificação de risco e tipo deve ser executada automaticamente após a análise. <br/><br/>

---

### Geração e Apresentação da Lista de Fissuras Analisadas

- **ID:** RF-SYS-006
- **Descrição:** Após a conclusão da análise dimensional e classificação de risco de uma imagem, o sistema deve gerar e apresentar uma lista consolidada de todas as fissuras identificadas, incluindo seus detalhes e classificação.
- **Critérios de Aceitação:**
  - O sistema deve gerar a lista de fissuras após a conclusão do pipeline de análise para uma imagem.
  - A lista deve incluir _todas_ as fissuras que foram detectadas e processadas.
  - Para cada fissura na lista, o sistema deve exibir:
    - Um identificador único para a fissura.
    - Seu formato geométrico predominante.
    - Seu nível de risco atribuído (Leve, Moderado, Elevado).
    - Um recorte da imagem daquela fissura
  - As informações apresentadas devem corresponder corretamente à fissura específica na imagem analisada.
  - A lista deve ser apresentada em uma interface navegável.
  - O sistema deve permitir a exportação da lista em um formato comum (ex: CSV, PDF, JSON).
  - Se nenhuma fissura for detectada na imagem, o sistema deve apresentar uma mensagem indicando que nenhuma fissura foi encontrada naquela imagem. <br/><br/>

---

### Armazenamento Local de Imagens Processadas e Resultados

- **ID:** RF-SYS-007
- **Descrição:** O sistema deve armazenar localmente as imagens originais que foram processadas, juntamente com os resultados completos da análise geométrica e de risco associados a cada imagem, para acesso futuro, sem necessidade reprocessamento.
- **Critérios de Aceitação:**
  - Ao concluir a análise completa de uma imagem (incluindo a geração da lista de resultados), o sistema deve armazenar a imagem original em um local de armazenamento local definido.
  - Os resultados da análise (detalhes das fissuras, riscos, etc.) devem ser armazenados de forma persistente e vinculados à sua imagem original correspondente.
  - As imagens e os resultados armazenados localmente devem ser recuperáveis e acessíveis para consulta futura através do sistema.
  - O armazenamento local deve preservar a integridade dos dados da imagem e dos resultados da análise ao longo do tempo.
