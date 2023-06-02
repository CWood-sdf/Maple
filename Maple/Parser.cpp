#include "Parser.h"
using std::operator "" s;

/// <summary>
/// A partial expression
///		::= variable
///		::= literal
/// </summary>
/// <returns></returns>
std::shared_ptr<AST::AST> AST::Parse::parsePartialExpression() {
	std::shared_ptr<AST> ret = nullptr;
	auto c = currentToken.type;
	bool wasName = false;
	switch (c) {
	case Type::CharacterLiteral:
		ret = std::make_shared<CharacterAST>(currentToken.str);
		break;
	case Type::StringLiteral:
		ret = std::make_shared<StringAST>(currentToken.str);
		break;
	case Type::IntLiteral:
		ret = std::make_shared<IntAST>(currentToken.str);
		break;
	case Type::FloatLiteral:
		ret = std::make_shared<FloatAST>(currentToken.str);
		break;
	case Type::Name:
		wasName = true;
		ret = std::make_shared<VariableAST>(currentToken.str);
		break;
	case (Type)'(':
		ret = parseParentheses();
		break;
	default:
		throwError("Unexpected token "s + currentToken.str.getReference(), currentToken.originLine);
		ret = nullptr;
		break;
	}
	getNextToken();
	if (currentToken.type == Type::Operator) {
		ret = parseBinaryOperator(getPrecedence(currentToken.str), ret);
	}
	else if (wasName && currentToken.type == (Type)'(') {
		throwError("Functions dont exist yet");
	}
	return ret;
}
std::shared_ptr<AST::AST> AST::Parse::parseParentheses() {
	//eat '('
	getNextToken();
	auto ret = parsePartialExpression();
	//eat ')'
	//getNextToken();
	return ret;
}

std::shared_ptr<AST::AST> AST::Parse::parseBinaryOperator(int precedence, std::shared_ptr<AST> left) {
	//Store the operator
	String op = currentToken.str;
	int currentPrecedence = getPrecedence(op);
	getNextToken();
	//Get the right expression   
	auto right = parsePartialExpression();
	auto tokenType = currentToken.type;
	if (tokenType == Type::Operator) {
		int nextOpPrecedence = getPrecedence(currentToken.str);
		if (nextOpPrecedence < currentPrecedence) {
			right = parseBinaryOperator(nextOpPrecedence, right);
		}
		if (nextOpPrecedence > precedence) {
			std::shared_ptr<AST> ret = std::make_shared<BinaryOperatorAST>(left, right, op);
			return parseBinaryOperator(nextOpPrecedence, ret);
		}
	}
	std::shared_ptr<AST> ret = std::make_shared<BinaryOperatorAST>(left, right, op);
	return ret;
}

std::shared_ptr<AST::AST> AST::Parse::parseDefinition() {
	std::vector<String> modifiers;
	int typeCount = 0;
	String type = "";

	do {
		if (currentToken.type == Type::Identifier) {
			typeCount++;
			type = currentToken.str;
			if (typeCount > 1) {
				throwError("Too many types given in variable declaration: " + currentToken.str.getReference(), currentToken.originLine);
			}
		}
		else if (currentToken.type != Type::IdentifierModifier) {
			throwError("Invalid token in type definition: "s + currentToken.str.getReference(), currentToken.originLine);
		}
		else {
			modifiers.push_back(currentToken.str);
		}
	} while (getNextToken() != Type::Name);
	if (typeCount == 0) {
		throwError("No type given in variable declaration", getLine());
	}
	String name = currentToken.str;
	//Make the AST node
	std::shared_ptr<AST> node = std::make_shared<VariableDeclarationAST>(modifiers, type, name);
	//Get the next token
	auto nextToken = getNextToken();
	//If it's an assignment, then we need to parse the expression
	if (nextToken == Type::Operator) {
		if (currentToken.str.getReference() == "=") {
			return parseBinaryOperator(getPrecedence("="), node);
		}
		else {
			throwError("Invalid operator after variable declaration: " + currentToken.str.getReference(), currentToken.originLine);
		}
	}
	//If it's EndOfStatement
	else if (nextToken == Type::EndOfStatement) {
		return node;
	}
	else if (nextToken == '(') {
		//Function time!!!
		throwError("Implement functions", currentToken.originLine);
	}
	return node;
}

//parseStatement
std::shared_ptr<AST::AST> AST::Parse::parseStatement() {
	
	//The operator is the current token
	String op = currentToken.str;
	//Get the next token
	auto nextToken = getNextToken();
	
	return nullptr;
}

std::vector<std::shared_ptr<AST::AST>> AST::Parse::parse() {
	std::vector<std::shared_ptr<AST>> code;
	std::shared_ptr<AST> currentNode = nullptr;
	while (true) {
		//Get the next Token
		Type type = getNextToken();
		std::cout << getCurrentToken().str.getReference() << std::endl;
		if (type == Type::EndOfFile) {
			break;
		}
		switch (type) {
		case Type::Identifier:
		case Type::IdentifierModifier:
			currentNode = parseDefinition();
			break;
		default:
			throwError("Unable to parse statement starting with '"s + getCurrentToken().str.getReference() + "'", getLine());
		}
		if (currentNode) {
			code.push_back(currentNode);
		}
	}
	return code;
}
