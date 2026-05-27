// Invert a binary tree (recursive).
//
//   Original:        Inverted:
//        4               4
//      /   \           /   \
//     2     7         7     2
//    / \   / \       / \   / \
//   1   3 6   9     9   6 3   1

class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;

  constructor(val = 0, left: TreeNode | null = null, right: TreeNode | null = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function invertTree(root: TreeNode | null): TreeNode | null {
  if (root === null) return null;
  [root.left, root.right] = [invertTree(root.right), invertTree(root.left)];
  return root;
}

const root = new TreeNode(
  4,
  new TreeNode(2, new TreeNode(1), new TreeNode(3)),
  new TreeNode(7, new TreeNode(6), new TreeNode(9)),
);

console.log("--- Original Tree ---");
console.log(JSON.stringify(root, null, 2));

const inverted = invertTree(root);

console.log("\n--- Inverted Tree ---");
console.log(JSON.stringify(inverted, null, 2));
