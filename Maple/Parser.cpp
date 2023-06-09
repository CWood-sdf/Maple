#include "Parser.h"
#include <memory>
#include <ranges>
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wswitch"
using std::operator"" s;
using namespace AST;

// TODO: Make sure every function is starting on the token it needs

std::shared_ptr<AST::ASTNode> parseParentheses();

std::shared_ptr<AST::ASTNode> parseBinaryOperator(
    std::shared_ptr<ASTNode> left);

std::shared_ptr<AST::ASTNode> parseDefinition();
std::shared_ptr<AST::ASTNode> parsePartialExpression(int maxPrecedence = -1);
std::shared_ptr<AST::ASTNode> parseStatement();

/// <summary>
/// A unary prefix operator expression
///		::= op partialExpression
/// </summary>
std::shared_ptr<AST::ASTNode> parseUnaryOperator() {
    auto op = getCurrentToken().str;
    getNextToken();
    auto precedence = getUnaryPrecedence(op);
    auto right = parsePartialExpression(precedence);
    return std::make_shared<UnaryOperatorAST>(right, op);
}

/// <summary>
/// A partial expression
///		::= variable
///		::= literal
/// </summary>
/// <returns></returns>
std::shared_ptr<AST::ASTNode> parsePartialExpression(int maxPrecedence) {
    std::shared_ptr<AST::ASTNode> ret = nullptr;
    auto c = getCurrentToken().type;
    bool wasName = false;
    String name = "";
    bool eat = true;
    switch (c) {
    case Type::CharacterLiteral:
        ret = std::make_shared<CharacterAST>(getCurrentToken().str);
        break;
    case Type::StringLiteral:
        ret = std::make_shared<StringAST>(getCurrentToken().str);
        break;
    case Type::IntLiteral:
        ret = std::make_shared<IntAST>(getCurrentToken().str);
        break;
    case Type::FloatLiteral:
        ret = std::make_shared<FloatAST>(getCurrentToken().str);
        break;
    case Type::BooleanLiteral:
        ret = std::make_shared<BoolAST>(getCurrentToken().str);
        break;
    case Type::Name:
        name = getCurrentToken().str;
        wasName = true;
        ret = std::make_shared<VariableAST>(name);
        break;
    case Type::Operator:
        eat = false;
        ret = parseUnaryOperator();
        break;
    case (Type)'(':
        ret = parseParentheses();
        break;
    default:
        throwError("Unexpected token \""s +
                       getCurrentToken().str.getReference() + "\""s,
            getCurrentToken().originLine);
        ret = nullptr;
        break;
    }
    // eat the token
    if (eat) {
        getNextToken();
    }
    if (wasName && getCurrentToken().type == (Type)'(') {
        // eat the '('
        getNextToken();
        std::vector<std::shared_ptr<ASTNode>> args;
        while (1) {
            args.push_back(parsePartialExpression());
            if (getCurrentToken().type == (Type)',') {
                getNextToken();
            } else if (getCurrentToken().type == (Type)')') {
                break;
            } else {
                throwError(
                    "Expected ',' or ')' in function argument list, got "s +
                        getCurrentToken().str.getReference(),
                    getCurrentToken().originLine);
            }
        }
        // eat the ')'
        getNextToken();
        ret = std::make_shared<FunctionCallAST>(name, args);
    }
    if (getCurrentToken().type == Type::Operator && maxPrecedence == -1) {
        // New binary operator
        ret = parseBinaryOperator(ret);
    } else if (getCurrentToken().type == Type::Operator &&
               getPrecedence(getCurrentToken().str) < maxPrecedence) {
        // ret = parseBinaryOperator(getPrecedence(getCurrentToken().str), ret);
        auto op = getCurrentToken().str;
        int precedence = getPrecedence(op);
        getNextToken();
        auto left = ret;
        auto right = parsePartialExpression(precedence);
        ret = std::make_shared<BinaryOperatorAST>(left, right, op);
    }
    return ret;
}
/// <summary>
/// Parses a parenthesized expression
///		::= '(' partialExpression ')'
/// </summary>
std::shared_ptr<AST::ASTNode> parseParentheses() {
    // eat '('
    getNextToken();
    auto ret = parsePartialExpression();
    // eat ')'
    // getNextToken();
    return ret;
}
/// <summary>
/// Parses a binary operator
///		::= partialExpression binaryOperator partialExpression
/// </summary>
std::shared_ptr<AST::ASTNode> parseBinaryOperator(
    std::shared_ptr<ASTNode> left) {
    // Store the operator
    String op = getCurrentToken().str;
    int precedence = getPrecedence(op);
    getNextToken();
    // Get the right expression
    auto right = parsePartialExpression(precedence);
    auto tokenType = getCurrentToken().type;
    if (tokenType == Type::Operator) {
        int nextOpPrecedence = getPrecedence(getCurrentToken().str);
        if (nextOpPrecedence <= precedence) {
            // make sure this operator is evaluated first
            right = parseBinaryOperator(right);
        } else if (nextOpPrecedence > precedence) {
            // turn current nodes into tree then attach so that they evaluated
            //   first
            std::shared_ptr<ASTNode> ret =
                std::make_shared<BinaryOperatorAST>(left, right, op);
            return parseBinaryOperator(ret);
        }
    }
    std::shared_ptr<ASTNode> ret =
        std::make_shared<BinaryOperatorAST>(left, right, op);
    return ret;
}
/// <summary>
/// Parses a partial definition
///     ::= type name
/// </summary>
std::shared_ptr<ASTNode> parsePartialDefinition() {
    std::vector<String> modifiers;
    int typeCount = 0;
    String type = "";

    do {
        if (getCurrentToken().type == Type::Identifier) {
            typeCount++;
            type = getCurrentToken().str;
            if (typeCount > 1) {
                throwError("Too many types given in variable declaration: " +
                               getCurrentToken().str.getReference(),
                    getCurrentToken().originLine);
            }
        } else if (getCurrentToken().type != Type::IdentifierModifier) {
            throwError("Invalid token in type definition: "s +
                           getCurrentToken().str.getReference(),
                getCurrentToken().originLine);
        } else {
            modifiers.push_back(getCurrentToken().str);
        }
    } while (getNextToken() != Type::Name);
    if (typeCount == 0) {
        throwError("No type given in variable declaration", getLine());
    }
    String name = getCurrentToken().str;
    // Make the AST node
    std::shared_ptr<ASTNode> node =
        std::make_shared<VariableDeclarationAST>(modifiers, type, name);
    // eat the name
    getNextToken();
    return node;
}
/// <summary>
/// Parses a definition
///		::= type name = expression
///		::= type name
/// </summary>
std::shared_ptr<AST::ASTNode> parseDefinition() {
    auto node = parsePartialDefinition();
    auto nextToken = getCurrentToken().type;
    // If it's an assignment, then we need to parse the expression
    if (nextToken == Type::Operator) {
        if (getCurrentToken().str.getReference() == "=") {
            return parseBinaryOperator(node);
        } else {
            throwError("Invalid operator after variable declaration: " +
                           getCurrentToken().str.getReference(),
                getCurrentToken().originLine);
        }
    } else if (nextToken == Type::EndOfStatement) {
        return node;
    } else {
        throwError("Invalid token after variable declaration: " +
                       getCurrentToken().str.getReference() +
                       "\n  note: maybe you forgot a newline",
            getCurrentToken().originLine);
    }
    return node;
}

