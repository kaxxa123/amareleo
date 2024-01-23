mod node_set;

use crate::node::*;
use std::path::PathBuf;

pub use node_set::*;

pub struct NodeSet<'a> {
    // Path to chain storage
    chain_path: PathBuf,

    // clear chain storage
    clear: bool,

    // List of node instances
    nodes: Vec<SnarkNode<'a>>,
}
