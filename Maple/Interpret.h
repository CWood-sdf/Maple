#pragma once
#include "AST.h"
#include <memory>
#include <string>
enum BlockExit {
    Return,
    Break,
    Continue,
    None
};

std::pair<std::shared_ptr<MemorySlot>, BlockExit> interpret(std::vector<std::shared_ptr<AST::ASTNode>> ast);
void interpret(std::string file);
