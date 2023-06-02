#ifndef VARIABLE_H
#define VARIABLE_H

#include "String.h"
#include <memory>
class MemorySlot {
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
class Undefined : public MemorySlot {
public:
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
		Val(){
			
		}
	public:
		Val(double f);
		Val(int i);
		Val(char c);
		Val(bool b);
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
	Value(double d);
	Value(int f);
	Value(char c);
	Value(bool b);
	Types getType();
	virtual Type getMemType();
	Val& getValue();
	template<class T>
	T getAs();
	double getAsFloat();
	int getAsInt();
	char getAsChar();
	bool getAsBool();
	virtual String getTypeName();
};
class Variable : public MemorySlot {
	std::shared_ptr<MemorySlot> value;
	String name;
	String type;
public:
	Variable(String name, String type);
	void setValue(std::shared_ptr<MemorySlot> v);
	String getName();
	virtual String getTypeName();
	virtual Type getMemType();
	std::shared_ptr<MemorySlot> getValue();
};

#endif // VARIABLE_H
