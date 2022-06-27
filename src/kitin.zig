const std = @import("std");
const io = std.io;

const initCommand = @import("init.zig");

pub const Commands = enum {
    git,
    init,
    exec,
    project,

    help
};

fn outErr(comptime format: [] const u8, args: anytype) void {
    io.getStdErr().writer().print(format, args) catch {};
}

pub fn handleCliCommand(argIterator: *std.process.ArgIterator) !void {
    _ = argIterator.skip();

    const commandName = argIterator.next() orelse showHelp();
    const command = std.meta.stringToEnum(Commands, commandName);
    if (command == null)
    {
        unknownCommand(commandName);
    }

    switch (command.?) {
        .init => {
            outErr("Res: {b}", .{initCommand.command()});
        },
        .help => showHelp(),
        else => {
            outErr("implementation for: {s} not implemented\n", .{commandName});
        },
    }
}

pub fn showHelp() noreturn {
    outErr("{s}", .{
        \\
        \\ Usage: kitin [command]
        \\
        \\ Commands:
        \\
        \\
    });
    inline for (std.meta.fields(Commands)) |command| {
        outErr(
            \\    {s}
            \\
        , .{command.name});
    }
    std.os.exit(0);
}

fn unknownCommand(commandName: [] const u8) noreturn {
    outErr("Unknown command: {s}", .{commandName});
    std.os.exit(1);
}