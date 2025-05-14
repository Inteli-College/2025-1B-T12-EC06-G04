#include <iostream>
#include <string>
#include <vector>
#include "opencv2/core.hpp"
#include <filesystem>
namespace fs = std::filesystem;
// @Wandermurem
// #include "classificador.cpp"

int resultadoAnalise(std::string imagem, int esperado){
    bool result = 0; //Apagar
    // int resultInt = classificador.validaModelo(imagem); //Descomentar
    result = true; //Apagar
    //result = resultInt == esperado; //Descomentar
    return result;
}

int main(){
    int acertos;
    int totalProcessado;
    std::string nomeArquivo;
    std::vector<std::pair<std::string, int>> classes = {
        {"../images/validacao/retracao", 0},
        {"../images/validacao/termica", 1},
    };
    for (auto& classe: classes){
        std::string path = classe.first;
        for (const auto& arquivo : fs::directory_iterator(path)){
            nomeArquivo = arquivo.path().string();
            bool tempResultado = resultadoAnalise(nomeArquivo, classe.second);
            if (tempResultado){
                acertos++;
            }
            totalProcessado++;
        }
    }
    std::cout << "Total de imagens processadas: " << totalProcessado << std::endl;
    std::cout << "Total de acertos: " << acertos << std::endl;
    std::cout << "Taxa de acerto: " << (static_cast<float>(acertos) / totalProcessado) * 100 << "%" << std::endl;

return 0;
}
