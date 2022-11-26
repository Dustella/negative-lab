use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::table::State;
#[derive(Clone, Debug)]
pub struct Node {
    ele: State,
    children: Vec<Rc<RefCell<Box<Node>>>>,
}
impl Deref for Node {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self
    }
}
impl Node {
    pub fn new(ele: State) -> Self {
        let children: Vec<Rc<RefCell<Box<Node>>>> = Vec::new();
        Node { children, ele }
    }
    pub fn has_children(&self) -> bool {
        self.children.len() != 0
    }
    pub fn is_all_terminal(&self) -> bool {
        let mut res = false;
        if let State::Terminal(_) = self.ele {
            res = true;
        } else {
            // for i in self.children.clone() {
            //     if i..ele.is_terminal() {
            //         res = true
            //     }
            // }
        }
        res
    }
    pub fn push(&mut self, to_push: &Vec<State>) {
        let mut lock = false;
        self._push(&to_push, &mut lock)
    }
    pub fn _push(&mut self, to_push: &Vec<State>, lock: &mut bool) {
        if *lock {
            return;
        }
        if self.ele.is_terminal() {
            return;
        }
        if !self.has_children() {
            for i in to_push {
                let node = Node::new(*i);
                self.borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(Box::new(node))));
            }
            *lock = true;
            return;
        } else {
            for mut i in self.children.clone() {
                i.clone().borrow_mut().get_mut()._push(to_push, &mut lock)
            }
            return;
        }
    }
}
