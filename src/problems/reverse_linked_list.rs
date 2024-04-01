// https://leetcode.com/problems/reverse-linked-list
use crate::shared::*;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // reverse_with_accumulator(None, head)
    reverse_iterative(head)
}

fn reverse_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut cur) = (None, head);
    while let Some(mut node) = cur {
        cur = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

// Linear time reverse with accumulator. The first parameter represents a
// partially reversed list. To help visualize the process:
// [] [1,2,3]
// [1] [2,3]
// [2,1] [3]
// [3,2,1] []
//
// This is faster than the naive way, which looks like this:
//   reverse [head, tail] = [reverse tail, head]
// Note that we're appending to the end of list every time.
//
// Note: you may be familiar with the imperative way, that looks like the below
// snippet. Notice that `tail` is already moved in the recursive call,
// and no longer available afterwards. Even if you work around this with
// as_mut(), new_head would not receive any modifications!
// ```
// let new_head = reverse(tail);
// head.next = null;
// tail.next = head; // tail already moved!
// new_head // its own data is not modified!
// ```
fn reverse_with_accumulator(
    new_head: Option<Box<ListNode>>,
    head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let Some(mut node) = head {
        let next = node.next.take();
        node.next = new_head;
        reverse_with_accumulator(Some(node), next)
    } else {
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(reverse_list(new_list(&[1])), new_list(&[1]));
    }

    #[test]
    fn test_two() {
        assert_eq!(reverse_list(new_list(&[1, 2])), new_list(&[2, 1]));
    }

    #[test]
    fn test_three() {
        assert_eq!(reverse_list(new_list(&[1, 2, 3])), new_list(&[3, 2, 1]));
    }
}
