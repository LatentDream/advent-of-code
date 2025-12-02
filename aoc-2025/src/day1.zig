const std = @import("std");
const aoc_2025 = @import("aoc_2025");

const fs = std.fs;
const print = std.debug.print;

fn part1Logic(curr: i64, side: u8, number: usize) i64 {
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

pub fn part1(lines: []const []const u8) !usize {
    var curr: i64 = 50;
    var nb_zero: usize = 0;

    for (lines) |line| {
        if (line.len < 2) continue;

        const side: u8 = line[0];
        const number_str = std.mem.trim(u8, line[1..], " ");
        const number = try std.fmt.parseInt(usize, number_str, 10);

        curr = part1Logic(curr, side, number);

        if (curr == 0) {
            nb_zero += 1;
        }
    }

    return nb_zero;
}

pub fn part2(lines: []const []const u8) !usize {
    var prev: i64 = 50;
    var nb_zero: usize = 0;

    std.debug.print("Starting part2 with prev={}\n", .{prev});

    for (lines) |line| {
        if (line.len < 2) continue;
        const side: u8 = line[0];
        const number_str = std.mem.trim(u8, line[1..], " ");
        const step = try std.fmt.parseInt(usize, number_str, 10);

        const nb_rotation = step / 100;
        const remainder = @mod(step, 100);
        var next: i64 = 0;
        if (side == 'L') {
            next = prev - @as(i64, @intCast(remainder));
        } else {
            next = prev + @as(i64, @intCast(remainder));
        }

        nb_zero += nb_rotation;

        if (next > 99 or (next == 0 and remainder > 0)) {
            nb_zero += 1;
        } else if (next < 0 and prev != 0) {
            nb_zero += 1;
        }

        next = @mod(next, 100);
        if (next < 0) {
            next = 100 - next;
        }

        prev = next;
    }

    std.debug.print("Final result: {}\n", .{nb_zero});
    return nb_zero;
}

fn day1() !void {
    const file = try fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var lines: std.ArrayList([]const u8) = .empty;

    defer {
        for (lines.items) |line| {
            allocator.free(line);
        }
        lines.deinit(allocator);
    }

    while (try reader.interface.takeDelimiter('\n')) |line| {
        const owned_line = try allocator.dupe(u8, line);
        try lines.append(allocator, owned_line);
    }

    const result1 = try part1(lines.items);
    print("Part 1 - Number of zero: {d}\n", .{result1});

    const result2 = try part2(lines.items);
    print("Part 2 - Number of zero: {d}\n", .{result2});
}

pub fn main() !void {
    try day1();
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

    const result = part1(&lines);
    try std.testing.expectEqual(3, result);
}

test "part2" {
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

    const result = part2(&lines);
    try std.testing.expectEqual(6, result);
}

test "part2 - multiple pass" {
    const lines = [_][]const u8{
        "L1000",
    };

    const result = part2(&lines);
    try std.testing.expectEqual(10, result);
}

test "part2 - return 2" {
    const lines = [_][]const u8{
        "R150",
        "R50",
    };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}
test "part2 - return 2 - 2" {
    const lines = [_][]const u8{ "L150", "L50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}
test "part2 - return 2 - 3" {
    const lines = [_][]const u8{ "L150", "R50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}
test "part2 - return 2 - 4" {
    const lines = [_][]const u8{ "R150", "L50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}

test "part2 - return 1 - 1" {
    const lines = [_][]const u8{ "R50", "L50" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}
test "part2 - return 1 - 2" {
    const lines = [_][]const u8{ "L50", "R50" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}
test "part2 - return 1 - 3" {
    const lines = [_][]const u8{ "L75", "R20" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}
test "part2 - return 1 - 4" {
    const lines = [_][]const u8{ "R75", "L20" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}

test "Left ending on zero" {
    const lines = [_][]const u8{ "L50", "R50" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}

test "Right ending on zero" {
    const lines = [_][]const u8{ "R50", "R50" };

    const result = part2(&lines);
    try std.testing.expectEqual(1, result);
}

test "Basic pass to the left" {
    const lines = [_][]const u8{"L200"};

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}

test "Basic pass to the right" {
    const lines = [_][]const u8{"R200"};

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}

test "Extra pass Left landing on zero 0" {
    const lines = [_][]const u8{ "R150", "L50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}

test "Extra pass Left landing on zero" {
    const lines = [_][]const u8{ "R150", "R50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}

test "Extra pass Right landing on zero" {
    const lines = [_][]const u8{ "R150", "L50" };

    const result = part2(&lines);
    try std.testing.expectEqual(2, result);
}
