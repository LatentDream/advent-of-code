const std = @import("std");
const aoc_2025 = @import("aoc_2025");

const fs = std.fs;
const print = std.debug.print;

fn countDigits(n: usize) usize {
    if (n == 0) return 1;
    var count: usize = 1;
    var num = n;
    while (num >= 10) {
        count += 1;
        num /= 10;
    }
    return count;
}

fn pow10(exp: usize) usize {
    var result: usize = 1;
    var i: usize = 0;
    while (i < exp) : (i += 1) {
        result *= 10;
    }
    return result;
}

fn isValid(number: usize) bool {
    const digit_count = countDigits(number);

    if (digit_count % 2 != 0) {
        return false;
    }

    const half_digits = digit_count / 2;
    const divisor = pow10(half_digits);

    const left_half = number / divisor;
    const right_half = number % divisor;

    return left_half == right_half;
}

pub fn part1(ranges: []const []const u8) !usize {
    var total: usize = 0;
    for (ranges) |range| {
        var it = std.mem.splitScalar(u8, range, '-');
        var begin: usize = 0;
        var end: usize = 0;
        if (it.next()) |begin_str| {
            const trimmed_begin = std.mem.trim(u8, begin_str, " \n\r\t");
            begin = try std.fmt.parseInt(usize, trimmed_begin, 10);
            print("Begin: {}\n", .{begin});
        }
        if (it.next()) |end_str| {
            print("Endstr: '{s}'\n", .{end_str});
            const trimmed_end = std.mem.trim(u8, end_str, " \n\r\t");
            print("Trimmed endstr: '{s}'\n", .{trimmed_end});
            end = try std.fmt.parseInt(usize, trimmed_end, 10);
            print("End: {}\n", .{end});
        }
        std.debug.print("{} - {}\n", .{ begin, end });

        while (begin <= end) {
            if (isValid(begin)) {
                total += begin;
            }
            begin += 1;
        }
    }
    return total;
}

pub fn part2(_: []const []const u8) !usize {
    return 1;
}

fn day2() !void {
    const file = try fs.cwd().openFile("input/day2.txt", .{});
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

    while (try reader.interface.takeDelimiter(',')) |line| {
        const owned_line = try allocator.dupe(u8, line);
        try lines.append(allocator, owned_line);
    }

    const result1 = try part1(lines.items);
    print("Part 1: {d}\n", .{result1});

    const result2 = try part2(lines.items);
    print("Part 2: {d}\n", .{result2});
}

pub fn main() !void {
    try day2();
}

test "processLines - full workflow" {
    const lines = [_][]const u8{
        "11-22",                 "95-115",
        "998-1012",              "1188511880-1188511890",
        "222220-222224",         "1698522-1698528",
        "446443-446449",         "38593856-38593862",
        "565653-565659",         "824824821-824824827",
        "2121212118-2121212124",
    };

    const result = part1(&lines);
    try std.testing.expectEqual(1227775554, result);
}

// test "part2" {
//     const lines = [_][]const u8{
//         "L68",
//         "L30",
//         "R48",
//         "L5",
//         "R60",
//         "L55",
//         "L1",
//         "L99",
//         "R14",
//         "L82",
//     };
//
//     const result = part2(&lines);
//     try std.testing.expectEqual(6, result);
// }
