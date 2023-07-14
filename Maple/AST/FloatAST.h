#include "ASTNode.h"

namespace ASTsdf {
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
	class Int64AST : public ASTNode {
	public:
		int64_t value;
		Int64AST(int64_t value, std::size_t line = getLine());
		Int64AST(String value, std::size_t line = getLine());
		virtual ~Int64AST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class BoolAST : public ASTNode {
	public:
		bool value;
		BoolAST(bool value, std::size_t line = getLine());
		BoolAST(String value, std::size_t line = getLine());
		virtual ~BoolAST() = default;
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
		std::unique_ptr<ASTNode> left;
		std::unique_ptr<ASTNode> right;
		String op;
		BinaryOperatorAST(std::unique_ptr<ASTNode> left,
			std::unique_ptr<ASTNode> right, String op,
			std::size_t line = getLine());
		virtual ~BinaryOperatorAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	// A binop ast for '='
	class SetOpAST : public ASTNode {
	public:
		std::unique_ptr<ASTNode> left;
		std::unique_ptr<ASTNode> right;

		SetOpAST(std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right,
			std::size_t line = getLine());
		virtual ~SetOpAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class UnaryOperatorAST : public ASTNode {
	public:
		std::unique_ptr<ASTNode> value;
		String op;
		UnaryOperatorAST(std::unique_ptr<ASTNode> value, String op,
			std::size_t line = getLine());
		virtual ~UnaryOperatorAST() = default;
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
		VariableDeclarationAST(std::vector<String> types, String type,
			String name, std::size_t line = getLine());
		virtual ~VariableDeclarationAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};

	class FunctionAST : public ASTNode {
	public:
		String returnType;
		std::vector<std::unique_ptr<ASTNode>> arguments;
		std::vector<std::unique_ptr<ASTNode>> statements;
		String name;
		FunctionAST(String returnType,
			std::vector<std::unique_ptr<ASTNode>> arguments,
			std::vector<std::unique_ptr<ASTNode>> statements, String name,
			std::size_t line = getLine());
		virtual ~FunctionAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
		std::shared_ptr<MemorySlot> call(
			std::vector<std::unique_ptr<ASTNode>> arguments,
			std::size_t callLine);
		String getType();
	};
	class FunctionCallAST : public ASTNode {
	public:
		String name;
		std::vector<std::unique_ptr<ASTNode>> arguments;
		FunctionCallAST(String name,
			std::vector<std::unique_ptr<ASTNode>> arguments,
			std::size_t line = getLine());
		virtual ~FunctionCallAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class ExitAST : public ASTNode {
	public:
		ExitType type;
		std::unique_ptr<ASTNode> value;
		ExitAST(ExitType t, std::unique_ptr<ASTNode> value,
			std::size_t line = getLine());
		virtual ~ExitAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class MemSlotAST : public ASTNode {
	public:
		std::shared_ptr<MemorySlot> value;
		MemSlotAST(
			std::shared_ptr<MemorySlot> value, std::size_t line = getLine());
		virtual ~MemSlotAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
	class IfAST : public ASTNode {
	public:
		std::unique_ptr<ASTNode> condition;
		std::vector<std::unique_ptr<ASTNode>> statements;
		std::vector<std::unique_ptr<IfAST>> elseIfs = {};
		std::vector<std::unique_ptr<ASTNode>> elseStatements = {};
		bool isAlone;
		IfAST(std::unique_ptr<ASTNode> condition,
			std::vector<std::unique_ptr<ASTNode>> statements, bool isAlone,
			std::size_t line = getLine());
		virtual ~IfAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
		void addElseIf(std::unique_ptr<IfAST> elseIf);
		void addElse(std::vector<std::unique_ptr<ASTNode>> elseStatements);
	};

	class WhileAST : public ASTNode {
	public:
		std::unique_ptr<ASTNode> condition;
		std::vector<std::unique_ptr<ASTNode>> statements;
		WhileAST(std::unique_ptr<ASTNode> condition,
			std::vector<std::unique_ptr<ASTNode>> statements,
			std::size_t line = getLine());
		virtual ~WhileAST() = default;
		std::shared_ptr<MemorySlot> getValue() override;
	};
}
