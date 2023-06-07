// Maple.cpp : This file contains the 'main' function. Program execution begins
// and ends there.
//

#include "Error.h"
#include "FileLoad.h"
#include "Interpret.h"
#include "Parser.h"
#include "Scope.h"
#include <chrono>
#include <iostream>
#include <string>

int main() {
    // start timer
    std::string file = loadFile("./Maple/maple.mpl");
    AST::prepareInterpreter(file);
    auto start = std::chrono::high_resolution_clock::now();
    initScope();
    writeOutput(file);
    /*while (AST::getNextToken() != AST::Type::EndOfFile) {
            std::cout << "Token: " << AST::getCurrentToken().second.getReference()
    << ", " << (int)AST::getCurrentToken().first << std::endl;
    }*/
    auto block = AST::Parse::parse(true);
    for (auto& b : block) {
        b->getValue();
    }

    // end timer
    auto end = std::chrono::high_resolution_clock::now();
    auto duration =
        std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    auto xvar = getVariable("x", 0);
    std::cout << "x: " << *(long*)(&((Value*)xvar->getValue().get())->getValue()) << std::endl;
    auto qvar = getVariable("q", 0);
    std::cout << "q: " << *(float*)(&((Value*)qvar->getValue().get())->getValue()) << std::endl;
    auto stupid = getVariable("stupid", 0);
    std::cout << "stupid: " << *(long*)(&((Value*)stupid->getValue().get())->getValue()) << std::endl;
    writeOutputNoLine("Done in " + std::to_string(duration.count() / 1000.0) + " ms\n");
    std::cout << "Done in " << duration.count() / 1000.0 << " ms" << std::endl;

    std::cout << "Press enter to exit..." << std::endl;
    std::cin.get();
}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started:
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add
//   Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project
//   and select the .sln file
