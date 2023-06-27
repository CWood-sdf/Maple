#ifndef VARIABLE_H
#define VARIABLE_H

#include "String.h"
#include <cstdint>
#include <memory>
namespace AST {
    class ASTNode;
    class FunctionAST;
}
class MemorySlot {
public:
    MemorySlot() {}
    enum class Type {
        Value,
        Variable,
        Undefined,
        Void,
        Function,
        BuiltinFunction
    };
    virtual Type getMemType() = 0;
    virtual String getTypeName() = 0;
};
class VoidSpot : public MemorySlot {
public:
    VoidSpot() {}
    virtual ~VoidSpot() = default;
    Type getMemType() { return Type::Void; }
    String getTypeName() { return "void"; }
};
class Undefined : public MemorySlot {
public:
    virtual ~Undefined() = default;
    Undefined() {}
    Type getMemType() override;
    String getTypeName() override;
};
class Value : public MemorySlot {
    union Val {
        friend class Value;
        double floatVal;
        int intVal;
        char charVal;
        bool boolVal;
        int64_t int64Val;
        Val() {}

    public:
        Val(double f);
        Val(int i);
        Val(char c);
        Val(bool b);
        Val(int64_t i);
    };

public:
    enum class Types { Double, Int, Char, Bool, Int64 };

private:
    Val value;
    Types type;
    String typeName;

public:
    Value(double d);
    Value(int f);
    Value(char c);
    Value(bool b);
    Value(int64_t i);
    virtual ~Value() = default;
    Types getType();
    virtual Type getMemType();
    Val& getValue();
    template <class T> T getAs() {
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
        case Value::Types::Int64:
            return (T)value.int64Val;
            break;
        default:
            break;
        }
    }
    double getAsFloat();
    int getAsInt();
    char getAsChar();
    bool getAsBool();
    int64_t getAsInt64();
    virtual String getTypeName();
};
class Variable : public MemorySlot {
    std::shared_ptr<MemorySlot> value;
    String name;
    String type;

public:
    Variable(String name, String type);
    virtual ~Variable() = default;
    void setValue(std::shared_ptr<MemorySlot> v);
    String getName();
    virtual String getTypeName();
    virtual Type getMemType();
    std::shared_ptr<MemorySlot> getValue();
};
class Function : public MemorySlot {
    std::shared_ptr<AST::FunctionAST> function;
    String name;
    String type;

public:
    Function(String name, std::shared_ptr<AST::FunctionAST> function);
    virtual ~Function() = default;
    std::shared_ptr<AST::FunctionAST> getFunction();
    virtual String getTypeName();
    virtual Type getMemType();
};

class BuiltinFunction : public MemorySlot {
    String name;
    String type;
    typedef std::shared_ptr<MemorySlot> (*FunctionType)(
        std::vector<std::shared_ptr<MemorySlot>>);
    FunctionType function;
    std::vector<String> argTypes;
    size_t argCount;
    String returnType;

public:
    BuiltinFunction(String name, FunctionType function, size_t argCount,
        String returnType, std::vector<String> argTypes);

    virtual ~BuiltinFunction() = default;
    virtual String getTypeName();
    virtual Type getMemType();
    std::shared_ptr<MemorySlot> call(
        std::vector<std::shared_ptr<AST::ASTNode>> args, size_t line);
};
#endif // VARIABLE_H
