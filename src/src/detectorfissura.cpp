#include "detectorfissura.hpp"

//Função para aplicar as máscaras na imagem, nesse caso usando o Canny para as bordas e duas função do Morforlogia para fazer
cv::Mat detectarRachaduras(const cv::Mat& imagemPreProcessada) {
    
    cv::Mat edges, morph;
    double med = cv::mean(imagemPreProcessada)[0];
    double lower = std::max(0.0, 0.2 * med);
    double upper = std::min(255.0, 1.1 * med);

    cv::Canny(imagemPreProcessada, edges, lower, upper);

    cv::dilate(edges, morph, cv::Mat(), cv::Point(-1, -1), 1);
    cv::erode(morph, morph, cv::Mat(), cv::Point(-1, -1), 1);

    return morph;
}