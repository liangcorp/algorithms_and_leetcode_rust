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
            print!("{}  ", self.data);
            self.right.as_ref().unwrap().inorder();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn preorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            print!("{}  ", self.data);
            self.left.as_ref().unwrap().preorder();
            self.right.as_ref().unwrap().preorder();
        } else {
            print!("{}  ", self.data);
        }
    }

    pub fn postorder(&self) {
        if self.left.is_some() && self.right.is_some() {
            self.left.as_ref().unwrap().postorder();
            self.right.as_ref().unwrap().postorder();
            print!("{}  ", self.data);
        } else {
            print!("{}  ", self.data);
        }
    }
}
