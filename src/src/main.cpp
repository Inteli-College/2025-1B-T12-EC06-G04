#include <iostream>
#include <opencv2/opencv.hpp>

#include "preprocessamento.hpp"
#include "detectorfissura.hpp"
#include "extrator_features.hpp"

int main() {
    std::string caminhoImagem = "../images/treinamento/retracao/FR1.PNG";
    cv::Mat imagemOriginal = cv::imread(caminhoImagem);

    if (imagemOriginal.empty()) {
        std::cerr << "Erro ao carregar imagem: " << caminhoImagem << std::endl;
        return -1;
    }

    // Etapa 1: pré-processamento
    cv::Mat pre = preprocessarImagem(imagemOriginal);

    // Etapa 2: detecção
    cv::Mat mask = detectarRachaduras(pre);

    // Etapa 3: extração de features
    cv::Mat features = extrairFeatures(mask);

    // Exibindo cada uma das features
    std::cout << "Features extraídas:\n";
    std::cout << " - Orientação média: " << features.at<float>(0, 0) << "\n";
    std::cout << " - Desvio padrão da orientação: " << features.at<float>(0, 1) << "\n";
    std::cout << " - Comprimento médio das linhas: " << features.at<float>(0, 2) << "\n";
    std::cout << " - Número de linhas detectadas: " << features.at<float>(0, 3) << "\n";
    std::cout << " - Densidade de rachaduras: " << features.at<float>(0, 4) << "\n";

    // Mostrar resultados
    cv::imshow("Imagem original", imagemOriginal);
    cv::imshow("Pre-processada", pre);
    cv::imshow("Mascara de rachadura", mask);

    cv::waitKey(0);
    return 0;
}
