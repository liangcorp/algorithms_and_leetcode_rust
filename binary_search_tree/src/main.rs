/*
 * Name: Computer Algorithms
 * Author: Chen Liang
 * Description: Implementation of binary search tree
 * Date: 21-January-2024
 */

mod binary_search_tree;

fn main() {
    let tree_node_ptr = binary_search_tree::Node::new(50);
    let tree_node = tree_node_ptr.as_ref().unwrap();

    tree_node.borrow_mut().insert(30);
    tree_node.borrow_mut().insert(20);
    tree_node.borrow_mut().insert(40);
    tree_node.borrow_mut().insert(70);
    tree_node.borrow_mut().insert(60);
    tree_node.borrow_mut().insert(80);

    println!("Inorder: ");
    tree_node.borrow().in_order();
    println!();

    println!("Preorder: ");
    tree_node.borrow().pre_order();
    println!();

    println!("Postorder: ");
    tree_node.borrow().post_order();
    println!();

    println!("Leaf Nodes:");
    tree_node.borrow().leaf_nodes();
    println!();

    println!("None Leaf Nodes:");
    tree_node.borrow().none_leaf_nodes();
    println!();

    println!("Number of levels: {}", tree_node.borrow().tree_height());
    println!("Number of nodes: {}", tree_node.borrow().node_count());

    println!("Print at level");
    for i in 0..tree_node.borrow().tree_height() {
        tree_node.borrow().given_level(i);
        println!();
        // println!("{:0i$}", i);
    }

    println!("Smallest value: {}", tree_node.borrow().min_value());

    println!("Print only right nodes");
    tree_node.borrow().right_nodes();

    println!("Print only left nodes");
    tree_node.borrow().left_nodes();

    // println!("Delete");
    // tree_node_ptr.unwrap().borrow().delete(80);
    // tree_node_ptr.unwrap().borrow().in_order();
    // println!();
}
