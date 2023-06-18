#include "AST.h"

Function::Function(String name, std::shared_ptr<AST::FunctionAST> function)
  : MemorySlot(), function(function), name(name), type(function->getType()) {}
std::shared_ptr<AST::FunctionAST> Function::getFunction() { return function; }

String Function::getTypeName() { return type; }

Function::Type Function::getMemType() { return Type::Function; }
