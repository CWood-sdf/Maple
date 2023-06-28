#ifndef SCOPE_H
#define SCOPE_H

#include "Error.h"
#include "LinkedList.h"
#include "String.h"
#include "Variable.h"
#include <stack>
#include <unordered_map>
enum class VariableType { Variable, Function };
enum class ExitType { Return, Break, Continue, None };
struct ReturnRegister {
	std::shared_ptr<MemorySlot> second;
	ExitType first;
	size_t line;
	ReturnRegister(
		std::shared_ptr<MemorySlot> second, ExitType first, size_t line)
	  : second(second), first(first), line(line) {}
};
class Scope {
	std::map<String, std::pair<std::shared_ptr<Variable>, VariableType>>
		variables;
	String name;
	std::shared_ptr<MemorySlot> returnRegister = nullptr;
	ExitType exitType = ExitType::None;
	size_t exitLine = 0;

public:
	Scope(String scopeName);
	Scope() = delete;
	std::shared_ptr<Variable> getVariable(String name);
	std::shared_ptr<Variable> getFunctionVariable(String name);
	std::shared_ptr<Variable> getGeneralVariable(String name);
	void setReturnRegister(std::shared_ptr<MemorySlot> reg, std::size_t line);
	std::shared_ptr<MemorySlot> getReturnRegister();
	bool isExit();
	void setExit(ExitType type);
	ExitType getExitType();
	size_t getExitCallLine();
	void addVariable(
		String name, std::shared_ptr<Variable> variable, std::size_t line);
	void addFunctionVariable(
		String name, std::shared_ptr<Variable> variable, std::size_t line);
};
extern std::LinkedList<Scope> globalScope;
void addVariable(std::shared_ptr<Variable> v, std::size_t line);
void addFunction(std::shared_ptr<Variable> v, std::size_t line);
void initScope();
void addScope(String name);
void removeScope();
bool variableExists(String name);
bool functionExists(String name);
void setReturnRegister(std::shared_ptr<MemorySlot> reg, std::size_t line);
std::shared_ptr<MemorySlot> getReturnRegister();
void setExit(ExitType type);
bool isExit();
ExitType getExitType();
size_t getExitCallLine();
ReturnRegister handleReturnRegister();
std::shared_ptr<Variable> getFunctionVariable(String name, std::size_t line);
std::shared_ptr<Variable> getVariable(String name, std::size_t line);

#endif // SCOPE_H
