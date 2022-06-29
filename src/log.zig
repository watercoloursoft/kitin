const std = @import("std");
const io = std.io;
const clap = @import("clap");

pub fn logErr(comptime format: []const u8, args: anytype) void {
    io.getStdErr().writer().print(format, args) catch {};
}

pub fn logOut(comptime format: []const u8, args: anytype) void {
    io.getStdOut().writer().print(format, args) catch {};
}

pub fn helpCommand(comptime summary: []const u8, comptime params: anytype, res: anytype) void {
    if (res.args.help) {
        logOut("{s}\n\n", .{summary});
        clap.help(io.getStdErr().writer(), clap.Help, params, .{}) catch {};
        logOut("\n", .{});
        std.os.exit(0);
    }
}
