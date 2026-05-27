// Reverse a singly linked list (iterative, three-pointer).
//
// Before: 1 -> 2 -> 3 -> 4 -> 5
// After:  5 -> 4 -> 3 -> 2 -> 1

class ListNode {
  val: number;
  next: ListNode | null;

  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

function reverseList(head: ListNode | null): ListNode | null {
  let prev: ListNode | null = null;
  let curr = head;
  while (curr !== null) {
    const next = curr.next;
    curr.next = prev;
    prev = curr;
    curr = next;
  }
  return prev;
}

function fromArray(xs: number[]): ListNode | null {
  let head: ListNode | null = null;
  for (let i = xs.length - 1; i >= 0; i--) {
    head = new ListNode(xs[i], head);
  }
  return head;
}

function toArray(head: ListNode | null): number[] {
  const out: number[] = [];
  while (head !== null) {
    out.push(head.val);
    head = head.next;
  }
  return out;
}

const head = fromArray([1, 2, 3, 4, 5]);
console.log("Before:", toArray(head));

const reversed = reverseList(head);
const result = toArray(reversed);
console.log("After: ", result);

const expected = [5, 4, 3, 2, 1];
if (JSON.stringify(result) !== JSON.stringify(expected)) {
  throw new Error(`expected ${JSON.stringify(expected)}, got ${JSON.stringify(result)}`);
}
console.log("OK — reversed list matches expected");
