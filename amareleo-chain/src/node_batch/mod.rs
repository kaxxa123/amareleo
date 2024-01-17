mod node_set;

use crate::node::*;

pub use node_set::default_node_args;

pub const NODE0_START_COMPLETE: &str = "No connected validators";
pub const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
pub const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
pub const NODE3_START_COMPLETE: &str = "Advanced to block";

pub struct NodeSet<'a> {
    // List of node instances
    nodes: Vec<SnarkNode<'a>>,
}
