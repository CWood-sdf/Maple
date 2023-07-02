#include "AST.h"
#include "Variable.h"
#include <memory>
#include <string>

Function::Function(String name, AST::FunctionAST* function)
  : MemorySlot(), name(name), type(function->getType()),
	arguments(std::move(function->arguments)),
	statements(std::move(function->statements)) {}

String Function::getTypeName() {
	return type;
}

Function::Type Function::getMemType() {
	return Type::Function;
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
