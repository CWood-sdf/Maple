#include "Scope.h"
#include <memory>

std::LinkedList<Scope> globalScope = {};
void addVariable(std::shared_ptr<Variable> v, std::size_t line) {
	String name = v->getName();
	globalScope.getBase()->addVariable(name, v, line);
}
void addFunction(std::shared_ptr<Variable> v, std::size_t line) {
	String name = v->getName();
	globalScope.getBase()->addFunctionVariable(name, v, line);
}
void initScope() {
	globalScope.pushBase(Scope("$_globalScope"));
}

ReturnRegister handleReturnRegister() {
	auto reg = getReturnRegister();
	auto type = getExitType();
	auto line = getExitCallLine();

	// Reset the exit type and return register so that they don't get shifted
	setExit(ExitType::None);
	setReturnRegister(nullptr, 0);
	return ReturnRegister(reg, type, line);
}

void addScope(String name) {
	globalScope.pushBase(Scope(name));
}

void removeScope() {
	auto ret = handleReturnRegister();
	globalScope.popBase();
	if (ret.first != ExitType::None) {
		if (globalScope.empty()) {
			throwError("Cannot return from global scope", 0);
		}
		setExit(ret.first);
		setReturnRegister(ret.second, ret.line);
	}
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
	throwError("Could not find function " + name.getReference(), line);
	return nullptr;
}
std::shared_ptr<Variable> getVariable(String name, std::size_t line) {
	for (auto i : globalScope) {
		auto var = i.getGeneralVariable(name);
		if (var) {
			return var;
		}
	}
	throwError("Could not find variable " + name.getReference(), line);
	return nullptr;
}
void setReturnRegister(std::shared_ptr<MemorySlot> reg, std::size_t line) {
	globalScope.getBase()->setReturnRegister(reg, line);
}
std::shared_ptr<MemorySlot> getReturnRegister() {
	return globalScope.getBase()->getReturnRegister();
}
void setExit(ExitType type) {
	globalScope.getBase()->setExit(type);
}
bool isExit() {
	return globalScope.getBase()->isExit();
}
ExitType getExitType() {
	return globalScope.getBase()->getExitType();
}
size_t getExitCallLine() {
	return globalScope.getBase()->getExitCallLine();
}
Scope::Scope(String scopeName) {
	name = scopeName;
	variables = std::unordered_map<String,
		std::pair<std::shared_ptr<Variable>, VariableType>, StringHash>();
}

std::shared_ptr<Variable> Scope::getVariable(String name) {
	auto v = variables.find(name);
	if (v != variables.end()) {
		auto var = v->second;
		if (var.second == VariableType::Variable) {
			return var.first;
		} else {
			return nullptr;
		}
	}
	return nullptr;
}
std::shared_ptr<Variable> Scope::getFunctionVariable(String name) {
	auto v = variables.find(name);
	if (v != variables.end()) {
		auto var = v->second;
		if (var.second == VariableType::Function) {
			return var.first;
		} else {
			return nullptr;
		}
	}
	return nullptr;
}
std::shared_ptr<Variable> Scope::getGeneralVariable(String name) {
	auto v = variables.find(name);
	if (v != variables.end()) {
		return v->second.first;
	}
	return nullptr;
}
void Scope::setReturnRegister(
	std::shared_ptr<MemorySlot> reg, std::size_t line) {
	returnRegister = reg;
	exitLine = line;
}
std::shared_ptr<MemorySlot> Scope::getReturnRegister() {
	return returnRegister;
}
bool Scope::isExit() {
	return exitType != ExitType::None;
}
void Scope::setExit(ExitType type) {
	exitType = type;
}
ExitType Scope::getExitType() {
	return exitType;
}
size_t Scope::getExitCallLine() {
	return exitLine;
}
void Scope::addVariable(
	String name, std::shared_ptr<Variable> variable, std::size_t line) {
	if (variables.find(name) != variables.end()) {
		throwError("Variable " + variable->getName().getReference() +
					   " already exists in the current scope",
			line);
	}
	variables[name] = std::pair(variable, VariableType::Variable);
}
void Scope::addFunctionVariable(
	String name, std::shared_ptr<Variable> variable, std::size_t line) {
	if (variables.find(name) != variables.end()) {
		throwError("Function " + variable->getName().getReference() +
					   " already exists in the current scope",
			line);
	}
	variables[name] = std::pair(variable, VariableType::Function);
}
