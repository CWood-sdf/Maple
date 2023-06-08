#include "AST.h"
#include "Interpret.h"
#include "Scope.h"
#include <memory>
// #include <vcruntime.h>
std::set<String> identifiers = {"char", "int", "float", "bool", "var"};
std::set<String> identifierModifiers = {"const", "static", "global"};
std::set<String> controlFlow = {
    "if", "while", "for",
    /*"switch",
    "case",
    "default",*/
};
std::set<String> exitStatements = {"break", "continue", "return"};
std::set<String> operators = {"=", "+", "-", "*", "==", ">"};
std::set<char> operatorFirstCharacters = {};
std::map<String, int> operatorPrecedence = {
    {"*", 5}, {"+", 6}, {"-", 6}, {">", 9}, {"==", 10}, {"=", 16}};
std::map<char, char> escapeCharacters = {{'n', '\n'}, {'t', '\t'}, {'r', '\r'},
    {'b', '\b'}, {'f', '\f'}, {'v', '\v'}, {'a', '\a'}, {'\\', '\\'},
    {'\'', '\''}, {'\"', '\"'}, {'?', '\?'}, {'0', '\0'}};

std::shared_ptr<MemorySlot> evalOperatorEql(
    std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue, std::size_t line) {
    if (leftValue->getMemType() != MemorySlot::Type::Variable) {
        throwError("Assignment operator must be called on a variable", line);
    }
    Variable* v = (Variable*)leftValue.get();
    if (rightValue->getMemType() == MemorySlot::Type::Variable) {
        rightValue = ((Variable*)rightValue.get())->getValue();
        if (!rightValue) {
            throwError("Variable has no value", line);
        }
    }
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
template <typename TFloat, typename TInt, typename TChar, typename TBool>
std::shared_ptr<MemorySlot> doOperator(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue, TFloat (*opFloat)(double, double),
    TInt (*opInt)(int, int), TChar (*opChar)(char, char),
    TBool (*opBool)(bool, bool), std::size_t line) {
    Value* left = nullptr;
    Value* right = nullptr;
    if (leftValue->getMemType() == MemorySlot::Type::Variable) {
        leftValue = ((Variable*)leftValue.get())->getValue();
    }
    // Get left as a value
    if (leftValue->getMemType() == MemorySlot::Type::Value) {
        left = (Value*)leftValue.get();
    } else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
        left = (Value*)((Variable*)leftValue.get())->getValue().get();
    } else {
        throwError("Cannot call operaters on non-value types\n  note: left "
                   "side is not a value",
            line);
    }
    if (rightValue->getMemType() == MemorySlot::Type::Variable) {
        rightValue = ((Variable*)rightValue.get())->getValue();
    }
    // Get right as a value
    if (rightValue->getMemType() == MemorySlot::Type::Value) {
        right = (Value*)rightValue.get();
    } else {
        throwError("Cannot call operaters on non-value types\n  note: right "
                   "side is not a value",
            line);
    }
    // If one is a double, cast both to double
    if (left->getType() == Value::Types::Double ||
        right->getType() == Value::Types::Double) {
        double leftValue = left->getAsFloat();
        double rightValue = right->getAsFloat();
        return std::make_shared<Value>(opFloat(leftValue, rightValue));
    }
    // If one is an int, cast both to int
    else if (left->getType() == Value::Types::Int ||
             right->getType() == Value::Types::Int) {
        int leftValue = left->getAsInt();
        int rightValue = right->getAsInt();
        return std::make_shared<Value>(opInt(leftValue, rightValue));
    }
    // If one is a char, cast both to char
    else if (left->getType() == Value::Types::Char ||
             right->getType() == Value::Types::Char) {
        char leftValue = left->getAsChar();
        char rightValue = right->getAsChar();
        return std::make_shared<Value>(opChar(leftValue, rightValue));
    }
    // If they are bools, cast them to ints
    else {
        bool leftValue = left->getAsBool();
        bool rightValue = right->getAsBool();
        return std::make_shared<Value>(opBool(leftValue, rightValue));
    }
}
std::shared_ptr<MemorySlot> evalOperatorPls(
    std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue, std::size_t line) {
    double (*lambdaFloat)(
        double, double) = [](double a, double b) { return a + b; };
    int (*lambdaInt)(int, int) = [](int a, int b) { return a + b; };
    int (*lambdaChar)(char, char) = [](char a, char b) { return a + b; };
    int (*lambdaBool)(bool, bool) = [](bool a, bool b) { return a + b; };
    return doOperator(leftValue, rightValue, lambdaFloat, lambdaInt, lambdaChar,
        lambdaBool, line);
}
std::shared_ptr<MemorySlot> evalOperatorMns(
    std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue, std::size_t line) { /*{{{*/ /*{{{*/
    double (*lambdaFloat)(
        double, double) = [](double a, double b) { return a - b; };
    int (*lambdaInt)(int, int) = [](int a, int b) { return a - b; };
    int (*lambdaChar)(char, char) = [](char a, char b) { return a - b; };
    int (*lambdaBool)(bool, bool) = [](bool a, bool b) { return a - b; };
    return doOperator(leftValue, rightValue, lambdaFloat, lambdaInt, lambdaChar,
        lambdaBool, line);
} /*}}}*/
std::shared_ptr<MemorySlot> /*}}}*/
evalOperatorMult(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue, std::size_t line) { /*{{{*/
    double (*lambdaFloat)(
        double, double) = [](double a, double b) { return a * b; };
    int (*lambdaInt)(int, int) = [](int a, int b) { return a * b; };
    int (*lambdaChar)(char, char) = [](char a, char b) { return a * b; };
    int (*lambdaBool)(bool, bool) = [](bool a, bool b) { return a * b; };
    return doOperator(leftValue, rightValue, lambdaFloat, lambdaInt, lambdaChar,
        lambdaBool, line);
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
bool AST::isExitStatement(String str) {
    return exitStatements.find(str) != exitStatements.end();
}

AST::FloatAST::FloatAST(double value, std::size_t line)
  : ASTNode(line), value(value) {}
AST::FloatAST::FloatAST(String value, std::size_t line)
  : ASTNode(line), value(std::stod(value.getReference())) {}
std::shared_ptr<MemorySlot> AST::FloatAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::IntAST::IntAST(int value, std::size_t line)
  : ASTNode(line), value(value) {}
AST::IntAST::IntAST(String value, std::size_t line)
  : ASTNode(line), value(std::stoi(value.getReference())) {}
std::shared_ptr<MemorySlot> AST::IntAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::BoolAST::BoolAST(bool value, std::size_t line)
  : ASTNode(line), value(value) {}
AST::BoolAST::BoolAST(String value, std::size_t line)
  : ASTNode(line), value(value.getReference() == "true") {}
std::shared_ptr<MemorySlot> AST::BoolAST::getValue() {
    return std::make_shared<Value>(value);
}
AST::StringAST::StringAST(String value, std::size_t line)
  : ASTNode(line), value(value) {}
std::shared_ptr<MemorySlot> AST::StringAST::getValue() {
    return std::make_shared<Undefined>();
}
AST::CharacterAST::CharacterAST(char value, std::size_t line)
  : ASTNode(line), value(value) {}
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
    if (!leftValue) {
        throwError(
            "Using void return value as left hand side of binary operator \""s +
                op.getReference() + "\"",
            line);
    }
    if (!rightValue) {
        throwError(
            "Using void return value as right hand side of binary operator \""s +
                op.getReference() + "\"",
            line);
    }
    // Somehow invoke the operator
    if (op == "="s) {
        return evalOperatorEql(leftValue, rightValue, line);
    } else if (op == "+"s) {
        return evalOperatorPls(leftValue, rightValue, line);
    } else if (op == "-"s) {
        return evalOperatorMns(leftValue, rightValue, line);
    } else if (op == "*"s) {
        return evalOperatorMult(leftValue, rightValue, line);
    }
    return std::make_shared<Undefined>();
}
AST::UnaryOperatorAST::UnaryOperatorAST(
    std::shared_ptr<ASTNode> value, String op, std::size_t line)
  : ASTNode(line), value(value), op(op) {}
