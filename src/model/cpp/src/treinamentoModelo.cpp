//VIBE CODING
#include <opencv2/opencv.hpp>
#include <opencv2/ml.hpp>
#include <filesystem>
#include <iostream>
#include <random>
#include "preprocessamento.hpp"
#include "detectorfissura.hpp"
#include "extrator_features.hpp"

//só para despoluir o código
using namespace cv;
using namespace cv::ml;
namespace fs = std::filesystem;

// Carrega e processa imagens com rótulo
std::vector<std::pair<cv::Mat, int>> carregarDados(const std::vector<std::pair<std::string, int>>& pastas) {
    std::vector<std::pair<cv::Mat, int>> dados;
    for (const auto& [pasta, rotulo] : pastas) {
        for (const auto& entrada : fs::directory_iterator(pasta)) {
            Mat img = imread(entrada.path().string());
            if (img.empty()) continue;

            Mat pre = preprocessarImagem(img);
            Mat mask = detectarRachaduras(pre);
            Mat features = extrairFeatures(mask);
            dados.push_back({features, rotulo});
        }
    }
    return dados;
}

// Padroniza (Z-score) todas as features
void normalizarDados(std::vector<std::pair<Mat, int>>& dados) {
    Mat todasFeatures;
    for (auto& [feat, _] : dados)
        todasFeatures.push_back(feat);

    Scalar mean, stddev;
    meanStdDev(todasFeatures, mean, stddev);
    todasFeatures = (todasFeatures - mean[0]) / stddev[0];

    for (size_t i = 0; i < dados.size(); ++i)
        dados[i].first = todasFeatures.row(static_cast<int>(i));
}

// Avalia o modelo em um conjunto de validação
float avaliarModelo(Ptr<SVM> modelo, const std::vector<std::pair<Mat, int>>& validacao) {
    int acertos = 0;
    for (const auto& [features, rotulo] : validacao) {
        int resultado = static_cast<int>(modelo->predict(features));
        if (resultado == rotulo) acertos++;
    }
    return 100.0f * acertos / validacao.size();
}

// Validação cruzada k-fold
float crossValidate(const std::vector<std::pair<cv::Mat, int>>& dados, double C, int degree, double coef0, double gamma, int kernelType, int k = 10) {
    int foldSize = dados.size() / k;
    float accTotal = 0.0f;

    for (int i = 0; i < k; ++i) {
        std::vector<std::pair<Mat, int>> treino, validacao;

        for (size_t j = 0; j < dados.size(); ++j) {
            if (j >= i * foldSize && j < (i + 1) * foldSize)
                validacao.push_back(dados[j]);
            else
                treino.push_back(dados[j]);
        }

        Mat treinoDados, treinoRotulos;
        for (const auto& [feat, rotulo] : treino) {
            treinoDados.push_back(feat);
            treinoRotulos.push_back(rotulo);
        }

        Ptr<SVM> modelo = SVM::create();
        modelo->setType(SVM::C_SVC);
        modelo->setKernel(kernelType);
        modelo->setC(C);
        modelo->setGamma(gamma);
        modelo->setTermCriteria(TermCriteria(TermCriteria::MAX_ITER + TermCriteria::EPS, 1000, 1e-6));

        if (kernelType == SVM::POLY) {
            modelo->setDegree(degree);
            modelo->setCoef0(coef0);
        }

        modelo->train(treinoDados, ROW_SAMPLE, treinoRotulos);
        float acc = avaliarModelo(modelo, validacao);
        accTotal += acc;
    }

    return accTotal / k;
}

