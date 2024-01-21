mod insert;
mod print;
mod search;

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

    pub fn tree_height(&self) -> i32 {
        let l_depth;
        let r_depth;
        if self.left.is_some() || self.right.is_some() {
            l_depth = self.left.as_ref().unwrap().tree_height();
            r_depth = self.right.as_ref().unwrap().tree_height();
            if l_depth > r_depth {
                l_depth + 1
            } else {
                r_depth + 1
            }
        } else {
            1
        }
    }

    pub fn node_count(&self) -> i32 {
        if self.left.is_some() || self.right.is_some() {
            self.left.as_ref().unwrap().node_count() + self.right.as_ref().unwrap().node_count() + 1
        } else {
            1
        }
    }

    pub fn min_value(&self) -> i32 {
        if self.left.is_some() {
            self.left.as_ref().unwrap().min_value()
        } else {
            self.data
        }
    }
}
