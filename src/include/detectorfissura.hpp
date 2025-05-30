#ifndef DETECTOR_HPP
#define DETECTOR_HPP

#include <opencv2/opencv.hpp>


cv::Mat detectarRachaduras(const cv::Mat& imagemPreProcessada);
// Detecta rachaduras em uma imagem pré-processada.
// Retorna uma máscara binária (rachaduras = branco)

#endif