#ifndef EXTRATOR_FEATURES_HPP
#define EXTRATOR_FEATURES_HPP

#include <opencv2/opencv.hpp>


cv::Mat extrairFeatures(const cv::Mat& mask);
// Extrai um vetor de características (features) da máscara binária

#endif
