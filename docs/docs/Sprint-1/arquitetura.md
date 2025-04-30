---
sidebar_position: 1
slug: /sprint-1/Arquitetura/arquitetura
---

# Proposta de Arquitetura

&emsp;&emsp; A arquitetura do projeto, consiste em um forma de documentar e demonstrar quais são as técnicas e padrões utilizados para desenvolver uma aplicação, a proposta de arquitetura pode ser utilizada como planejamento do que deve ser desenvolvido para garantir o funcionamento do projeto, levando em consideração diversos fatores como regras de negócios e restrições técnicas, uma vez que ela descreve todos os componentes que fazem parte do sistema.

## Diagrama de arquitetura

<p style={{textAlign: 'center'}}>Figura 1 - Diagrama de Blocos da Arquitetura</p>

<div style={{textAlign: 'center'}}>
    <img src="/img/Sprint1/Arquitetura/diagrama-arquitetura.png" style={{width: 400}} />
</div>

<p style={{textAlign: 'center'}}>Fonte: Elaboração própria (2025)</p>

&emsp;&emsp;A partir do diagrama, é possível descrever de forma mais detalhada cada bloco da arquitetura.

### Drone

&emsp;&emsp;Dentro do contexto do projeto, o drone é o principal responsável pela captação de fotos e vídeos de fissuras. Ele possui a capacidade de sobrevoar diferentes locais e edificações capturando fotos e vídeos das fissuras mapeadas.

- **Unidade de memória externa**

&emsp;&emsp;A unidade de memória externa consiste em qualquer dispositivo externo capaz de armazenar dados, como um pendrive ou cartão SD. No contexto do projeto, esse dispositivo vai conectado ao drone durante o voo, permitindo que todas as fotos e vídeos capturados pelo drone sejam armazenadas em uma unidade de memória externa, que posteriormente será utilizada para dar acesso dos dados aos pesquisadores.

### Aplicação Desktop

&emsp;&emsp;Após a captação e armazenamento das imagens, é necessário uma aplicação desktop, que servirá como plataforma acessível ao usuário, permitindo que ele selecione as imagens, que serão processadas e classificadas pelo sistema de forma automatizada, gerando um relatório ao fim do processo. Sendo assim, a aplicação desktop possui três principais funcionalidades, sendo elas, a seleção das imagens de fissuras, o processamento de imagem e classificação de fissuras e, por último, a geração de relatório.

- **Seleção das imagens de fissuras**

&emsp;&emsp;Através desta funcionalidade, a aplicação desktop vai permitir que o usuário, após conectar a unidade de memória externa em seu computador, selecione quais fotos de fissuras capturadas pelo drone ele deseja que sejam processadas e classificadas.

- **Processamento de imagem e classificação de fissuras**

&emsp;&emsp;No momento do processamento da imagem, o objetivo é reunir as fotos selecionadas pelo usuário, e através de visão computacional, identificar as fissuras em cada uma das fotos. Após a identificação das fissuras, essas imagens alimentam um modelo preditivo, que tem como principal objetivo classificar as fissuras em seus diferentes tipos.

- **Geração de relatório**

&emsp;&emsp;Por fim, após a identificação e classificação de cada uma das fissuras, o sistema deverá possuir a capacidade de gerar relatórios completos no template desejado pelo usuário. Estes relatórios visam registrar todo o processo de seleção, processamento e classificação, além de possuir uma descrição de cada imagem de fissura, junto com sua classificação.

## Conclusão

&emsp;&emsp;Sendo assim, é possível concluir que a proposta de arquitetura consiste em definir todas as funcionalidades do projeto, em formato hierárquico, visando facilitar o desenvolvimento e trazer clareza no entendimento do funcionamento do sistema. Além disso, a arquitetura proposta serve como um guia para a equipe de desenvolvimento, garantindo que todos os componentes do sistema estejam alinhados com os objetivos do projeto e atendam às necessidades dos usuários finais. Essa abordagem permite uma melhor organização, identificação de possíveis riscos e ajustes necessários antes da implementação, promovendo maior eficiência e qualidade no desenvolvimento do sistema.
