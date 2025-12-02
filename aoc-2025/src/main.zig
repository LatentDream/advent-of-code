const std = @import("std");
const aoc_2025 = @import("aoc_2025");

const fs = std.fs;
const print = std.debug.print;

fn processLine(curr: i64, side: u8, number: usize) i64 {
    if (side == 'L') {
        var out: i64 = curr - @as(i64, @intCast(number));
        while (out < 0) {
            out += 100;
        }
        return out;
    } else {
        const num_i64 = @as(i64, @intCast(number));
        return @mod(curr + num_i64, 100);
    }
    return curr;
}

pub fn processLines(lines: []const []const u8) !usize {
    var curr: i64 = 50;
    var nb_zero: usize = 0;

    for (lines) |line| {
        if (line.len < 2) continue;

        const side: u8 = line[0];
        const number_str = std.mem.trim(u8, line[1..], " ");
        const number = try std.fmt.parseInt(usize, number_str, 10);

        curr = processLine(curr, side, number);

        if (curr == 0) {
            nb_zero += 1;
        }
    }

    return nb_zero;
}

fn part1() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var lines: std.ArrayList([]const u8) = .empty;

    // Later, when you append (or appendSlice), you pass the allocator:
    defer {
        for (lines.items) |line| {
            allocator.free(line);
        }
        lines.deinit(allocator);
    }

    // Read all lines into memory
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const owned_line = try allocator.dupe(u8, line);
        try lines.append(allocator, owned_line);
    }

    const result = try processLines(lines.items);
    print("Part 1 - Number of zero: {d}\n", .{ result });
}

pub fn main() !void {
    try part1();
}

test "processLines - full workflow" {
    const lines = [_][]const u8{
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
    };

    const result = processLines(&lines);
    try std.testing.expectEqual(3, result);
}
