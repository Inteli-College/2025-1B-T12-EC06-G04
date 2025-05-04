---
sidebar_position: 1
title: Requisitos Funcionais
---

## Requisitos Funcionais do Sistema

### Requisito Funcional 1

- **ID:** RF-SYS-001
- **Nome:** Recepção de Imagens de Inspeção (Manual)
- **Descrição:** O sistema deve permitir que um usuário (operador ou analista) faça o upload manual de arquivos de imagem contendo registros visuais de superfícies a serem inspecionadas em busca de rachaduras. Este mecanismo serve como um ponto de entrada de dados alternativo ou primário para a análise.
- **Pré-condições:**
  - O usuário deve ter permissão para acessar a funcionalidade de upload.
  - A imagem deve estar acessível no dispositivo do usuário.
- **Pós-condições/Saídas:**
  - A imagem é armazenada temporariamente no sistema.
  - A imagem está disponível para o processo de validação e processamento (RF-SYS-003).
  - Uma confirmação de upload bem-sucedido é exibida ao usuário, ou uma mensagem de erro caso contrário.
- **Critérios de Aceitação:**
  - O sistema deve fornecer uma interface clara para o upload de arquivos de imagem.
  - O sistema deve aceitar e processar o upload de um ou múltiplos arquivos de imagem simultaneamente.
  - O sistema não deve perder dados visuais relevantes durante o processo de upload.
  - Após o upload bem-sucedido, a imagem deve estar disponível para as próximas etapas de processamento em um local acessível internamente.
  - O sistema deve notificar o usuário em caso de falha no upload, indicando a causa (ex: arquivo corrompido, formato inválido - validação detalhada em RF-SYS-003).
- **Prioridade:** Alta 

---

### Requisito Funcional 2

- **ID:** RF-SYS-002
- **Nome:** Integração e Recepção de Imagens (Automática)
- **Descrição:** O sistema deve ser capaz de receber imagens automaticamente de dispositivos externos designados para inspeção (como Turtlebots ou drones) através de uma interface de comunicação programática ou protocolo definido.
- **Pré-condições:**
  - Os dispositivos externos devem estar configurados para se conectar ao sistema.
  - O sistema e os dispositivos externos devem estar na mesma rede ou ter conectividade estabelecida.
  - A interface de comunicação definida deve estar ativa e pronta para receber dados.
- **Pós-condições/Saídas:**
  - As imagens recebidas dos dispositivos externos são armazenadas temporariamente no sistema.
  - As imagens estão disponíveis para o processo de validação e pré-processamento (RF-SYS-003).
  - Um registro da recepção da imagem (sucesso/falha) é mantido.
- **Critérios de Aceitação:**
  - O sistema deve estabelecer e manter a comunicação com os tipos de dispositivos externos especificados (ex: Turtlebot, Drone).
  - As imagens devem ser transferidas com sucesso dos dispositivos externos para o sistema.
  - A interface de comunicação deve suportar o protocolo e formato de dados exigidos pelos dispositivos externos.
  - O sistema deve implementar mecanismos de tratamento de erros para falhas na comunicação ou transferência de dados, notificando o problema.
  - A recepção das imagens deve ocorrer sem perda de dados visuais relevante.
- **Prioridade:** Alta

---

### Requisito Funcional 3

- **ID:** RF-SYS-003
- **Nome:** Validação e Pré-processamento de Formato de Imagem
- **Descrição:** Após a recepção (via upload manual ou integração automática), o sistema deve validar o formato de arquivo da imagem e realizar pré-processamentos necessários para prepará-la para a análise dimensional. Imagens em formatos incompatíveis devem ser identificadas e tratadas adequadamente.
- **Pré-condições:**
  - Uma imagem foi recebida com sucesso e armazenada temporariamente (via RF-SYS-001 ou RF-SYS-002).
  - A imagem está acessível pelo módulo de processamento de formato.
- **Pós-condições/Saídas:**
  - Se válida, a imagem é marcada como pronta para análise dimensional (RF-SYS-004).
  - Se inválida, a imagem é rejeitada e um evento de erro/notificação é gerado.
