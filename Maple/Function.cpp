#include "AST.h"
#include "Interpret.h"
#include "Variable.h"
#include <memory>
#include <string>

Function::Function(String name, AST::FunctionAST* function)
  : MemorySlot(), name(name), type(function->getType()),
	arguments(std::move(function->arguments)),
	statements(std::move(function->statements)),
	returnType(function->returnType), declLine(function->getOriginLine()) {}

String Function::getTypeName() {
	return type;
}

Function::Type Function::getMemType() {
	return Type::Function;
}
std::shared_ptr<MemorySlot> Function::call(
	std::vector<std::shared_ptr<MemorySlot>> args, std::size_t line) {

	if (args.size() != arguments.size()) {
		throwError("Invalid number of arguments in call to function "s +
					   name.getReference() + "\n  note: expected "s +
					   std::to_string(arguments.size()) + " arguments, got "s +
					   std::to_string(args.size()) +
					   "\n  note: function declared at line "s +
					   std::to_string(this->declLine),
			line);
	}
	// Preprocess the arguments before new scope is added
	std::vector<std::unique_ptr<AST::MemSlotAST>> argASTs = {};
	for (size_t i = 0; i < args.size(); i++) {
		argASTs.push_back(std::make_unique<AST::MemSlotAST>(args[i]));
	}
	addScope(name);
	for (size_t i = 0; i < argASTs.size(); i++) {
		auto decl = arguments[i]->getValue();

		auto equals = std::make_shared<AST::BinaryOperatorAST>(
			std::make_unique<AST::MemSlotAST>(decl), std::move(argASTs[i]),
			"="s, line);
		equals->getValue();
	}
	interpret(statements);
	std::shared_ptr<MemorySlot> ret = nullptr;
	if (getExitType() == ExitType::Return) {
		auto reg = handleReturnRegister();
		ret = reg.second;
		auto line = reg.line;
		if (ret->getTypeName() != returnType) {
			throwError(
				"Invalid return type in function "s + name.getReference() +
					"\n  note: expected "s + returnType.getReference() +
					", got "s + ret->getTypeName().getReference() +
					"\n  note: return called at line "s + std::to_string(line),
				line);
		}
	} else if (getExitType() != ExitType::None) {
		throwError("Invalid exit type in function "s + name.getReference() +
					   "  note: only valid type is 'return'",
			line);
	}
	removeScope();
	if (ret == nullptr && returnType != "void"s) {
		throwError(
			"Missing return statement in function "s + name.getReference(),
			line);
	}
	return ret;
}
BuiltinFunction::BuiltinFunction(String name,
	BuiltinFunction::FunctionType func, size_t argCount, String returnType,
	std::vector<String> argTypes)
  : MemorySlot(), name(name), function(func), argTypes(argTypes),
	argCount(argCount), returnType(returnType) {
	std::string t = returnType.getReference() + "(";
	for (std::size_t i = 0; i < argTypes.size(); i++) {
		t += argTypes[i].getReference();
		if (i != argTypes.size() - 1) {
			t += ",";
		}
	}
	t += ")";

	type = t;
}

String BuiltinFunction::getTypeName() {
	return type;
}

BuiltinFunction::Type BuiltinFunction::getMemType() {
	return Type::BuiltinFunction;
}

std::shared_ptr<MemorySlot> BuiltinFunction::call(
	std::vector<std::shared_ptr<MemorySlot>> args, std::size_t line) {
	// Check if the number of arguments is correct
	if (args.size() != argCount) {
		throwError("Incorrect number of arguments for builtin function '" +
					   name.getReference() + "'\n  note: expected " +
					   std::to_string(args.size()) + " arguments but got " +
					   std::to_string(argCount),
			line);
	}
	int i = 0;
	// Assert the types are correct too
	for (auto& arg : args) {

		if (!arg) {
			throwError(
				"Atttempting to use a void return value as an argument "
				"for builtin function '" +
					name.getReference() +
					"'\n  note: void value passed as parameter number " +
					std::to_string(i + 1),
				line);
		}
		if (arg->getMemType() == MemorySlot::Type::Variable) {
			// unpack the variable
			arg = ((Variable*)arg.get())->getValue();
		}
		if (arg->getTypeName() != argTypes[i]) {
			throwError("Incorrect type for argument " + std::to_string(i + 1) +
						   " for builtin function '" + name.getReference() +
						   "'\n  note: expected " + argTypes[i].getReference() +
						   " but got " + arg->getTypeName().getReference(),
				line);
		}
		i++;
	}
	// Call the function
	std::shared_ptr<MemorySlot> ret = function(args);
	// Assert the return type is correct
	if (ret->getTypeName() != returnType) {
		throwError("Incorrect return type for builtin function '" +
					   name.getReference() + "'\n  note: expected " +
					   returnType.getReference() + " but got " +
					   ret->getTypeName().getReference() +
					   "\n  note: this is an internal library error, please "
					   "report it to the developer of the library",
			line);
	}
	return ret;
}
