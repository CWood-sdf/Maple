#include "Lexer.h"
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
        // If the operator is not letter based, then add the first character to the
        // set
        if ((op[0] >= 'a' && op[0] <= 'z') || (op[0] >= 'A' && op[0] <= 'Z')) {
            continue;
        }
        operatorFirstCharacters.insert(op[0]);
    }
}
AST::Type AST::getNextToken() {
    // EOF
    if (i >= file.length()) {
        currentToken = Token(Type::EndOfFile, String(""));
        return Type::EndOfFile;
    }
    if (file[i] == ' ' || file[i] == '\t') {
        // Safe increasing of i, because if we overshoot, next executed command
        // would be EOF
        i++;
        return getNextToken();
    }
    // If it's a newline, return end of statement
    if (file[i] == '\n' || file[i] == '\r') {
        currentLine++;
        // Safe inc
        i++;
        if (i < file.size() && (file[i] == '\n' || file[i] == '\r') && file[i] != file[i - 1]) {
            i++;
        }
        currentToken = Token(Type::EndOfStatement, String("\n"));
        return Type::EndOfStatement;
    }

    // If it's a number, return number
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

    // If it's a letter, return identifier
    if ((file[i] >= 'a' && file[i] <= 'z') ||
        (file[i] >= 'A' && file[i] <= 'Z') || file[i] == '_') {
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
            return Type::FunctionDefinition;
        } else if (identifier == "void") {
            currentToken = Token(Type::Void, identifier);
            return Type::Void;
        } else if (isIdentifier(identifier)) {
            currentToken = Token(Type::Identifier, identifier);
            return Type::Identifier;
        } else if (isIdentifierModifier(identifier)) {
            currentToken = Token(Type::IdentifierModifier, identifier);
            return Type::IdentifierModifier;
        } else if (isOperator(identifier)) {
            currentToken = Token(Type::Operator, identifier);
            return Type::Operator;
        } else if (isControlFlow(identifier)) {
            currentToken = Token(Type::ControlFlow, identifier);
            return Type::ControlFlow;
        } else if (isBooleanLiteral(identifier)) {
            currentToken = Token(Type::BooleanLiteral, identifier);
            return Type::BooleanLiteral;
        }

        currentToken = Token(Type::Name, identifier);
        return Type::Name;
    }
    // Comments must be handled before operators, because they can start with the
    // same character Skip comments
    if (file[i] == '/' && file[i + 1] == '/') {
        while (file[i] != '\n') {
            if (!incI()) {
                break;
            }
        }
        // don't increase line because the newline will do that
        //  currentLine++;
        //  Don't increase i, so that getNextToken() returns EOF or EndOfStatement
        return getNextToken();
    }
    // Skip multiline comments
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
    // If it's a potential operator
    if (operatorFirstCharacters.find(file[i]) != operatorFirstCharacters.end()) {
        std::string op = "";
        int count = 0;
        do {
            op += file[i];
            // Have to increment i before checking count, otherwise we could get count
            // forcing a break 	before i is incremented
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
            } else {
                // Pop off last string character
                str = str.substr(0, str.size() - 1);
                // Add the escaped character
                str += escapeCharacters[file[i]];
            }
            if (file[i] == '\n') {
                throwError("Unclosed string literal (newlines can't be in strings)",
                    getLine());
                currentLine++;
            }
            if (!incI()) {
                throwError("Unclosed string literal at end of file: \"" + actualStr,
                    getLine());
            }
        }
        // Skip past the "
        i++;
        currentToken = Token(Type::StringLiteral, str);
        return Type::StringLiteral;
    }

    // Handle character literals
    if (file[i] == '\'') {
        if (!incI()) {
            throwError("Unclosed character literal", getLine());
        }
        std::string str = "";
        int count = 0;
        char lastChar = ' ';
        while (file[i] != '\'') {
            if (file[i] == '\n') {
                throwError("Unclosed character literal (newlines can't be in character "
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
        // If the count is greater than 1, then it's an invalid character literal,
        // unless it's escape
        if (count > 1 && str[0] != '\\') {
            throwError("Invalid character literal: '" + str + "'", getLine());
        }
        // If the count is greater than 2, then it's an invalid character literal
        else if (count > 2) {
            throwError("Invalid character literal: '" + str + "'", getLine());
        }
        if (lastChar == '\\') {
            if (i >= file.size() - 1) {
                // Since we're here, we can't check if there is another ', thus there
                // isnt one
                throwError("Unclosed character literal: '" + str + "'", getLine());
            } else if (file[i + 1] == '\'') {
                // Eat the first of the two ', the second one will be done later
                str += file[i];
                i++;
            } else {
                throwError("Unclosed character literal: '" + str + "'", getLine());
            }
        }
        if (str[0] == '\\') {
            str = escapeCharacters[str[1]];
        }
        // Skip past the '
        i++;
        currentToken = Token(Type::CharacterLiteral, str);
        return Type::CharacterLiteral;
    }
    char c = file[i];
    i++;
    currentToken = Token((Type)c, String(c));
    return (Type)c;
}
