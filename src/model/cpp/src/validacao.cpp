#include <opencv2/opencv.hpp>
#include <opencv2/ml.hpp>
#include <filesystem>
#include <iostream>
#include "../include/validacao.hpp"
#include "../include/preprocessamento.hpp"
#include "../include/detectorfissura.hpp"
#include "../include/extrator_features.hpp"
#include "../include/classificador.hpp"

//Deixar menos poluído
using namespace cv;
using namespace cv::ml;
namespace fs = std::filesystem;

int validaModelo(const std::string& imagemPath) {
    // Caminho do melhor modelo treinado
    std::string modeloPath = "src/build/melhor_modelo_POLY_C0.100000_D2_CO0.400000_G2.000000.xml"; 
    Ptr<SVM> modelo = SVM::load(modeloPath);
    if (modelo.empty()) {
        std::cerr << "Erro ao carregar modelo SVM.\n";
        return -1;
    }

 Mat imagem = imread(imagemPath);
    if (imagem.empty()) {
        std::cerr << "Erro ao carregar imagem: " << imagemPath << std::endl;
        return -1;
    }

    Mat imagem_prep = preprocessarImagem(imagem);
    Mat fissuras = detectarRachaduras(imagem_prep);
    Mat features = extrairFeatures(fissuras);

    int resultado = static_cast<int>(modelo->predict(features));
    return resultado;
}