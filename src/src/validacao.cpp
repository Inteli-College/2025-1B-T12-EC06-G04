#include <opencv2/opencv.hpp>
#include <opencv2/ml.hpp>
#include <filesystem>
#include <iostream>
#include "preprocessamento.hpp"
#include "detectorfissura.hpp"
#include "extrator_features.hpp"
#include "classificador.hpp"

//Deixar menos poluído
using namespace cv;
using namespace cv::ml;
namespace fs = std::filesystem;

int main() {

    std::string modeloPath = "../build/melhor_modelo_POLY_C0.100000_D2_CO0.400000_G2.000000.xml"; 
    Ptr<SVM> modelo = SVM::load(modeloPath);
    if (modelo.empty()) {
        std::cerr << "Erro ao carregar modelo SVM.\n";
        return -1;
    }

    int total = 0;
    int acertos = 0;

    vector<std::pair<std::string, int>> classes = {
        {"../images/validacao/retracao", 0},
        {"../images/validacao/termica", 1}
    };
    int validaModelo(entrada){
                Mat imagem = imread(entrada.path().string());
                if (imagem.empty()) continue;

                Mat imagem_prep = preprocessarImagem(imagem);
                Mat fissuras = detectarRachaduras(imagem_prep);
                Mat features = extrairFeatures(fissuras);

                std::string classe_predita = classificarTipoFissura(features, modelo);
                int resultado = (classe_predita == "retracao") ? 0 : 1;
                return resultado;
            }
    return 0;
}
