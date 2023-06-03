#include "Variable.h"
MemorySlot::Type Undefined::getMemType() {
    return Type::Undefined;
}

String Undefined::getTypeName() {
    return "undefined";
}

Value::Val::Val(double f) : floatVal(f) {}

Value::Val::Val(int i) : intVal(i) {}

Value::Val::Val(char c) : charVal(c) {}

Value::Val::Val(bool b) : boolVal(b) {}

Value::Value(double d) {
    value = Val(d);
    type = Types::Double;
    typeName = "float";
}

Value::Value(int f) {
    value = Val(f);
    type = Types::Int;
    typeName = "int";
}

Value::Value(char c) {
    value = Val(c);
    type = Types::Char;
    typeName = "char";
}

Value::Value(bool b) {
    value = Val(b);
    type = Types::Bool;
    typeName = "bool";
}

Value::Types Value::getType() {
    return type;
}

MemorySlot::Type Value::getMemType() {
    return Type::Value;
}

Value::Val& Value::getValue() {
    return value;
}

template <class T>
T Value::getAs() {
    switch (type) {
    case Value::Types::Double:
        return (T)value.floatVal;
        break;
    case Value::Types::Int:
        return (T)value.intVal;
        break;
    case Value::Types::Char:
        return (T)value.charVal;
        break;
    case Value::Types::Bool:
        return (T)value.boolVal;
        break;
    default:
        break;
    }
}

double Value::getAsFloat() {
    return getAs<double>();
}

int Value::getAsInt() {
    return getAs<int>();
}

char Value::getAsChar() {
    return getAs<char>();
}

bool Value::getAsBool() {
    return getAs<bool>();
}

String Value::getTypeName() {
    return typeName;
}

Variable::Variable(String name, String type) {
    this->name = name;
    this->type = type;
}

void Variable::setValue(std::shared_ptr<MemorySlot> v) {
    value = v;
}

String Variable::getName() {
    return name;
}

String Variable::getTypeName() {
    return type;
}

MemorySlot::Type Variable::getMemType() {
    return Type::Variable;
}

// getValue
std::shared_ptr<MemorySlot> Variable::getValue() {
    return value;
}
