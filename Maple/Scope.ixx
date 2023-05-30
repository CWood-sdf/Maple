export module Scope;
import <stack>;
import <map>;
import <memory>;
import String;
import LinkedList;
import Variable;
import Error;
class Scope {
	std::map<String, std::shared_ptr<Variable>> variables = {};
	String name;
public:
	Scope(String scopeName) {
		name = scopeName;
	}
	std::shared_ptr<Variable> getVariable(String name) {
		if (variables.find(name) != variables.end()) {
			return variables[name];
		}
		return nullptr;
	}
	void addVariable(String name, std::shared_ptr<Variable> variable) {
		if (variables.find(name) != variables.end()) {
			throwError("Variable " + variable->getName().getReference() +" already exists in the current scope", 0);
		}
		variables[name] = variable;
	}
};
std::LinkedList<Scope> globalScope = {};
export void addVariable(std::shared_ptr<Variable> v) {
	String name = v->getName();
	globalScope.getBase()->addVariable(name, v);
}
export void initScope() {
	globalScope.pushBase(Scope("$_globalScope"));
}
export void addScope(String name) {
	globalScope.pushBase(Scope(name));
}
export void removeScope() {
	globalScope.popBase();
}
export std::shared_ptr<Variable> getVariable(String name) {
	for (auto i : globalScope) {
		auto var = i.getVariable(name);
		if (var) {
			return var;
		}
	}
	throwError("Could not find variable " + name.getReference(), 0);
	return nullptr;
}