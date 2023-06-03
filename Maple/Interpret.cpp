#include "Interpret.h"
#include <memory>
std::pair<std::shared_ptr<MemorySlot>, BlockExit> interpret(std::vector<std::shared_ptr<AST::ASTNode>> ast) {
    return {nullptr, BlockExit::None};
}

void interpret(std::string file) {
}
