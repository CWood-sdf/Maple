#ifndef AST_H
#define AST_H

#include "Variable.h"
#include "Error.h"
#include "String.h"
#include "Scope.h"
#include <set>
#include <string>
using std::operator "" s;
extern std::set<String> identifiers;
extern std::set<String> identifierModifiers;
extern std::set<String> controlFlow;
extern std::set<String> operators;
extern std::set<char> operatorFirstCharacters;
extern std::map<String, int> operatorPrecedence;
extern std::map<char, char> escapeCharacters;
extern int i;
extern uint32_t indentationLevel;
extern std::string file;
extern std::size_t currentLine;
bool incI();
std::shared_ptr<MemorySlot> evalOperatorEql(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot> evalOperatorPls(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot> evalOperatorMns(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot> evalOperatorMult(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue);
namespace AST {
	int getPrecedence(String op);
	bool isIdentifier(String str);
	bool isIdentifierModifier(String str);
	bool isControlFlow(String str);
	bool isOperator(String str);
	bool isBooleanLiteral(String str);
	enum class Type : int {
		EndOfFile = -1,
		FloatLiteral = -2,
		IntLiteral = -3,
		Identifier = -4,
		ClassModifier = -5,
		IdentifierModifier = -6,
		Operator = -7,
		Name = -8,
		ControlFlow = -9,
		EndOfStatement = -10,
		StringLiteral = -11,
		CharacterLiteral = -12,
		BooleanLiteral = -13
	};
	bool operator==(const Type& value, char c);
	std::size_t getLine();
	class Token {
	public:
		Type type;
		String str;
		int originLine;
		Token(Type t, String s);
		Token(const Token&) = default;
		Token& operator=(Token&&) = default;
	};
	class AST {
	public:
		virtual std::shared_ptr<MemorySlot> getValue() = 0;
	};
	class FloatAST : public AST {
	public:
		double value;
		FloatAST(double value) : value(value) {}
		FloatAST(String value) : value(std::stod(value.getReference())) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class IntAST : public AST {
	public:
		int value;
		IntAST(int value) : value(value) {}
		IntAST(String value) : value(std::stoi(value.getReference())) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class BoolAST : public AST {
	public:
		bool value;
		BoolAST(bool value) : value(value) {}
		BoolAST(String value) : value(value.getReference() == "true") {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class StringAST : public AST {
	public:
		String value;
		StringAST(String value) : value(value) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class CharacterAST : public AST {
	public:
		char value;
		CharacterAST(char value) : value(value) {}
		CharacterAST(String value) : value(value.getReference()[0]) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class BinaryOperatorAST : public AST {
	public:
		std::shared_ptr<AST> left;
		std::shared_ptr<AST> right;
		String op;
		BinaryOperatorAST(std::shared_ptr<AST> left, std::shared_ptr<AST> right, String op) : left(left), right(right), op(op) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class UnaryOperatorAST : public AST {
	public:
		std::shared_ptr<AST> value;
		String op;
		UnaryOperatorAST(std::shared_ptr<AST> value, String op) : value(value), op(op) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class VariableAST : public AST {
	public:
		String name;
		VariableAST(String name) : name(name) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class VariableDeclarationAST : public AST {
	public:
		std::vector<String> modifiers;
		String type;
		String name;
		VariableDeclarationAST(std::vector<String> types, String type, String name) : modifiers(types), type(type), name(name) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};

	class FunctionAST : public AST {
	public:
		std::shared_ptr<AST> returnType;
		std::vector<std::shared_ptr<AST>> arguments;
		std::vector<std::shared_ptr<AST>> statements;
		String name;
		FunctionAST(std::shared_ptr<AST> returnType, std::vector<std::shared_ptr<AST>> arguments, std::vector<std::shared_ptr<AST>> statements, String name) : returnType(returnType), arguments(arguments), statements(statements), name(name) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class FunctionCallAST : public AST {
	public:
		std::shared_ptr<AST> function;
		std::vector<std::shared_ptr<AST>> arguments;
		FunctionCallAST(std::shared_ptr<AST> function, std::vector<std::shared_ptr<AST>> arguments) : function(function), arguments(arguments) {}
		std::shared_ptr<MemorySlot> getValue() override;
	};

	extern Token currentToken;

	Token getCurrentToken();
	uint32_t getIndentationLevel();
	void prepareInterpreter(std::string f);
	
	Type getNextToken();

}

#endif // AST_H
