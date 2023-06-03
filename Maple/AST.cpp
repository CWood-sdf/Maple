#include "AST.h"
#include <memory>
// #include <vcruntime.h>
std::set<String> identifiers = {"char", "int", "float", "bool", "var"};
std::set<String> identifierModifiers = {"const", "static", "global"};
std::set<String> controlFlow = {"if", "else", "while", "for",
    /*"switch",
    "case",
    "default",*/
    "break", "continue", "return"};
std::set<String> operators = {"=", "+", "-", "*", "==", ">"};
std::set<char> operatorFirstCharacters = {};
std::map<String, int> operatorPrecedence = {{"*", 5}, {"+", 6}, {"-", 6},
    {">", 9}, {"==", 10}, {"=", 16}};
std::map<char, char> escapeCharacters = {
    {'n', '\n'}, {'t', '\t'}, {'r', '\r'}, {'b', '\b'},
    {'f', '\f'}, {'v', '\v'}, {'a', '\a'}, {'\\', '\\'},
    {'\'', '\''}, {'\"', '\"'}, {'?', '\?'}, {'0', '\0'}};

std::shared_ptr<MemorySlot>
evalOperatorEql(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue) {
    if (leftValue->getMemType() != MemorySlot::Type::Variable) {
        throwError("Assignment operator must be called on a variable", 0);
    }
    Variable* v = (Variable*)leftValue.get();
    if (rightValue->getMemType() == MemorySlot::Type::Value) {
        Value* val = (Value*)rightValue.get();
        if (v->getTypeName() == "float"s) {
            if (val->getType() == Value::Types::Double) {
                v->setValue(rightValue);
            } else {
                v->setValue(std::make_shared<Value>(val->getAsFloat()));
            }
        } else if (v->getTypeName() == "int"s) {
            if (val->getType() == Value::Types::Int) {
                v->setValue(rightValue);
            } else {
                v->setValue(std::make_shared<Value>(val->getAsInt()));
            }
        } else if (v->getTypeName() == "char"s) {
            if (val->getType() == Value::Types::Char) {
                v->setValue(rightValue);
            } else {
                v->setValue(std::make_shared<Value>(val->getAsChar()));
            }
        } else if (v->getTypeName() == "bool"s) {
            if (val->getType() == Value::Types::Bool) {
                v->setValue(rightValue);
            } else {
                v->setValue(std::make_shared<Value>(val->getAsBool()));
            }
        }
    } else {
        v->setValue(rightValue);
    }
    return leftValue;
}
std::shared_ptr<MemorySlot>
evalOperatorPls(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue) {
    Value* left = nullptr;
    Value* right = nullptr;
    // Get left as a value
    if (leftValue->getMemType() == MemorySlot::Type::Value) {
        left = (Value*)leftValue.get();
    } else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
        left = (Value*)((Variable*)leftValue.get())->getValue().get();
    } else {
        throwError("Cannot add non-value types", 0);
    }
    // Get right as a value
    if (rightValue->getMemType() == MemorySlot::Type::Value) {
        right = (Value*)rightValue.get();
    } else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
        right = (Value*)((Variable*)rightValue.get())->getValue().get();
    } else {
        throwError("Cannot add non-value types", 0);
    }
    // If one is a double, cast both to double
    if (left->getType() == Value::Types::Double ||
        right->getType() == Value::Types::Double) {
        double leftValue = left->getAsFloat();
        double rightValue = right->getAsFloat();
        return std::make_shared<Value>(leftValue + rightValue);
    }
    // If one is an int, cast both to int
    else if (left->getType() == Value::Types::Int ||
             right->getType() == Value::Types::Int) {
        int leftValue = left->getAsInt();
        int rightValue = right->getAsInt();
        return std::make_shared<Value>(leftValue + rightValue);
    }
    // If one is a char, cast both to char
    else if (left->getType() == Value::Types::Char ||
             right->getType() == Value::Types::Char) {
        char leftValue = left->getAsChar();
        char rightValue = right->getAsChar();
        return std::make_shared<Value>(leftValue + rightValue);
    }
    // If they are bools, cast them to ints
    else {
        int leftValue = left->getAsBool();
        int rightValue = right->getAsBool();
        return std::make_shared<Value>(leftValue + rightValue);
    }
}
std::shared_ptr<MemorySlot>
evalOperatorMns(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue) {
    Value* left = nullptr;
    Value* right = nullptr;
    // Get left as a value
    if (leftValue->getMemType() == MemorySlot::Type::Value) {
        left = (Value*)leftValue.get();
    } else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
        left = (Value*)((Variable*)leftValue.get())->getValue().get();
    } else {
        throwError("Cannot subtract non-value types", 0);
    }
    // Get right as a value
    if (rightValue->getMemType() == MemorySlot::Type::Value) {
        right = (Value*)rightValue.get();
    } else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
        right = (Value*)((Variable*)rightValue.get())->getValue().get();
    } else {
        throwError("Cannot subtract non-value types", 0);
    }
    // If one is a double, cast both to double
    if (left->getType() == Value::Types::Double ||
        right->getType() == Value::Types::Double) {
        double leftValue = left->getAsFloat();
        double rightValue = right->getAsFloat();
        return std::make_shared<Value>(leftValue - rightValue);
    }
    // If one is an int, cast both to int
    else if (left->getType() == Value::Types::Int ||
             right->getType() == Value::Types::Int) {
        int leftValue = left->getAsInt();
        int rightValue = right->getAsInt();
        return std::make_shared<Value>(leftValue - rightValue);
    }
    // If one is a char, cast both to char
    else if (left->getType() == Value::Types::Char ||
             right->getType() == Value::Types::Char) {
        char leftValue = left->getAsChar();
        char rightValue = right->getAsChar();
        return std::make_shared<Value>(leftValue - rightValue);
    }
    // If they are bools, cast them to ints
    else {
        int leftValue = left->getAsBool();
        int rightValue = right->getAsBool();
        return std::make_shared<Value>(leftValue - rightValue);
    }
}
std::shared_ptr<MemorySlot>
evalOperatorMult(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue) {
    Value* left = nullptr;
    Value* right = nullptr;
    // Get left as a value
    if (leftValue->getMemType() == MemorySlot::Type::Value) {
        left = (Value*)leftValue.get();
    } else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
        left = (Value*)((Variable*)leftValue.get())->getValue().get();
    } else {
        throwError("Cannot multiply non-value types", 0);
    }
    // Get right as a value
    if (rightValue->getMemType() == MemorySlot::Type::Value) {
        right = (Value*)rightValue.get();
    } else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
        right = (Value*)((Variable*)rightValue.get())->getValue().get();
    } else {
        throwError("Cannot multiply non-value types", 0);
    }
    // If one is a double, cast both to double
    if (left->getType() == Value::Types::Double ||
        right->getType() == Value::Types::Double) {
        double leftValue = left->getAsFloat();
        double rightValue = right->getAsFloat();
        return std::make_shared<Value>(leftValue * rightValue);
    }
    // If one is an int, cast both to int
    else if (left->getType() == Value::Types::Int ||
             right->getType() == Value::Types::Int) {
        int leftValue = left->getAsInt();
        int rightValue = right->getAsInt();
        return std::make_shared<Value>(leftValue * rightValue);
    }
    // If one is a char, cast both to char
    else if (left->getType() == Value::Types::Char ||
             right->getType() == Value::Types::Char) {
        char leftValue = left->getAsChar();
        char rightValue = right->getAsChar();
        return std::make_shared<Value>(leftValue * rightValue);
    }
    // If they are bools, cast them to ints
    else {
        int leftValue = left->getAsBool();
        int rightValue = right->getAsBool();
        return std::make_shared<Value>(leftValue * rightValue);
    }
}
int AST::getPrecedence(String op) {
    if (operatorPrecedence.find(op) == operatorPrecedence.end()) {
        return -1;
    }
    return operatorPrecedence[op];
}
bool AST::isIdentifier(String str) {
    return identifiers.find(str) != identifiers.end();
}
bool AST::isIdentifierModifier(String str) {
    return identifierModifiers.find(str) != identifierModifiers.end();
}
bool AST::isControlFlow(String str) {
    return controlFlow.find(str) != controlFlow.end();
}
bool AST::isOperator(String str) {
    return operators.find(str) != operators.end();
}
bool AST::isBooleanLiteral(String str) {
    return str.getReference() == "true" || str.getReference() == "false";
}

