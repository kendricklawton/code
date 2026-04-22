// Max depth of a binary tree (recursive).
//
//         4       depth = 3
//       /   \
//      2     7
//     / \   / \
//    1   3 6   9

#[allow(dead_code)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn node(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { val, left, right }))
}

fn leaf(val: i32) -> Option<Box<Node>> {
    node(val, None, None)
}

fn max_depth(root: Option<&Node>) -> i32 {
    match root {
        None => 0,
        Some(n) => 1 + max_depth(n.left.as_deref()).max(max_depth(n.right.as_deref())),
    }
}

fn main() {
    let root = node(4,
        node(2, leaf(1), leaf(3)),
        node(7, leaf(6), leaf(9)));

    let depth = max_depth(root.as_deref());
    println!("Tree depth: {}", depth);
    assert_eq!(depth, 3);
    println!("OK — depth matches expected");
}
