impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut prev = None;
        while let Some(mut current_node) = current {
            let next_node = current_node.next.take();
            current_node.next = prev.take();
            prev = Some(current_node);
            current = next_node;
        }
        prev
    }
}