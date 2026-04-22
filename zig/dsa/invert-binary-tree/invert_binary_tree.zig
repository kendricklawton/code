// Invert a binary tree (mirror left/right).
//
//        4                 4
//      /   \             /   \
//     2     7    -->    7     2
//    / \   / \         / \   / \
//   1   3 6   9       9   6 3   1
//
// Build/run:  zig run invert_binary_tree.zig

const std = @import("std");

const Node = struct {
    val: i32,
    left: ?*Node = null,
    right: ?*Node = null,
};

fn newNode(alloc: std.mem.Allocator, val: i32, left: ?*Node, right: ?*Node) !*Node {
    const n = try alloc.create(Node);
    n.* = .{ .val = val, .left = left, .right = right };
    return n;
}

fn invert(root: ?*Node) void {
    const r = root orelse return;
    const tmp = r.left;
    r.left = r.right;
    r.right = tmp;
    invert(r.left);
    invert(r.right);
}

fn levelOrder(root: ?*Node, out: []i32) usize {
    const r = root orelse return 0;
    var queue: [32]*Node = undefined;
    var head: usize = 0;
    var tail: usize = 0;
    queue[tail] = r;
    tail += 1;
    var count: usize = 0;
    while (head < tail) {
        const n = queue[head];
        head += 1;
        out[count] = n.val;
        count += 1;
        if (n.left) |l| {
            queue[tail] = l;
            tail += 1;
        }
        if (n.right) |rc| {
            queue[tail] = rc;
            tail += 1;
        }
    }
    return count;
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

    const root = try newNode(a, 4,
        try newNode(a, 2,
            try newNode(a, 1, null, null),
            try newNode(a, 3, null, null)),
        try newNode(a, 7,
            try newNode(a, 6, null, null),
            try newNode(a, 9, null, null)));

    var buf: [16]i32 = undefined;

    var n = levelOrder(root, &buf);
    printInts("Before: ", buf[0..n]);

    invert(root);

    n = levelOrder(root, &buf);
    printInts("After:  ", buf[0..n]);

    const expected = [_]i32{ 4, 7, 2, 9, 6, 3, 1 };
    std.debug.assert(n == expected.len);
    for (buf[0..n], expected) |got, exp| std.debug.assert(got == exp);
    std.debug.print("OK — inverted tree matches expected\n", .{});
}
