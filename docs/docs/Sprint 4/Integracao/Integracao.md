---
title: Integração entre Python e Rust
sidebar_position: 0
---

---
title: Integração entre Python e Rust
sidebar_position: 0
---

import Admonition from '@theme/Admonition';


# Introdução
&emsp; Documentação da Integração: Modelo de Visão Computacional e Aplicação Desktop
Esta seção detalha a integração entre o modelo de visão computacional (Python) e a aplicação desktop (Rust com Dioxus), abordando o fluxo de dados, a estrutura dos arquivos JSON utilizados e o uso de templates para a geração de relatórios.

## 1. Visão Geral da Integração
&emsp; Durante a Sprint 4, o foco principal foi integrar as duas grandes componentes do projeto: o modelo de visão computacional e o aplicativo desktop. Dada a natureza local da aplicação e a ausência de um banco de dados persistente, a troca de informações entre as páginas do aplicativo e, crucialmente, entre o aplicativo e o modelo de visão computacional, foi cuidadosamente desenhada.

&emsp; A abordagem adotada para gerenciar o fluxo de dados e a apresentação de informações foi a utilização de páginas de template e arquivos JSON. Isso permite que a aplicação gere visualizações dinâmicas e relatórios baseados nas informações contidas nos JSONs.

Existem dois arquivos JSON principais que orquestram esse fluxo de dados:

- **project.json:** Armazena as informações básicas do projeto fornecidas pelo usuário no momento da criação.

- **detection_results.json:**  Contém os resultados detalhados da análise de fotos realizada pelo modelo de visão computacional.

&emsp; O fluxo se inicia quando o usuário cria um novo projeto, delimitando informações como nome e ano da construção, que são salvas no project.json. Posteriormente, ao fazer o upload das fotos, o script do modelo de visão computacional é acionado, gerando o detection_results.json com as detecções e classificações das fissuras. Ambos os arquivos JSON são então utilizados para compor o relatório final.

&emsp; A primeira fase da integração, focada na comunicação do upload de fotos com as detecções do modelo, foi concluída. A Sprint 5 se dedicará a tornar essa integração mais robusta, incluindo tratamento de erros e a integração completa das páginas de estatísticas/gráficos e dos relatórios finais.

## 2. Estrutura dos Arquivos JSON
&emsp; Para garantir uma comunicação eficiente e estruturada, dois arquivos JSON são fundamentais no fluxo de dados da aplicação. Abaixo, detalhamos seus schemas.

### 2.1. project.json

&emsp; Este arquivo é criado no momento da definição de um novo projeto pelo usuário e armazena metadados essenciais.

Localização: `Projects/<NomeDoProjeto>/project.json` (Assumindo a estrutura de pastas do projeto)

``` json
Schema (JSON):

{
  "name": "string",         // Nome do projeto (ex: "Reforma Edifício Central")
  "description": "string",  // Descrição do projeto
  "year": "string",         // Ano de construção (lido como string na aplicação Rust)
  "leader": "string",       // Nome do líder/responsável pelo projeto
  "structure_type": "string", // Tipo da estrutura (ex: "Concreto Armado", "Alvenaria")
  "observations": "string", // Observações gerais sobre o projeto
  "status": "string"        // Status atual do projeto (ex: "Created", "Processing", "Completed")
}

```

### 2.2. detection_results.json
Este arquivo é gerado pelo modelo de visão computacional (Python) e contém os resultados da análise de fissuras para cada imagem processada. A aplicação Rust lê este arquivo como uma lista de objetos, onde cada objeto representa os dados de detecção de uma imagem.

Localização: `Projects/<NomeDoProjeto>/detection_results.json`

``` json
Schema (JSON):

[
  {
    "path": "string",          // Caminho completo da imagem original (ex: '/home/usuario/projeto/imagens/fachada_norte/img1.jpg')
    "fissura": [               // Array de fissuras detectadas nesta imagem
      {
        "name": "string",      // Nome da classificação da fissura (ex: "retracao", "termica")
        "confidence": "number" // Nível de confiança da detecção (0.0 a 1.0)
      },
      // ... outras fissuras detectadas nesta imagem
    ]
  },
  // ... outros objetos para outras imagens analisadas
]

```

### 2.3. Dados Preparados para o Template (report_template.md)

&emsp; A aplicação Rust processa os dados de project.json (implicitamente através dos parâmetros project_name e building_name) e detection_results.json, e então organiza-os em uma estrutura específica que é passada para o motor de template Handlebars. Esta é a estrutura de dados final que o template consome:

&emsp; Estrutura de Dados Passada para o Template (JSON):

