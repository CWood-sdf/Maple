#include "Interpret.h"
#include <memory>
std::shared_ptr<MemorySlot> interpret(std::vector<std::shared_ptr<AST::ASTNode>> ast) {
    for (auto i : ast) {
        i->getValue();
    }
    return nullptr;
}

void interpret(std::string file) {
}