- **Critérios de Aceitação:**
  - O sistema deve identificar corretamente o formato de arquivo da imagem recebida (ex: JPEG, PNG, BMP - listar formatos suportados).
  - O sistema deve carregar e pré-processar imagens nos formatos especificados sem erros de leitura ou corrupção de dados.
  - O sistema deve detectar e rejeitar imagens que não estejam nos formatos especificados.
  - Ao rejeitar uma imagem, o sistema deve gerar uma notificação ou log indicando a imagem e o motivo da rejeição (formato inválido).
  - O pré-processamento (se houver, ex: redimensionamento, correção básica) deve ser aplicado consistentemente sem degradar a qualidade da imagem para análise subsequente.
- **Prioridade:** Alta

---

### Requisito Funcional 4

- **ID:** RF-SYS-004
- **Nome:** Análise Dimensional e Geométrica de Fissuras
- **Descrição:** Utilizando algoritmos de processamento de imagem, o sistema deve analisar as imagens pré-processadas para detectar a presença de fissuras e extrair suas características dimensionais e geométricas.
- **Pré-condições:**
  - Uma imagem válida e pré-processada está disponível para análise (proveniente de RF-SYS-003).
  - Os algoritmos de análise de imagem estão calibrados e prontos para execução.
- **Pós-condições/Saídas:**
  - Uma lista de fissuras detectadas na imagem, cada uma com suas características dimensionais (comprimento, largura média/variável, área) e geométricas (ex: linear, ramificada).
  - Se nenhuma fissura for detectada, uma indicação de "nenhuma fissura encontrada" para aquela imagem.
- **Critérios de Aceitação:**
  - O sistema deve detectar as fissuras presentes nas imagens com uma taxa de sucesso [especificar percentual ou métrica, ex: F1 Score > X].
  - Para cada fissura detectada, o sistema deve calcular seu comprimento total com precisão mínima de [especificar unidade e valor, ex: ± 1 mm].
  - Para cada fissura detectada, o sistema deve medir sua largura (média ou em múltiplos pontos) com precisão mínima de [especificar unidade e valor, ex: ± 0.1 mm].
  - O sistema deve calcular a área total coberta por cada fissura detectada.
  - O sistema deve identificar e classificar o formato geométrico predominante de cada fissura detectada (ex: linear, ramificada, irregular).
  - O sistema deve processar a imagem e gerar os resultados da análise em um tempo razoável [especificar tempo máximo, ex: X segundos por imagem].
  - Se nenhuma fissura for detectada na imagem, o sistema deve indicar explicitamente que nenhuma fissura foi encontrada.
- **Prioridade:** Crítica

---

### Requisito Funcional 5

- **ID:** RF-SYS-005
- **Nome:** Classificação de Risco Estrutural de Fissuras
- **Descrição:** Com base nas características dimensionais e geométricas extraídas pela análise de imagem (RF-SYS-004) e regras de negócio definidas, o sistema deve atribuir um nível de risco estrutural a cada fissura detectada.
- **Pré-condições:**
  - Uma lista de fissuras foi detectada e suas características dimensionais/geométricas foram extraídas (proveniente de RF-SYS-004).
  - As regras de classificação de risco estão configuradas no sistema.
- **Pós-condições/Saídas:**
  - Para cada fissura identificada, um nível de risco atribuído (Leve, Moderado, ou Elevado).
  - Opcionalmente, a classificação do tipo da fissura (ex: retração, assentamento) se a regra de negócio incluir essa determinação baseada nas características.
- **Critérios de Aceitação:**
  - O sistema deve atribuir um nível de risco (Leve, Moderado, ou Elevado) a _cada_ fissura detectada em RF-SYS-004.
  - A atribuição do nível de risco deve ser consistentemente baseada nos dados de análise (comprimento, largura, área, formato) e nas regras de negócio/limiares definidos.
  - Se aplicável e definido pelas regras de negócio, o sistema deve classificar o tipo da rachadura.
  - A classificação de risco e tipo deve ser executada automaticamente após a análise dimensional.
