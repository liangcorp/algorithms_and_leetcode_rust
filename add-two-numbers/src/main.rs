#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn add(&mut self, data: i32) {
        let val = self.val;
        let next = self.next.take();

        let previous_pointer = Box::new(ListNode { val, next });
        self.val = data;
        self.next = Some(previous_pointer);
    }

    fn display_list(&self) {
        let mut next_node = &self.next;

        print!("{:?}", self.val);

        while let Some(node) = &next_node {
            print!(", {:?}", node.val);
            next_node = &node.next;
        }
        println!("\n");
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut sum;
    let mut left_over;
    let mut carry_over = 0;

    let mut l1_next_node = l1.as_ref().unwrap();
    let mut l2_next_node = l2.as_ref().unwrap();

    let mut result_head = Box::new(ListNode::new(0));
    let mut result_node = result_head.as_mut();

    let mut l1_val;
    let mut l2_val;

    l1_val = l1_next_node.val;
    l2_val = l2_next_node.val;

    loop {
        sum = l1_val + l2_val + carry_over;

        if sum > 9 {
            left_over = sum % 10;
            result_node.val = left_over;
            carry_over = 1;
        } else {
            result_node.val = sum;
            carry_over = 0;
        }

        if l1_next_node.next.is_none() && l2_next_node.next.is_none() && carry_over != 1{
            break;
        }

        match l1_next_node.next.as_ref() {
            Some(p) => { l1_next_node = p; l1_val = l1_next_node.val },
            None => l1_val = 0,
        }

        match l2_next_node.next.as_ref() {
            Some(p) => { l2_next_node = p; l2_val = l2_next_node.val },
            None => l2_val = 0,
        }

        result_node.next = Some(Box::new(ListNode::new(0)));
        result_node = result_node.next.as_mut().unwrap();
    }

    Some(result_head)
}

fn main() {
    let mut l1 = ListNode::new(9);
    l1.add(9);
    l1.add(9);
    l1.add(9);
    l1.add(9);
    l1.add(9);
    l1.add(9);
    l1.add(9);

    // l1.display_list();

    let mut l2 = ListNode::new(9);
    l2.add(9);
    l2.add(9);
    l2.add(9);

    // l2.display_list();
    //
    add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))).unwrap().display_list();
}
