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

fn isValid_part1(number: usize) bool {
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
        }
        if (it.next()) |end_str| {
            const trimmed_end = std.mem.trim(u8, end_str, " \n\r\t");
            end = try std.fmt.parseInt(usize, trimmed_end, 10);
        }

        while (begin <= end) {
            if (isValid_part1(begin)) {
                total += begin;
            }
            begin += 1;
        }
    }
    return total;
}

fn getFilter(allocator: std.mem.Allocator, firstHalf: []const u8) ![][]const u8 {
    var filters: std.ArrayList([]const u8) = .empty;
    defer filters.deinit(allocator);

    // Generate progressive prefixes: "1", "12", "123"
    for (1..firstHalf.len + 1) |i| {
        const prefix = try allocator.dupe(u8, firstHalf[0..i]);
        try filters.append(allocator, prefix);
    }

    return try allocator.dupe([]const u8, filters.items);
}

fn isRepeatingPattern(text: []const u8, pattern: []const u8) bool {
    if (pattern.len == 0) return false;
    if (text.len % pattern.len != 0) return false;
    var pos: usize = 0;

    while (pos + pattern.len <= text.len) {
        if (!std.mem.eql(u8, text[pos .. pos + pattern.len], pattern)) {
            return false;
        }
        pos += pattern.len;
    }

    if (pos < text.len) {
        const remaining = text[pos..];
        if (!std.mem.eql(u8, remaining, pattern[0..remaining.len])) {
            return false;
        }
    }

    return true;
}

fn isValid_part2(allocator: std.mem.Allocator, number: usize) !bool {
    var buffer: [32]u8 = undefined;
    const number_str = try std.fmt.bufPrint(buffer[0..], "{}", .{number});

    if (number_str.len < 2) {
        return false;
    }

    const half_len = number_str.len / 2;
    const left_half = number_str[0..half_len];
    const right_half = number_str[half_len..];

    const filters = try getFilter(allocator, left_half);
    defer {
        for (filters) |filter| {
            allocator.free(filter);
        }
        allocator.free(filters);
    }

    for (filters) |filter| {
        if (isRepeatingPattern(number_str, filter)) {
            return true;
        }
    }

    return std.mem.eql(u8, left_half, right_half);
}

pub fn part2(ranges: []const []const u8) !usize {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var total: usize = 0;
    for (ranges) |range| {
        var it = std.mem.splitScalar(u8, range, '-');
        var begin: usize = 0;
        var end: usize = 0;
        if (it.next()) |begin_str| {
            const trimmed_begin = std.mem.trim(u8, begin_str, " \n\r\t");
            begin = try std.fmt.parseInt(usize, trimmed_begin, 10);
        }
        if (it.next()) |end_str| {
            const trimmed_end = std.mem.trim(u8, end_str, " \n\r\t");
            end = try std.fmt.parseInt(usize, trimmed_end, 10);
        }

        var valid_count: usize = 0;
        while (begin <= end) {
            const is_valid = try isValid_part2(allocator, begin);
            if (is_valid) {
                print("{}\n", .{begin});
                total += begin;
                valid_count += 1;
            }
            begin += 1;
        }
    }
    return total;
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

test "part1" {
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

test "part2" {
    const lines = [_][]const u8{
        "11-22",                 "95-115",
        "998-1012",              "1188511880-1188511890",
        "222220-222224",         "1698522-1698528",
        "446443-446449",         "38593856-38593862",
        "565653-565659",         "824824821-824824827",
        "2121212118-2121212124",
    };

    const result = part2(&lines);
    try std.testing.expectEqual(4174379265, result);
}

test "part2 - should not match" {
    const lines = [_][]const u8{ "1231233-1231234", "1113111-1113112", "3111111-3111112", "0-9", "269226922-269226923" };

    const result = part2(&lines);
    try std.testing.expectEqual(0, result);
}

test "part2 - should match" {
    const lines = [_][]const u8{"824824824-824824825"};

    const result = part2(&lines);
    try std.testing.expectEqual(824824824, result);
}
