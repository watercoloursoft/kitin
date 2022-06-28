const std = @import("std");
const io = std.io;

const kit = @import("kit.zig");

const log = @import("log.zig");
const logErr = log.logErr;

pub const Commands = enum {
    git,
    init,
    exec,
    project,

    help
};

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
            logErr("Res: {}", .{kit.command()});
        },
        .help => showHelp(),
        else => {
            logErr("{s} is not implemented\n", .{commandName});
        },
    }
}

pub fn showHelp() noreturn {
    logErr("{s}", .{
        \\
        \\ Usage: kitin [command]
        \\
        \\ Commands:
        \\
        \\
    });
    inline for (std.meta.fields(Commands)) |command| {
        logErr(
            \\    {s}
            \\
        , .{command.name});
    }
    std.os.exit(0);
}

fn unknownCommand(commandName: [] const u8) noreturn {
    logErr("Unknown command: {s}", .{commandName});
    std.os.exit(1);
}
