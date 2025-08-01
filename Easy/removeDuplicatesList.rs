// problem 83 

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;
    let mut current = head;
    let mut prev_val: Option<i32> = None;

    while let Some(mut node) = current {
      current = node.next.take();
      if prev_val.is_none() || prev_val.unwrap() != node.val {
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
      }
      prev_val = Some(tail.val);
    }

    tail.next = None;
    dummy.next
  }
}