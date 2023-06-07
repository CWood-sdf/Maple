#include "Error.h"
#include <fstream>
#define FILE_WRITE 0
#define STD_OUT 0
void throwError(std::string message, std::size_t line) {
    std::cerr << message << std::endl;
    std::cerr << "At line: " << line << std::endl;
// Write error to file
#if FILE_WRITE == 1
    std::ofstream errorFile("./error.txt", std::ios::app);
    if (errorFile.is_open()) {
        errorFile << message << std::endl;
        errorFile << "At line: " << line << std::endl;
        errorFile.close();
    }
#endif
    std::cout << "Press enter to exit..." << std::endl;
    std::cin.get();
    exit(1);
}
void writeOutput(std::string message, std::size_t line) {
#if STD_OUT == 1
    std::cout << message << std::endl;
    std::cout << "At line: " << line << std::endl;
#endif
// Write output to file
#if FILE_WRITE == 1
    std::ofstream outputFile("./output.txt", std::ios::app);
    if (outputFile.is_open()) {
        outputFile << message << std::endl;
        outputFile << "At line: " << line << std::endl;
        outputFile.close();
    }
#endif
}
void writeOutputNoLine(std::string message) {
#if STD_OUT == 1
    std::cout << message << std::endl;
#endif
// Write output to file
#if FILE_WRITE == 1
    std::ofstream outputFile("./output.txt", std::ios::app);
    if (outputFile.is_open()) {
        outputFile << message << std::endl;
        outputFile.close();
    }
#endif
}