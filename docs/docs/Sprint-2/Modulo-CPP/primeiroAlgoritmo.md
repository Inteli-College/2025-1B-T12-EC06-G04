---
title: Primeira Versão Algoritmo
sidebar_position: 1
---

# Primeira Versão Modelo de Classificação de Fissuras
&emsp; O objetivo desse modelo é a  classificação automática de fissuras em imagens, voltado para uso pelo Instituto de Pesquisas Tecnológicas (IPT), com o objetivo de aumentar a eficiência no processo de análise das fissiras em edifícios parceiros ao IPT.

## Estrutura do Projeto
Para o desenvolvimento do projeto foi desenvolvida a seguinte estrutura de pastas:
```txt
.
├── build/                     
├── images/
│   └── validacao/
│       ├── retracao/
│       └── termica/
│   └── treinamento/
│       ├── retracao/
│       └── termica/
├── include/                  
│   ├── classificador.hpp
│   ├── detectorfissura.hpp
│   ├── extrator_features.hpp
│   ├── preprocessamento.hpp
│   └── validacao.hpp
├── src/                      
│   ├── classificador.cpp
│   ├── detectorfissura.cpp
│   ├── extrator_features.cpp
│   ├── preprocessamento.cpp
│   ├── treinamentoModelo.cpp
│   ├── validacao.cpp
│   └── main.cpp
├── Testes/                   
├── CMakeLists.txt            
├── .gitignore
└── README.md

```
- **Build:** Diretório responsável por compilar o projeto com o CMake. Nesse diretório estão os arquivos execultáveis do projeto, como o treinamento do modelo e o arquivo main, responsável por analisar as máscaras do pré processamento.

- **Images:** Diretório responsável por armazenar as imagens para treinamento e validação do modelo. As imagens de treinamento são imagens usadas para que o modelo possa identificar os padrões, e os de validação são para validar a acurácia do modelo.

- **Include:** Cabeçalhos dos arquivos .cpp.

- **SRC:** Arquivos .cpp, onde os códigos responsáveis por pré processar, treinar e validar o modelo, são armazenados.

- **Testes:** Testes para validação do modelo

- **CMakeLists.txt:** Arquivo responsável por indicar ao CMake, o que deve ser compilado.

- **.gitignore:** Arquivo responsável por impedir que arquivos sejam enviados para o repositório

- **README.md:** Documento que ensina como execultar o projeto.

## Funcionamento e fluxo
Para que o projeto possa funcionar, ele deve respeitar o seguinte fluxo:

### Pré-processamento da Imagem

 - Redimensionamento e padronização das imagens

 - Aplicação de filtros para realçar bordas das imagens

- Extração de máscara binária utilizando Canny + morfologia.

### Extração de Features

São extraídas 5 características para cada imagem:

 - **Orientação média das fissuras:** O ângulo médio das linhas detectadas (rachaduras) em uma imagem, geralmente medido em graus ou radianos. Pode ajudar a distinguir tipos de fissuras que ocorrem em padrões estruturais diferentes (por exemplo, retração geralmente é mais aleatória, enquanto fissuras térmicas podem ser mais direcionadas).

 - **Desvio padrão da orientação:** Mede a variação nos ângulos das linhas detectadas. Um desvio padrão alto indica direções variadas de rachaduras, enquanto um valor baixo sugere um padrão mais uniforme.

 - **Comprimento médio das linhas detectadas:** Reflete o tamanho médio das fissuras visíveis na imagem. tipos de fissuras podem se diferenciar pelo comprimento (fissuras de retração costumam ser curtas e numerosas, enquanto as térmicas podem ser mais longas).

 - **Número de linhas detectadas:** Quantidade total de fissuras identificadas na imagem após o pré-processamento.Um número elevado de fissuras pode indicar um tipo específico de dano (como fissuras por retração), enquanto um número baixo pode sugerir outra natureza (como trincas localizadas).

 - **Densidade de rachaduras:** Proporção da área da imagem ocupada pelas rachaduras (ex: razão entre pixels da máscara e total de pixels).

### Classificação com SVM

Inicialmente, foi escolhido um modelo SVM (Support Vector Machine) que é um algoritmo de aprendizado de máquina supervisionado usado para classificação e regressão. Ele busca encontrar a melhor linha ou hiperplano que separe diferentes classes de dados, maximizando a margem entre elas. 

Ademais para esse projeto foi usada uma busca em grade para testar múltiplos parâmetros:

```cpp
for (int kernelType : kernels)
  for (double C : Cs)
    for (double gamma : gammas)

```
Com esses loops, puderam ser testadas várias combinações de hiper parâmetros para encontrar a melhor combinação possível para o modelo SVM, sendo eles:

**Kernel:** Define a função de kernel usada para transformar os dados em um espaço de maior dimensão, permitindo que o classificador encontre fronteiras de decisão não-lineares.

**C:** Controla o trade-off entre obter uma margem larga e classificar corretamente os pontos de treinamento. Valores menores de C aumentam a penalização para margens pequenas (mais generalização).

**Degree (Grau do polinômio):** Aplica a potência ao resultado do produto interno ajustado pelo coef0. Na primeira versão, o kernel usado é um polinômio de segundo grau, o que permite capturar relações quadráticas entre as features.

**Coef0 (Coeficiente livre):** Valor independente adicionado ao produto interno antes da elevação à potência. Útil para ajustar a influência de termos de menor ordem em kernels como POLY ou SIGMOID.

**Gamma (Coeficiente de escala):** Controla a influência de um único exemplo de treino. Em kernels não-lineares, gamma afeta a curvatura da fronteira de decisão.

Por fim imagens foram divididas em diretórios separados para treino e validação. A acurácia dos modelos foi avaliada por validação cruzada (k=10).

---

Melhores parâmetros encontrados e melhor resultado obtido:

- **Kernel:** POLY
- **C:** 0.5
- **Degree:** 2
- **Coef0:** 0
- **Gamma:** 0.5

***Acurácia média:** 60.625%

---

## Problemas enfrentados e Próximos passos

&emsp; Os principais problemas para o desenvolvimento dessa primeira versão foram ligadas principalmente ao pré processamento, o que impacta diretamente o desempenho do modelo desenvolvido.

 - As máscaras em algumas imagens não estavam sendo geradas corretamente, resultando em imagens totalmente pretas.

 - O modelo ficou em overfitting durante boa parte do treinamento, sendo solucionado apenas no final, o que resultou na baixa precisão inicial.

Para próximos passos, é esperado uma melhora no pré processamento das imagens e o teste de outros modelos de classificação. Após esses testes, espera-se um melhor desempenho e acurácia para a melhora da qualidade do projeto.
