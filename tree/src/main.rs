use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node {
    ele: String,
    children: Vec<Rc<RefCell<Box<Node>>>>,
}

impl Node {
    fn new(ele: String) -> Rc<RefCell<Box<Node>>> {
        Rc::new(RefCell::new(Box::new(Node {
            ele,
            children: Vec::new(),
        })))
    }

    fn push(&mut self, to_push: &Vec<Rc<RefCell<Box<Node>>>>) {
        for i in to_push {
            self.children.push(i.clone());
        }
    }
    // firstorder traversal, change every element to "a"
    pub fn firstorder(&mut self) {
        self.ele = "a".to_string();
        for i in &self.children {
            i.borrow_mut().firstorder();
        }
        dbg!(self);
    }
}
fn main() {
    // construct a tree
    let mut a = Node::new("a".to_string());
    a.borrow_mut().push(&vec![Node::new("b".to_string())]);
    a.borrow_mut().push(&vec![Node::new("c".to_string())]);
    a.borrow_mut().push(&vec![Node::new("d".to_string())]);
    a.borrow_mut().firstorder();

    println!("Hello, world!");
}
