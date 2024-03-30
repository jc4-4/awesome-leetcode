// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn new_list(values: &[i32]) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    let mut tail = None;
    for &v in values.iter().rev() {
        tail = Some(Box::new(ListNode { val: v, next: tail }));
    }
    tail
}