std::shared_ptr<MemorySlot> AST::UnaryOperatorAST::getValue() {
    auto opValue = value->getValue();
    // Somehow invoke the operator
    return std::make_shared<Undefined>();
}
AST::VariableAST::VariableAST(String name, std::size_t line)
  : ASTNode(line), name(name) {}
std::shared_ptr<MemorySlot> AST::VariableAST::getValue() {
    return getVariable(name, this->line);
}
AST::VariableDeclarationAST::VariableDeclarationAST(
    std::vector<String> types, String type, String name, std::size_t line)
  : ASTNode(line), modifiers(types), type(type), name(name) {}
std::shared_ptr<MemorySlot> AST::VariableDeclarationAST::getValue() {
    auto var = std::make_shared<Variable>(name, type);
    addVariable(var, this->line);
    return var;
}
AST::FunctionAST::FunctionAST(String returnType,
    std::vector<std::shared_ptr<ASTNode>> arguments,
    std::vector<std::shared_ptr<ASTNode>> statements, String name,
    std::size_t line)
  : ASTNode(line), returnType(returnType), arguments(arguments),
    statements(statements), name(name) {}
std::shared_ptr<MemorySlot> AST::FunctionAST::getValue() {
    auto var = std::make_shared<Variable>(name, getType());
    addFunction(var, this->line);
    var->setValue(std::make_shared<Function>(name, this->selfReference));
    return var;
}
std::shared_ptr<MemorySlot> AST::FunctionAST::call(
    std::vector<std::shared_ptr<ASTNode>> args, std::size_t callLine) {
    if (args.size() != arguments.size()) {
        throwError("Invalid number of arguments in call to function "s +
                       name.getReference() + "\n  note: expected "s +
                       std::to_string(arguments.size()) + " arguments, got "s +
                       std::to_string(args.size()) +
                       "\n  note: function declared at line "s +
                       std::to_string(this->line),
            callLine);
    }
    // Preprocess the arguments before new scope is added
    std::vector<std::shared_ptr<MemSlotAST>> argASTs = {};
    for (auto arg : args) {
        argASTs.push_back(std::make_shared<MemSlotAST>(arg->getValue()));
    }
    addScope(name);
    for (size_t i = 0; i < argASTs.size(); i++) {
        auto declAST = arguments[i];
        auto equals = std::make_shared<BinaryOperatorAST>(
            declAST, argASTs[i], "="s, callLine);
        equals->getValue();
    }
    interpret(statements);
    std::shared_ptr<MemorySlot> ret = nullptr;
    if (getExitType() == ExitType::Return) {
        ret = handleReturnRegister().second;
        if (ret->getTypeName() != returnType) {
            throwError("Invalid return type in function "s +
                           name.getReference() + "\n  note: expected "s +
                           returnType.getReference() + ", got "s +
                           ret->getTypeName().getReference(),
                callLine);
        }
    } else if (getExitType() != ExitType::None) {
        throwError("Invalid exit type in function "s + name.getReference() +
                       "  note: only valid type is 'return'",
            callLine);
    }
    removeScope();
    if (ret == nullptr && returnType != "void"s) {
        throwError(
            "Missing return statement in function "s + name.getReference(),
            callLine);
    }
    return ret;
}
void AST::FunctionAST::setSelfReference(std::shared_ptr<FunctionAST> self) {
    this->selfReference = self;
}
String AST::FunctionAST::getType() {
    std::string type = returnType.getReference() + "(";
    for (size_t i = 0; i < arguments.size(); i++) {
        VariableDeclarationAST* var =
            dynamic_cast<VariableDeclarationAST*>(arguments[i].get());
        type += var->type.getReference();
        if (i != arguments.size() - 1) {
            type += ",";
        }
    }
    type += ")";
    return returnType;
}
AST::FunctionCallAST::FunctionCallAST(String name,
    std::vector<std::shared_ptr<ASTNode>> arguments, std::size_t line)
  : ASTNode(line), name(name), arguments(arguments) {}
