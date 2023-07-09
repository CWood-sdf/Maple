#include "Interpret.h"
#include <memory>
std::shared_ptr<MemorySlot> interpret(
	std::vector<std::unique_ptr<AST::ASTNode>>& ast) {
	for (auto& i : ast) {
		i->getValue();
		if (isExit()) {
			return getReturnRegister();
		}
	}
	return nullptr;
}

void interpret(std::string file [[maybe_unused]]) {
	throwError(
		"Not implemented yet (interpret(std::string file))", AST::getLine());
}
