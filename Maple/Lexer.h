#ifndef LEXER_H
#define LEXER_H

#include "AST.h"
// #include <vcruntime.h>
extern uint32_t indentationLevel;
extern std::string file;
extern std::size_t currentLine;
bool incI();
namespace AST {
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
        BooleanLiteral = -13,
        FunctionDefinition = -14,
        Void = -15,
        Exit = -16,
    };
    bool operator==(const Type& value, char c);
    class Token {
    public:
        Type type;
        String str;
        int originLine;
        Token(Type t, String s);
        Token(const Token&) = default;
        Token& operator=(Token&&) = default;
    };

    size_t getLine();
    Token getCurrentToken();
    uint32_t getIndentationLevel();
    void prepareInterpreter(std::string f);

    Type getNextToken();
} // namespace AST

#endif // LEXER_H
