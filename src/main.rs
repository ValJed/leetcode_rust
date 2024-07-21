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
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut list1 = l1;
        let mut list2 = l2;
        let mut rest = 0;

        while list1.is_some() || list2.is_some() || rest != 0 {
            let val1 = if list1.is_some() {
                list1.as_ref().unwrap().val
            } else {
                0
            };
            let val2: i32 = if list2.is_some() {
                list2.as_ref().unwrap().val
            } else {
                0
            };

            let sum = val1 + val2 + rest;
            let sum_digit;
            if sum >= 10 {
                sum_digit = sum % 10;
                rest = 1;
            } else {
                sum_digit = sum;
                rest = 0
            }

            res = insert_node(res, sum_digit);

            list1 = if list1.is_some() {
                list1.unwrap().next
            } else {
                None
            };
            list2 = if list2.is_some() {
                list2.unwrap().next
            } else {
                None
            };
        }

        res
    }
}

fn insert_node(mut list: Option<Box<ListNode>>, node_val: i32) -> Option<Box<ListNode>> {
    let new_node = Some(Box::new(ListNode::new(node_val)));

    if list.is_none() {
        list = new_node;
        return list;
    }

    let mut tail = list.as_mut();

    loop {
        let node = tail.unwrap();
        if node.next == None {
            node.next = new_node;
            break;
        }

        tail = node.next.as_mut();
    }

    list
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
    let head1_vec = vec![2, 4, 3];
    let head2_vec = vec![5, 6, 4];
    let head1 = from_vec(head1_vec);
    let head2 = from_vec(head2_vec);

    let res = Solution::add_two_numbers(head1, head2);
    println!("res: {:?}", res);
}
