const io = @import("std").io;

pub fn logErr(comptime format: [] const u8, args: anytype) void {
    io.getStdErr().writer().print(format, args) catch {};
}

pub fn logOut(comptime format: [] const u8, args: anytype) void {
    io.getStdOut().writer().print(format, args) catch {};
}