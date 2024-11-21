mod node_set;

use crate::node::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::console::ConsoleManager;

pub struct NodeSet<'a> {
    // Path to chain storage
    chain_path: PathBuf,

    // clear chain storage
    clear: bool,

    // List of node instances
    nodes: Vec<SnarkNode<'a>>,

    // Thread-safe access to console object
    console: &'a Arc<Mutex<ConsoleManager>>,
}
