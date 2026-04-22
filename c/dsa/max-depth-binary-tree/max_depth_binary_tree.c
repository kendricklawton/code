// Max depth of a binary tree (recursive).
/*
        4       depth = 3
      /   \
     2     7
    / \   / \
   1   3 6   9
*/
// Build:  cc -O2 -Wall -Wextra -o depth max_depth_binary_tree.c
// Run:    ./depth

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int val;
    struct Node *left, *right;
} Node;

static Node *new_node(int val, Node *left, Node *right) {
    Node *n = malloc(sizeof(*n));
    n->val = val;
    n->left = left;
    n->right = right;
    return n;
}

static int max_depth(Node *root) {
    if (!root) return 0;
    int l = max_depth(root->left);
    int r = max_depth(root->right);
    return 1 + (l > r ? l : r);
}

static void free_tree(Node *root) {
    if (!root) return;
    free_tree(root->left);
    free_tree(root->right);
    free(root);
}

int main(void) {
    Node *root = new_node(4,
        new_node(2, new_node(1, NULL, NULL), new_node(3, NULL, NULL)),
        new_node(7, new_node(6, NULL, NULL), new_node(9, NULL, NULL)));

    int depth = max_depth(root);
    printf("Tree depth: %d\n", depth);
    assert(depth == 3);
    printf("OK — depth matches expected\n");

    free_tree(root);
    return 0;
}
