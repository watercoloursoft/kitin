const std = @import("std");
const json = std.json;
const fs = std.fs;

pub const KIT_INFO = ".kitin";

pub const InitResult = enum {
    Initialised,
    Exists,
    Failed
};

pub fn command() InitResult {
    if (isKitinProject(fs.cwd()))
    {
        return .Exists;
    }
    return .Initialised;
}

pub fn isKitinProject(directory: fs.Dir) bool {
    directory.access(KIT_INFO, .{}) catch {
        return false;
    };
    return true;
}