- **Prioridade:** Crítica

---

### Requisito Funcional 6

- **ID:** RF-SYS-006
- **Nome:** Geração e Apresentação da Lista de Fissuras Analisadas
- **Descrição:** Após a conclusão da análise dimensional (RF-SYS-004) e classificação de risco (RF-SYS-005) de uma imagem, o sistema deve gerar e apresentar uma lista consolidada de todas as fissuras identificadas, incluindo seus detalhes dimensionais e de classificação.
- **Pré-condições:**
  - A análise dimensional (RF-SYS-004) e a classificação de risco (RF-SYS-005) foram concluídas para pelo menos uma imagem.
- **Pós-condições/Saídas:**
  - Uma lista digital (ex: na interface do usuário, arquivo de relatório) contendo as informações de cada fissura detectada.
  - A lista está disponível para visualização, filtragem e/ou exportação.
- **Critérios de Aceitação:**
  - O sistema deve gerar a lista de fissuras após a conclusão do pipeline de análise para uma imagem.
  - A lista deve incluir _todas_ as fissuras que foram detectadas e processadas em RF-SYS-004.
  - Para cada fissura na lista, o sistema deve exibir:
    - Um identificador único para a fissura.
    - Suas características dimensionais (comprimento, largura média, área).
    - Seu formato geométrico predominante.
    - Seu nível de risco atribuído (Leve, Moderado, Elevado).
    - Opcionalmente, seu tipo (se classificado em RF-SYS-005).
    - Uma referência ou link para a localização da fissura na imagem original ou uma representação visual (ex: recorte).
  - As informações apresentadas devem corresponder corretamente à fissura específica na imagem analisada.
  - A lista deve ser apresentada em um formato legível e navegável na interface do usuário.
  - O sistema deve permitir a exportação da lista em um formato comum (ex: CSV, PDF, JSON).
  - Se nenhuma fissura for detectada na imagem, o sistema deve apresentar uma mensagem indicando que nenhuma fissura foi encontrada, em vez de uma lista vazia.
- **Prioridade:** Alta

---

### Requisito Funcional 7

- **ID:** RF-SYS-007
- **Nome:** Gerenciamento do Ciclo de Vida de Imagens e Dados Temporários
- **Descrição:** O sistema deve gerenciar o armazenamento temporário das imagens recebidas e dos dados intermediários gerados durante o processo de análise, garantindo que sejam mantidos apenas pelo tempo necessário para o processamento e removidos após a conclusão bem-sucedida ou falha tratada da análise.
- **Pré-condições:**
  - Uma imagem foi recebida e armazenada temporariamente.
  - Dados intermediários (ex: resultados parciais da análise) foram gerados.
- **Pós-condições/Saídas:**
  - Imagens e dados temporários relacionados a um processo de análise concluído (sucesso ou falha tratada) são removidos do armazenamento temporário.
  - O espaço de armazenamento temporário é liberado.
- **Critérios de Aceitação:**
  - O sistema deve armazenar as imagens recebidas e os dados intermediários em um local designado para armazenamento temporário.
  - O módulo de processamento deve conseguir acessar os dados armazenados temporariamente.
  - Após a conclusão bem-sucedida da análise de uma imagem (geração da lista de fissuras, RF-SYS-006), a imagem original e os dados temporários associados devem ser automaticamente removidos ou marcados para exclusão imediata do armazenamento temporário.
  - Em caso de falha no processamento de uma imagem (após tratamento de erro), a imagem e os dados temporários associados devem ser removidos ou marcados para exclusão após um período configurável [especificar tempo, ex: 24 horas] ou acionamento manual.
  - O espaço de armazenamento temporário não deve crescer indefinidamente devido a arquivos não removidos de processos concluídos ou falhos.
  - Deve existir uma política clara sobre por quanto tempo os dados temporários são retidos em caso de falhas ou processos incompletos.
- **Prioridade:** Média
