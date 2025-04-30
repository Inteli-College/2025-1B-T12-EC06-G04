---
sidebar_position: 1
title: Matriz de Risco
slug: /sprint-1/negocios/matriz-de-risco
---
# Matriz de Risco e Oportunidade
&emsp;A Matriz de Risco é uma ferramenta utilizada para identificar e avaliar os riscos associados a um projeto. Ela ajuda a priorizar os riscos com base em sua probabilidade de ocorrência e impacto no projeto. Riscos na área verde podem ser considerados de baixa prioridade, seja por terem baixa probabilidade de ocorrência ou por terem um impacto baixo no projeto. Já os riscos na área vermelha são considerados de alta prioridade, pois têm alta probabilidade de ocorrência e/ou alto impacto no projeto. Os riscos na área amarela são considerados de prioridade média.
&emsp;De forma semelhante, a Matriz de Oportunidade é uma ferramenta que ajuda a identificar e avaliar as oportunidades que podem surgir durante o projeto. Ela também ajuda a priorizar as oportunidades com base em sua probabilidade de ocorrência e impacto no projeto. As oportunidades na área vermelha são mais interessantes para o projeto, 
<br />
![Matriz de Risco](/img/matrizRisco14bis.png)

# Riscos

### Treinamento de Modelos levar um tempo exagerado

&emsp;Um dos riscos associados com a produção de um modelo de visão computacional é o tempo de treinamento. É preciso ter um balanço entre a qualidade requerida do sistema e o treino do modelo, de modo a não comprometer o tempo de entrega do projeto e a acurácia. O principal problema é que, na maioria dos casos, o tempo de treino é diretamente proporcional à quantidade de imagens de referência, o que eleva o tempo de treino.

### Comunicação do Drone ser incompatível com ROS2

&emsp;Uma das ideias do projeto é fazer o controle do drone por ROS2, mas há o risco da comunicação do drone utilizado pelo parceiro não ser compatível com o nosso sistema. Isso pode afetar o desenvolvimento do projeto e até a geração de ideias para a solução, mas tem uma baixa probabilidade de interferir em algum aspecto do desenvolvimento.

### Não ter certeza sobre como funciona o sistema do IPT

&emsp;O IPT é um dos parceiros do projeto e, por isso, é importante entender como funciona o sistema deles. Durante a primeira sprint, houve vários momentos de dúvida e confusão sobre como poderíamos integrar o drone com o nosso modelo, como faríamos a comunicação, entre outros pontos. Embora tenha uma probabilidade média por ser esperado que esse risco caia com a próxima conversa, do referencial atual é importante ter esse risco em mente, pois pode afetar a solução final drasticamente.

### Imagens de treino não resultarem em modelo funcional

&emsp;Similar ao primeiro risco, o treino do modelo pode não ser confiável para o uso. Isso pode ocorrer por diversos fatores, como a discussão realizada sobre o tempo de treino, a qualidade das imagens, quantidade, entre outros. Esse risco tem uma probabilidade relativamente baixa, e é esperado que o modelo tenha uma acurácia razoável, mas é importante considerar esse risco, pois afeta diretamente a qualidade do solução.

# Oportunidades

### Ter acesso ao sistema do drone (ou similar)

&emsp;Admitidamente, é improvável que teríamos acesso ao mesmo drone que é usado pelo IPT, principalmente devido aos riscos associados à seu custo. Apesar disso, o uso de sistemas similares ajuda a compreender como funciona o sistema e a desenvolver o nosso modelo. Está registrado como alta probabilidade por termos acesso a um drone que, até onde chegaram as informações, tem um sistema de controle similar. O impacto dessa oportunidade é extremamente alto, já que permite já adaptar o sistema pensando no produto final.

### Se utilizar de modelos de imagem já existentes.

&emsp;O principal dessa oportunidade é sobre o modelo em sí, e não o treinamento. O uso de modelos de visão computacional já testados facilita e agiliza o processo de produção da solução para conseguir entregar o projeto com um funcionamento preciso. Tem uma probabilidade alta, e impactos médios, resultando em um nível de prioridade médio para a oportunidade.