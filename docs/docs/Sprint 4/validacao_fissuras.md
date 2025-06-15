---
sidebar_label: "Tela de Validação"
sidebar_position: 4
---

# Tela de Validação de Fissuras

&emsp;Durante a Sprint&nbsp;4, uma das entregas centrais foi a criação da **Tela de Validação**. Ela é a ponte entre o algoritmo de detecção automática de fissuras e a decisão humana. Mais do que um recurso visual, essa página dá a garantia de que os relatórios sejam confirmados por pesquisadores especialistas.

## Mas equipe 14 bis, por que essa etapa é tão importante?

1. **Segurança Estrutural** –&nbsp;Uma fissura classificada incorretamente pode gerar diagnósticos equivocados sobre a integridade do edifício. A validação protege até mesmo os individuos que futuramente forem fazer uso dos edifícios.
2. **Responsabilidade Técnica** –&nbsp;A adição da tela de validação exige que laudos de inspeção contenham evidências revisadas. A tela permite registrar quais imagens foram checadas e quais detecções foram aceitas ou rejeitadas.
3. **Confiança no Sistema** –&nbsp;Ao oferecer um momento de revisão manual, reforçamos que o uso de IA não elimina a participação do profissional, mas sim amplia sua eficiência.
4. **Rastreabilidade** –&nbsp;Cada validação gera um arquivo `validation_results.json`, deixando claro quem aprovou quais imagens e quando, facilitando auditorias futuras.

## Como funciona para o usuário?

* **Visualização sequencial** – As imagens aparecem uma a uma, destacando as fissuras identificadas.
* **Marcação simples** – Em um clique, o especialista marca a detecção como *correta* ou *incorreta*.
* **Resumo em tempo real** – Um contador mostra quantas imagens já foram vistas e quantas precisam de atenção.
* **Confirmação final** – Antes de encerrar, o sistema relembra o usuário se ainda há imagens não visualizadas, evitando esquecimentos.

## Próximos passos

* **Retreinamento Contínuo** – Utilizar o conjunto de imagens corrigidas para atualizar o modelo YOLO, reduzindo falsos positivos e melhorando a precisão.
* **Relatório Consolidado** – Gerar automaticamente um documento que combine os resultados de detecção com as correções manuais, facilitando a entrega ao cliente e a rastreabilidade das decisões. 