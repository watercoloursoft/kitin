const std = @import("std");
const clap = @import("clap");

const debug = std.debug;
const io = std.io;

pub fn main() anyerror!void {
    const params = comptime clap.parseParamsComptime(
        \\-h, --help     Display this help and exit.
        \\-v, --version  Output version information and exit.
        \\
    );

    var res = try clap.parse(clap.Help, &params, clap.parsers.default, .{});
    defer res.deinit();

    // `clap.help` is a function that can print a simple help message. It can print any `Param`
    // where `Id` has a `describtion` and `value` method (`Param(Help)` is one such parameter).
    // The last argument contains options as to how `help` should print those parameters. Using
    // `.{}` means the default options.
    if (res.args.help)
        return clap.help(std.io.getStdErr().writer(), clap.Help, &params, .{});
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
