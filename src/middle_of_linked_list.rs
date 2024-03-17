struct Solution;

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

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut index = 1;
        let mut mid_node: Option<Box<ListNode>> = None;
        let mut cur = head;

        while cur.is_some() {
            if index == 1 {
                mid_node = cur.clone();
            }
            if index % 2 == 0 {
                mid_node = mid_node.unwrap().next;
            }

            cur = cur.unwrap().next;
            index += 1;
        }

        mid_node
    }
}

fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;

    for num in vec.into_iter().rev() {
        let mut node = ListNode::new(num);
        node.next = cur;
        cur = Some(Box::new(node));
    }

    cur
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let head = from_vec(vec);

    let res = Solution::middle_node(head);

    println!("res: {:?}", res);
}