AST::FloatAST::FloatAST(double value, std::size_t line) : ASTNode(line), value(value) {}
AST::FloatAST::FloatAST(String value, std::size_t line)
  : ASTNode(line), value(std::stod(value.getReference())) {}
std::shared_ptr<MemorySlot> AST::FloatAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::IntAST::IntAST(int value, std::size_t line) : ASTNode(line), value(value) {}
AST::IntAST::IntAST(String value, std::size_t line) : ASTNode(line), value(std::stoi(value.getReference())) {}
std::shared_ptr<MemorySlot> AST::IntAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::BoolAST::BoolAST(bool value, std::size_t line) : ASTNode(line), value(value) {}
AST::BoolAST::BoolAST(String value, std::size_t line) : ASTNode(line), value(value.getReference() == "true") {}
std::shared_ptr<MemorySlot> AST::BoolAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::StringAST::StringAST(String value, std::size_t line) : ASTNode(line), value(value) {}
std::shared_ptr<MemorySlot> AST::StringAST::getValue() {
    return std::make_shared<Undefined>();
}
AST::CharacterAST::CharacterAST(char value, std::size_t line) : ASTNode(line), value(value) {}
AST::CharacterAST::CharacterAST(String value, std::size_t line)
  : ASTNode(line), value(value.getReference()[0]) {}
