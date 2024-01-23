use binary_search_tree::binary_search_tree::Node as bst_node;

#[test]
fn binary_search_tree_node_count() {
    let mut tree_node = bst_node::new(50);
    tree_node.insert(30);
    tree_node.insert(20);
    tree_node.insert(40);
    tree_node.insert(70);
    tree_node.insert(60);
    tree_node.insert(80);

    assert_eq!(7, tree_node.node_count());
}

#[test]
fn binary_search_tree_minimum_value() {
    let mut tree_node = bst_node::new(50);
    tree_node.insert(30);
    tree_node.insert(20);
    tree_node.insert(40);
    tree_node.insert(70);
    tree_node.insert(60);
    tree_node.insert(80);

    assert_eq!(20, tree_node.min_value());
}
