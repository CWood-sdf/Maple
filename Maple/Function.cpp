#include "AST.h"
#include "Variable.h"
#include <memory>
#include <string>

Function::Function(String name, std::shared_ptr<AST::FunctionAST> function)
  : MemorySlot(), function(function), name(name), type(function->getType()) {}
std::shared_ptr<AST::FunctionAST> Function::getFunction() { return function; }

String Function::getTypeName() { return type; }

Function::Type Function::getMemType() { return Type::Function; }

BuiltinFunction::BuiltinFunction(String name,
    BuiltinFunction::FunctionType func, size_t argCount, String returnType,
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
    std::vector<std::shared_ptr<AST::ASTNode>> args, std::size_t line) {
    // Check if the number of arguments is correct
    if (args.size() != argCount) {
        throwError("Incorrect number of arguments for builtin function '" +
                       name.getReference() + "'\n  note: expected " +
                       std::to_string(args.size()) + " arguments but got " +
                       std::to_string(argCount),
            line);
    }
    // Assert the types are correct too
    std::vector<std::shared_ptr<MemorySlot>> argValues = {};
    for (std::shared_ptr<AST::ASTNode> arg : args) {
        argValues.push_back(arg->getValue());

        if (!argValues.back()) {
            throwError("Atttempting to use a void return value as an argument "
                       "for builtin function '" +
                           name.getReference() +
                           "'\n  note: void value passed as parameter number " +
                           std::to_string(argValues.size()),
                line);
        }
        if (argValues.back()->getMemType() == MemorySlot::Type::Variable) {
            // unpack the variable
            argValues.back() = ((Variable*)argValues.back().get())->getValue();
        }
        if (argValues.back()->getTypeName() != argTypes[argValues.size() - 1]) {
            throwError("Incorrect type for argument " +
                           std::to_string(argValues.size()) +
                           " for builtin function '" + name.getReference() +
                           "'\n  note: expected " +
                           argTypes[argValues.size() - 1].getReference() +
                           " but got " +
                           argValues.back()->getTypeName().getReference(),
                line);
        }
    }
    // Call the function
    std::shared_ptr<MemorySlot> ret = function(argValues);
    // Assert the return type is correct
    if (ret->getTypeName() != returnType) {
        throwError("Incorrect return type for builtin function '" +
                       name.getReference() + "'\n  note: expected " +
                       returnType.getReference() + " but got " +
                       ret->getTypeName().getReference() +
                       "\n  note: this is an internal library error, please "
                       "report it to the developer of the library",
            line);
    }
    return ret;
}
