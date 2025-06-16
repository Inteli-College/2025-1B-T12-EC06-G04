---
sidebar_position: 1
title: Requisitos Não Funcionais
---

## Requisitos Não Funcionais do Sistema

### Usabilidade na Visualização e Gerenciamento de Imagens

- **ID:** RNF-SYS-002  
- **Descrição:** A interface do sistema deve ser intuitiva para técnicos e pesquisadores, permitindo visualização, seleção, marcação e descarte de imagens antes, durante e após o processamento.  
- **Critérios de Aceitação:**
  - Deve obter uma pontuação média no SUS (System Usability Scale) superior a 68.
  - A taxa de sucesso na conclusão das tarefas críticas (ex: selecionar imagens para processamento) deve ser de no mínimo 95%.
  - O sistema deve ser testado com usuários representativos em ambiente simulado, com tarefas reais e questionário de usabilidade.
  - Devem ser registrados tempo de execução, taxa de sucesso e feedbacks qualitativos.
  <br/><br/>

---

### Precisão na Detecção de Fissuras

- **ID:** RNF-SYS-003  
- **Descrição:** O sistema deve detectar a presença e o contorno das fissuras com alta precisão, minimizando falsos positivos e principalmente falsos negativos, mesmo sob variações nas condições das imagens (iluminação, sujeira, texturas).  
- **Critérios de Aceitação:**
  - O sistema deve alcançar um F1-score de pelo menos 90% na detecção de fissuras.
  - O sistema deve atingir Recall de no mínimo 92% no conjunto de teste.
  - A variação (desvio padrão) do F1-score entre diferentes condições de imagem não deve exceder 0.03.
  - Devem ser calculadas métricas como Acurácia de Pixels, IoU, Precisão, Recall e F1-score com base em anotações manuais de referência.
  - O conjunto de teste deve incluir no mínimo 100 imagens com diversidade de fissuras e condições ambientais.
  <br/><br/>
