// Maple.cpp : This file contains the 'main' function. Program execution begins
// and ends there.
//

#include "Builtins.h"
#include "Error.h"
#include "FileLoad.h"
#include "Interpret.h"
#include "Parser.h"
#include "Scope.h"
#include <chrono>
#include <cmath>
#include <iostream>
#include <string>
int main() {
    // start timer
    std::string file = loadFile("./Maple/maple.mpl");
    AST::prepareInterpreter(file);
    auto start = std::chrono::high_resolution_clock::now();
    initScope();
    addBuiltins();
    // make a variable for cos function
    writeOutput(file);
    /*while (AST::getNextToken() != AST::Type::EndOfFile) {
            std::cout << "Token: " <<
    AST::getCurrentToken().second.getReference()
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
    std::cout << "x: " << *(int*)(&((Value*)xvar->getValue().get())->getValue())
              << std::endl;
    auto qvar = getVariable("q", 0);
    std::cout << "q: "
              << *(double*)(&((Value*)qvar->getValue().get())->getValue())
              << std::endl;
    auto stupid = getVariable("stupid", 0);
    std::cout << "stupid: "
              << *(int*)(&((Value*)stupid->getValue().get())->getValue())
              << std::endl;
    auto o = getVariable("o", 0);
    std::cout << "o: "
              << *(int64_t*)(&((Value*)o->getValue().get())->getValue())
              << std::endl;
    auto o2 = getVariable("o2", 0);
    std::cout << "o2: "
              << *(int64_t*)(&((Value*)o2->getValue().get())->getValue())
              << std::endl;
    writeOutputNoLine(
        "Done in " + std::to_string(duration.count() / 1000.0) + " ms\n");
    std::cout << "Done in " << duration.count() / 1000.0 << " ms" << std::endl;

    std::cout << "Press enter to exit..." << std::endl;
    std::cin.get();
}
