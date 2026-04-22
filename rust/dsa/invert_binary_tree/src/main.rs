// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    match root {
        Some(mut node) => {
            // Recursively invert the left and right subtrees
            let inverted_right = invert_tree(node.right);
            let inverted_left = invert_tree(node.left);

            // Swap the children
            node.left = inverted_right;
            node.right = inverted_left;

            Some(node)
        }
        None => None,
    }
}

fn main() {
    // Constructing the following sample tree:
    //       4
    //     /   \
    //    2     7
    //   / \   / \
    //  1   3 6   9

    let mut root = TreeNode::new(4);

    // Build the left subtree
    let mut left_child = TreeNode::new(2);
    left_child.left = Some(Box::new(TreeNode::new(1)));
    left_child.right = Some(Box::new(TreeNode::new(3)));

    // Build the right subtree
    let mut right_child = TreeNode::new(7);
    right_child.left = Some(Box::new(TreeNode::new(6)));
    right_child.right = Some(Box::new(TreeNode::new(9)));

    // Attach children to the root
    root.left = Some(Box::new(left_child));
    root.right = Some(Box::new(right_child));

    // Wrap the root in Option<Box<T>> as expected by the function
    let tree = Some(Box::new(root));

    println!("--- Original Tree ---");
    println!("{:#?}", tree);

    // Invert the tree
    let inverted_tree = invert_tree(tree);

    println!("\n--- Inverted Tree ---");
    println!("{:#?}", inverted_tree);
}
