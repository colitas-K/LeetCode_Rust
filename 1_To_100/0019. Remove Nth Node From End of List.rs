impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut result;
        let mut fast = &slow.clone();
        
        for _ in 0..=n {
            if let Some(inner) = fast {
                fast = &inner.next;
            }
        }
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        result.unwrap().next
    }
}
