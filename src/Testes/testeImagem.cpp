#include <iostream>
#include <string>
#include <vector>
#include "opencv2/core.hpp"

bool resultadoAnalise(std::string imagem){
    std::string resultado;
    resultado = "imagem";
    if(imagem.find("imagem") != std::string::npos){
        return true;
    }else{
        return false;
    }
}

int main(){
    int acertos;
    int totalProcessado;
    std::string nomeArquivo;
    std::vector<std::string> imagens;
    imagens.push_back("imagem1.jpg");
    imagens.push_back("imagem2.jpg");
    imagens.push_back("not.jpg");
    imagens.push_back("imagem3.jpg");
    imagens.push_back("nao.jpg");
    imagens.push_back("imagem4.jpg");
    imagens.push_back("fim.png");
    imagens.push_back("");
    std::vector<bool> resultado;
    for (int i = 0; i < imagens.size(); i++){
        bool analise = resultadoAnalise(imagens[i]);
        if(analise == true){
            acertos++;
            totalProcessado++;
        }
        else{
            totalProcessado++;
        }
        resultado.push_back(analise);
    }
    std::cout << "Total de imagens processadas: " << totalProcessado << std::endl;
    std::cout << "Total de acertos: " << acertos << std::endl;
    std::cout << resultado.size() << std::endl;
    for (int i = 0; i < resultado.size(); i++){
        std::cout << "Resultado da imagem " << i+1 << ": " << resultado[i] << std::endl;
    }

return 0;
}
