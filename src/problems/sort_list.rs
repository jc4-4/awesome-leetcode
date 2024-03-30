// https://leetcode.com/problems/sort-list
use crate::shared::*;

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    merge_sort(head)
}

fn merge_sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    match split(head.as_mut()) {
        Some(node) => merge(merge_sort(head), merge_sort(Some(node))),
        None => head,
    }
}

// this signature allows us to take the Option in next
// whereas &mut Option would not allow us to mutate the inside value
fn split(head: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    // minus one since we always take the next node and not the current
    // this happens when head is a singleton node.
    let mut mid = len(&head.as_deref()) / 2 - 1;
    let mut current: &mut Box<ListNode> = head.unwrap();
    while mid > 0 {
        current = current.next.as_mut().expect("next does not exist");
        mid -= 1;
    }
    current.next.take()
}

fn len(head: &Option<&Box<ListNode>>) -> i32 {
    head.as_ref()
        .map(|boxed_node| 1 + len(&boxed_node.next.as_ref()))
        .unwrap_or(0)
}

fn merge(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if left.is_none() {
        return right;
    }

    if right.is_none() {
        return left;
    }

    let (mut left, mut right) = (left.unwrap(), right.unwrap());
    if left.val < right.val {
        left.next = merge(left.next.take(), Some(right));
        Some(left)
    } else {
        // left.val >= right.val
        right.next = merge(Some(left), right.next.take());
        Some(right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &[i32]) {
        let mut sorted = input.to_vec();
        sorted.sort();
        assert_eq!(sort_list(new_list(input)), new_list(&sorted));
    }

    #[test]
    fn test_empty() {
        check(&[]);
    }

    #[test]
    fn test_singleton() {
        check(&[1]);
    }

    #[test]
    fn test_sorted() {
        check(&[1, 2, 3]);
    }

    #[test]
    fn test_reverse() {
        check(&[3, 2, 1]);
    }

    #[test]
    fn test_five() {
        check(&[3, 1, 2, 4, 5]);
    }
}
