export module AST;
import Error;
import <string>;
import <memory>;
import <vector>;
import <map>;
import <iostream>;
import <set>;
import String;
import Variable;
import Scope;
using std::operator "" s;
std::set<String> identifiers = {
	"char",
	"int",
	"float",
	"bool",
	"var"
};
std::set<String> identifierModifiers = {
	"const",
	"static",
	"global"
};
std::set<String> controlFlow = {
	"if",
	"else",
	"while",
	"for",
	/*"switch",
	"case",
	"default",*/
	"break",
	"continue",
	"return"
};
std::set<String> operators = {
	"=",
	"+",
	"-",
	"*",
	"==",
	">"
};
std::set<char> operatorFirstCharacters = {};
std::map<String, int> operatorPrecedence = {
	{"*", 5},
	{"+", 6},
	{"-", 6},
	{">", 9},
	{"==", 10},
	{"=", 16}
};
std::map<char, char> escapeCharacters = {
	{'n', '\n'},
	{'t', '\t'},
	{'r', '\r'},
	{'b', '\b'},
	{'f', '\f'},
	{'v', '\v'},
	{'a', '\a'},
	{'\\', '\\'},
	{'\'', '\''},
	{'\"', '\"'},
	{'?', '\?'},
	{'0', '\0'}
};
int i = 0;
uint32_t indentationLevel = 0;
std::string file = "";
std::size_t currentLine = 0;
bool incI() {
	if (i >= file.length()) return false;
	i++;
	return true;
}
std::shared_ptr<MemorySlot> evalOperatorEql(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue) {
	if (leftValue->getMemType() != MemorySlot::Type::Variable) {
		throwError("Assignment operator must be called on a variable", 0);
	}
	Variable* v = (Variable*)leftValue.get();
	if (rightValue->getMemType() == MemorySlot::Type::Value) {
		Value* val = (Value*)rightValue.get();
		if (v->getTypeName() == "float"s) {
			if (val->getType() == Value::Types::Double) {
				v->setValue(rightValue);
			}
			else {
				v->setValue(std::make_shared<Value>(val->getAsFloat()));
			}
		}
		else if (v->getTypeName() == "int"s) {
			if (val->getType() == Value::Types::Int) {
				v->setValue(rightValue);
			}
			else {
				v->setValue(std::make_shared<Value>(val->getAsInt()));
			}
		}
		else if (v->getTypeName() == "char"s) {
			if (val->getType() == Value::Types::Char) {
				v->setValue(rightValue);
			}
			else {
				v->setValue(std::make_shared<Value>(val->getAsChar()));
			}
		}
		else if (v->getTypeName() == "bool"s) {
			if (val->getType() == Value::Types::Bool) {
				v->setValue(rightValue);
			}
			else {
				v->setValue(std::make_shared<Value>(val->getAsBool()));
			}
		}
	}
	else {
		v->setValue(rightValue);
	}
	return leftValue;
}
std::shared_ptr<MemorySlot> evalOperatorPls(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue) {
	Value* left = nullptr;
	Value* right = nullptr;
	//Get left as a value
	if (leftValue->getMemType() == MemorySlot::Type::Value) {
		left = (Value*)leftValue.get();
	}
	else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
		left = (Value*)((Variable*)leftValue.get())->getValue().get();
	}
	else {
		throwError("Cannot add non-value types", 0);
	}
	//Get right as a value
	if (rightValue->getMemType() == MemorySlot::Type::Value) {
		right = (Value*)rightValue.get();
	}
	else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
		right = (Value*)((Variable*)rightValue.get())->getValue().get();
	}
	else {
		throwError("Cannot add non-value types", 0);
	}
	//If one is a double, cast both to double
	if (left->getType() == Value::Types::Double || right->getType() == Value::Types::Double) {
		double leftValue = left->getAsFloat();
		double rightValue = right->getAsFloat();
		return std::make_shared<Value>(leftValue + rightValue);
	}
	//If one is an int, cast both to int
	else if (left->getType() == Value::Types::Int || right->getType() == Value::Types::Int) {
		int leftValue = left->getAsInt();
		int rightValue = right->getAsInt();
		return std::make_shared<Value>(leftValue + rightValue);
	}
	//If one is a char, cast both to char
	else if (left->getType() == Value::Types::Char || right->getType() == Value::Types::Char) {
		char leftValue = left->getAsChar();
		char rightValue = right->getAsChar();
		return std::make_shared<Value>(leftValue + rightValue);
	}
	//If they are bools, cast them to ints
	else {
		int leftValue = left->getAsBool();
		int rightValue = right->getAsBool();
		return std::make_shared<Value>(leftValue + rightValue);
	}
}
std::shared_ptr<MemorySlot> evalOperatorMns(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue) {
	Value* left = nullptr;
	Value* right = nullptr;
	//Get left as a value
	if (leftValue->getMemType() == MemorySlot::Type::Value) {
		left = (Value*)leftValue.get();
	}
	else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
		left = (Value*)((Variable*)leftValue.get())->getValue().get();
	}
	else {
		throwError("Cannot subtract non-value types", 0);
	}
	//Get right as a value
	if (rightValue->getMemType() == MemorySlot::Type::Value) {
		right = (Value*)rightValue.get();
	}
	else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
		right = (Value*)((Variable*)rightValue.get())->getValue().get();
	}
	else {
		throwError("Cannot subtract non-value types", 0);
	}
	//If one is a double, cast both to double
	if (left->getType() == Value::Types::Double || right->getType() == Value::Types::Double) {
		double leftValue = left->getAsFloat();
		double rightValue = right->getAsFloat();
		return std::make_shared<Value>(leftValue - rightValue);
	}
	//If one is an int, cast both to int
	else if (left->getType() == Value::Types::Int || right->getType() == Value::Types::Int) {
		int leftValue = left->getAsInt();
		int rightValue = right->getAsInt();
		return std::make_shared<Value>(leftValue - rightValue);
	}
	//If one is a char, cast both to char
	else if (left->getType() == Value::Types::Char || right->getType() == Value::Types::Char) {
		char leftValue = left->getAsChar();
		char rightValue = right->getAsChar();
		return std::make_shared<Value>(leftValue - rightValue);
	}
	//If they are bools, cast them to ints
	else {
		int leftValue = left->getAsBool();
		int rightValue = right->getAsBool();
		return std::make_shared<Value>(leftValue - rightValue);
	}
}
std::shared_ptr<MemorySlot> evalOperatorMult(std::shared_ptr<MemorySlot> leftValue, std::shared_ptr<MemorySlot> rightValue) {
	Value* left = nullptr;
	Value* right = nullptr;
	//Get left as a value
	if (leftValue->getMemType() == MemorySlot::Type::Value) {
		left = (Value*)leftValue.get();
	}
	else if (leftValue->getMemType() == MemorySlot::Type::Variable) {
		left = (Value*)((Variable*)leftValue.get())->getValue().get();
	}
	else {
		throwError("Cannot multiply non-value types", 0);
	}
	//Get right as a value
	if (rightValue->getMemType() == MemorySlot::Type::Value) {
		right = (Value*)rightValue.get();
	}
	else if (rightValue->getMemType() == MemorySlot::Type::Variable) {
		right = (Value*)((Variable*)rightValue.get())->getValue().get();
	}
	else {
		throwError("Cannot multiply non-value types", 0);
	}
	//If one is a double, cast both to double
	if (left->getType() == Value::Types::Double || right->getType() == Value::Types::Double) {
		double leftValue = left->getAsFloat();
		double rightValue = right->getAsFloat();
		return std::make_shared<Value>(leftValue * rightValue);
	}
	//If one is an int, cast both to int
	else if (left->getType() == Value::Types::Int || right->getType() == Value::Types::Int) {
		int leftValue = left->getAsInt();
		int rightValue = right->getAsInt();
		return std::make_shared<Value>(leftValue * rightValue);
	}
	//If one is a char, cast both to char
	else if (left->getType() == Value::Types::Char || right->getType() == Value::Types::Char) {
		char leftValue = left->getAsChar();
		char rightValue = right->getAsChar();
		return std::make_shared<Value>(leftValue * rightValue);
	}
	//If they are bools, cast them to ints
	else {
		int leftValue = left->getAsBool();
		int rightValue = right->getAsBool();
		return std::make_shared<Value>(leftValue * rightValue);
	}
}
export namespace AST {
	export int getPrecedence(String op) {
		if (operatorPrecedence.find(op) == operatorPrecedence.end()) {
			return -1;
		}
		return operatorPrecedence[op];
	}
	export bool isIdentifier(String str) {
		return identifiers.find(str) != identifiers.end();
	}
	export bool isIdentifierModifier(String str) {
		return identifierModifiers.find(str) != identifierModifiers.end();
	}
	export bool isControlFlow(String str) {
		return controlFlow.find(str) != controlFlow.end();
	}
	export bool isOperator(String str) {
		return operators.find(str) != operators.end();
	}
	export bool isBooleanLiteral(String str) {
		return str.getReference() == "true" || str.getReference() == "false";
	}
	export enum class Type : int {
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
	bool operator==(const Type& value, char c) {
		return value == (Type)c;
	}
	std::size_t getLine();
	class Token {
	public:
		Type type;
		String str;
		int originLine;
		Token(Type t, String s) : originLine(getLine()), type(t), str(s) {

		}
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
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Value>(value);
		}
	};
	class IntAST : public AST {
	public:
		int value;
		IntAST(int value) : value(value) {}
		IntAST(String value) : value(std::stoi(value.getReference())) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Value>(value);
		}
	};
	class BoolAST : public AST {
	public:
		bool value;
		BoolAST(bool value) : value(value) {}
		BoolAST(String value) : value(value.getReference() == "true") {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Value>(value);
		}
	};
	class StringAST : public AST {
	public:
		String value;
		StringAST(String value) : value(value) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Undefined>();
		}
	};
	class CharacterAST : public AST {
	public:
		char value;
		CharacterAST(char value) : value(value) {}
		CharacterAST(String value) : value(value.getReference()[0]) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Value>(value);
		}
	};
	class BinaryOperatorAST : public AST {
	public:
		std::shared_ptr<AST> left;
		std::shared_ptr<AST> right;
		String op;
		BinaryOperatorAST(std::shared_ptr<AST> left, std::shared_ptr<AST> right, String op) : left(left), right(right), op(op) {}
		std::shared_ptr<MemorySlot> getValue() override {
			auto leftValue = left->getValue();
			auto rightValue = right->getValue();
			//Somehow invoke the operator
			if (op == "="s) {
				return evalOperatorEql(leftValue, rightValue);
			}
			else if (op == "+"s) {
				return evalOperatorPls(leftValue, rightValue);
			}
			else if (op == "-"s) {
				return evalOperatorMns(leftValue, rightValue);
			}
			else if (op == "*"s) {
				return evalOperatorMult(leftValue, rightValue);
			}
			return std::make_shared<Undefined>();
		}
	};
	class UnaryOperatorAST : public AST {
	public:
		std::shared_ptr<AST> value;
		String op;
		UnaryOperatorAST(std::shared_ptr<AST> value, String op) : value(value), op(op) {}
		std::shared_ptr<MemorySlot> getValue() override {
			auto opValue = value->getValue();
			//Somehow invoke the operator
			return std::make_shared<Undefined>();
		}
	};
	class VariableAST : public AST {
	public:
		String name;
		VariableAST(String name) : name(name) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return getVariable(name);
		}
	};
	class VariableDeclarationAST : public AST {
	public:
		std::vector<String> modifiers;
		String type;
		String name;
		VariableDeclarationAST(std::vector<String> types, String type, String name) : modifiers(types), type(type), name(name) {}
		std::shared_ptr<MemorySlot> getValue() override {
			auto var = std::make_shared<Variable>(name, type);
			addVariable(var);
			return var;
		}
	};

	class FunctionAST : public AST {
	public:
		std::shared_ptr<AST> returnType;
		std::vector<std::shared_ptr<AST>> arguments;
		std::vector<std::shared_ptr<AST>> statements;
		String name;
		FunctionAST(std::shared_ptr<AST> returnType, std::vector<std::shared_ptr<AST>> arguments, std::vector<std::shared_ptr<AST>> statements, String name) : returnType(returnType), arguments(arguments), statements(statements), name(name) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Undefined>();
		}
	};
	class FunctionCallAST : public AST {
	public:
		std::shared_ptr<AST> function;
		std::vector<std::shared_ptr<AST>> arguments;
		FunctionCallAST(std::shared_ptr<AST> function, std::vector<std::shared_ptr<AST>> arguments) : function(function), arguments(arguments) {}
		std::shared_ptr<MemorySlot> getValue() override {
			return std::make_shared<Undefined>();
		}
	};

	Token currentToken = Token((Type)0, String(""));

	//A function that returns currentToken
	export Token getCurrentToken() {
		return currentToken;
	}
	export uint32_t getIndentationLevel() {
		return indentationLevel;
	}
	export void prepareInterpreter(std::string f) {
		file = f;
		i = 0;
		currentLine = 1;
		operatorFirstCharacters.clear();
		for (auto& op : operators) {
			//If the operator is not letter based, then add the first character to the set
			if (op[0] >= 'a' && op[0] <= 'z' || op[0] >= 'A' && op[0] <= 'Z') {
				continue;
			}
			operatorFirstCharacters.insert(op[0]);
		}
	}
	export size_t getLine() {
		return currentLine;
	}
	export Type getNextToken() {
		//EOF
		if (i >= file.length()) {
			currentToken = Token(Type::EndOfFile, String(""));
			return Type::EndOfFile;
		}
		if (file[i] == ' ' || file[i] == '\t') {
			//Safe increasing of i, because if we overshoot, next executed command would be EOF
			i++;
			return getNextToken();
		}
		//If it's a newline, return end of statement
		if (file[i] == '\n') {
			currentLine++;
			//Safe inc
			i++;
			currentToken = Token(Type::EndOfStatement, String("\n"));
			return Type::EndOfStatement;
		}

		//If it's a number, return number
		if (file[i] >= '0' && file[i] <= '9') {
			std::string number = "";
			unsigned int decimalCount = 0;
			do {
				number += file[i];
				if (file[i] == '.') {
					decimalCount++;
				}
				if (!incI()) {
					break;
				}
			} while ((file[i] >= '0' && file[i] <= '9') || file[i] == '.');
			if (decimalCount > 1) {
				throwError("Invalid number: " + number, getLine());
			}
			if (decimalCount == 1) {
				currentToken = Token(Type::FloatLiteral, String(number));
				return Type::FloatLiteral;
			}
			currentToken = Token(Type::IntLiteral, String(number));
			return Type::IntLiteral;
		}

		//If it's a letter, return identifier
		if ((file[i] >= 'a' && file[i] <= 'z') || (file[i] >= 'A' && file[i] <= 'Z') || file[i] == '_') {
			std::string identifier = "";
			do {
				identifier += file[i];
				if (!incI()) {
					break;
				}
			} while ((file[i] >= 'a' && file[i] <= 'z') || (file[i] >= 'A' && file[i] <= 'Z') || (file[i] >= '0' && file[i] <= '9') || file[i] == '_');
			if (isIdentifier(identifier)) {
				currentToken = Token(Type::Identifier, identifier);
				return Type::Identifier;
			}
			else if (isIdentifierModifier(identifier)) {
				currentToken = Token(Type::IdentifierModifier, identifier);
				return Type::IdentifierModifier;
			}
			else if (isOperator(identifier)) {
				currentToken = Token(Type::Operator, identifier);
				return Type::Operator;
			}
			else if (isControlFlow(identifier)) {
				currentToken = Token(Type::ControlFlow, identifier);
				return Type::ControlFlow;
			}
			else if (isBooleanLiteral(identifier)) {
				currentToken = Token(Type::BooleanLiteral, identifier);
				return Type::BooleanLiteral;
			}

			currentToken = Token(Type::Name, identifier);
			return Type::Name;
		}
		//Comments must be handled before operators, because they can start with the same character
		//Skip comments
		if (file[i] == '/' && file[i + 1] == '/') {
			while (file[i] != '\n') {
				if (!incI()) {
					break;
				}
			}
			currentLine++;
			//Don't increase i, so that getNextToken() returns EOF or EndOfStatement
			return getNextToken();
		}
		//Skip multiline comments
		if (file[i] == '/' && i < file.size() - 1 && file[i + 1] == '*') {
			i += 2;
			while (file[i] != '*' && file[i + 1] != '/') {
				if (file[i] == '\n') {
					currentLine++;
				}
				if (!incI() || i >= file.size() - 1) {
					throwError("Unclosed multiline comment", getLine());
				}
			}
			i += 2;
			return getNextToken();
		}
		//If it's a potential operator
		if (operatorFirstCharacters.find(file[i]) != operatorFirstCharacters.end()) {
			std::string op = "";
			int count = 0;
			do {
				op += file[i];
				//Have to increment i before checking count, otherwise we could get count forcing a break
				//	before i is incremented
				bool inc = incI();
				count++;
				if (count > 3) {
					break;
				}
				if (!inc) {
					break;
				}
			} while (!isOperator(op));
			if (isOperator(op)) {
				currentToken = Token(Type::Operator, op);
				return Type::Operator;
			}
			//else {
			//	i--;
			//	//Remove the last char from op
			//	op = op.substr(0, op.size() - 1);
			//}
			//if (isOperator(op)) {
			//	currentToken = Token(Type::Operator, op);
			//	return Type::Operator;
			//}
			throwError("Invalid operator: " + op, getLine());
		}

		//Handle string literals
		if (file[i] == '"') {
			if (!incI()) {
				throwError("Unclosed string literal at end of file: \"", getLine());
			}
			std::string str = "";
			std::string actualStr = "";
			char lastChar = '\0';
			while (file[i] != '"') {
				actualStr += file[i];
				if (lastChar != '\\') {
					str += file[i];
					lastChar = file[i];
				}
				else {
					//Pop off last string character
					str = str.substr(0, str.size() - 1);
					//Add the escaped character
					str += escapeCharacters[file[i]];
				}
				if (file[i] == '\n') {
					throwError("Unclosed string literal (newlines can't be in strings)", getLine());
					currentLine++;
				}
				if (!incI()) {
					throwError("Unclosed string literal at end of file: \"" + actualStr, getLine());
				}
			}
			//Skip past the "
			i++;
			currentToken = Token(Type::StringLiteral, str);
			return Type::StringLiteral;
		}

		//Handle character literals
		if (file[i] == '\'') {
			if (!incI()) {
				throwError("Unclosed character literal", getLine());
			}
			std::string str = "";
			int count = 0;
			char lastChar = ' ';
			while (file[i] != '\'') {
				if (file[i] == '\n') {
					throwError("Unclosed character literal (newlines can't be in character literals)", getLine());
					currentLine++;
				}
				str += file[i];
				lastChar = file[i];
				count++;
				if (!incI()) {
					throwError("Unclosed character literal", getLine());
				}
			}
			//If the count is greater than 1, then it's an invalid character literal, unless it's escape
			if (count > 1 && str[0] != '\\') {
				throwError("Invalid character literal: '" + str + "'", getLine());
			}
			//If the count is greater than 2, then it's an invalid character literal
			else if (count > 2) {
				throwError("Invalid character literal: '" + str + "'", getLine());
			}
			if (lastChar == '\\') {
				if (i >= file.size() - 1) {
					//Since we're here, we can't check if there is another ', thus there isnt one
					throwError("Unclosed character literal: '" + str + "'", getLine());
				}
				else if (file[i + 1] == '\'') {
					//Eat the first of the two ', the second one will be done later
					str += file[i];
					i++;
				}
				else {
					throwError("Unclosed character literal: '" + str + "'", getLine());
				}
			}
			if (str[0] == '\\') {
				str = escapeCharacters[str[1]];
			}
			//Skip past the '
			i++;
			currentToken = Token(Type::CharacterLiteral, str);
			return Type::CharacterLiteral;
		}
		char c = file[i];
		i++;
		currentToken = Token((Type)c, String(c));
		return (Type)c;
	}

}