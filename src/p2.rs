use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        let mut val: i32 = 0;
        loop {
            if let Some(node1) = p {
                if let Some(node2) = q {
                    val += node1.val + node2.val;
                    q = node2.next.as_ref();
                } else {
                    val += node1.val;
                }
                p = node1.next.as_ref();
            } else if let Some(node2) = q {
                val += node2.val;
                q = node2.next.as_ref();
            } else if val == 0 {
                break;
            }

            current.next = Some(Box::new(ListNode::new(val % 10)));
            current = current.next.as_mut().unwrap();
            val /= 10;
        }

        dummy_head.next
    }
}