// parseStatement
/// <summary>
/// Parses a statement
///		::= operator stuff
///		::= function call
/// basically anything that can be put as a function parameter
/// </summary>
std::shared_ptr<AST::ASTNode> parseStatement() {
    return parsePartialExpression();
}

// parseFunctionDefintion
/// <summary>
/// Parses a function definition
///		::= fn name '(' parameters ')' ret_type '{' statements '}'
/// </summary>
std::shared_ptr<ASTNode> parseFunctionDefinition() {
    // eat fn
    getNextToken();
    // Get the name
    if (getCurrentToken().type != Type::Name) {
        throwError("Expected name after fn", getCurrentToken().originLine);
    }
    String name = getCurrentToken().str;
    // eat name
    getNextToken();
    // eat '('
    if (getCurrentToken().type != (Type)'(') {
        throwError(
            "Expected '(' after function name", getCurrentToken().originLine);
    }
    getNextToken();
    // Get the parameters
    std::vector<std::shared_ptr<ASTNode>> parameters;
    while (1) {
        parameters.push_back(parsePartialDefinition());
        if (getCurrentToken().type == (Type)',') {
            getNextToken();
        } else if (getCurrentToken().type == (Type)')') {
            break;
        } else {
            throwError("Expected ',' or ')' after function parameter",
                getCurrentToken().originLine);
        }
    }
    // eat ')'
    getNextToken();
    // Get the return type
    if (getCurrentToken().type != Type::Identifier &&
        getCurrentToken().type != Type::Void) {
        throwError("Expected return type after function parameters",
            getCurrentToken().originLine);
    }
    String returnType = getCurrentToken().str;
    // eat return type
    getNextToken();

    //  Get the statements
    std::vector<std::shared_ptr<ASTNode>> statements = Parse::parse(false);
    if (getCurrentToken().type != Type::EndOfStatement) {
        throwError("Expected newline after function block",
            getCurrentToken().originLine);
    }
    // eat newline
    getNextToken();

    // Make the AST node
    std::shared_ptr<FunctionAST> node =
        std::make_shared<FunctionAST>(returnType, parameters, statements, name);
    node->setSelfReference(node);
    return node;
}

