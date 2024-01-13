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
    OpEqEq,
    OpPls,
    EOF,
    EndOfStatement,
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
    input: Arr([]u8) = undefined,
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
            .input = lines,
        };
    }
    pub fn deinit(self: *Lexer) void {
        self.input.deinit();
    }
    fn get_char(self: *Lexer, line: usize, char: usize) u8 {
        if (line >= self.input.items.len) {
            return 0;
        }
        if (char == self.input.items[line].len) {
            return '\n';
        }
        if (char > self.input.items[line].len) {
            return 0;
        }
        return self.input.items[line][char];
    }
    fn read_ident(self: *Lexer) !Token {
        const start_char = self.char;
        var end_char = self.char + 1;
        self.char += 1;
        while (is_alnum(self.get_char(self.line, end_char))) {
            end_char += 1;
            self.char += 1;
        }
        const str = self.input.items[self.line][start_char..end_char];
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
        while (is_num(self.get_char(self.line, end_char))) {
            end_char += 1;
            self.char += 1;
        }
        const str = self.input.items[self.line][start_char..end_char];
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
        if (self.line >= self.input.items.len) {
            return Token{
                .type = TokenType.EOF,
                .start_line = self.line,
                .start_column = self.char,
                .end_column = self.char,
            };
        }
        if (self.char == self.input.items[self.line].len) {
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
        switch (self.input.items[self.line][self.char]) {
            'a'...'z', '_', 'A'...'Z' => return try self.read_ident(),
            ' ', '\t' => {
                self.char += 1;
                return try self.get_next_token();
            },
            '+' => {
                self.char += 1;
                return Token{
                    .type = TokenType.OpPls,
                    .start_line = self.line,
                    .start_column = self.char - 1,
                    .end_column = self.char,
                };
            },
            '=' => {
                if (self.get_char(self.line, self.char + 1) == '=') {
                    self.char += 2;
                    return Token{
                        .type = TokenType.OpEqEq,
                        .start_line = self.line,
                        .start_column = self.char - 2,
                        .end_column = self.char,
                    };
                }
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

fn get_token_t(token: TokenType) u32 {
    switch (token) {
        .Var => return 1,
        .EOF => return 2,
        .EndOfStatement => return 3,
        .OpEq => return 4,
        .OpEqEq => return 5,
        .OpPls => return 6,
        .Ident => return 7,
        .Number => return 8,
        .Const => return 9,
    }
}
fn token_eq(a: TokenType, b: TokenType) bool {
    if (get_token_t(a) != get_token_t(b)) {
        std.debug.print("token types don't match\n", .{});
        return false;
    }
    switch (a) {
        .Ident => {
            std.debug.print("comparing idents {s} and {s}\n", .{ a.Ident, b.Ident });
            const res = std.mem.eql(u8, a.Ident, b.Ident);
            std.debug.print("result: {}\n", .{res});
            return res;
        },
        .Number => {
            std.debug.print("comparing numbers {f} and {f}\n", .{ a.Number, b.Number });
            const res = a.Number == b.Number;
            std.debug.print("result: {}\n", .{res});
            return res;
        },
        else => return true,
    }
}
fn expect_tokens(lexer: *Lexer, tokens: []const TokenType) !bool {
    for (tokens) |token| {
        const next_token = try lexer.next_token();
        if (!token_eq(next_token.type, token)) {
            return false;
        }
    }
    return true;
}

test "lexer 1" {
    const allocator = std.testing.allocator;
    const file = try std.fs.cwd().openFile("tests/lexer/lexer_1.mpl", std.fs.File.OpenFlags{});
    const fileStr = try file.readToEndAlloc(allocator, 10000000);
    defer allocator.free(fileStr);
    var lexer = try Lexer.init(allocator, fileStr);
    defer lexer.deinit();
    const xStr = try allocator.alloc(u8, 1);
    defer allocator.free(xStr);
    xStr[0] = 'x';
    const yStr = try allocator.alloc(u8, 1);
    defer allocator.free(yStr);
    yStr[0] = 'y';
    const zStr = try allocator.alloc(u8, 1);
    defer allocator.free(zStr);
    zStr[0] = 'z';
    const tokenArr = [_]TokenType{
        TokenType.Var,
        TokenType{ .Ident = xStr },
        TokenType.OpEq,
        TokenType{ .Number = 1.0 },
        TokenType.EndOfStatement,
        TokenType.Var,
        TokenType{ .Ident = yStr },
        TokenType.OpEq,
        TokenType{ .Number = 2.0 },
        TokenType.EndOfStatement,
        TokenType.Var,
        TokenType{ .Ident = zStr },
        TokenType.OpEq,
        TokenType{ .Ident = xStr },
        TokenType.OpPls,
        TokenType{ .Ident = yStr },
        TokenType.EndOfStatement,
        TokenType.EOF,
    };
    try std.testing.expect(try expect_tokens(&lexer, &tokenArr));
}
