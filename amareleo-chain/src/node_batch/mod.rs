mod node_set;

use crate::node::*;

pub struct NodeSet<'a> {
    // List of node instances
    nodes: Vec<SnarkNode<'a>>,
}