// parseExitStatement
/// <summary>
/// Parses an exit statement
///		::= return expression
///     ::= break expression
///     ::= return
///     ::= break
///     ::= continue
/// </summary>
std::shared_ptr<ASTNode> parseExitStatement() {
    auto name = getCurrentToken().str;
    ExitType type = ExitType::Return;
    if (name == "break"s) {
        type = ExitType::Break;
    } else if (name == "continue"s) {
        type = ExitType::Continue;
    }
    getNextToken();
    if (getCurrentToken().type == Type::EndOfStatement) {
        return std::make_shared<ExitAST>(type, nullptr);
    }
    if (type == ExitType::Continue) {
        throwError("Invalid token after continue statement: " +
                       getCurrentToken().str.getReference() +
                       "\n  note: expected a newline because continue can not "
                       "emit a value",
            getCurrentToken().originLine);
    }
    auto node = parsePartialExpression();
    return std::make_shared<ExitAST>(type, node);
}

// parseIfStatement
/// <summary>
/// Parses an if statement
///		::= if expression '{' statements '}' else '{' statements '}'
///		::= if expression '{' statements '}'
/// </summary>
std::shared_ptr<ASTNode> parseIfStatement(bool isAlone) {
    // Get the expression
    auto expression = parsePartialExpression();
    // Get the statements
    std::vector<std::shared_ptr<ASTNode>> statements = Parse::parse(false);
    // Ignore newlines
    while (getCurrentToken().type == Type::EndOfStatement) {
        getNextToken();
    }
    std::shared_ptr<IfAST> ret =
        std::make_shared<IfAST>(expression, statements, isAlone);

    while (getCurrentToken().str == "elseif"s) {
        getNextToken();
        // Get the expression
        auto elseIfCondition = parsePartialExpression();
        // Get the statements
        std::vector<std::shared_ptr<ASTNode>> elseifStatements =
            Parse::parse(false);
        // Ignore newlines
        while (getCurrentToken().type == Type::EndOfStatement) {
            getNextToken();
        }

        ret->addElseIf(std::make_shared<IfAST>(
            elseIfCondition, elseifStatements, isAlone));
    }
    // Check if there's an else statement
    if (getCurrentToken().str == "else"s) {
        getNextToken();
        // Ignore newlines
        while (getCurrentToken().type == Type::EndOfStatement) {
            getNextToken();
        }

        // Get the else statements
        std::vector<std::shared_ptr<ASTNode>> elseStatements =
            Parse::parse(false);

        ret->addElse(elseStatements);
    }
    addFakeToken(Type::EndOfStatement, "\n");
    getNextToken();
    return ret;
}
/// parseWhileStatement
/// <summary>
/// Parses a while statement
///		::= while expression '{' statements '}'
/// </summary>
std::shared_ptr<ASTNode> parseWhileStatement(bool isAlone) {
    // // eat while
    // getNextToken();
    // Get the expression
    auto expression = parsePartialExpression();
    // Get the statements
    std::vector<std::shared_ptr<ASTNode>> statements = Parse::parse(false);
    // Make the AST node
    std::shared_ptr<WhileAST> node =
        std::make_shared<WhileAST>(expression, statements, isAlone);
    return node;
}

