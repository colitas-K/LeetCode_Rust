impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;
        let mut sum = 0;
        let mut result = None;
        let mut cur = &mut result;

        while l1.is_some() || l2.is_some() || carry != 0 {
            sum = carry;
            if let Some(list_node) = l1 {
                sum += list_node.val;
                l1 = &list_node.next;
            }
            if let Some(list_node) = l2 {
                sum += list_node.val;
                l2 = &list_node.next;
            }
            carry = sum / 10;
            *cur = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        result
    }
}

// fn main(){
//     let mut l1 = Some(Box::new(ListNode::new(3)));
//     let mut l2 = Some(Box::new(ListNode::new(4)));
//     l1 = Some(Box::new(ListNode {
//         val: 4,
//         next: l1,
//     }));
//     l1 = Some(Box::new(ListNode {
//         val: 2,
//         next: l1,
//     }));
//     l2 = Some(Box::new(ListNode {
//         val: 6,
//         next: l2,
//     }));
//     l2 = Some(Box::new(ListNode {
//         val: 5,
//         next: l2,
//     }));
//     let mut l3 = Solution::add_two_numbers(l1, l2);
//     for i in 0..3 {
//         match l3 {
//             Some(temp) => {
//                 println!("{}", temp.val);
//                 l3 = temp.next;
//             },
//             None => {},
//         }
//     }
// }
