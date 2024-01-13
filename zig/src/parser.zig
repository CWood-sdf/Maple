const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const Allocator = @import("std").mem.Allocator;

pub const Parser = struct {
    lexer: *lexer.Lexer,
    allocator: Allocator,
    pub fn init(a: Allocator, l: *lexer.Lexer) Parser {
        return Parser{ .lexer = l, .allocator = a };
    }

    pub fn parse(self: *Parser) !*ast.AstNode {
        var astNode = try self.allocator.create(ast.AstNode);
        astNode.kind = ast.AstType{ .Number = 0 };
        return astNode;
    }
};
