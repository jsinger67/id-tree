extern crate id_tree;

use id_tree::InsertBehavior::*;
use id_tree::{Node, Tree, TreeBuilder};

type NodeData = i32;

#[test]
fn post_order_traversal_ids() {
    let mut tree: Tree<NodeData> = TreeBuilder::new().build();

    //    0 ----------\
    //   /  \         |
    //  1    3        4
    //  |            /|\
    //  2           5 6 7

    let root_0 = tree.insert(Node::new(0), AsRoot).ok().unwrap();
    let node_1 = tree.insert(Node::new(1), UnderNode(&root_0)).ok().unwrap();
    let _node_ = tree.insert(Node::new(2), UnderNode(&node_1)).ok().unwrap();
    let node_3 = tree.insert(Node::new(3), UnderNode(&root_0)).ok().unwrap();
    let node_4 = tree.insert(Node::new(4), UnderNode(&root_0)).ok().unwrap();
    let _node_ = tree.insert(Node::new(5), UnderNode(&node_4)).ok().unwrap();
    let _node_ = tree.insert(Node::new(6), UnderNode(&node_4)).ok().unwrap();
    let _node_ = tree.insert(Node::new(7), UnderNode(&node_4)).ok().unwrap();

    let node_data = tree
        .traverse_post_order_ids(&root_0)
        .unwrap()
        .into_iter()
        .map(|node_id| *tree.get(&node_id).unwrap().data())
        .collect::<Vec<NodeData>>();
    assert_eq!(node_data, [2, 1, 3, 5, 6, 7, 4, 0]);

    let node_data = tree
        .traverse_post_order_ids(&node_4)
        .unwrap()
        .into_iter()
        .map(|node_id| *tree.get(&node_id).unwrap().data())
        .collect::<Vec<NodeData>>();
    assert_eq!(node_data, [5, 6, 7, 4]);

    let node_data = tree
        .traverse_post_order_ids(&node_3)
        .unwrap()
        .into_iter()
        .map(|node_id| *tree.get(&node_id).unwrap().data())
        .collect::<Vec<NodeData>>();
    assert_eq!(node_data, [3]);
}