// parseControlFlow
/// <summary>
/// Parses a control flow statement
///		::= if expression '{' statements '}' else '{' statements '}'
///		::= if expression '{' statements '}'
///		::= while expression '{' statements '}'
/// </summary>
std::shared_ptr<ASTNode> parseControlFlow(bool isAlone) {
    // eat the keyword
    auto keyword = getCurrentToken().str;
    getNextToken();
    if (keyword == "if"s) {
        return parseIfStatement(isAlone);
    } else if (keyword == "while"s) {
        return parseWhileStatement(isAlone);
    }
    return nullptr;
}

std::vector<std::shared_ptr<AST::ASTNode>> AST::Parse::parse(bool topLevel) {
    std::vector<std::shared_ptr<ASTNode>> code;
    std::shared_ptr<ASTNode> currentNode = nullptr;
    if (topLevel) {
        // Get the first token
        getNextToken();
    } else {
        // eat '{'
        if (getCurrentToken().type != (Type)'{') {
            throwError("Expected '{' to start code block",
                getCurrentToken().originLine);
        }
        if (getNextToken() != Type::EndOfStatement) {
            throwError(
                "Expected newline after '{'", getCurrentToken().originLine);
        }
        // eat newline
        getNextToken();
    }
    while (true) {
        // Get the next Token
        Type type = getCurrentToken().type;
        writeOutput(getCurrentToken().str.getReference());
        if (type == Type::EndOfFile) {
            if (!topLevel) {
                throwError("Unexpected end of file while parsing code block "
                           "(AKA unmatched '{')",
                    getCurrentToken().originLine);
            }
            break;
        }
        switch (type) {
        case Type::ControlFlow:
            currentNode = parseControlFlow(true);
            break;
        case Type::Exit:
            currentNode = parseExitStatement();
            break;
        case Type::Identifier:
        case Type::IdentifierModifier:
            currentNode = parseDefinition();
            break;
        case Type::FunctionDefinition:
            currentNode = parseFunctionDefinition();
            break;
        case Type::Name:
        case Type::Operator:
            currentNode = parseStatement();
            break;
        case (Type)'}':
            if (topLevel) {
                throwError("Unexpected top level '}'\n  note: this may be "
                           "caused by excess closing braces",
                    getLine());
            } else {
                goto END;
            }
            break;
        case Type::EndOfStatement:
            // Ignore empty statements
            // Eat the token
            getNextToken();
            continue;
            // break;
        default:
            throwError("Unable to parse statement starting with '"s +
                           getCurrentToken().str.getReference() + "'",
                getLine());
        }
        if (currentNode) {
            code.push_back(currentNode);
        }
        if (getCurrentToken().type != Type::EndOfStatement &&
            getCurrentToken().type != Type::EndOfFile) {
            throwError("Expected '\\n' after statement\n  note: got \""s +
                           getCurrentToken().str.getReference() + "\"",
                getCurrentToken().originLine);
            getNextToken();
        } else {
            getNextToken();
        }
    }
END:
    if (!topLevel) {
        // eat '}'
        if (getCurrentToken().type != (Type)'}') {
            throwError("Expected '}' at end of code block",
                getCurrentToken().originLine);
        }
        getNextToken();
    }
    return code;
}
#pragma clang diagnostic pop
