# Documentação de Pré-processamento de Imagens para Detecção de Fissuras com YOLO

O modelo YOLO (You Only Look Once) é uma das abordagens mais eficazes para **detecção em tempo real de objetos** em imagens. Para que esse modelo funcione corretamente, é **essencial que as imagens de entrada estejam bem anotadas** — ou seja, que cada objeto de interesse (neste caso, as fissuras térmicas e de retração) esteja corretamente delimitado por uma **bounding box** com sua respectiva classe. Esse processo de anotação, também conhecido como **pré-processamento supervisionado**, é a base do aprendizado de máquina para modelos de detecção.

Sem esse passo, o modelo não conseguiria aprender **o que é uma fissura térmica e o que é uma fissura de retração**, onde ela aparece na imagem e como diferenciá-la de outros elementos visuais.

##  Por que utilizamos o LabelImg e como ele funciona

O **LabelImg** foi a ferramenta escolhida para realizar a anotação manual das imagens por ser:

* **Simples de usar**
* **Open-source**
* Compatível com o formato de anotação do YOLO 
* Possuir interface gráfica leve e suporte a atalhos de teclado

O LabelImg permite ao usuário **desenhar caixas retangulares** ao redor de regiões de interesse em uma imagem. Cada caixa é associada a uma **classe (label)** e salva em um arquivo de anotação, que será posteriormente utilizado no treinamento do modelo.

---

## Processo realizado: marcação e geração de arquivos

Durante o pré-processamento, realizamos a **marcação manual de 50 imagens contendo fissuras térmicas**. Para cada imagem:

1. Foi criada uma **caixa de marcação (bounding box)** envolvendo a fissura mais visível.
2. A classe associada foi definida como `fissura_termica`.
3. O LabelImg gerou um novo **arquivo ** correspondente à imagem, contendo as coordenadas da caixa e a classe associada.

Esses arquivos de anotação são essenciais para treinar o YOLO, pois indicam ao modelo **onde está a fissura** e **como ela deve ser identificada**.

---

##  Critério técnico de anotação: conhecimento prático com apoio do IPT

A detecção manual foi feita com base em conhecimentos adquiridos em sala de aula, especialmente nas ** na explicação feita pelo IPT (Instituto de Pesquisas Tecnológicas)**. Foram apresentados:

* Os **diferentes tipos de fissuras** (como térmicas e de retração)
* **Características visuais específicas** de cada tipo (direção, localização, padrão de ramificações)
* **Boas práticas de anotação**, como priorizar fissuras principais e manter consistência visual

Esse embasamento foi fundamental para garantir a **qualidade e coerência das anotações**, contribuindo diretamente para a eficácia do modelo que será treinado.

## Próximos passos: uso das imagens anotadas no YOLO

Com o conjunto de imagens anotadas, os próximos passos incluem:

1. **Separar o dataset em treino e teste** (ex: 80% treino / 20% teste)
2. Organizar as imagens e anotações nas pastas `images/train`, `images/val`, `labels/train`, `labels/val`
3. **Configurar o ambiente de treinamento do YOLO** com as imagens pré-processadas
4. Iniciar o **treinamento supervisionado** para que o modelo aprenda a detectar fissuras térmicas automaticamente
5. Avaliar o desempenho do modelo em termos de precisão, recall e IoU (intersection over union)


## Conclusão

O processo de pré-processamento com o LabelImg foi uma etapa fundamental para o desenvolvimento do nosso sistema de detecção automática de fissuras térmicas com YOLO. A anotação cuidadosa, baseada em conhecimento técnico do IPT, garante que o modelo terá uma base de aprendizado consistente e confiável. A partir daqui, seguiremos para o treinamento do modelo e avaliação de seu desempenho, buscando construir uma solução robusta e eficiente para o reconhecimento automático de fissuras em imagens de fachadas.

---

Se quiser, posso formatar isso em `.md`, `.pdf`, `.tex` ou integrar à sua documentação do Docusaurus. Deseja em algum desses formatos?
