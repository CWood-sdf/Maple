#pragma once
#include "AST.h"
#include <memory>
#include <string>

std::shared_ptr<MemorySlot> interpret(
	std::vector<std::unique_ptr<AST::ASTNode>>& ast);
// void interpret(std::string file);
