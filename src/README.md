# Classificador OpenCV e SVM

---

## Requisitos

### Sistema Operacional utilizado
- Desenvolvido e testado em **Ubuntu 22.04 LTS**.

### 📦 Bibliotecas Utilizadas
O projeto utiliza as seguintes bibliotecas:

- [OpenCV 4.x](https://opencv.org/)
- Padrão C++17
- `std::filesystem` (C++17)

### 🔧 Dependências do sistema

Instale as dependências com:

```bash
sudo apt update
sudo apt install build-essential cmake libopencv-dev
```
ou para Arch
```bash
sudo pacman -Syu
sudo pacman -S base-devel cmake opencv
```

```
projeto/
├── CMakeLists.txt
├── src/
│   ├── main.cpp                # Acompanhamento das imagens
│   ├── validacao.cpp           # Validação do modelo
│   ├── classificador.cpp       # Função para classificar imagem
│   ├── detectorfissura.cpp     # Detecção com Canny + morfologia
│   ├── extrator_features.cpp   # Extração de 5 features
│   └── preprocessamento.cpp    # Pré-processamento com CLAHE + bilateral
├── include/
│   ├── classificador.hpp
│   ├── detectorfissura.hpp
│   ├── extrator_features.hpp
│   └── preprocessamento.hpp
├── images/
│   └── validacao/
│       ├── retracao/
│       └── termica/
│   └── treinamento/
│       ├── retracao/
│       └── termica/
└── build/    # Diretório gerado pelo CMake
└── README.md
└── .gitignore                  
```
