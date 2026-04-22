// Reverse a singly linked list (iterative, three-pointer).
//
// Before: 1 -> 2 -> 3 -> 4 -> 5
// After:  5 -> 4 -> 3 -> 2 -> 1
//
// Build/run:  zig run reverse_linked_list.zig

const std = @import("std");

const Node = struct {
    val: i32,
    next: ?*Node = null,
};

fn newNode(alloc: std.mem.Allocator, val: i32, next: ?*Node) !*Node {
    const n = try alloc.create(Node);
    n.* = .{ .val = val, .next = next };
    return n;
}

fn reverse(head: ?*Node) ?*Node {
    var prev: ?*Node = null;
    var curr = head;
    while (curr) |c| {
        const next = c.next;
        c.next = prev;
        prev = c;
        curr = next;
    }
    return prev;
}

fn toArray(head: ?*Node, out: []i32) usize {
    var count: usize = 0;
    var curr = head;
    while (curr) |c| {
        out[count] = c.val;
        count += 1;
        curr = c.next;
    }
    return count;
}

fn fromArray(alloc: std.mem.Allocator, xs: []const i32) !?*Node {
    var head: ?*Node = null;
    var i = xs.len;
    while (i > 0) {
        i -= 1;
        head = try newNode(alloc, xs[i], head);
    }
    return head;
}

fn printInts(label: []const u8, xs: []const i32) void {
    std.debug.print("{s}", .{label});
    for (xs) |x| std.debug.print("{d} ", .{x});
    std.debug.print("\n", .{});
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const a = arena.allocator();

    const input = [_]i32{ 1, 2, 3, 4, 5 };
    var head = try fromArray(a, &input);

    var buf: [16]i32 = undefined;
    var n = toArray(head, &buf);
    printInts("Before: ", buf[0..n]);

    head = reverse(head);

    n = toArray(head, &buf);
    printInts("After:  ", buf[0..n]);

    const expected = [_]i32{ 5, 4, 3, 2, 1 };
    std.debug.assert(n == expected.len);
    for (buf[0..n], expected) |got, exp| std.debug.assert(got == exp);
    std.debug.print("OK — reversed list matches expected\n", .{});
}
