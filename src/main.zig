const std = @import("std");
const clap = @import("clap");

const kitin = @import("cli.zig");

const debug = std.debug;
const io = std.io;

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    var argIterator = try std.process.ArgIterator.initWithAllocator(allocator);
    defer argIterator.deinit();

    try kitin.handleCliCommand(allocator, &argIterator);
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
