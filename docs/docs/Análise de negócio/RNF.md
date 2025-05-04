---
sidebar_position: 1
title: Requisitos Não Funcionais
---

## Requisitos Não Funcionais

### Precisão na Classificação de Fissuras

- **Requisito:** O sistema deve classificar as fissuras detectadas em categorias de risco (Leve, Moderado, Elevado) e/ou tipo (se aplicável, com alta precisão. A performance da classificação deve ser robusta a variações nas características geométricas das fissuras e nas condições das imagens de origem (iluminação, texturas, etc.).

- **Método de Teste:**
  - **Preparação:**
    - Criar ou obter um conjunto de teste contendo no mínimo [especificar número, ex: 80] instâncias de fissuras detectadas e extraídas.
    - Para cada instância de fissura no conjunto de teste, definir uma "verdade fundamental" para sua categoria de risco e, se aplicável, seu tipo (classificados manualmente por especialistas).
    - Garantir que o conjunto de teste inclua uma distribuição representativa das diferentes categorias de risco/tipo esperadas e varie as características das fissuras e as condições das imagens de onde foram extraídas.
    - Este conjunto de teste deve ser independente de quaisquer dados usados para treinar ou ajustar os algoritmos de classificação.<br/><br/>
  - **Execução:**
    - Alimentar as características geométricas das fissuras do conjunto de teste no módulo de Classificação de Risco Estrutural do sistema.
    - Coletar as categorias de risco e/ou tipos atribuídos pelo sistema para cada fissura.<br/><br/>
  - **Medição:**
    - Comparar as classificações atribuídas pelo sistema com a "verdade fundamental" para cada fissura no conjunto de teste.
    - Calcular a matriz de confusão para avaliar o desempenho da classificação entre as diferentes categorias.
    - Calcular métricas de avaliação de classificação padrão, como precisão, recall e F1-score, para cada categoria de risco/tipo e uma média ponderada global.
    - Se relevante, analisar o desempenho das métricas em subconjuntos do teste com base nas condições da imagem de origem ou características da fissura.<br/><br/>
  - **Análise:**
    - Analisar os resultados da matriz de confusão e das métricas para identificar pontos fortes e fracos do classificador (ex: quais categorias são frequentemente confundidas, qual o erro mais comum).
    - Avaliar a performance média e a variabilidade entre as diferentes condições de imagem ou características das fissuras.<br/><br/>
  - **Critério de Sucesso:**
    - O sistema deve atingir um F1-score ponderado médio de no mínimo 88% na classificação de risco/tipo sobre o conjunto de teste independente.
    - Nenhuma categoria de risco (Leve, Moderado, Elevado) deve ter um Recall inferior a 90% para garantir que poucas fissuras de alto risco sejam classificadas incorretamente como de baixo risco.
    - A variação (desvio padrão) do F1-score em subconjuntos relevantes das condições de imagem não deve exceder 0.05.

---

### Usabilidade na Visualização e Gerenciamento de Imagens

- **Requisito:** A interface do usuário do sistema, especificamente as funcionalidades de visualização e gerenciamento de imagens ligados à momentos antes, durante, e após o processamento, deve ser intuitiva para os usuários alvo (técnicos e pesquisadores), permitindo que visualizem as imagens capturadas, selecionem, marquem ou descartem fotos.
- **Método de Teste:**
  - **Preparação:**
    - Recrutar um grupo mínimo de usuários que sejam representativos dos usuários alvo.
    - Preparar um ambiente de teste com o sistema instalado e configurado.
    - Carregar no sistema um conjunto de imagens, simulando o cenário real de imagens recém-recebidas.
    - Elaborar um script de teste de usabilidade com tarefas claras e representativas (ex: navegar por um lote de imagens, identificar uma imagem específica, marcar imagens para processamento, etc.).
    - Preparar um questionário de usabilidade pós-teste (ex: System Usability Scale - SUS, ou questionário com escala Likert) para medir a satisfação e a facilidade percebida. <br/><br/>
  - **Execução:**
    - Instruir cada usuário participante a executar as tarefas definidas no script de teste.
    - Observar e registrar o desempenho do usuário (ex: tempo para completar tarefas, erros cometidos).
    - Após a conclusão das tarefas, solicitar que cada usuário preencha o questionário de usabilidade.<br/><br/>
  - **Medição:**
    - Calcular a pontuação total ou a métrica relevante do questionário de usabilidade para cada usuário.
    - Coletar dados de desempenho, como tempo médio de conclusão da tarefa e taxa de sucesso na conclusão das tarefas.
    - Analisar os comentários qualitativos fornecidos pelos usuários.<br/><br/>
  - **Análise:**
    - Calcular a pontuação média e a distribuição das pontuações de usabilidade entre todos os participantes.
    - Analisar a porcentagem de usuários que atingiram ou excederam a pontuação de sucesso definida.
    - Relacionar os dados de desempenho com as pontuações percebidas de usabilidade.
    - Identificar os principais pontos de dificuldade ou confusão relatados pelos usuários.<br/><br/>
  - **Critério de Sucesso:**
    - Uma pontuação de usabilidade média de SUS Score maior que 68.
    - A taxa de sucesso na conclusão das tarefas críticas (ex: selecionar imagens para processamento) deve ser de no mínimo 95%.

---

### Precisão na Detecção de Fissuras

-   **Requisito:** O sistema deve detectar a presença e o contorno das fissuras nas imagens de inspeção com alta precisão, minimizando falsos positivos (identificar algo como fissura quando não é) e, crucialmente, falsos negativos (não identificar uma fissura real). Esta precisão deve ser robusta a variações comuns nas imagens de inspeção, como iluminação, texturas superficiais, e presença de sujeira ou sombras.

-   **Método de Teste:**
    *   **Preparação:**
        *   Criar ou obter um conjunto de teste representativo contendo no mínimo 100 imagens de superfícies inspecionadas, incluindo uma variedade de tipos de fissuras (em termos de tamanho, forma, severidade) e diferentes condições de captura (iluminação, texturas de material, presença de elementos como manchas ou sombras).
        *   Para cada imagem no conjunto de teste, criar uma verdade fundamental que consiste em anotações (ex: máscaras de segmentação pixel a pixel ou polígonos delimitadores muito precisos) definindo a localização e o contorno exato de todas as fissuras reais presentes. Este conjunto de teste deve ser independente de quaisquer dados usados para treinar os algoritmos de detecção.
    *   **Execução:**
        *   Processar cada imagem do conjunto de teste através do módulo de análise geométrica de fissuras do sistema, que realiza a detecção.
        *   Coletar as saídas de detecção do sistema para cada imagem (ex: máscaras de segmentação ou contornos detectados).
    *   **Medição:**
        *   Comparar as saídas de detecção do sistema com as anotações para cada imagem.
        *   Utilizar métricas de avaliação padrão para detecção ou segmentação de objetos, como:
            *   Acurácia de Pixels: Porcentagem de pixels corretamente classificados (fissura e não-fissura).
            *   IoU / Coeficiente de Jaccard: Uma métrica que avalia a sobreposição entre a área detectada pelo sistema e a área real da fissura. Calcular o IoU médio sobre todas as fissuras detectadas corretamente.
            *   Precisão: De todas as áreas que o sistema detectou como fissura, quantas são realmente fissuras?
            *   Recall (Sensibilidade): De todas as fissuras reais presentes, quantas o sistema conseguiu detectar?
            *   F1-score: A média entre Precision e Recall
        *   Calcular estas métricas globalmente sobre todo o conjunto de teste e/ou por categoria de condições de imagem (ex: baixa luz, alta textura).
    *   **Análise:**
        *   Analisar os resultados das métricas, focando especialmente no recall para garantir que poucas fissuras reais sejam perdidas, e na precisão para evitar excesso de falsos alarmes. O F1-score fornece uma visão generalizada equilibrada.
        *   Identificar casos específicos de falha (falsos positivos ou falsos negativos) para entender as limitações do algoritmo.
    *   **Critério de Sucesso:**
        *   O sistema deve atingir um F1-score de no mínimo 90% na detecção de fissuras no conjunto de teste independente.
        *   O sistema deve atingir um Recall de no mínimo 92% para garantir que a maioria das fissuras reais seja identificada.
        *   A variação (desvio padrão) do F1-score entre diferentes categorias de condições de imagem (iluminação, textura, etc.) não deve exceder 0.03, o que indica robustez do modelo.