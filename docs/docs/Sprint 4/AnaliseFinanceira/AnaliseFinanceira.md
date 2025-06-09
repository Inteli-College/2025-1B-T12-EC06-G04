---
title: Análise financeira do projeto
sidebar_position: 0
---
# Análise financeira do projeto para o IPT

## 1. Visão Geral do Projeto
Este projeto consiste no desenvolvimento de um aplicativo desktop offline, capaz de realizar a análise automatizada de fissuras em edifícios a partir de imagens, classificando-as como **térmicas** ou de **retração**. O projeto utiliza o **YOLO**, um modelo de classificação de imagens utilizado para diferenciar dois tipos específicos de fissuras, gráficos analíticos e geração de relatórios técnicos, sendo desenvolvido com tecnologias de alto desempenho como Rust e Python.

O cliente é o Instituto de Pesquisas Tecnológicas (IPT), uma instituição pública, o que exige qualidade de entrega compatível com ambiente científico e institucional.

**Tempo de execução do projeto:** 2 meses
**Modelo de contrato:** Projeto fechado + manutenção mensal (SLA de 48h)

---
## 2. Estrutura da Equipe
Muitos projetos de tecnologia falham não pela ausência de desenvolvedores, mas por uma má composição da equipe. Aqui, na startup 14 Bis, estruturamos a equipe com base em papéis funcionais e complementares, para garantir não só a entrega, mas a excelência técnica, a segurança da aplicação, e uma experiência de uso adequada ao público-alvo do IPT.
Todas as funções estão conectadas à minimização de riscos operacionais e cada um tem uma justificativa técnica clara e estratégica para o projeto desenvolvido.

| Cargo                     | Qtde | Justificativa Crítica                                                                                                                                                      |
| ------------------------- | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Dev Sênior**            | 1    | Garante arquitetura sólida, escolhas tecnológicas (como a integração Rust/Python), e revisão de código. Seu papel está na **mitigação de riscos técnicos**.  |
| **Devs Pleno**            | 2    | Responsáveis pela codificação diária das funcionalidades. O volume de trabalho técnico justifica duas posições, dado que o prazo é de apenas dois meses.                   |
| **Analista de Segurança** | 1    | Embora seja um software offline, o IPT trabalha com dados sensíveis envolvendo os seus clientes. Esse perfil realiza auditorias internas de segurança, valida empacotamento e hardening.               |
| **Product Owner**         | 1    | Garante que os requisitos do IPT sejam interpretados e priorizados corretamente. Atua como mediador entre a equipe técnica e o cliente, com foco em evitar o retrabalho. |
| **QA (Testes)**           | 1    | O  QA garante estabilidade nas entregas, com testes manuais e automatizados.                                          |
| **Designer UX/UI**        | 1    | O público final não é leigo, mas precisa de usabilidade e clareza. O designer garante acessibilidade e fluxo visual adequado ao contexto técnico.                          |
| **Financeiro**            | 1    | Gerencia contratos PJ, impostos e comunicação contábil. Reduz risco de passivos fiscais, essencial quando se lida com cliente público como o IPT.                          |


---

## 3. Custos da Equipe e Ferramentas
### 3.1 Custos com a Equipe (PJ)
Remunerações estão dentro da média de mercado para junho/2025. Cargos mais especializados, como Dev Sênior e Analista de Segurança, possuem valores compatíveis com sua criticidade.

| Cargo                 | Quantidade | Valor Médio PJ/mês | Total Mensal |
| --------------------- | ---------- | ------------------ | ------------ |
| Dev Sênior            | 1          | R\$ 14.000,00         | R\$ 14.000,00   |
| Dev Pleno             | 2          | R\$ 11.500,00         | R\$ 23.000,00   |
| Analista de Segurança | 1          | R\$ 11.000,00         | R\$ 11.000,00   |
| PO                    | 1          | R\$ 10.000,00        | R\$ 10.000,00   |
| QA                    | 1          | R\$ 8.000,00          | R\$ 8.000,00    |
| Designer              | 1          | R\$ 8.000,00          | R\$ 8.000,00    |
| Financeiro            | 1          | R\$ 7.000,00          | R\$ 7.000,00    |

