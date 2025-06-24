## Introdução

&emsp; Este documento tem como objetivo apresentar o resultado dos **testes de requisitos funcionais e não funcionais** conduzidos na Sprint 5 do projeto. O foco é validar se cada requisito mapeado anteriormente foi corretamente implementado, e se está em condições de atender aos objetivos definidos em conjunto com o IPT (Laboratório de Materiais para Produtos de Construção).

&emsp; Os testes foram realizados a partir de:
- Verificação manual da interface do sistema;
- Execução dos fluxos principais da aplicação;
- Observação do comportamento do sistema durante o **Teste de Usabilidade com Cliente**;
- Execução de scripts de teste automatizados e/ou manuais, conforme aplicável.

&emsp; Esta validação é essencial para garantir que a **prova de conceito** esteja alinhada com o esperado, e para identificar pontos que ficarão para planos futuros do projeto.

---

## Tabela de Resultados — Testes de Requisitos

| ID Requisito   | Descrição                                             | Status              | Evidência/Teste Realizado / Observações |
| -------------- | ---------------------------------------------------- | ------------------- | -------------------------------------- |
| **RF-SYS-001** | Recepção de Imagens de Inspeção por Armazenamento Externo | Parcialmente Cumprido | Testado na Sprint 5. Importação funciona, mas processamento automático incompleto. |
| **RF-SYS-002** | Integração e Recepção de Imagens por Drone            | Não Avaliado        | Não testado nesta Sprint. |
| **RF-SYS-003** | Validação e Pré-processamento de Formato de Imagem    | Não Avaliado        | Não testado nesta Sprint. |
| **RF-SYS-004** | Análise Geométrica de Fissuras                        | Não Avaliado        | Avaliado no componente IA (documentação YOLO). Não avaliado diretamente via UI. |
| **RF-SYS-005** | Classificação de Risco Estrutural de Fissuras         | Não Avaliado        | Métrica do modelo; não testada diretamente via UI. |
| **RF-SYS-006** | Geração e Apresentação da Lista de Fissuras Analisadas  | Parcialmente Cumprido | Visualização OK. Exportação PDF/DOCX com falha (bloqueante). |
| **RF-SYS-007** | Armazenamento Local de Imagens Processadas e Resultados | Não Avaliado        | Não testado nesta Sprint. |
| **RNF-SYS-001** | Desempenho do Sistema                                 | Não Avaliado        | Não testado com métricas formalizadas. |
| **RNF-SYS-002** | Usabilidade na Visualização e Gerenciamento de Imagens | Parcialmente Cumprido | Teste de Usabilidade com Cliente: boa intuição em algumas funções; ausência de feedback visual e campos obrigatórios não destacados. |
| **RNF-SYS-003** | Precisão na Detecção de Fissuras                      | Não Avaliado        | Avaliado separadamente no teste de IA (YOLO), não diretamente pelo cliente. |

---

## Análise Geral

&emsp; A Sprint 5 possibilitou a validação parcial de diversos requisitos, com foco nas funcionalidades que compõem o fluxo principal de uso pelo IPT:

- **RF-SYS-001** e **RF-SYS-006** apresentaram progresso considerável, embora ainda existam pontos críticos a serem resolvidos.
- **RNF-SYS-002 (Usabilidade)** recebeu feedback direto e já aponta prioridades de melhoria claras.
- Diversos requisitos relacionados à integração com drone e análise técnica profunda de IA ainda não foram avaliados formalmente nesta sprint (ficando pendentes para etapas futuras).

&emsp; A execução dos testes evidencia que a prova de conceito já apresenta **fluxo funcional consistente**, ainda que com limitações pontuais, e que o trabalho das próximas etapas deverá priorizar:

1. Conclusão das funcionalidades de exportação;
2. Correção do fluxo automático de processamento de imagens;
3. Melhorias em usabilidade e feedback de sistema;
4. Testes adicionais para requisitos não avaliados.

---

## Conclusão

&emsp; Os testes de requisitos demonstram que a solução já cumpre, **em parte significativa**, os requisitos essenciais para um fluxo básico de uso real por parte do IPT, especialmente na gestão de projetos e visualização de resultados. 

&emsp; As lacunas identificadas serão tratadas com prioridade para garantir que o produto final atenda integralmente às expectativas do cliente e aos critérios definidos no escopo original.

