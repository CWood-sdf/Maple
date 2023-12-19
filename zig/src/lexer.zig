const std = @import("std");
const Allocator = @import("std").mem.Allocator;
const Arr = std.ArrayList;
fn is_alpha(c: u8) bool {
    return (c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or c == '_';
}
fn is_num(c: u8) bool {
    return c >= '0' and c <= '9';
}
fn is_alnum(c: u8) bool {
    return is_alpha(c) or is_num(c);
}
pub const TokenType = union(enum) {
    Ident: []u8,
    Number: f64,
    Var,
    Const,
    OpEq,
    EOF,
    EndOfStatement,
    pub fn to_string(self: TokenType) []u8 {
        switch (self) {
            .Ident => return "Ident",
            .Number => return "Number",
            .Var => return "Var",
            .Const => return "Const",
            .OpEq => return "OpEq",
            .EOF => return "EOF",
            .EndOfStatement => return "EndOfStatement",
        }
    }
};
pub const Token = struct {
    type: TokenType,
    start_line: usize,
    start_column: usize,
    end_column: usize,
};
pub const LexerError = error{
    InvalidChar,
};
pub const Lexer = struct {
    current_token: Token = undefined,
    line: usize = 0,
    char: usize = 0,
    input: [][]u8 = undefined,
    pub fn init(allocator: Allocator, input: []u8) !Lexer {
        var newlines: usize = 0;
        for (0..input.len) |i| {
            if (input[i] == '\n') {
                newlines += 1;
            }
        }
        var lines = try Arr([]u8).initCapacity(allocator, newlines);
        var lineStart: usize = 0;
        var lineEnd: usize = 0;
        for (0..input.len) |i| {
            if (input[i] == '\n') {
                lineEnd = i;
                try lines.append(input[lineStart..lineEnd]);
                lineStart = lineEnd + 1;
            }
        }
        if (lineStart < input.len and lineEnd + 1 != input.len) {
            try lines.append(input[lineStart..input.len]);
        }
        return Lexer{
            .input = lines.items,
        };
    }
    pub fn deinit(self: *Lexer) void {
        std.ArrayList.deinit(self.input);
    }
    fn get_char(self: *Lexer, line: usize, char: usize) !u8 {
        if (line >= self.input.len) {
            return 0;
        }
        if (char == self.input[line].len) {
            return '\n';
        }
        if (char > self.input[line].len) {
            return 0;
        }
        return self.input[line][char];
    }
    fn read_ident(self: *Lexer) !Token {
        const start_char = self.char;
        var end_char = self.char + 1;
        self.char += 1;
        while (is_alnum(try self.get_char(self.line, end_char))) {
            end_char += 1;
            self.char += 1;
        }
        const str = self.input[self.line][start_char..end_char];
        if (std.mem.eql(u8, str, "var")) {
            return Token{
                .type = TokenType.Var,
                .start_line = self.line,
                .start_column = start_char,
                .end_column = end_char,
            };
        } else if (std.mem.eql(u8, str, "const")) {
            return Token{
                .type = TokenType.Const,
                .start_line = self.line,
                .start_column = start_char,
                .end_column = end_char,
            };
        } else {
            return Token{
                .type = TokenType{ .Ident = str },
                .start_line = self.line,
                .start_column = start_char,
                .end_column = end_char,
            };
        }
    }
    fn read_number(self: *Lexer) !Token {
        const start_char = self.char;
        var end_char = self.char + 1;
        self.char += 1;
        while (is_num(try self.get_char(self.line, end_char))) {
            end_char += 1;
            self.char += 1;
        }
        const str = self.input[self.line][start_char..end_char];
        return Token{
            .type = TokenType{ .Number = try std.fmt.parseFloat(f64, str) },
            .start_line = self.line,
            .start_column = start_char,
            .end_column = end_char,
        };
    }
    fn get_next_token(self: *Lexer) !Token {
        if (self.line != 0 and self.char != 0 and self.current_token.type == TokenType.EOF) {
            return self.current_token;
        }
        if (self.line >= self.input.len) {
            return Token{
                .type = TokenType.EOF,
                .start_line = self.line,
                .start_column = self.char,
                .end_column = self.char,
            };
        }
        if (self.char == self.input[self.line].len) {
            const line = self.line;
            const char = self.char;
            self.line += 1;
            self.char = 0;
            return Token{
                .type = TokenType.EndOfStatement,
                .start_line = line,
                .start_column = char,
                .end_column = char + 1,
            };
        }
        switch (self.input[self.line][self.char]) {
            'a'...'z', '_', 'A'...'Z' => return try self.read_ident(),
            ' ', '\t' => {
                self.char += 1;
                return try self.get_next_token();
            },
            '=' => {
                self.char += 1;
                return Token{
                    .type = TokenType.OpEq,
                    .start_line = self.line,
                    .start_column = self.char - 1,
                    .end_column = self.char,
                };
            },
            '0'...'9' => return try self.read_number(),
            else => return LexerError.InvalidChar,
        }
    }

    pub fn next_token(self: *Lexer) !Token {
        const token = try self.get_next_token();
        self.current_token = token;
        return token;
    }
    pub fn get_current_token(self: *Lexer) !Token {
        if (self.line == 0 and self.char == 0) {
            return try self.next_token();
        }
        return self.current_token;
    }
};
