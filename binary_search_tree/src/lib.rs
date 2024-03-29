mod print;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Rc<RefCell<Node>>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => match &self.left {
                Some(left) => (**left).borrow_mut().insert(data),
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node::new(data))));
                }
            },
            Ordering::Greater => match &self.right {
                Some(left) => (**left).borrow_mut().insert(data),
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node::new(data))));
                }
            },
            Ordering::Equal => (),
        }
    }

    pub fn tree_height(&self) -> i32 {
        let l_depth = match &self.left {
            Some(left) => left.borrow().tree_height(),
            None => 0,
        };

        let r_depth = match &self.right {
            Some(right) => right.borrow().tree_height(),
            None => 0,
        };

        if l_depth > r_depth {
            l_depth + 1
        } else {
            r_depth + 1
        }
    }

    pub fn node_count(&self) -> i32 {
        let l_node_count = match &self.left {
            Some(left) => left.borrow().node_count(),
            None => 0,
        };

        let r_node_count = match &self.right {
            Some(right) => right.borrow().node_count(),
            None => 0,
        };

        l_node_count + r_node_count + 1
    }

    pub fn max_left(&self) -> i32 {
        if self.left.is_some() || self.right.is_some() {
            match &self.right {
                Some(right) => right.borrow().max_left(),
                None => self.data,
            }
        } else {
            self.data
        }
    }

    pub fn min_value(&self) -> i32 {
        match &self.left {
            Some(left) => left.borrow().min_value(),
            None => self.data,
        }
    }

    // DONE: Node to be deleted is the leaf node:
    //  Its simple you can just null it out.
    //
    // DONE Node to be deleted has one child:
    //  You can just replace the node with the child node.
    //
    // DONE: Node to be deleted has two child:
    //  Need to figure out what will be the
    //  replacement of the node to be deleted.
    //  Want minimal disruption to the existing tree structure
    //  Can table the replacement node from the deleted
    //      nodes left or right subtree.
    //  If taking if from the left subtree,
    //      we have to take the largest value in the left subtree.
    //  If taking if from the right subtree,
    //      we have to take the smallest value in the right subtree.
    //  Choose one approach and stick to it.
    //
    //  Currently using Max element at left method
    pub fn delete(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => {
                if let Some(left) = &self.left {
                    if (**left).borrow_mut().data == data {
                        if (**left).borrow_mut().left.is_none()
                            && (**left).borrow_mut().right.is_none()
                        {
                            // delete leaf node at right
                            self.left = None;
                        } else if (**left).borrow_mut().left.is_some()
                            && (**left).borrow_mut().right.is_none()
                        {
                            let temp = (**left).borrow_mut().left.clone();
                            // replace node with left child;
                            self.left = temp;
                        } else if (**left).borrow_mut().left.is_none()
                            && (**left).borrow_mut().right.is_some()
                        {
                            let temp = (**left).borrow_mut().right.clone();
                            // replace node with right child;
                            self.left = temp;
                        } else {
                            (**left).borrow_mut().delete(data);
                        }
                    } else {
                        (**left).borrow_mut().delete(data);
                    }
                }
            }
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    if (**right).borrow_mut().data == data {
                        if (**right).borrow_mut().left.is_none()
                            && (**right).borrow_mut().right.is_none()
                        {
                            // delete leaf node
                            self.right = None;
                        } else if (**right).borrow_mut().left.is_some()
                            && (**right).borrow_mut().right.is_none()
                        {
                            let temp = (**right).borrow_mut().left.clone();
                            // replace node with left child
                            self.right = temp;
                        } else if (**right).borrow_mut().left.is_none()
                            && (**right).borrow_mut().right.is_some()
                        {
                            let temp = (**right).borrow_mut().right.clone();
                            // replace node with right child
                            self.right = temp;
                        } else {
                            (**right).borrow_mut().delete(data);
                        }
                    } else {
                        (**right).borrow_mut().delete(data);
                    }
                }
            }
            Ordering::Equal => {
                // find large node at the left
                let max_left_value = self.max_left();
                //  recursively call delete function
                //  to delete max node at left
                self.delete(max_left_value);
                // replace value of found node
                self.data = max_left_value;
            }
        }
    }
}
