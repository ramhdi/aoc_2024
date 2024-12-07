// Definition for a linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_array(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &num in nums.iter() {
            *current = Some(Box::new(ListNode::new(num)));
            current = &mut current.as_mut().unwrap().next;
        }

        return head;
    }

    pub fn to_array(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let mut curr = head;
        while curr.is_some() {
            let curr_content = curr.unwrap();
            result.push(curr_content.val);
            curr = curr_content.next;
        }
        return result;
    }
}
