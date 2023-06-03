#ifndef AST_H
#define AST_H

#include "Error.h"
#include "Scope.h"
#include "String.h"
#include "Variable.h"
#include <set>
#include <string>

using std::operator"" s;
extern std::set<String> identifiers;
extern std::set<String> identifierModifiers;
extern std::set<String> controlFlow;
extern std::set<String> operators;
extern std::set<char> operatorFirstCharacters;
extern std::map<String, int> operatorPrecedence;
extern std::map<char, char> escapeCharacters;
// extern u i;
std::shared_ptr<MemorySlot>
evalOperatorEql(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot>
evalOperatorPls(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot>
evalOperatorMns(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue);
std::shared_ptr<MemorySlot>
evalOperatorMult(std::shared_ptr<MemorySlot> leftValue,
    std::shared_ptr<MemorySlot> rightValue);
namespace AST {
    int getPrecedence(String op);
    bool isIdentifier(String str);
    bool isIdentifierModifier(String str);
    bool isControlFlow(String str);
    bool isOperator(String str);
    bool isBooleanLiteral(String str);
    size_t getLine();

    class ASTNode {
    protected:
        const std::size_t line = getLine();

    public:
        ASTNode(std::size_t l) : line(l) {}
        virtual std::shared_ptr<MemorySlot> getValue() = 0;
    };
    class FloatAST : public ASTNode {
    public:
        double value;
        FloatAST(double value, std::size_t line = getLine());
        FloatAST(String value, std::size_t line = getLine());
        virtual ~FloatAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class IntAST : public ASTNode {
    public:
        int value;
        IntAST(int value, std::size_t line = getLine());
        IntAST(String value, std::size_t line = getLine());
        virtual ~IntAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class BoolAST : public ASTNode {
    public:
        bool value;
        BoolAST(bool value, std::size_t line = getLine());
        BoolAST(String value, std::size_t line = getLine());
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class StringAST : public ASTNode {
    public:
        String value;
        StringAST(String value, std::size_t line = getLine());
        virtual ~StringAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class CharacterAST : public ASTNode {
    public:
        char value;
        CharacterAST(char value, std::size_t line = getLine());
        CharacterAST(String value, std::size_t line = getLine());
        virtual ~CharacterAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class BinaryOperatorAST : public ASTNode {
    public:
        std::shared_ptr<ASTNode> left;
        std::shared_ptr<ASTNode> right;
        String op;
        BinaryOperatorAST(std::shared_ptr<ASTNode> left, std::shared_ptr<ASTNode> right,
            String op, std::size_t line = getLine());
        virtual ~BinaryOperatorAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class UnaryOperatorAST : public ASTNode {
    public:
        std::shared_ptr<ASTNode> value;
        String op;
        UnaryOperatorAST(std::shared_ptr<ASTNode> value, String op, std::size_t line = getLine());
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class VariableAST : public ASTNode {
    public:
        String name;
        VariableAST(String name, std::size_t line = getLine());
        virtual ~VariableAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };
    class VariableDeclarationAST : public ASTNode {
    public:
        std::vector<String> modifiers;
        String type;
        String name;
        VariableDeclarationAST(std::vector<String> types, String type, String name, std::size_t line = getLine());
        virtual ~VariableDeclarationAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
    };

    class FunctionAST : public ASTNode {
    public:
        String returnType;
        std::vector<std::shared_ptr<ASTNode>> arguments;
        std::vector<std::shared_ptr<ASTNode>> statements;
        String name;
        FunctionAST(String returnType,
            std::vector<std::shared_ptr<ASTNode>> arguments,
            std::vector<std::shared_ptr<ASTNode>> statements, String name, std::size_t line = getLine());
        virtual ~FunctionAST() = default;
        std::shared_ptr<MemorySlot> getValue() override;
        std::shared_ptr<MemorySlot> call(std::vector<std::shared_ptr<ASTNode>> arguments);
        String getType();
    };
    class FunctionCallAST : public ASTNode {
    public:
        std::shared_ptr<ASTNode> function;
        std::vector<std::shared_ptr<ASTNode>> arguments;
        FunctionCallAST(std::shared_ptr<ASTNode> function,
            std::vector<std::shared_ptr<ASTNode>> arguments, std::size_t line = getLine());
        std::shared_ptr<MemorySlot> getValue() override;
    };

} // namespace AST

#endif // AST_H
