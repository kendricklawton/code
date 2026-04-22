// Reverse a singly linked list (iterative, three-pointer).
//
// Before: 1 -> 2 -> 3 -> 4 -> 5
// After:  5 -> 4 -> 3 -> 2 -> 1
//
// Build:  cc -O2 -Wall -Wextra -o reverse reverse_linked_list.c
// Run:    ./reverse

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int val;
    struct Node *next;
} Node;

static Node *new_node(int val, Node *next) {
    Node *n = malloc(sizeof(*n));
    n->val = val;
    n->next = next;
    return n;
}

static Node *reverse(Node *head) {
    Node *prev = NULL, *curr = head;
    while (curr) {
        Node *next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    return prev;
}

static int to_array(Node *head, int *out, int cap) {
    int n = 0;
    while (head && n < cap) {
        out[n++] = head->val;
        head = head->next;
    }
    return n;
}

static Node *from_array(const int *xs, int n) {
    Node *head = NULL;
    for (int i = n - 1; i >= 0; i--) head = new_node(xs[i], head);
    return head;
}

static void free_list(Node *head) {
    while (head) {
        Node *next = head->next;
        free(head);
        head = next;
    }
}

static void print_ints(const char *label, const int *xs, int n) {
    printf("%s", label);
    for (int i = 0; i < n; i++) printf("%d ", xs[i]);
    printf("\n");
}

int main(void) {
    const int input[] = {1, 2, 3, 4, 5};
    const int expected[] = {5, 4, 3, 2, 1};
    const int n = (int)(sizeof(input) / sizeof(*input));

    Node *head = from_array(input, n);
    int buf[16];

    int len = to_array(head, buf, 16);
    print_ints("Before: ", buf, len);

    head = reverse(head);

    len = to_array(head, buf, 16);
    print_ints("After:  ", buf, len);

    assert(len == n);
    for (int i = 0; i < n; i++) assert(buf[i] == expected[i]);
    printf("OK — reversed list matches expected\n");

    free_list(head);
    return 0;
}