std::shared_ptr<MemorySlot> AST::CharacterAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::BinaryOperatorAST::BinaryOperatorAST(std::shared_ptr<ASTNode> left,
    std::shared_ptr<ASTNode> right, String op, std::size_t line)
  : ASTNode(line), left(left), right(right), op(op) {}
std::shared_ptr<MemorySlot> AST::BinaryOperatorAST::getValue() {
    auto leftValue = left->getValue();
    auto rightValue = right->getValue();
    // Somehow invoke the operator
    if (op == "="s) {
        return evalOperatorEql(leftValue, rightValue);
    } else if (op == "+"s) {
        return evalOperatorPls(leftValue, rightValue);
    } else if (op == "-"s) {
        return evalOperatorMns(leftValue, rightValue);
    } else if (op == "*"s) {
        return evalOperatorMult(leftValue, rightValue);
    }
    return std::make_shared<Undefined>();
}
AST::UnaryOperatorAST::UnaryOperatorAST(std::shared_ptr<ASTNode> value, String op, std::size_t line)
  : ASTNode(line), value(value), op(op) {}
std::shared_ptr<MemorySlot> AST::UnaryOperatorAST::getValue() {
    auto opValue = value->getValue();
    // Somehow invoke the operator
    return std::make_shared<Undefined>();
}
AST::VariableAST::VariableAST(String name, std::size_t line) : ASTNode(line), name(name) {}
std::shared_ptr<MemorySlot> AST::VariableAST::getValue() {
    return getVariable(name);
}
AST::VariableDeclarationAST::VariableDeclarationAST(std::vector<String> types,
    String type, String name, std::size_t line)
  : ASTNode(line), modifiers(types), type(type), name(name) {}
std::shared_ptr<MemorySlot> AST::VariableDeclarationAST::getValue() {
    auto var = std::make_shared<Variable>(name, type);
    addVariable(var, this->line);
    return var;
}
AST::FunctionAST::FunctionAST(String returnType,
    std::vector<std::shared_ptr<ASTNode>> arguments,
    std::vector<std::shared_ptr<ASTNode>> statements,
    String name, std::size_t line)
  : ASTNode(line), returnType(returnType), arguments(arguments), statements(statements),
    name(name) {}
std::shared_ptr<MemorySlot> AST::FunctionAST::getValue() {
    auto var = std::make_shared<Variable>(name, getType());
    addVariable(var, this->line);
    var->setValue(std::make_shared<Function>(name, this));
    return var;
}
std::shared_ptr<MemorySlot> AST::FunctionAST::call(std::vector<std::shared_ptr<ASTNode>> args) {
    return std::make_shared<Undefined>();
}
String AST::FunctionAST::getType() {
    std::string type = returnType.getReference() + "(";
    for (size_t i = 0; i < arguments.size(); i++) {
        VariableDeclarationAST* var = dynamic_cast<VariableDeclarationAST*>(arguments[i].get());
        type += var->type.getReference();
        if (i != arguments.size() - 1) {
            type += ",";
        }
    }
    type += ")";
    return returnType;
}
AST::FunctionCallAST::FunctionCallAST(
    std::shared_ptr<ASTNode> function, std::vector<std::shared_ptr<ASTNode>> arguments, std::size_t line)
  : ASTNode(line), function(function), arguments(arguments) {}
std::shared_ptr<MemorySlot> AST::FunctionCallAST::getValue() {
    return std::make_shared<Undefined>();
}
