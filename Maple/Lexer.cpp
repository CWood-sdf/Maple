#include "Lexer.h"
#include <queue>
using namespace AST;
std::queue<AST::Token> fakeTokens;
uint32_t i = 0;
uint32_t indentationLevel = 0;
std::string file = "";
std::size_t currentLine = 0;
bool incI() {
    if (i >= file.length())
        return false;
    i++;
    return true;
}
bool AST::operator==(const Type& value, char c) { return value == (Type)c; }
// A function that returns currentToken

AST::Token::Token(Type t, String s) : type(t), str(s), originLine(getLine()) {}
AST::Token currentToken = AST::Token((AST::Type)0, String(""));

size_t AST::getLine() {
    if (currentToken.type == Type::EndOfStatement) {
        return currentLine - 1;
    }
    return currentLine;
}

AST::Token AST::getCurrentToken() { return currentToken; }
uint32_t AST::getIndentationLevel() { return indentationLevel; }
void AST::prepareInterpreter(std::string f) {
    file = f;
    i = 0;
    currentLine = 1;
    operatorFirstCharacters.clear();
    for (auto& op : operators) {
        // If the operator is not letter based, then add the first character to
        // the set
        if ((op[0] >= 'a' && op[0] <= 'z') || (op[0] >= 'A' && op[0] <= 'Z')) {
            continue;
        }
        operatorFirstCharacters.insert(op[0]);
    }
    for (auto& op : unaryOperators) {
        // If the operator is not letter based, then add the first character to
        // the set
        if ((op[0] >= 'a' && op[0] <= 'z') || (op[0] >= 'A' && op[0] <= 'Z')) {
            continue;
        }
        operatorFirstCharacters.insert(op[0]);
    }
}
void AST::addFakeToken(Type t, String s) {
    fakeTokens.push(Token(t, s));
    fakeTokens.push(currentToken);
}
AST::Token readIdent() {

    std::string identifier = "";
    do {
        identifier += file[i];
        if (!incI()) {
            break;
        }
    } while ((file[i] >= 'a' && file[i] <= 'z') ||
             (file[i] >= 'A' && file[i] <= 'Z') ||
             (file[i] >= '0' && file[i] <= '9') || file[i] == '_');
    if (identifier == "fn") {
        currentToken = Token(Type::FunctionDefinition, identifier);
    } else if (isExitStatement(identifier)) {
        currentToken = Token(Type::Exit, identifier);
    } else if (identifier == "void") {
        currentToken = Token(Type::Void, identifier);
    } else if (isIdentifier(identifier)) {
        currentToken = Token(Type::Identifier, identifier);
    } else if (isIdentifierModifier(identifier)) [[unlikely]] {
        currentToken = Token(Type::IdentifierModifier, identifier);
    } else if (isOperator(identifier)) [[unlikely]] {
        currentToken = Token(Type::Operator, identifier);
    } else if (isControlFlow(identifier)) {
        currentToken = Token(Type::ControlFlow, identifier);
    } else if (isBooleanLiteral(identifier)) {
        currentToken = Token(Type::BooleanLiteral, identifier);
    } else [[likely]] {
        currentToken = Token(Type::Name, identifier);
    }
    return currentToken;
}
AST::Token readNumber() {
    std::string number = "";
    unsigned int decimalCount = 0;
    bool isI64 = false;
    do {
        if (isI64) {
            throwError(
                "Unexpected continuation of number after ending character 'l'",
                getLine());
        }
        number += file[i];
        if (file[i] == '.') {
            decimalCount++;
        }
        if (!incI()) {
            break;
        }
        if (file[i] == 'l') {
            isI64 = true;
            if (decimalCount > 0) [[unlikely]] {
                throwError("Invalid number: " + number +
                               "\n  note: number ends with an 'l' to signify "
                               "that it is int64, but it has a decimal point"
                               ", which signifies that it's a float",
                    getLine());
            }
            if (!incI()) {
                break;
            }
        }
    } while ((file[i] >= '0' && file[i] <= '9') || file[i] == '.');
    if (decimalCount == 0 && !isI64) {
        try {
            std::stoi(number);
        } catch (std::out_of_range& e) {
            throwError("Given numeric literal \"" + number +
                           "\" does not fit in 32 bit integer, try adding an "
                           "'l' to the end to make it an int64",
                getLine());
        }
    }
    if (decimalCount > 1) [[unlikely]] {
        throwError("Invalid number: " + number, getLine());
    } else if (decimalCount == 1) {
        currentToken = Token(Type::FloatLiteral, String(number));
    } else if (isI64) {
        currentToken = Token(Type::Int64Literal, String(number));
    } else [[likely]] {
        currentToken = Token(Type::IntLiteral, String(number));
    }
    return currentToken;
}
AST::Type AST::getNextToken() {
    if (!fakeTokens.empty()) {
        currentToken = fakeTokens.front();
        fakeTokens.pop();
        return currentToken.type;
    }
    // EOF
    if (i >= file.length()) {
        currentToken = Token(Type::EndOfFile, String(""));
        return Type::EndOfFile;
    }
    while (file[i] == ' ' || file[i] == '\t') {
        // Safe increasing of i, because if we overshoot, next executed command
        // would be EOF
        i++;
        if (i >= file.length()) {
            currentToken = Token(Type::EndOfFile, String(""));
            return Type::EndOfFile;
        }
    }
    // If it's a newline, return end of statement
    if (file[i] == '\n' || file[i] == '\r') {
        currentLine++;
        // Safe inc
        i++;
        if (i < file.size() && (file[i] == '\n' || file[i] == '\r') &&
            file[i] != file[i - 1]) {
            i++;
        }
        currentToken = Token(Type::EndOfStatement, String("\n"));
        return Type::EndOfStatement;
    }

    // If it's a number, return number
    else if (file[i] >= '0' && file[i] <= '9') {
        return readNumber().type;
    }

    // If it's a letter, return identifier
    else if ((file[i] >= 'a' && file[i] <= 'z') ||
             (file[i] >= 'A' && file[i] <= 'Z') || file[i] == '_') {
        return readIdent().type;
    }
    // Comments must be handled before operators, because they can start with
    // the same character Skip comments
    else if (file[i] == '/' && file[i + 1] == '/') {
        while (file[i] != '\n') {
            if (!incI()) {
                break;
            }
        }
        // don't increase line because the newline will do that
        //  currentLine++;
        //  Don't increase i, so that getNextToken() returns EOF or
        //  EndOfStatement
        return getNextToken();
    }
    // Skip multiline comments
    else if (file[i] == '/' && i < file.size() - 1 && file[i + 1] == '*') {
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
    // If it's a potential operator
    else if (operatorFirstCharacters.find(file[i]) !=
             operatorFirstCharacters.end()) [[likely]] {
        std::string op = "";
        int count = 0;
        do {
            op += file[i];
            // Have to increment i before checking count, otherwise we could
            // get count forcing a break 	before i is incremented
            bool inc = incI();
            count++;
            if (count > 3) {
                break;
            }
            if (!inc) {
                break;
            }
        } while (!isOperator(op) && !isUnaryOperator(op));
        do {
            op += file[i];
            // Have to increment i before checking count, otherwise we could
            // get count forcing a break 	before i is incremented
            bool inc = incI();
            count++;
            if (count > 3) {
                break;
            }
            if (!inc) {
                break;
            }
        } while (isOperator(op) || isUnaryOperator(op));
        op = op.substr(0, op.size() - 1);
        i--;
        if (isOperator(op) || isUnaryOperator(op)) {
            currentToken = Token(Type::Operator, op);
            return Type::Operator;
        }
        // else {
        //	i--;
        //	//Remove the last char from op
        //	op = op.substr(0, op.size() - 1);
        // }
        // if (isOperator(op)) {
        //	currentToken = Token(Type::Operator, op);
        //	return Type::Operator;
        // }
        throwError("Invalid operator: " + op, getLine());
    }

    // Handle string literals
    else if (file[i] == '"') [[unlikely]] {
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
            } else {
                // Pop off last string character
                str = str.substr(0, str.size() - 1);
                // Add the escaped character
                str += escapeCharacters[file[i]];
            }
            if (file[i] == '\n') {
                throwError("Unclosed string literal (newlines can't be in "
                           "strings)",
                    getLine());
                currentLine++;
            }
            if (!incI()) {
                throwError(
                    "Unclosed string literal at end of file: \"" + actualStr,
                    getLine());
            }
        }
        // Skip past the "
        i++;
        currentToken = Token(Type::StringLiteral, str);
        return Type::StringLiteral;
    }

    // Handle character literals
    else if (file[i] == '\'') [[unlikely]] {
        if (!incI()) {
            throwError("Unclosed character literal", getLine());
        }
        std::string str = "";
        int count = 0;
        char lastChar = ' ';
        while (file[i] != '\'') {
            if (file[i] == '\n') {
                throwError("Unclosed character literal (newlines can't be in "
                           "character "
                           "literals)",
                    getLine());
                currentLine++;
            }
            str += file[i];
            lastChar = file[i];
            count++;
            if (!incI()) {
                throwError("Unclosed character literal", getLine());
            }
        }
        // If the count is greater than 1, then it's an invalid character
        // literal, unless it's escape
        if (count > 1 && str[0] != '\\') {
            throwError("Invalid character literal: '" + str + "'", getLine());
        }
        // If the count is greater than 2, then it's an invalid character
        // literal
        else if (count > 2) {
            throwError("Invalid character literal: '" + str + "'", getLine());
        }
        if (lastChar == '\\') {
            if (i >= file.size() - 1) {
                // Since we're here, we can't check if there is another ',
                // thus there isnt one
                throwError(
                    "Unclosed character literal: '" + str + "'", getLine());
            } else if (file[i + 1] == '\'') {
                // Eat the first of the two ', the second one will be done
                // later
                str += file[i];
                i++;
            } else {
                throwError(
                    "Unclosed character literal: '" + str + "'", getLine());
            }
        }
        if (str[0] == '\\') {
            str = escapeCharacters[str[1]];
        }
        // Skip past the '
        i++;
        currentToken = Token(Type::CharacterLiteral, str);
        return Type::CharacterLiteral;
    } else [[likely]] {
        char c = file[i];
        i++;
        currentToken = Token((Type)c, String(c));
        return (Type)c;
    }
    // idk what to put here bc its impossible to be here but clangd is angry
    // otherwise
    return Type::EndOfFile;
}
