#include "AST.h"
#include "Variable.h"

Function::Function(String name, std::shared_ptr<AST::FunctionAST> function)
  : MemorySlot(), function(function), name(name), type(function->getType()) {}
std::shared_ptr<AST::FunctionAST> Function::getFunction() { return function; }

String Function::getTypeName() { return type; }

Function::Type Function::getMemType() { return Type::Function; }

BuiltinFunction::BuiltinFunction(String name,
    BuiltinFunction::FunctionType func, int argCount, String returnType,
    std::vector<String> argTypes)
  : MemorySlot(), name(name), function(func), argTypes(argTypes),
    argCount(argCount), returnType(returnType) {
    std::string t = returnType.getReference() + "(";
    for (std::size_t i = 0; i < argTypes.size(); i++) {
        t += argTypes[i].getReference();
        if (i != argTypes.size() - 1) {
            t += ",";
        }
    }
    t += ")";

    type = t;
}

String BuiltinFunction::getTypeName() { return type; }

BuiltinFunction::Type BuiltinFunction::getMemType() {
    return Type::BuiltinFunction;
}

std::shared_ptr<MemorySlot> BuiltinFunction::call(
    std::vector<std::shared_ptr<MemorySlot>> args, std::size_t line) {
    // Check if the number of arguments is correct
    // Assert the types are correct too
    // Call the function
    // Assert the return type is correct
    return nullptr;
}
