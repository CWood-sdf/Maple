#ifndef SCOPE_H
#define SCOPE_H

#include "Error.h"
#include "LinkedList.h"
#include "String.h"
#include "Variable.h"
#include <stack>
#include <unordered_map>
enum class VariableType {
    Variable,
    Function
};
enum class ExitType {
    Return,
    Break,
    Continue,
    None
};

class Scope {
    std::unordered_map<String, std::pair<std::shared_ptr<Variable>, VariableType>, StringHash> variables;
    String name;
    std::shared_ptr<MemorySlot> returnRegister;
    ExitType exitType;

public:
    Scope(String scopeName);
    std::shared_ptr<Variable> getVariable(String name);
    std::shared_ptr<Variable> getFunctionVariable(String name);
    std::shared_ptr<Variable> getGeneralVariable(String name);
    void setReturnRegister(std::shared_ptr<MemorySlot> reg);
    std::shared_ptr<MemorySlot> getReturnRegister();
    bool isExit();
    void setExit(ExitType type);
    void addVariable(String name, std::shared_ptr<Variable> variable, std::size_t line);
    void addFunctionVariable(String name, std::shared_ptr<Variable> variable, std::size_t line);
};
extern std::LinkedList<Scope> globalScope;
void addVariable(std::shared_ptr<Variable> v, std::size_t line);
void addFunction(std::shared_ptr<Variable> v, std::size_t line);
void initScope();
void addScope(String name);
void removeScope();
bool variableExists(String name);
bool functionExists(String name);
std::shared_ptr<Variable> getFunctionVariable(String name, std::size_t line);
std::shared_ptr<Variable> getVariable(String name, std::size_t line);

#endif // SCOPE_H
