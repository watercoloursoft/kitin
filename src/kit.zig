const std = @import("std");
const json = std.json;
const fs = std.fs;

const log = @import("log.zig");
const logOut = log.logOut;
const logErr = log.logErr;
const helpCommand = log.helpCommand;

const clap = @import("clap");

pub const KIT_PROJECT = ".kitproj";
pub const READ_PROJECT_BUFFER_SIZE = 100_000;

pub const KitModule = struct {};

pub const KitProject = struct {
    name: ?[]const u8 = undefined,

    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) KitProject {
        return KitProject{ .allocator = allocator };
    }

    pub fn deinit(self: *KitProject) void {
        if (self.name) |nameMemory|
            self.allocator.free(nameMemory);
    }

    pub const LoadFromFileError = error{ NotProject, InvalidName };

    pub fn loadFromDirectory(self: *KitProject, directory: fs.Dir) !void {
        var isProject = KitProject.isDirectoryKitProject(directory);

        if (!isProject) {
            return LoadFromFileError.NotProject;
        }

        self.loadFromFile(directory, KIT_PROJECT) catch |e| {
            _ = try std.io.getStdErr().writer().print("{e}\n", .{e});
            return e;
        };
    }

    pub fn loadFromFile(self: *KitProject, directory: fs.Dir, projectFile: []const u8) !void {
        var file = try directory.openFile(projectFile, .{});
        defer file.close();

        const fileBuffer = try file.readToEndAlloc(self.allocator, READ_PROJECT_BUFFER_SIZE);
        defer self.allocator.free(fileBuffer);

        var parser = json.Parser.init(self.allocator, false);
        defer parser.deinit();

        var tree = try parser.parse(fileBuffer);
        defer tree.deinit();

        var rootObject = tree.root.Object;
        if (rootObject.get("name")) |nameValue| {
            switch (nameValue) {
                .String => |*name| {
                    var temp = try self.allocator.alloc(u8, name.len);
                    self.name = temp;
                    std.mem.copy(u8, temp, name.*);
                },
                else => {
                    return LoadFromFileError.InvalidName;
                },
            }
        }
    }

    pub fn isDirectoryKitProject(directory: fs.Dir) bool {
        directory.access(KIT_PROJECT, .{}) catch {
            return false;
        };
        return true;
    }
};

pub fn command(
    allocator: std.mem.Allocator,
    argIterator: anytype,
) !void {
    var project = KitProject.init(allocator);
    defer project.deinit();

    const summary = "Initialises an empty kit project.";
    const params = comptime clap.parseParamsComptime(
        \\-h, --help             Help message
        \\<str>...
        \\
    );

    if (KitProject.isDirectoryKitProject(fs.cwd())) {
        try project.loadFromDirectory(fs.cwd());
        logOut("Kitin project \"{s}\" already exists.\n", .{project.name});
        return;
    }

    var diag: clap.Diagnostic = undefined;
    var res = clap.parseEx(clap.Help, &params, clap.parsers.default, argIterator, .{
        .allocator = allocator,
        .diagnostic = &diag,
    }) catch |err| {
        // Report any useful error and exit
        diag.report(std.io.getStdErr().writer(), err) catch {};
        return err;
    };
    defer res.deinit();
    helpCommand(summary, &params, res);

    var projectName: []const u8 = undefined;

    if (res.positionals.len < 1) {
        var buf: [std.fs.MAX_PATH_BYTES]u8 = undefined;
        const cwd = try std.os.getcwd(&buf);
        var split = std.mem.split(u8, cwd, fs.path.sep_str);
        while (split.next()) |chunk| {
            projectName = chunk;
        }
    } else {
        projectName = res.positionals[0];
    }

    logOut("Initialising kit project with name {s}.\n", .{projectName});
}
