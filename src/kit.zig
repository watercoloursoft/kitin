const std = @import("std");
const json = std.json;
const fs = std.fs;

const log = @import("log.zig");
const logOut = log.logOut;

pub const KIT_PROJECT = ".kitproj";
pub const READ_PROJECT_BUFFER_SIZE = 100_000;

pub const InitResult = enum { Initialised, Exists };

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
        var isProject = isKitProject(directory);

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
};

pub fn command() !InitResult {
    var gp = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    defer _ = gp.deinit();
    var allocator = gp.allocator();

    var project = KitProject.init(allocator);
    defer project.deinit();

    if (isKitProject(fs.cwd())) {
        try project.loadFromDirectory(fs.cwd());
        std.debug.print("{s}\n", .{project.name});
        logOut("Kitin already exists.\n", .{});

        return .Exists;
    }

    var buf: [std.fs.MAX_PATH_BYTES] u8 = undefined;
    const cwd = try std.os.getcwd(&buf);
    var split = std.mem.split(u8, cwd, fs.path.sep_str);
    while (split.next()) |chunk| {
        std.debug.print("e {s}\n", .{chunk});
    }
    logOut("Initialising kitin project with name {s}.\n", .{cwd});

    return .Initialised;
}

pub fn isKitProject(directory: fs.Dir) bool {
    directory.access(KIT_PROJECT, .{}) catch {
        return false;
    };
    return true;
}
