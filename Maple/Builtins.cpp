#include "Builtins.h"
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
void makeBuiltin(auto fn, String ret, String name, std::vector<String> args) {
    auto builtinFn =
        std::make_shared<BuiltinFunction>(name, fn, args.size(), ret, args);
    auto builtinVar =
        std::make_shared<Variable>(name, builtinFn->getTypeName());
    builtinVar->setValue(builtinFn);
    addFunction(builtinVar, 0);
}
void addBuiltins() {

    auto builtinCosFn = std::make_shared<BuiltinFunction>(
        "cos", builtinCos, 1, "float", std::vector<String>({"float"}));
    auto builtinCosVar =
        std::make_shared<Variable>("cos", builtinCosFn->getTypeName());
    builtinCosVar->setValue(builtinCosFn);
    addFunction(builtinCosVar, 0);
}
