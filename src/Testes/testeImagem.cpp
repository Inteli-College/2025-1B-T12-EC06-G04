#include <iostream>
#include <string>
#include <vector>
#include "opencv2/core.hpp"
#include <filesystem>
namespace fs = std::filesystem;
// @Wandermurem
#include "../include/validacao.hpp"

int resultadoAnalise(std::string imagem, int esperado){
     int resultInt = validaModelo(imagem); //Descomentar
    return resultInt == esperado;
}

int main(){
    int acertos = 0;
    int totalProcessado = 0;
    std::string nomeArquivo;
    std::vector<std::pair<std::string, int>> classes = {
        {"src/images/validacao/retracao", 0},
        {"src/images/validacao/termica", 1},
    };
    for (auto& classe: classes){
        std::string path = classe.first;
        for (const auto& arquivo : fs::directory_iterator(path)){
            nomeArquivo = arquivo.path().string();
            bool tempResultado = resultadoAnalise(nomeArquivo, classe.second);
            if (tempResultado){
                acertos++;
                std::cout << " acertou  \n" << acertos << std::endl;
            }
            totalProcessado++;
        }
    }
    std::cout << "Total de imagens processadas: " << totalProcessado << std::endl;
    std::cout << "Total de acertos: " << acertos << std::endl;
    std::cout << "Taxa de acerto: " << (static_cast<float>(acertos) / totalProcessado) * 100 << "%" << std::endl;

return 0;
}
