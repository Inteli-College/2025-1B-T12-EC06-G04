#include "extrator_features.hpp"
#include <cmath>
#include <numeric>

//Essa função pega a máscara do pré processamento e extrai, com algumas funções do Open CV e das lib de math 5 features
cv::Mat extrairFeatures(const cv::Mat& mask) {
    std::vector<cv::Vec4i> linhas;
    cv::HoughLinesP(mask, linhas, 1, CV_PI / 180, 50, 20, 10);

    float somaAngulos = 0.0f, somaComprimento = 0.0f;
    std::vector<float> angulos;

    for (const auto& l : linhas) {
        float dx = l[2] - l[0];
        float dy = l[3] - l[1];
        float comprimento = std::sqrt(dx * dx + dy * dy);
        float angulo = std::atan2(dy, dx) * 180.0f / CV_PI;

        somaAngulos += std::abs(angulo);
        somaComprimento += comprimento;
        angulos.push_back(angulo);
    }

    // Feature 1 = Médias dos ângulos das fissruas
    float mediaAngulo = linhas.empty() ? 0.0f : somaAngulos / linhas.size();

    //Feature 2 = Comprimento médio das linhas das fissuras
    float comprimentoMedio = linhas.empty() ? 0.0f : somaComprimento / linhas.size();

    //Feature 3: Desvio padrão dos ângulos
    float desvio = 0.0f;
    if (!angulos.empty()) {
        float media = std::accumulate(angulos.begin(), angulos.end(), 0.0f) / angulos.size();
        for (auto a : angulos) desvio += (a - media) * (a - media);
        desvio = std::sqrt(desvio / angulos.size());
    }

    // Feature 4: Desnsidade das linhas da máscara
    float densidade = static_cast<float>(cv::countNonZero(mask)) / (mask.rows * mask.cols);

    // Feature 5: Quantidade de linhas da máscara
    float numLinhas = static_cast<float>(linhas.size());

    return (cv::Mat_<float>(1, 5) << mediaAngulo, desvio, comprimentoMedio, numLinhas, densidade);
    
}
