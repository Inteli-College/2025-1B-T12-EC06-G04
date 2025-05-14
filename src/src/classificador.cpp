#include "classificador.hpp"

std::string classificarTipoFissura(const cv::Mat& features, const cv::Ptr<cv::ml::SVM>& modelo) {
    if (modelo.empty()) {
        return "Erro: modelo não encontrado.";
    }

    int resposta = static_cast<int>(modelo->predict(features));
    return (resposta == 0) ? "retracao" : "termica";
}
