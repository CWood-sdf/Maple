#include "Scope.h"
#include <memory>

std::LinkedList<Scope> globalScope = {};
void addVariable(std::shared_ptr<Variable> v, std::size_t line) {
    String name = v->getName();
    globalScope.getBase()->addVariable(name, v, line);
}

void initScope() {
    globalScope.pushBase(Scope("$_globalScope"));
}

void addScope(String name) {
    globalScope.pushBase(Scope(name));
}

void removeScope() {
    globalScope.popBase();
}
bool variableExists(String name) {
    for (auto i : globalScope) {
        if (i.getVariable(name)) {
            return true;
        }
    }
    return false;
}
bool functionExists(String name) {
    for (auto i : globalScope) {
        if (i.getFunctionVariable(name)) {
            return true;
        }
    }
    return false;
}
std::shared_ptr<Variable> getFunctionVariable(String name, std::size_t line) {
    for (auto i : globalScope) {
        auto var = i.getFunctionVariable(name);
        if (var) {
            return var;
        }
    }
    if (variableExists(name)) {
        throwError("Variable " + name.getReference() + " is not a function", line);
    } else {
        throwError("Could not find function " + name.getReference(), line);
    }
    return nullptr;
}
std::shared_ptr<Variable> getVariable(String name, std::size_t line) {
    for (auto i : globalScope) {
        auto var = i.getVariable(name);
        if (var) {
            return var;
        }
    }
    // Need this because first class functions
    for (auto i : globalScope) {
        auto var = i.getFunctionVariable(name);
        if (var) {
            return var;
        }
    }
    throwError("Could not find variable " + name.getReference(), line);
    return nullptr;
}

Scope::Scope(String scopeName) {
    name = scopeName;
    variables = std::unordered_map<String, std::shared_ptr<Variable>, StringHash>();
}

std::shared_ptr<Variable> Scope::getVariable(String name) {
    if (variables.find(name) != variables.end()) {
        return variables[name];
    }
    return nullptr;
}
std::shared_ptr<Variable> Scope::getFunctionVariable(String name) {
    if (functionVariables.find(name) != functionVariables.end()) {
        return functionVariables[name];
    }
    return nullptr;
}

void Scope::addVariable(String name, std::shared_ptr<Variable> variable, std::size_t line) {
    if (variables.find(name) != variables.end()) {
        throwError("Variable " + variable->getName().getReference() + " already exists in the current scope", line);
    }
    if (functionVariables.find(name) != functionVariables.end()) {
        throwError("Attempting to override function " + variable->getName().getReference() + " as a variable", line);
    }
    variables[name] = variable;
}
void Scope::addFunctionVariable(String name, std::shared_ptr<Variable> variable, std::size_t line) {
    if (functionVariables.find(name) != functionVariables.end()) {
        throwError("Function " + variable->getName().getReference() + " already exists in the current scope", line);
    }
    if (variables.find(name) != variables.end()) {
        throwError("Attempting to override variable " + variable->getName().getReference() + " as a function", line);
    }
    functionVariables[name] = variable;
}