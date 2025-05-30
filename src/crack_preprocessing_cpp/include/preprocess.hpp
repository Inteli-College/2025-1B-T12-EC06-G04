#ifndef PREPROCESS_HPP
#define PREPROCESS_HPP

#include <opencv2/opencv.hpp>
#include <string>

void preprocessImage(const std::string& inputPath, const std::string& outputPath);
cv::Mat detectarRachaduras(const cv::Mat& imagemPreProcessada);

#endif
