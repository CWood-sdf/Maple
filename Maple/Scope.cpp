#include "Scope.h"

std::LinkedList<Scope> globalScope = {};
void addVariable(std::shared_ptr<Variable> v) {
	String name = v->getName();
	globalScope.getBase()->addVariable(name, v);
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
}

std::shared_ptr<Variable> Scope::getVariable(String name) {
	if (variables.find(name) != variables.end()) {
		return variables[name];
	}
	return nullptr;
}

void Scope::addVariable(String name, std::shared_ptr<Variable> variable) {
	if (variables.find(name) != variables.end()) {
		throwError("Variable " + variable->getName().getReference() +" already exists in the current scope", 0);
	}
	variables[name] = variable;
}
