#include "Error.h"
#include "Lexer.h"
#include "String.h"
#include <memory>
#include <string>
#include <vector>

namespace AST {
	namespace Parse {
		std::vector<std::unique_ptr<ASTNode>> parse(bool topLevel);
	} // namespace Parse
} // namespace AST