std::shared_ptr<MemorySlot> AST::FunctionCallAST::getValue() {
    // Get function
    auto func = getFunctionVariable(name, this->line);
    // Get function AST5
    auto fn = dynamic_cast<Function*>(func->getValue().get());
    if (fn == nullptr) {
        throwError("Function "s + name.getReference() + " is not defined"s,
            this->line);
    }
    // Call function
    auto fnAST = fn->getFunction();
    return fnAST->call(arguments, this->line);
}

AST::ExitAST::ExitAST(
    ExitType t, std::shared_ptr<ASTNode> value, std::size_t line)
  : ASTNode(line), type(t), value(value) {}

std::shared_ptr<MemorySlot> AST::ExitAST::getValue() {
    std::shared_ptr<MemorySlot> ret = nullptr;
    if (value) {
        ret = value->getValue();
    }
    setExit(type);
    setReturnRegister(ret);
    return ret;
}

AST::MemSlotAST::MemSlotAST(std::shared_ptr<MemorySlot> value, std::size_t line)
  : ASTNode(line), value(value) {}

std::shared_ptr<MemorySlot> AST::MemSlotAST::getValue() { return value; }

AST::IfAST::IfAST(std::shared_ptr<ASTNode> condition,
    std::vector<std::shared_ptr<ASTNode>> statements, bool isAlone,
    std::size_t line)
  : ASTNode(line), condition(condition), statements(statements),
    isAlone(isAlone) {}

