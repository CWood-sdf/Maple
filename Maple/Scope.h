#ifndef SCOPE_H
#define SCOPE_H

#include <stack>
#include <map>
#include "String.h"
#include "Variable.h"
#include "Error.h"
#include "LinkedList.h"
class Scope {
	std::map<String, std::shared_ptr<Variable>> variables = {};
	String name;
public:
	Scope(String scopeName);
	std::shared_ptr<Variable> getVariable(String name);
	void addVariable(String name, std::shared_ptr<Variable> variable);
};
extern std::LinkedList<Scope> globalScope;
void addVariable(std::shared_ptr<Variable> v);
void initScope();
void addScope(String name);
void removeScope();
std::shared_ptr<Variable> getVariable(String name);

#endif // SCOPE_H