```json
{
  "nome_projeto": "string",    // Nome do projeto (derivado do project.json)
  "nome_predio": "string",     // Nome do prédio (derivado do building_name_prop)
  "data_geracao": "string",    // Data e hora da geração do relatório (formato "%Y-%m-%d %H:%M:%S")
  "fissuras": [                // Array "achatado" contendo detalhes de CADA fissura detectada
    {
      "caminho_imagem": "string", // Caminho completo da imagem onde a fissura foi detectada
      "classificacao": "string",  // Classificação da fissura (ex: "retracao", "termica")
      "confianca": "number",      // Confiança do modelo para esta detecção
      "faceta_id": "string",      // Nome da faceta/área (extraído do caminho da imagem, ex: "fachada-Leste")
      "orientacao": "string",     // Valor fixo "N/A"
      "observacoes": "string",    // Valor fixo "N/A"
      "id_fissura": "string"      // ID único aleatório gerado para a fissura (ex: "f_123456789")
    },
    // ... detalhes de outras fissuras
  ]
}
```

## 3. Uso e Estrutura dos Templates
&emsp; A geração de relatórios e visualizações é realizada por meio de templates Markdown, que são preenchidos com os dados JSON processados e então convertidos para formatos finais como PDF ou DOCX utilizando a ferramenta pandoc.

### 3.1. Motor de Template
&emsp; A aplicação utiliza a biblioteca handlebars em Rust como motor de template. Ele permite a interpolação de dados JSON em um template textual, que neste caso é um arquivo Markdown.

### 3.2. report_template.md
&emsp; Este é o template principal responsável pela estrutura e conteúdo do relatório de inspeção. Ele consome a estrutura de dados detalhada na seção 2.3.

Localização: `src/app-rust/Template/report_template.md (conforme include_str!)`

&emsp; Exemplo da Estrutura do Template (Markdown com sintaxe Handlebars):

```md
# Relatório de Inspeção de Fissuras

**Nome do Projeto:** {{nome_projeto}}
**Data da Análise:** {{data_geracao}}
**Responsável Técnico:** **Identificação do Prédio:** {{nome_predio}}
**Endereço:**

---

## 1. Descrição Geral do Prédio

- **Número de andares:**
- **Ano de construção:**
- **Tipo de estrutura:**
- **Observações gerais:**

---

## 2. Facetas Inspecionadas

| ID da Faceta | Orientação | Nº de Rachaduras | Observações |
|--------------|------------|------------------|-------------|
|              |            |                  |             |
<!-- A tabela acima é um placeholder. A contagem de rachaduras e observações por faceta precisaria ser populada por lógica adicional ou um pré-processamento mais complexo para o template. -->

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


**Recomendações:**


---

## 5. Assinatura

**Nome:**
**Função:**
**Empresa:**

---

📄 *Relatório gerado automaticamente pela Plataforma de Análise de Fissuras.*

```
 <Admonition type="info" title = "Explicação do Uso do Template">
     

&emsp; **Interpolação Simples:** Variáveis como `{{nome_projeto}}`, `{{nome_predio}}` e `{{data_geracao}}`são diretamente substituídas pelos valores correspondentes do objeto JSON passado para o template.

---
&emsp;**Iteração de Fissuras:** O bloco `{{#each fissuras}} ... {{/each}}` itera sobre o array fissuras no JSON. Para cada item (fissura), ele renderiza o conteúdo definido dentro do bloco each, permitindo que `{{id_fissura}}`, `{{faceta_id}}`, `{{classificacao}}`, `{{confianca}}`, `{{caminho_imagem}}`, etc., sejam preenchidos com os dados da fissura atual.

---
&emsp; **Geração de Imagens:** A tag `![Imagem da Fissura]({{caminho_imagem}})` utiliza o caminho_imagem para incorporar a imagem da fissura diretamente no relatório Markdown. É importante que esses caminhos sejam acessíveis no ambiente onde o relatório é visualizado ou convertido.

---
&emsp; **Dados Estáticos vs. Dinâmicos:** Note que campos como "Responsável Técnico", "Endereço", "Número de andares", "Ano de construção", "Tipo de estrutura", e "Observações gerais" (na seção 1), e a tabela "Facetas Inspecionadas" (seção 2) não são atualmente populados dinamicamente pelo JSON fornecido ao template.

Eles precisariam ser adicionados à estrutura JSON passada para o template ou preenchidos manualmente se a intenção for que sejam dinâmicos. Da mesma forma, as seções "Conclusões e Recomendações" e "Assinatura" são campos de texto livre no template.
 </Admonition>

### 3.3. Processo de Exportação (Pandoc)
&emsp; Após o template Markdown ser preenchido com os dados, a aplicação utiliza a ferramenta de linha de comando pandoc para converter o conteúdo Markdown em outros formatos de documento, como PDF, DOCX, etc.

- O conteúdo Markdown gerado é salvo em um arquivo temporário.

- Um comando `pandoc` é executado, tomando o arquivo Markdown temporário como entrada e gerando o arquivo de saída no formato desejado `(-o <caminho_do_arquivo>).`

- Este processo garante a flexibilidade na geração de relatórios em múltiplos formatos padrão.