std::shared_ptr<MemorySlot> AST::IfAST::getValue() {
    auto conditionRet = condition->getValue();
    if (conditionRet->getTypeName() != "bool"s) {
        throwError("Invalid type in if statement\n  note: expected \"bool\" "
                   "but got \"" +
                       conditionRet->getTypeName().getReference() + "\"",
            this->line);
    }
    std::shared_ptr<MemorySlot> value = nullptr;
    if (conditionRet->getMemType() == MemorySlot::Type::Variable) {
        value = dynamic_cast<Variable*>(conditionRet.get())->getValue();
    } else {
        value = conditionRet;
    }
    bool isTrue = dynamic_cast<Value*>(value.get())->getAs<bool>();
    if (isTrue) {
        addScope("if");
        interpret(statements);
        std::shared_ptr<MemorySlot> ret = nullptr;
        removeScope();
        return ret;
    } else {
        for (auto elseIf : elseIfs) {
            auto elseIfRet = elseIf->condition->getValue();
            if (elseIfRet->getTypeName() != "bool"s) {
                throwError("Invalid type in else if statement\n  note: "
                           "expected \"bool\" but got \"" +
                               elseIfRet->getTypeName().getReference() + "\"",
                    this->line);
            }
            std::shared_ptr<MemorySlot> value = nullptr;
            if (elseIfRet->getMemType() == MemorySlot::Type::Variable) {
                value = dynamic_cast<Variable*>(elseIfRet.get())->getValue();
            } else {
                value = elseIfRet;
            }
            bool isTrue = dynamic_cast<Value*>(value.get())->getAs<bool>();
            if (isTrue) {
                addScope("if");
                interpret(elseIf->statements);
                removeScope();
                return nullptr;
            }
        }
        if (elseStatements.size() > 0) {
            addScope("if");
            interpret(elseStatements);
            removeScope();
            return nullptr;
        }
    }
    return nullptr;
}

void AST::IfAST::addElseIf(std::shared_ptr<IfAST> elseIf) {
    elseIfs.push_back(elseIf);
}

void AST::IfAST::addElse(std::vector<std::shared_ptr<ASTNode>> elseStatements) {
    this->elseStatements = elseStatements;
}
