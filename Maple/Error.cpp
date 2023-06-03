#include "Error.h"
#include <fstream>
void throwError(std::string message, std::size_t line) {
    std::cerr << message << std::endl;
    std::cerr << "At line: " << line << std::endl;
    // Write error to file
    std::ofstream errorFile("../../error.txt", std::ios::app);
    if (errorFile.is_open()) {
        errorFile << message << std::endl;
        errorFile << "At line: " << line << std::endl;
        errorFile.close();
    }
    exit(1);
}
void writeOutput(std::string message, std::size_t line) {
    std::cout << message << std::endl;
    std::cout << "At line: " << line << std::endl;
    // Write output to file
    std::ofstream outputFile("../../output.txt", std::ios::app);
    if (outputFile.is_open()) {
        outputFile << message << std::endl;
        outputFile << "At line: " << line << std::endl;
        outputFile.close();
    }
}
void writeOutputNoLine(std::string message) {
    std::cout << message << std::endl;
    // Write output to file
    std::ofstream outputFile("../../output.txt", std::ios::app);
    if (outputFile.is_open()) {
        outputFile << message << std::endl;
        outputFile.close();
    }
}