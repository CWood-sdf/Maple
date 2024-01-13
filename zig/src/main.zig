const std = @import("std");
const lex = @import("lexer.zig");
const parser = @import("parser.zig");

pub fn main() !void {
    std.debug.print("Hello, world!\n", .{});
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = alloc.allocator();
    const file = try std.fs.cwd().openFile("maple.mpl", std.fs.File.OpenFlags{});
    // this string must live for the duration of the interpreting process
    const fileStr = try file.readToEndAlloc(allocator, 10000000);
    defer allocator.free(fileStr);
    std.debug.print("file: {s}\n", .{fileStr});
    var lexer = try lex.Lexer.init(allocator, fileStr);
    defer lexer.deinit();
    var parse = parser.Parser.init(allocator, &lexer);
    _ = try parse.parse();
    // std.debug.print("parse: {s}\n", .{parse});
}
//
// test "simple test" {
//     var list = std.ArrayList(i32).init(std.testing.allocator);
//     defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
//     try list.append(42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
