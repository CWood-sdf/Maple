#ifndef ERROR_H
#define ERROR_H

#include <iostream>
namespace AST {
    std::size_t getLine();
}
void throwError(std::string message, std::size_t line = AST::getLine());
void writeOutput(std::string message, std::size_t line = AST::getLine());
void writeOutputNoLine(std::string message);
#endif // ERROR_H
