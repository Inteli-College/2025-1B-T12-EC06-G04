---
sidebar_position: 1
slug: /sprint-3/Aplicação-Desktop/gerar-relatorio.md
title: Gerador de relatório
---

# Gerador de Relatório

## Introdução

&emsp;&emsp;O gerador de relatório foi desenvolvido utilizando a biblioteca [handlebars-rust](https://github.com/sunng87/handlebars-rust), permitindo a criação automatizada de relatórios a partir de templates em Markdown e dados estruturados em JSON. Essa abordagem garante flexibilidade e padronização na geração dos documentos, facilitando a manutenção e a personalização conforme as necessidades do projeto.

## Funcionamento

&emsp;&emsp;O processo de geração do relatório consiste em dois principais componentes:

1. **Template padrão em Markdown:**  
   Um arquivo markdown que define a estrutura e o layout do relatório, utilizando placeholders do Handlebars para os campos dinâmicos.

2. **Template padrão de dados em JSON:**  
   Um arquivo JSON que contém todas as informações do projeto, incluindo dados gerais, facetas inspecionadas, detalhamento das fissuras, conclusões e recomendações.

Ao utilizar o handlebars, eu posso armazenar um template, e a partir de alguma fonte de dados, preencher todas as variáveis do template, no caso, foi decidido utilizar um template padrão de JSON para armazenar e passar os dados para o handlebars.

## Template Markdown

O template markdown utiliza a sintaxe do Handlebars para preencher os campos dinâmicos e iterar sobre listas. Veja um exemplo real abaixo:

```
#  Relatório de Inspeção de Fissuras

**Nome do Projeto:** {{nome_projeto}}  
**Data da Análise:** {{data_analise}}  
**Responsável Técnico:** {{nome_responsavel}}  
**Identificação do Prédio:** {{nome_predio}}  
**Endereço:** {{endereco_predio}}

---

## 1. Descrição Geral do Prédio

- **Número de andares:** {{numero_andares}}
- **Ano de construção:** {{ano_construcao}}
- **Tipo de estrutura:** {{tipo_estrutura}}
- **Observações gerais:**  
  {{observacoes_gerais}}

---

## 2. Facetas Inspecionadas

| ID da Faceta | Orientação | Nº de Rachaduras | Observações |
|--------------|------------|------------------|-------------|
{{#each facetas}}
| {{id}} | {{orientacao}} | {{qtd_rachaduras}} | {{observacoes}} |
{{/each}}

---

## 3. Detalhamento das Fissuras

{{#each fissuras}}
### Fissura {{id_fissura}}

- **Faceta:** {{faceta_id}}
- **Orientação:** {{orientacao}}
- **Classificação:** **{{classificacao}}**
- **Confiança do Modelo:** {{confianca}}%
- **Observações da Faceta:**  
  {{observacoes}}

**Imagem:**  
![Imagem da Fissura]({{caminho_imagem}})

---
{{/each}}

## 4. Conclusões e Recomendações

**Conclusões:**
{{conclusao_geral}}

**Recomendações:**  
{{recomendacoes}}

---

## 5. Assinatura

**Nome:** {{nome_responsavel}}  
**Função:** {{funcao_responsavel}}  
**Empresa:** {{nome_empresa}}

---

📄 *Relatório gerado automaticamente pela Plataforma de Análise de Fissuras.*
```

## Template JSON

O JSON de entrada deve conter todos os campos necessários para preencher o template, incluindo listas de facetas e detalhamento das fissuras. Veja um exemplo real abaixo:

```json
{
  "nome_projeto": "Galpão Logístico XPTO",
  "data_analise": "2025-05-18",
  "nome_responsavel": "Eng. Mariana Lima",
  "nome_predio": "Galpão 3",
  "endereco_predio": "Rod. dos Imigrantes, km 22 - Diadema/SP",
  "numero_andares": 1,
  "ano_construcao": 2005,
  "tipo_estrutura": "Estrutura metálica com fundação de concreto",
  "observacoes_gerais": "Estrutura apresenta sinais de recalque diferencial e movimentação térmica significativa.",
  "dados_modelo": {
    "algorithm": "Yolo",
    "tipo_modelo": "Classificação"
  },
  "facetas": [
    {
      "id": "F01",
      "orientacao": "Sul",
      "qtd_rachaduras": 3,
      "observacoes": "Fissuras próximas a pilares e juntas de dilatação."
    },
    {
      "id": "F02",
      "orientacao": "Oeste",
      "qtd_rachaduras": 2,
      "observacoes": "Rachaduras próximas ao piso e na ligação com o telhado."
    }
  ],
  "dados_fissuras": [
    {
      "id_faceta": "Faceta 1",
      "orientacao": "Sul",
      "qtd_rachaduras": 3,
      "observacoes": "Fissuras próximas a pilares e juntas de dilatação.",
      "imagens_fissuras": [
        {
          "id_imagem": "0",
          "caminho_imagem": "images/fissura_conexao.jpg",
          "fissuras": [
            {
              "id_fissura": 0,
              "classificacao_fissura": "térmica",
              "porcentagem_confianca_modelo": 92
            },
            {
              "id_fissura": 1,
              "classificacao_fissura": "retração",
              "porcentagem_confianca_modelo": 95
            }
          ]
        }
      ]
    },
    {
      "id_faceta": "Faceta 2",
      "orientacao": "Norte",
      "qtd_rachaduras": 3,
      "observacoes": "Fissuras próximas a muros e juntas de dilatação.",
      "imagens_fissuras": [
        {
          "id_imagem": "0",
          "caminho_imagem": "images/fissura_conexao.jpg",
          "fissuras": [
            {
              "id_fissura": 0,
              "classificacao_fissura": "térmica",
              "porcentagem_confianca_modelo": 92
            },
            {
              "id_fissura": 1,
              "classificacao_fissura": "retração",
              "porcentagem_confianca_modelo": 95
            }
          ]
        }
      ]
    }
  ],
  "conclusao_geral": "Há indícios de problemas estruturais sérios que exigem intervenção imediata.",
  "recomendacoes": "Contratar perícia estrutural detalhada, interditar área próxima ao pilar P3 e reparar as juntas.",
  "funcao_responsavel": "Engenheira Estrutural",
  "nome_empresa": "IPT - Instituto de Pesquisas Tecnológicas"
}
```

## Geração Automatizada

&emsp;&emsp;Ao executar o gerador de relatório, o sistema carrega o template markdown e o arquivo de dados JSON, processando ambos com a handlebars-rust. O resultado é um arquivo markdown preenchido automaticamente com as informações fornecidas, incluindo tabelas, imagens e listas detalhadas, pronto para ser exportado ou convertido para outros formatos, como MD, PDF ou Docx.

## Conclusão

&emsp;&emsp;Usar o handlebars-rust para gerar relatórios deixa tudo mais prático, flexível e fácil de adaptar. Assim, é possível atualizar e personalizar os relatórios conforme as necessidades do projeto, sem complicação.


