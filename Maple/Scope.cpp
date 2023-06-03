#include "Scope.h"

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

std::shared_ptr<Variable> getVariable(String name) {
    for (auto i : globalScope) {
        auto var = i.getVariable(name);
        if (var) {
            return var;
        }
    }
    throwError("Could not find variable " + name.getReference(), 0);
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

void Scope::addVariable(String name, std::shared_ptr<Variable> variable, std::size_t line) {
    if (variables.find(name) != variables.end()) {
        throwError("Variable " + variable->getName().getReference() + " already exists in the current scope", line);
    }
    variables[name] = variable;
}
