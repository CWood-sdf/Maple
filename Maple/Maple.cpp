// Maple.cpp : This file contains the 'main' function. Program execution begins
// and ends there.
//

#include "Error.h"
#include "FileLoad.h"
#include "Interpret.h"
#include "Parser.h"
#include "Scope.h"
#include <iostream>
#include <string>

int main() {
    std::string file = loadFile("C:/Users/woodc/maple.mpl");
    AST::prepareInterpreter(file);
    initScope();
    writeOutput(file); //
    /*while (AST::getNextToken() != AST::Type::EndOfFile) {
            std::cout << "Token: " << AST::getCurrentToken().second.getReference()
    << ", " << (int)AST::getCurrentToken().first << std::endl;
    }*/
    auto block = AST::Parse::parse(true);
    for (auto& b : block) {
        b->getValue();
    }
    auto xvar = getVariable("x");
    writeOutputNoLine("x: ");
    writeOutputNoLine(std::to_string(*(long*)(&((Value*)xvar->getValue().get())->getValue())) + "\n");
    auto qvar = getVariable("q");
    writeOutputNoLine("q: ");
    writeOutputNoLine(std::to_string(*(double*)(&((Value*)qvar->getValue().get())->getValue())) + "\n");

    writeOutputNoLine("Done");
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
