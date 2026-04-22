// Invert a binary tree (mirror left/right).
/*
        4                 4
      /   \             /   \
     2     7    -->    7     2
    / \   / \         / \   / \
   1   3 6   9       9   6 3   1
*/
// Build:   cc -O2 -Wall -Wextra -o invert invert_binary_tree.c
// Run:     ./invert

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

static void invert(Node *root) {
    if (!root) return;
    Node *tmp = root->left;
    root->left = root->right;
    root->right = tmp;
    invert(root->left);
    invert(root->right);
}

static int level_order(Node *root, int *out, int cap) {
    if (!root) return 0;
    Node *queue[64];
    int head = 0, tail = 0, count = 0;
    queue[tail++] = root;
    while (head < tail && count < cap) {
        Node *n = queue[head++];
        out[count++] = n->val;
        if (n->left)  queue[tail++] = n->left;
        if (n->right) queue[tail++] = n->right;
    }
    return count;
}

static void print_ints(const char *label, const int *xs, int n) {
    printf("%s", label);
    for (int i = 0; i < n; i++) printf("%d ", xs[i]);
    printf("\n");
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

    int buf[16];
    int n = level_order(root, buf, 16);
    print_ints("Before: ", buf, n);

    invert(root);

    n = level_order(root, buf, 16);
    print_ints("After:  ", buf, n);

    const int expected[] = {4, 7, 2, 9, 6, 3, 1};
    assert(n == (int)(sizeof(expected) / sizeof(*expected)));
    for (int i = 0; i < n; i++) assert(buf[i] == expected[i]);
    printf("OK — inverted tree matches expected\n");

    free_tree(root);
    return 0;
}
