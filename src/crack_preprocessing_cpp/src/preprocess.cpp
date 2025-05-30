#include "preprocess.hpp"
#include <string>
#include <iostream>
#include <opencv2/opencv.hpp>

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

void preprocessImage(const std::string& inputPath, const std::string& outputPath) {
    cv::Mat input = cv::imread(inputPath, cv::IMREAD_COLOR);
    if (input.empty()) {
        std::cerr << "Erro ao carregar imagem: " << inputPath << std::endl;
        return;
    }

    cv::Mat resized;
    cv::resize(input, resized, cv::Size(640, 640));

    cv::Mat gray;
    if (resized.channels() == 3) {
        cv::cvtColor(resized, gray, cv::COLOR_BGR2GRAY);
    } else {
        gray = resized.clone();
    }

    cv::Mat blurred;
    cv::GaussianBlur(gray, blurred, cv::Size(5, 5), 1.5);

    cv::Mat resultado = detectarRachaduras(blurred);
    cv::imwrite(outputPath, resultado);
}
