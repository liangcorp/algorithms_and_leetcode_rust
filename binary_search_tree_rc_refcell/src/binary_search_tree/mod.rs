use std::{borrow::BorrowMut, cell::RefCell, cmp::Ordering, rc::Rc};

mod print;
mod search;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Rc<RefCell<Node>>>;

pub struct BinarySearchTree(Pointer);

impl BinarySearchTree {
    pub fn new(data: i32) -> BinarySearchTree {
        BinarySearchTree(Some(Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }))))
    }

    pub fn insert(&mut self, data: i32) {
        let new_node;

        match data.cmp(self.data.borrow_mut()) {
            Ordering::Less => match self.left.borrow_mut() {
                Some(left) => {
                    (**left).borrow_mut().insert(data);
                }
                None => {
                    new_node = Node::new(data);
                    self.left = Some(Rc::new(RefCell::new(new_node)));
                }
            },
            Ordering::Greater => match self.right.borrow_mut() {
                Some(right) => {
                    (**right).borrow_mut().insert(data);
                }
                None => {
                    new_node = Node::new(data);
                    self.right = Some(Rc::new(RefCell::new(new_node)));
                }
            },
            Ordering::Equal => (),
        }
    }

    pub fn tree_height(&self) -> i32 {
        if self.left.is_none() || self.right.is_none() {
            return 1;
        }

        let l_depth = match &self.left {
            Some(left) => left.borrow().tree_height(),
            None => 1,
        };

        let r_depth = match &self.right {
            Some(right) => right.borrow().tree_height(),
            None => 1,
        };

        if l_depth > r_depth {
            l_depth + 1
        } else {
            r_depth + 1
        }
    }

    pub fn node_count(&self) -> i32 {
        if self.left.is_none() || self.right.is_none() {
            return 1;
        }

        let l_count = match &self.left {
            Some(left) => left.borrow().node_count(),
            None => 0,
        };

        let r_count = match &self.right {
            Some(right) => right.borrow().node_count(),
            None => 0,
        };

        l_count + r_count + 1
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, data: i32) {
        let mut c_node = self;

        unsafe {
            loop {
                match data.cmp(&self.data) {
                    Ordering::Less => {
                        println!("Going left");
                        if c_node.left.is_some() {
                            c_node = (*c_node.left.unwrap());
                        }
                    }
                    Ordering::Greater => {
                        println!("Going right");
                        if c_node.right.is_some() {
                            c_node = c_node.right.unwrap();
                        }
                    }
                    Ordering::Equal => {
                        if c_node.left.is_none() && c_node.right.is_none() {
                            drop(c_node);
                            break;
                        }
                    }
                }
            }
        }
    }
}
