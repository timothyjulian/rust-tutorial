// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() && list2.is_none() {
        return None;
    }
    let mut iter1 = list1;
    let mut iter2 = list2;
    let mut answer_vec = Vec::new();
    loop {
        if iter1.is_some() && iter2.is_some() {
            if iter1.as_ref().unwrap().val < iter2.as_ref().unwrap().val {
                answer_vec.push(iter1.as_ref().unwrap().val);
                iter1 = iter1.as_mut().unwrap().next.take();
            } else {
                answer_vec.push(iter2.as_ref().unwrap().val);
                iter2 = iter2.as_mut().unwrap().next.take();
            }
        } else if iter1.is_some() && iter2.is_none() {
            answer_vec.push(iter1.as_ref().unwrap().val);
            iter1 = iter1.as_mut().unwrap().next.take();
        } else if iter1.is_none() && iter2.is_some() {
            answer_vec.push(iter2.as_ref().unwrap().val);
            iter2 = iter2.as_mut().unwrap().next.take();
        } else {
            break;
        }
    }
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;
    for num in &answer_vec {
        current.next = Some(Box::new(ListNode::new(*num)));
        current = current.next.as_mut().unwrap();
    }
    return dummy.next;
}

fn main() {
    merge_two_lists(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    );
}
