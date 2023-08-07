#ifndef INC_AST_H
#define INC_AST_H

#include "../Error.h"
#include "../Scope.h"
#include "../String.h"
#include "../Variable.h"
#include <memory>
#include <set>
#include <string>
#include <unordered_set>
// double stdDev(std::vector<double> stuff){
namespace ASTsdf {
	using std::operator"" s;
	extern std::set<String> identifiers;
	extern std::set<String> identifierModifiers;
	extern std::set<String> controlFlow;
	extern std::set<String> exitStatements;
	extern std::set<String> operators;
	extern std::set<char> operatorFirstCharacters;
	extern std::map<String, int> operatorPrecedence;
	extern std::set<String> unaryOperators;
	extern std::map<char, char> escapeCharacters;
	void initASTGlobals();
}
// extern std::unor
// extern u i;
namespace ASTsdf {
	int getPrecedence(String op);
	bool isIdentifier(String str);
	bool isIdentifierModifier(String str);
	bool isControlFlow(String str);
	bool isOperator(String str);
	bool isBooleanLiteral(String str);
	bool isExitStatement(String str);
	bool isUnaryOperator(String str);
	int getUnaryPrecedence(String op);
	size_t getLine();

	class ASTNode {
	protected:
		const std::size_t line = getLine();

	public:
		ASTNode(std::size_t l) : line(l) {}
		virtual ~ASTNode() = default;
		virtual std::shared_ptr<MemorySlot> getValue() = 0;
		virtual std::size_t getOriginLine() {
			return line;
		}
	};

} // namespace AST

#endif // INC_AST_H
