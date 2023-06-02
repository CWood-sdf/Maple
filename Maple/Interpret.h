#pragma once
#include <memory>
#include <string>
#include "AST.h"
void interpret(std::shared_ptr<AST::AST> ast);
void interpret(std::string file);
