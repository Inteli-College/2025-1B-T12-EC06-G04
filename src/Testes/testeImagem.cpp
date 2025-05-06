#include <iostream>
#include <string>

std::string resultadoAnalise();

int main(){
    std::string nomeArquivo;
    std::cout << "Digite o nome do arquivo de imagem (exemplo: imagem.png): ";
    std::getline(std::cin, nomeArquivo);
    std::cout << "O nome do arquivo de imagem é: " << nomeArquivo << std::endl;
    if (nomeArquivo.empty()){
        std::cout << "Nenhum arquivo foi inserido." << std::endl;
        return 1;
    }else if (nomeArquivo + ".png" == resultadoAnalise()){
        std::cout << "Resultado positivo" << std::endl;
    }else{
        std::cout << "Resultado negativo" << std::endl;
    }
return 0;
}
std::string resultadoAnalise(){
    std::string resultado;
    //Adicionar aqui a chamada da análise de imagem
    //resultado = analiseImagem(nomeArquivo);
    resultado = "imagem.png";
    return resultado;
}