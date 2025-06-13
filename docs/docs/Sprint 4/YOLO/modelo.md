---
title: Melhorias no Modelo YOLO (Sprint 4)
sidebar_position: 1
---

# 1. Resumo Executivo

&emsp;Nesta **Sprint 4**, o foco principal do projeto foi aprofundar a análise e otimizar o **modelo YOLO de detecção de objetos**. Apesar dos avanços na Sprint 3, as **métricas de recall** não atingiram o desempenho esperado, indicando que o modelo falhava em identificar uma parcela significativa das fissuras reais. A equipe concentrou-se em **diagnosticar e corrigir** as causas raiz desse problema, revisando exaustivamente **imagens de treino, validação e seus arquivos de anotação (labels e caixas delimitadoras)**.

&emsp;As modificações implementadas no processo de anotação e nos hiperparâmetros do modelo resultaram em melhorias expressivas. Conseguimos elevar o **recall em quase 50 pontos percentuais**, alcançando impressionantes **94%**, enquanto mantivemos uma **precisão robusta de 90%**. Este avanço é crucial para a confiabilidade da nossa solução no ambiente do IPT, garantindo que a ferramenta identifique a maioria das fissuras com alta assertividade.

---

# 2. Glossário Essencial

Para facilitar a compreensão das métricas discutidas:

* **Precisão (Precision):** Mede a **confiabilidade das previsões positivas**. Indica quantas das detecções realizadas pelo modelo estavam, de fato, corretas. Um valor alto significa que, quando o modelo detecta uma fissura, há grande chance de que ela realmente exista.
    * *Em termos simples:* Das que o modelo disse que são fissuras, quantas realmente eram.
* **Recall (Sensibilidade):** Mede a **capacidade do modelo de encontrar todas as instâncias relevantes**. Indica a proporção de fissuras reais que foram corretamente identificadas pelo modelo. Um valor alto garante que poucas fissuras sejam "perdidas" pela análise.
    * *Em termos simples:* Das fissuras que existem de verdade, quantas o modelo conseguiu achar.
* **Caixa Delimitadora (Bounding Box):** Um **retângulo traçado ao redor de um objeto** (neste caso, uma fissura) em uma imagem, indicando sua localização e tamanho. São anotadas manualmente em ferramentas como o **LabelImg** e servem para treinar o modelo a reconhecer esses objetos em novas imagens.

---

# 3. O Cenário na Sprint 3

&emsp;A **Sprint 3** marcou o início do desenvolvimento do nosso modelo YOLO para detecção de objetos. Embora os testes iniciais de **classificação de imagens** inteiras fossem promissores (atingindo até 99% de acurácia), essa performance não se traduziu diretamente para a **detecção de objetos (bounding boxes)**. Com uma **precisão aceitável de 89%**, nosso principal desafio era o **recall**, que girava em torno de apenas **45%**. Isso significava que, apesar de detectarmos fissuras com alta certeza (boa precisão), estávamos **deixando de identificar** quase metade das fissuras existentes nas imagens.

---

# 4. A Análise Diagnóstica (Investigação das Causas)

&emsp;Diante do baixo recall, a equipe de desenvolvimento iniciou uma investigação aprofundada das causas. Nossas principais frentes de análise incluíram:

* **Reclassificação e Padronização das Anotações:**
    * Revisamos exaustivamente todas as imagens de treino e validação utilizando o **LabelImg**. O objetivo foi ajustar as **caixas delimitadoras** e suas **labels**, garantindo que fossem mais precisas e, crucialmente, que seguissem um **padrão consistente** em todo o dataset.
* **Correção de Labels Inconsistentes:**
    * Identificamos um problema significativo: devido à anotação inicial ter sido realizada por duas pessoas, ocorreram **inconsistências nas labels**. Por exemplo, imagens de fissuras térmicas (label = 1) foram erroneamente marcadas como de retração (label = 0), e vice-versa. Essa inconsistência gerava confusão para o modelo. A equipe revisou e corrigiu minuciosamente essas labels para garantir a correspondência correta.
* **Otimização das Caixas Delimitadoras (Bounding Boxes):**
    * Uma análise aprofundada revelou que algumas predições do modelo, embora aparentemente incorretas, eram na verdade reflexo de **caixas delimitadoras redundantes ou excessivas** em nossas anotações. Por exemplo, uma única fissura complexa poderia ter sido anotada com múltiplas caixas, enquanto o modelo tendia a prever uma única caixa abrangente. Isso levava o sistema de validação a considerar a "falta" das caixas adicionais como uma não detecção, impactando negativamente o recall. Otimizar a anotação para o **número mínimo e mais representativo de boxes por fissura** tornou-se um objetivo chave.

---

# 5. Ações Implementadas e Resultados Alcançados

&emsp;Baseados em nossas descobertas, implementamos uma série de ações iterativas:

1.  **Primeira Tentativa: Excessiva Marcação de Boxes:**
    * Inicialmente, tentamos marcar o **máximo possível de caixas delimitadoras** para cada fissura, visando cobrir todo o "espaço vazio". No entanto, essa abordagem foi **inefetiva**. O modelo não aprendeu a marcar o excesso de caixas, e o resultado foi o mesmo que o original, confirmando nossa hipótese sobre a redundância de boxes.
2.  **Correção das Labels e Primeiro Salto:**
    * Em seguida, priorizamos a **correção das labels equivocadas**. Essa ação teve um impacto significativo: o **recall aumentou para 60%** sem perda de precisão.
    * Com **algumas iterações de épocas de treinamento**, o modelo alcançou até **62% de recall**, mas ainda não era o esperado.
3.  **Otimização de Caixas Delimitadoras (Mínimo Essencial):**
    * Ao perceber que a **criação excessiva de caixas** continuava sendo um gargalo, decidimos **reclassificar as imagens novamente**, desta vez focando em ter o **mínimo de caixas delimitadoras possíveis**, evitando redundâncias.
    * Aplicamos essa nova abordagem inicialmente às **fissuras térmicas**, onde o problema era mais evidente. Os resultados foram muito positivos, com o **recall subindo para 72%** e a precisão mantendo-se em **90%**.
    * Estendemos essa otimização para as **fissuras de retração**, obtendo um resultado ainda melhor: **recall de 88%** e **precisão de 92%**.
4.  **Ajuste Final do Treinamento:**
    * Por fim, ajustamos a **quantidade de épocas de treinamento de 30 para 200**. Essa alteração resultou na **melhora final**: um impressionante **recall de 94%** e uma leve adaptação na precisão para **90%**.

---

# 6. Conclusão e Próximos Passos

&emsp;A equipe de desenvolvimento do modelo YOLO obteve um sucesso notável na identificação e correção dos problemas que afetavam as métricas de recall. A metodologia de **reclassificação estratégica das imagens**, **correção de labels** e, principalmente, a **otimização rigorosa do número de caixas delimitadoras** foram ações fundamentais. Com essas melhorias, o modelo YOLO alcançou uma **melhora de quase 50 pontos percentuais no recall**, atingindo um desempenho de **94% de recall e 90% de precisão**.

&emsp;Apesar dos excelentes resultados, a equipe considera relevante continuar a busca por melhorias, explorando talvez uma **alteração mais específica nos hiperparâmetros** do modelo ou expandindo o dataset com dados ainda mais variados, visando otimizar a performance para casos de uso específicos do IPT.

---