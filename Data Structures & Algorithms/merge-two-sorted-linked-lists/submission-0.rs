// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    // we can make one of the lists to be the new merged one so like edit in it to not create a new one in memory
    // we have both heads at the start, so comparing head -> next in the merged one to the head of the other one to see which one is the lower
    // the lower will be inserted in the merged one and update the poionters, the curr, the next pointers in each list and continue until one of the list finishes
    // then insert the other at its end
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            let take_l1 = l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val;

            if take_l1 {
                let next = l1.as_mut().unwrap().next.take();
                tail.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                tail.next = l2;
                l2 = next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };

        dummy.next
    }
}
