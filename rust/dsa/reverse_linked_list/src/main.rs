// Reverse a singly linked list (iterative, three-pointer).
//
// Before: 1 -> 2 -> 3 -> 4 -> 5
// After:  5 -> 4 -> 3 -> 2 -> 1

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut curr) = head {
        head = curr.next.take();
        curr.next = prev;
        prev = Some(curr);
    }
    prev
}

fn to_vec(mut head: Option<&Node>) -> Vec<i32> {
    let mut out = vec![];
    while let Some(n) = head {
        out.push(n.val);
        head = n.next.as_deref();
    }
    out
}

fn from_vec(xs: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in xs.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn main() {
    let head = from_vec(&[1, 2, 3, 4, 5]);
    println!("Before: {:?}", to_vec(head.as_deref()));

    let head = reverse(head);

    let result = to_vec(head.as_deref());
    println!("After:  {:?}", result);

    assert_eq!(result, vec![5, 4, 3, 2, 1]);
    println!("OK — reversed list matches expected");
}
