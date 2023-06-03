#ifndef SCOPE_H
#define SCOPE_H

#include "Error.h"
#include "LinkedList.h"
#include "String.h"
#include "Variable.h"
#include <stack>
#include <unordered_map>

class Scope {
    std::unordered_map<String, std::shared_ptr<Variable>, StringHash> variables;
    String name;

public:
    Scope(String scopeName);
    std::shared_ptr<Variable> getVariable(String name);
    void addVariable(String name, std::shared_ptr<Variable> variable, std::size_t line);
};
extern std::LinkedList<Scope> globalScope;
void addVariable(std::shared_ptr<Variable> v, std::size_t line);
void initScope();
void addScope(String name);
void removeScope();
std::shared_ptr<Variable> getVariable(String name);

#endif // SCOPE_H
