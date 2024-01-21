mod search;
mod insert;

pub struct Node {
    data: i32,
    left: Pointer,
    right: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn inorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().inorder();
            println!("{}", self.data);
            self.right.as_ref().unwrap().inorder();
        } else if self.left.is_some() && self.right.is_none() {
            self.left.as_ref().unwrap().inorder();
            println!("{}", self.data);
        } else if self.left.is_none() && self.right.is_some() {
            println!("{}", self.data);
            self.right.as_ref().unwrap().inorder();
        } else {
            println!("{}", self.data);
        }
    }
}
