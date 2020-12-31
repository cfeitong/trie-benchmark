use std::rc::{Rc, Weak};
use std::cell::RefCell;

type RefNode = RefCell<Node>;
pub struct NaiveAc {
    head: Rc<RefNode>,
}

impl NaiveAc {
    pub fn insert(&mut self, data: &[u8]) {
        let mut data = data;
        let mut node = self.head.clone();
        let mut fail = self.head.clone();
        while let Some((&cur, rest)) = data.split_first() {
            let next = &node.borrow().next;
            node = if let Some(next_node) = &next[cur as usize] {
                next_node.clone()
            } else {
                todo!()
            };

            data = rest;
        }
    }
}


struct Node {
    next: Vec<Option<Rc<RefNode>>>,
    fail: Weak<RefNode>,
}