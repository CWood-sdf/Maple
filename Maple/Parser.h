#include <string>
#include <memory>
#include <vector>
#include "AST.h"
#include "String.h"
#include "Error.h"
namespace AST {
namespace Parse {
	std::shared_ptr<AST> parsePartialExpression();
	std::shared_ptr<AST> parseParentheses();
	std::shared_ptr<AST> parseBinaryOperator(int precedence, std::shared_ptr<AST> left);
	
	std::shared_ptr<AST> parseDefinition();
	
	std::shared_ptr<AST> parseStatement();
	std::vector<std::shared_ptr<AST>> parse();
}
}
