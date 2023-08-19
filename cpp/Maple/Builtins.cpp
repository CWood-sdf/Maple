#include "Builtins.h"
#include <chrono>
template <typename T>
T unpackValueArg(std::shared_ptr<MemorySlot> arg, int argNum, String fnName) {
	MemorySlot* argSlot = arg.get();
	if (argSlot->getMemType() != MemorySlot::Type::Value) {
		throwError("Argument " + std::to_string(argNum) + " of '" +
				   fnName.getReference() + "' must be a basic value");
	}
	Value* argVal = (Value*)argSlot;
	return argVal->getAs<T>();
}
std::shared_ptr<MemorySlot> builtinCos(
	std::vector<std::shared_ptr<MemorySlot>> args) {
	// MemorySlot* arg1Slot = args[0].get();
	// if (arg1Slot->getMemType() != MemorySlot::Type::Value) {
	//     throwError("Argument 1 of cos must be a value");
	// }
	double arg1Val = unpackValueArg<double>(args[0], 1, "cos");
	return std::make_shared<Value>(std::cos(arg1Val));
}
std::shared_ptr<MemorySlot> builtinMicro(
	std::vector<std::shared_ptr<MemorySlot>> args [[maybe_unused]]) {
	int64_t microSeconds =
		std::chrono::duration_cast<std::chrono::microseconds>(
			std::chrono::high_resolution_clock::now().time_since_epoch())
			.count();
	return std::make_shared<Value>(microSeconds);
}
std::shared_ptr<MemorySlot> builtinPrint(
	std::vector<std::shared_ptr<MemorySlot>> args) {

	MemorySlot* arg1Slot = args[0].get();
	if (arg1Slot->getMemType() != MemorySlot::Type::Value) {
		throwError("Argument 1 of print must be a value");
	}
	Value* arg1Val = (Value*)arg1Slot;
	switch (arg1Val->getType()) {
	case Value::Types::Double:
		std::cout << arg1Val->getAs<double>();
		break;
	case Value::Types::Int:
		std::cout << arg1Val->getAs<int>();
		break;
	case Value::Types::Char:
		std::cout << arg1Val->getAs<char>();
		break;
	case Value::Types::Bool:
		std::cout << arg1Val->getAs<bool>();
		break;
	case Value::Types::Int64:
		std::cout << arg1Val->getAs<int64_t>();
		break;
	}
	std::cout << std::endl;

	return nullptr;
}
void makeBuiltin(
	std::shared_ptr<MemorySlot> (*fn)(std::vector<std::shared_ptr<MemorySlot>>),
	String ret, String name, std::vector<String> args) {
	auto builtinFn =
		std::make_shared<BuiltinFunction>(name, fn, args.size(), ret, args);
	auto builtinVar =
		std::make_shared<Variable>(name, builtinFn->getTypeName());
	builtinVar->setValue(builtinFn);
	addFunction(builtinVar, 0);
}
void addBuiltins() {

	makeBuiltin(builtinCos, "float", "cos", std::vector<String>({"float"}));
	makeBuiltin(builtinMicro, "int64", "micro", std::vector<String>({}));
	makeBuiltin(builtinPrint, "void", "print", std::vector<String>({"var"}));

	// auto builtinCosFn = std::make_shared<BuiltinFunction>(
	//     "cos", builtinCos, 1, "float", std::vector<String>({"float"}));
	// auto builtinCosVar =
	//     std::make_shared<Variable>("cos", builtinCosFn->getTypeName());
	// builtinCosVar->setValue(builtinCosFn);
	// addFunction(builtinCosVar, 0);
}
