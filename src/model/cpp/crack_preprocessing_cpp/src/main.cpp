#include "preprocess.hpp"
#include <filesystem>
#include <iostream>

namespace fs = std::filesystem;

int main() {
    std::string inputDir = "../images";
    std::string outputBase = "../output";

    fs::create_directory(outputBase);

    for (const auto& entry : fs::recursive_directory_iterator(inputDir)) {
        if (fs::is_regular_file(entry) && entry.path().extension() == ".PNG") {
            std::string filename = entry.path().filename().string();
            std::string subfolder;

            if (filename.rfind("FR", 0) == 0) {
                subfolder = "FR";
            } else if (filename.rfind("FT", 0) == 0) {
                subfolder = "FT";
            } else {
                subfolder = "outros";
            }

            std::string outputFolder = outputBase + "/" + subfolder;
            fs::create_directory(outputFolder);

            std::string inputPath = entry.path().string();
            std::string outputPath = outputFolder + "/" + filename;

            preprocessImage(inputPath, outputPath);
        }
    }

    std::cout << "Pré-processamento concluído com separação por tipo!" << std::endl;
    return 0;
}
