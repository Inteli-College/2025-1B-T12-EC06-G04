#include "../include/preprocessamento.hpp"

cv::Mat preprocessarImagem(const cv::Mat& input) {
    cv::Mat redimensionada, gray, blurred, equalized;

    // Redimensiona
    cv::resize(input, redimensionada, cv::Size(640, 640));

    // Converte para cinza
    if (redimensionada.channels() == 3) {
        cv::cvtColor(redimensionada, gray, cv::COLOR_BGR2GRAY);
    } else {
        gray = redimensionada.clone();
    }

    // Suaviza
    cv::GaussianBlur(gray, blurred, cv::Size(5, 5), 1.5);

    return blurred;
}
