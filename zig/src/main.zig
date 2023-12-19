const std = @import("std");
const lex = @import("lexer.zig");

pub fn main() !void {
    std.debug.print("Hello, world!\n", .{});
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = alloc.allocator();
    const file = try std.fs.cwd().openFile("maple.mpl", std.fs.File.OpenFlags{});
    // this string must live for the duration of the interpreting process
    const fileStr = try file.readToEndAlloc(allocator, 10000000);
    std.debug.print("file: {s}\n", .{fileStr});
    var lexer = try lex.Lexer.init(allocator, fileStr);
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    std.debug.print("token: {}\n", .{try lexer.next_token()});
    switch ((try lexer.get_current_token()).type) {
        .Ident => |str| std.debug.print("ident: {s}\n", .{str}),
        else => |token| std.debug.print("token: {}\n", .{token}),
    }
}
//
// test "simple test" {
//     var list = std.ArrayList(i32).init(std.testing.allocator);
//     defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
//     try list.append(42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