**Total mensal da equipe:** R$ 81.000

**Total em 2 meses:** R$ 162.000

_Valores com base em mercado para contratos PJ (jun/2025), considerando o perfil da equipe e referência em sites como [Glassdoor](https://www.glassdoor.com.br/index.htm) e [Revelo](https://www.revelo.com.br/)._

### 3.2 Ferramentas e Infraestrutura
Considerando que o projeto é 100% remoto, os custos com infraestrutura física são eliminados. No entanto, há despesas com ferramentas e serviços essenciais:
| Ferramenta          | Qtde Usuários | Valor Mensal/Usuário (USD/R$) | Cotação 04/06/2025 (R\$ 5,64) | Custo Mensal (R\$)    |
| ------------------- | ------------- | -------------------------- | ----------------------------- | --------------------- |
| GitHub Team         | 7             | \$4                        | R\$ 5,64                      | R\$ 158,00            |
| Figma Professional  | 1             | \$16                       | R\$ 5,64                      | R\$ 90,24             |
| Notion Plus         | 7             | \$10                       | R\$ 5,64                      | R\$ 394,80            |
| Slack Pro           | 7             | \$8.75                     | R\$ 5,64                      | R\$ 344,85            |
| Google Workspace    | 8             | R\$ 70                     | —                             | R\$ 560,00            |
| **Total** | 0          | —                          | —                             |  **R\$ 1547,89** | 


_Câmbio de referência: US$ 1 = R$ 5,64 (cotação de 04/06/2025, Banco Central)_

- **GitHub Team(7 usuários):**  Utilizado como repositório privado com controle de versão, revisões de código via pull requests e automações de CI/CD. A versão Team permite permissões refinadas e insights de produtividade, essenciais para times com múltiplos devs e para segurança do código-fonte;

- **Notion Plus (7 usuários):**	Atua como centro de documentação viva: cronogramas, requisitos do cliente, atas de reunião e decisões técnicas ficam registradas e acessíveis. A versão Plus permite permissões avançadas, backups automáticos e histórico de edição — ideal para prestação de contas em projetos com cliente público;

- **Slack Pro (7 usuários):**	Comunicação síncrona da equipe, com integração direta ao GitHub, Notion e Google Calendar. A versão Pro garante busca ilimitada no histórico e automações, evitando perda de contexto técnico ao longo das iterações.

- **Figma Professional (1 usuário):** Ferramenta de design colaborativo, utilizada para prototipação de telas e construção do layout final. Um único assento é suficiente, dado que o projeto conta com apenas um designer, mas exige recursos avançados como versões e comentários por parte do time.

- **Google Workspace Business Standard (8 usuários):** Suporte à comunicação formal com o cliente (via Gmail), armazenamento de documentos (Drive), reuniões (Google Meet) e integração com calendários. Utilizado tanto para documentação interna quanto para comunicação institucional com o IPT.

Apesar de parecer um custo elevado, essas ferramentas são importantes para manter qualidade e governança no projeto. Como o cliente é uma instituição pública, com potencial para fiscalizações e auditorias, ferramentas que garantem transparência, backup de decisões e histórico de atividades são indispensáveis.

Além disso, ao adotar soluções como GitHub, Notion e Slack — todas líderes em seus segmentos — é possível garantir a produtividade e integração contínua entre as partes da equipe. Além do mais o custo representa menos de 5% do orçamento total e tem alto retorno sobre investimento em termos de redução de retrabalho, melhoria da comunicação, transparência  e rastreabilidade.

---

## 4. Carga Tributária (Simples Nacional + Fator R)

A empresa responsável pelo projeto está enquadrada no Simples Nacional, regime tributário simplificado voltado a micro e pequenas empresas. Por se tratar de uma empresa de prestação de serviços técnicos em tecnologia da informação (TI), como o desenvolvimento de software sob demanda e manutenção, sua tributação recairia, em regra, sobre a Tabela V do Simples Nacional, que possui alíquotas mais elevadas — chegando a 22,45%.

No entanto, aplica-se o Fator R, um mecanismo do Simples Nacional que permite a migração para a Tabela III quando os custos com folha de pagamento (incluindo pró-labore, salários e encargos) forem superiores a 28% da receita bruta dos últimos 12 meses. No caso deste projeto:

- A equipe técnica é composta por 8 pessoas atuando em regime PJ (Pessoa Jurídica), com custos elevados proporcionalmente à receita;

- O projeto gerará uma receita estimada de R$ 250.000,00 ao longo de dois meses;

- Os custos com pessoal representam mais de 50% da receita, o que habilita a empresa ao benefício do Fator R com segurança jurídica.

Tendo em vista isso:

**Faixa de receita estimada:** R$ 250.000,00 (projeto entregue em 2 meses)

**Faixa aplicável:** 4ª Faixa (de R$ 180.000,01 até R$ 360.000,00)

**Alíquota nominal:** 16,93%

**Alíquota efetiva (com desconto)**: 13,5% a 15%

Para segurança, utilizaremos uma alíquota de **15% sobre a receita bruta**.

---

## 5. Precificação e Margem de Lucro(20%)

A margem de **20%** é prática comum no setor de serviços de tecnologia para:

- Cobrir riscos (revisões, retrabalho, suporte adicional).

- Compensar variações cambiais e imprevisibilidade de custos em serviços digitais.

- Assegurar saúde financeira e reinvestimento.

A margem de 20% está alinhada com estudos do [Sebrae](https://sebrae.com.br/sites/PortalSebrae/artigos/entenda-e-calcule-corretamente-a-margem-de-lucro,f2bbca017749e410VgnVCM1000003b74010aRCRD) e benchmarks da [Contabilizei](https://www.contabilizei.com.br/contabilidade-online/contabilidade-para-empresas-profissionais-ti/) para prestadores de serviços técnicos (média entre 15% e 30%).

---

## 6. Precificação Final

| Item                            | Valor              |
| ------------------------------- | ------------------ |
| Custo de equipe (2 meses)       | R\$ 162.000,00        |
| Ferramentas (2 meses)           | R\$ 3.095,78       |
| **Subtotal operacional**        | R\$ 165.695,78     |
| **Lucro (20%)**                 | R\$ 33.139,16      |
| **Preço de venda**     | **R\$ 198.834,94** |
| Impostos Simples Nacional (15%) | R\$ 29.825,24      |
| **Preço final com impostos**    | **R\$ 228.060,18** |

---
## 7. Precificação da Manutenção Mensal (SLA de 48h)

A manutenção é oferecida como serviço adicional, com tempo de resposta garantido de 48 horas úteis. Considera-se que o suporte será feito por um desenvolvedor pleno por 1 hora por dia útil, totalizando 22 horas por mês.

- **Valor hora desenvolvedor:** R$ 11.500,00 / 160 h = R$ 71,88

- **Custo base mensal:** R$ 1.581,36

- **Acrescido 30% (gestão + margem):** R$ 2.055,77

Este valor garante previsibilidade ao cliente, sem comprometer a sustentabilidade da operação.

---
## 8. Conclusão
A estrutura de custos foi construída com base em premissas realistas, utilizando benchmarks de mercado e práticas financeiras recomendadas. A composição da equipe, os recursos e os valores propostos estão tecnicamente justificados, operacionalmente sustentáveis e comerciais o suficiente para atender o setor público.

Essa proposta pode ser usada como base de negociação comercial e como documento de prestação de contas para investidores, parceiros ou o próprio IPT.

---
## Referências

BANCO CENTRAL DO BRASIL. Fechamento de dólar. Disponível em: https://www.bcb.gov.br/estabilidadefinanceira/fechamentodolar. Acesso em: 4 jun. 2025.

SEBRAE. Entenda e calcule corretamente a margem de lucro. Disponível em: https://sebrae.com.br/sites/PortalSebrae/artigos/entenda-e-calcule-corretamente-a-margem-de-lucro,f2bbca017749e410VgnVCM1000003b74010aRCRD. Acesso em: 4 jun. 2025.

CONTABILIZEI. Contabilidade para empresas de profissionais de TI. Disponível em: https://www.contabilizei.com.br/contabilidade-online/contabilidade-para-empresas-profissionais-ti/. Acesso em: 4 jun. 2025.