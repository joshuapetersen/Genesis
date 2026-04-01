#include "Sovereign_Tensor_Core.h"
#include <iostream>

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "Usage: test_gguf.exe <path_to_gguf_file>\n";
        return 1;
    }

    std::string filepath = argv[1];
    std::cout << "\n[Sovereign Core] Initiating Native Tensor Extraction..." << std::endl;
    std::cout << "[Sovereign Core] Target: " << filepath << std::endl;

    Sovereign::SovereignGGUF parser;
    if (parser.LoadFile(filepath)) {
        std::cout << "[Sovereign Core] Target locked and memory mapped successfully." << std::endl;
        // In this PoC, we just prove the header reading is valid without external libraries.
    } else {
        std::cout << "[FATAL] Failed to map or validate GGUF Volumetric Math Engine." << std::endl;
        return 1;
    }

    return 0;
}
