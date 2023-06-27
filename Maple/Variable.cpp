#include "Variable.h"
using std::operator"" s;
MemorySlot::Type Undefined::getMemType() { return Type::Undefined; }

String Undefined::getTypeName() { return "undefined"; }

Value::Val::Val(double f) : floatVal(f) {}

Value::Val::Val(int i) : intVal(i) {}

Value::Val::Val(char c) : charVal(c) {}

Value::Val::Val(bool b) : boolVal(b) {}

Value::Val::Val(int64_t i) : int64Val(i) {}

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

Value::Value(int64_t i) {
    value = Val(i);
    type = Types::Int64;
    typeName = "int64";
}

Value::Types Value::getType() { return type; }

MemorySlot::Type Value::getMemType() { return Type::Value; }

Value::Val& Value::getValue() { return value; }

double Value::getAsFloat() { return getAs<double>(); }

int Value::getAsInt() { return getAs<int>(); }

char Value::getAsChar() { return getAs<char>(); }

bool Value::getAsBool() { return getAs<bool>(); }

int64_t Value::getAsInt64() { return getAs<int64_t>(); }

String Value::getTypeName() { return typeName; }

Variable::Variable(String name, String type) {
    this->name = name;
    this->type = type;
}

void Variable::setValue(std::shared_ptr<MemorySlot> v) { value = v; }

String Variable::getName() { return name; }

String Variable::getTypeName() {
    if (this->type == "var"s) {
        return value->getTypeName();
    } else {
        return type;
    }
}

MemorySlot::Type Variable::getMemType() { return Type::Variable; }

// getValue
std::shared_ptr<MemorySlot> Variable::getValue() { return value; }
