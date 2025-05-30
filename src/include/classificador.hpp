#ifndef CLASSIFICADOR_HPP
#define CLASSIFICADOR_HPP

#pragma once
#include <opencv2/core.hpp>
#include <opencv2/ml.hpp>

//retorna o tipo da fissura passsando como parâmetro as features das imagens e o modelo usado
std::string classificarTipoFissura(const cv::Mat& features, const cv::Ptr<cv::ml::SVM>& modelo);


#endif
