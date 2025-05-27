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
### Fissura {{@index}}

- **Faceta:** {{faceta_id}}
- **Localização:** {{localizacao}}
- **Classificação:** **{{classificacao}}**
- **Descrição:**  
  {{descricao}}

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
