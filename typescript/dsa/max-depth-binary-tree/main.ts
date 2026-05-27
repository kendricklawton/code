// Max depth of a binary tree (recursive).
//
//         4       depth = 3
//       /   \
//      2     7
//     / \   / \
//    1   3 6   9

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

function maxDepth(root: TreeNode | null): number {
  if (root === null) return 0;
  return 1 + Math.max(maxDepth(root.left), maxDepth(root.right));
}

const root = new TreeNode(
  4,
  new TreeNode(2, new TreeNode(1), new TreeNode(3)),
  new TreeNode(7, new TreeNode(6), new TreeNode(9)),
);

const depth = maxDepth(root);
console.log(`Tree depth: ${depth}`);

if (depth !== 3) {
  throw new Error(`expected 3, got ${depth}`);
}
console.log("OK — depth matches expected");
