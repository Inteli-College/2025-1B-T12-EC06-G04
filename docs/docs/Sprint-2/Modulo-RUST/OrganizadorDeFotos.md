---
sidebar_position: 1
title: Organizador de arquivos
slug: /sprint-2/programação/Organizadordearquivos
---

## Organizador de Fotos por Localização

&emsp; é no arquivo `image_processor.rs` que tudo acontece, o coração do sistema de organização das imagens em pastas:

---

### 1. Extração de Metadados EXIF

1. Tenta primeiro usar **ExifTool** para extrair as informações.
2. Se falhar ou não estiver funcional, utiliza a **biblioteca exif-rust**.
3. Retorna `ImageMetadata` ou exibe erro.

**ExifTool**

* Executa `exiftool` no arquivo de imagem.
* Caso o exiftool falhe utiliza exif-rust (biblioteca nativa)
* Processa a saída, extraindo GPS e direção a partir de REGEX (Expressões Regulares).

---

### 2. Reorganização de dados do GPS

Suporta múltiplos formatos via REGEX e converte tudo para decimal:

* **DMS**: `"16 deg 38' 18.20\" S"`
* **GPS Position**: `"16 deg 38' 18.20\" S, 161 deg 7' 31.68\" E"`
* **Decimal com referência**: `"16.6384 S"`

&emsp;O sistema usa regex para identificar graus, minutos, segundos e referência (`N/S/E/W`), e faz a conversão para `f64`. O que é uma tentativa de suportar diversos tipos de metadados.

---

### 3. Agrupamento por Proximidade Geográfica (Organização nas pastas)

1. Para cada imagem:

    * Verifica-se a posição geográfica e compara com as anteriores.
    * Verifica-se a direção à qual o drone está virado (dado em graus).

**Distância**
&emsp;É utilizada para mensurar distância entre as duas imagens e classificar se vieram do mesmo prédio ou não.

* Leva em conta a curvatura da Terra.
* Retorna distância em metros entre duas coordenadas.

**Direção**
&emsp;É utilizado para verificar a fachada onde o drone tirou a foto (separação de fachadas para relatório e análise).

* Leva em conta os pontos cardeais para identificar e classificar fachadas (Norte, Sul, Leste e Oeste)
* Tenta mitigar problemas de estruturas com fachadas irregulares a partir do erro de 30°

---

### 4. Classificação por Fachada

&emsp;A classificação de qual fachada está situada a fissura, baseia-se em `GPSImgDirection` para identificar o ângulo de para o qual aponta o drone, e utilizando os pontos cardeais chegamos em:

&emsp;_vale ressaltar que mesmo que não sejam corretos os pontos em cada ângulo é possível classificar da mesma forma, tendo em vista que a classificação por si só não tem tanto importância, e sim a junção de fissuras de mesma fachada._
* **Norte**: 315° ≤ dir < 360° ou 0° ≤ dir < 45°
* **Leste**: 45° ≤ dir < 135°
* **Sul**: 135° ≤ dir < 225°
* **Oeste**: 225° ≤ dir < 315°
* **Indefinida**: sem direção disponível

---

### 5. Organização de Arquivos

Para cada prédio identificado:

1. Cria a pasta `Predio-X/`.
2. Para cada imagem nele:

   * Determina a fachada (Norte, Leste, Sul, Oeste ou Indefinida).
   * Separa em `Predio-X/fachada-{Nome}/`.
   * Adiciona os arquivos na mesma pasta de importação.

``` bash
  |Predio-1/
    ├── fachada-Norte/
    ├── fachada-Sul/
    ├── fachada-Leste/
    └── fachada-Oeste/
        
  |Predio-2/
    ├── fachada-Leste/
    └── fachada-Oeste/
    .
    .
    .
```

&emsp;Cada prédio não precisa necessáriamente ter o mesmo número de fachadas, neste caso são 4 ou menos.

---

## Considerações Finais

&emsp;Assim, crê-se que com esta funcionalidade será possível fazer a classificação das fissuras com maior precisão e completude, pois todas as imagens estarão organizadas por fachada e prédio no contexto destes empreendimentos. Por fim, acredita-se que é de suma importância que este processo ocorra para a mensuração do valor de reforma (caso necessária) e da análise de risco estrutural das construções civis.