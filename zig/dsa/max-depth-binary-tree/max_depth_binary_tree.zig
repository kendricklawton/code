// Max depth of a binary tree (recursive).
//
//         4       depth = 3
//       /   \
//      2     7
//     / \   / \
//    1   3 6   9
//
// Build/run:  zig run max_depth_binary_tree.zig

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

fn maxDepth(root: ?*Node) i32 {
    const r = root orelse return 0;
    const l = maxDepth(r.left);
    const rd = maxDepth(r.right);
    return 1 + @max(l, rd);
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

    const depth = maxDepth(root);
    std.debug.print("Tree depth: {d}\n", .{depth});
    std.debug.assert(depth == 3);
    std.debug.print("OK — depth matches expected\n", .{});
}
