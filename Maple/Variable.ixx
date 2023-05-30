export module Variable;
import <memory>;
import String;
export class MemorySlot {
public:
	MemorySlot() {}
	enum class Type {
		Value,
		Variable,
		Undefined
	};
	virtual Type getMemType() = 0;
	virtual String getTypeName() = 0;
};
export class Undefined : public MemorySlot {
public:
	Undefined() {}
	Type getMemType() override {
		return Type::Undefined;
	}
	String getTypeName() override {
		return "undefined";
	}
};
export class Value : public MemorySlot {
	union Val {
		friend class Value;
		double floatVal;
		int intVal;
		char charVal;
		bool boolVal;
		Val(){
			
		}
	public:
		Val(double f) : floatVal(f) {}
		Val(int i) : intVal(i) { }
		Val(char c) : charVal(c) {}
		Val(bool b) : boolVal(b) {}
	};
public:
	enum class Types {
		Double,
		Int,
		Char,
		Bool
	};
private:
	Val value;
	Types type;
	String typeName;
public:
	Value(double d) {
		value = Val(d);
		type = Types::Double;
		typeName = "float";
	}
	Value(int f) {
		value = Val(f);
		type = Types::Int;
		typeName = "int";
	}
	Value(char c) {
		value = Val(c);
		type = Types::Char;
		typeName = "char";
	}
	Value(bool b) {
		value = Val(b);
		type = Types::Bool;
		typeName = "bool";
	}
	Types getType() {
		return type;
	}
	virtual Type getMemType() {
		return Type::Value;
	}
	Val& getValue() {
		return value;
	}
	template<class T>
	T getAs() {
		switch (type)
		{
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
	double getAsFloat() {
		return getAs<double>();
	}
	int getAsInt() {
		return getAs<int>();
	}
	char getAsChar() {
		return getAs<char>();
	}
	bool getAsBool() {
		return getAs<bool>();
	}
	virtual String getTypeName() {
		return typeName;
	}
};
export class Variable : public MemorySlot {
	std::shared_ptr<MemorySlot> value;
	String name;
	String type;
public:
	Variable(String name, String type) {
		this->name = name;
		this->type = type;
	}
	void setValue(std::shared_ptr<MemorySlot> v) {
		value = v;
	}
	String getName() {
		return name;
	}
	virtual String getTypeName() {
		return type;
	}
	virtual Type getMemType() {
		return Type::Variable;
	}
	//getValue
	std::shared_ptr<MemorySlot> getValue() {
		return value;
	}
};