int main() {
    // Caminho para os dados de treinamento e validação
    std::vector<std::pair<std::string, int>> pastas = {
        {"../images/treinamento/retracao", 0},
        {"../images/treinamento/termica", 1}
    };

    // Carrega e processa os dados
    auto dados = carregarDados(pastas);
    std::random_shuffle(dados.begin(), dados.end());
    normalizarDados(dados);

    // Definindo a busca pelos melhores parâmetros
    std::vector<double> Cs = {0.1, 0.25, 0.5, 0.75, 1.0};
    std::vector<int> degrees = {1, 2, 3, 4, 5};
    std::vector<double> coef0s = {0.0, 0.25, 0.4, 0.5};
    std::vector<double> gammas = {0.001, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0};
    std::vector<int> kernels = {SVM::POLY, SVM::RBF};

    // Parâmetros de melhor desempenho
    float melhorAcc = 0.0f;
    Ptr<SVM> melhorModelo;
    std::string melhorNome;

    // Busca pelos melhores parâmetros
    for (int kernelType : kernels) {
        for (double C : Cs) {
            for (double gamma : gammas) {
                if (kernelType == SVM::POLY) {
                    for (int degree : degrees) {
                        for (double coef0 : coef0s) {
                            float accCV = crossValidate(dados, C, degree, coef0, gamma, kernelType);

                            std::cout << "POLY - C: " << C << ", Degree: " << degree << ", Coef0: " << coef0
                                      << ", gamma: " << gamma << " -> Acurácia média (10-fold): " << accCV << "%\n";

                            if (accCV > melhorAcc) {
                                melhorAcc = accCV;
                                melhorModelo = SVM::create();
                                melhorModelo->setType(SVM::C_SVC);
                                melhorModelo->setKernel(SVM::POLY);
                                melhorModelo->setC(C);
                                melhorModelo->setGamma(gamma);
                                melhorModelo->setDegree(degree);
                                melhorModelo->setCoef0(coef0);
                                melhorModelo->setTermCriteria(TermCriteria(TermCriteria::MAX_ITER + TermCriteria::EPS, 1000, 1e-6));

                                // Treinamento com todos os dados após a busca
                                Mat treinoDados, treinoRotulos;
                                for (const auto& [feat, rotulo] : dados) {
                                    treinoDados.push_back(feat);
                                    treinoRotulos.push_back(rotulo);
                                }
                                melhorModelo->train(treinoDados, ROW_SAMPLE, treinoRotulos);

                                melhorNome = "../build/melhor_modelo_POLY_C" + std::to_string(C) +
                                             "_D" + std::to_string(degree) +
                                             "_CO" + std::to_string(coef0) +
                                             "_G" + std::to_string(gamma) + ".xml";
                            }
                        }
                    }
                } else if (kernelType == SVM::RBF) {
                    float accCV = crossValidate(dados, C, 0, 0.0, gamma, kernelType);

                    std::cout << "RBF - C: " << C << ", gamma: " << gamma << " -> Acurácia média (10-fold): " << accCV << "%\n";

                    if (accCV > melhorAcc) {
                        melhorAcc = accCV;
                        melhorModelo = SVM::create();
                        melhorModelo->setType(SVM::C_SVC);
                        melhorModelo->setKernel(SVM::RBF);
                        melhorModelo->setC(C);
                        melhorModelo->setGamma(gamma);
                        melhorModelo->setTermCriteria(TermCriteria(TermCriteria::MAX_ITER + TermCriteria::EPS, 1000, 1e-6));

                        Mat treinoDados, treinoRotulos;
                        for (const auto& [feat, rotulo] : dados) {
                            treinoDados.push_back(feat);
                            treinoRotulos.push_back(rotulo);
                        }
                        melhorModelo->train(treinoDados, ROW_SAMPLE, treinoRotulos);

                         melhorNome = "../build/melhor_modelo_RBF_C" + std::to_string(C) +
                                     "_G" + std::to_string(gamma) + ".xml";
                    }
                }
            }
        }
    };

    if (melhorModelo) {
        melhorModelo->save(melhorNome);
        std::cout << "\nModelo salvo como: " << melhorNome
                  << " com acurácia média: " << melhorAcc << "%\n";
    };

    return 0;
}
