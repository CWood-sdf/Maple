const std = @import("std");
const Tuple = std.meta.Tuple(&.{ *AstNode, *AstNode });
pub const AstType = union(enum) {
    OpEq: Tuple,
    Number: u64,
};

pub const AstNode = struct {
    kind: AstType,
    line: u64,
    col: u64,
};
