const std = @import("std");

fn parseInput(allocator: std.mem.Allocator, filename: []const u8) !std.ArrayList([]const u8) {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    var lines = std.ArrayList([]const u8).init(allocator);
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const line_copy = try allocator.dupe(u8, line);
        try lines.append(line_copy);
    }

    return lines;
}

fn part1(lines: std.ArrayList([]const u8)) !i64 {
    _ = lines;
    // TODO: Implement part 1
    return 0;
}

fn part2(lines: std.ArrayList([]const u8)) !i64 {
    _ = lines;
    // TODO: Implement part 2
    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: {s} <input_file>\n", .{args[0]});
        return error.MissingArgument;
    }

    const filename = args[1];
    const lines = try parseInput(allocator, filename);
    defer {
        for (lines.items) |line| {
            allocator.free(line);
        }
        lines.deinit();
    }

    const result1 = try part1(lines);
    const result2 = try part2(lines);

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Part 1: {d}\n", .{result1});
    try stdout.print("Part 2: {d}\n", .{result2});
}
