package main

import (
	"encoding/json"
	"fmt"
)

// Definition for a binary tree node.
// We add json tags to make the pretty-printing look cleaner.
type TreeNode struct {
	Val   int       `json:"val"`
	Left  *TreeNode `json:"left,omitempty"`
	Right *TreeNode `json:"right,omitempty"`
}

func invertTree(root *TreeNode) *TreeNode {
	// Base case: if the tree is empty, return nil
	if root == nil {
		return nil
	}

	// Simultaneously swap and recursively invert the left and right children
	root.Left, root.Right = invertTree(root.Right), invertTree(root.Left)

	return root
}

// A helper function to pretty-print our tree using JSON formatting
func printTree(title string, root *TreeNode) {
	fmt.Printf("--- %s ---\n", title)

	// MarshalIndent formats the output with spaces for readability
	prettyJSON, err := json.MarshalIndent(root, "", "  ")
	if err != nil {
		fmt.Println("Error printing tree:", err)
		return
	}

	fmt.Println(string(prettyJSON))
	fmt.Println()
}

func main() {
	// Constructing the following sample tree:
	//       4
	//     /   \
	//    2     7
	//   / \   / \
	//  1   3 6   9

	root := &TreeNode{
		Val: 4,
		Left: &TreeNode{
			Val:   2,
			Left:  &TreeNode{Val: 1},
			Right: &TreeNode{Val: 3},
		},
		Right: &TreeNode{
			Val:   7,
			Left:  &TreeNode{Val: 6},
			Right: &TreeNode{Val: 9},
		},
	}

	// Print the original tree
	printTree("Original Tree", root)

	// Invert the tree
	invertedTree := invertTree(root)

	// Print the inverted tree
	printTree("Inverted Tree", invertedTree)
}
