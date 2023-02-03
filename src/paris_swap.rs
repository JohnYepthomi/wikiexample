// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    fn push_start(&mut self, val: i32) {
        let old_node = Some(
            Box::new(ListNode {
                val: self.val,
                next: self.next.take(),
            })
        );
        self.val = val;
        self.next = old_node;
    }
}

fn main() {
    let mut list = ListNode::new(21);
    ListNode::push_start(&mut list, 5);
    ListNode::push_start(&mut list, 16);

    println!("list {:?}", list);
}

macro_rules! next_node_as_ref {
    ($a:expr) => {
        $a.as_ref().unwrap().next
    };
}
macro_rules! next_node_as_mut {
    ($a:expr) => {
        $a.as_mut().unwrap().next
    };
}

pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut cur_node = &mut head;

    // while we have something to work with (.next has something)
    while cur_node.is_some() && next_node_as_ref!(cur_node).is_some() {
        // .take() Takes the value out of the option, leaving a None in its place.
        let mut even_node = next_node_as_mut!(cur_node).take();

        // taking next odd node's value
        let next_odd_node = next_node_as_mut!(even_node).take();
        // cur_node's .next points to next odd node
        next_node_as_mut!(cur_node) = next_odd_node;
        // even node's .next points to cur_node
        next_node_as_mut!(even_node) = cur_node.take();
        // switching cur_node with even node
        cur_node.replace(even_node.unwrap());
        // point cursor to .next.next
        cur_node = &mut next_node_as_mut!(next_node_as_mut!(cur_node));
    }
    head
}