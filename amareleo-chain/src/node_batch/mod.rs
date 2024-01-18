mod node_set;

use crate::node::*;
use std::path::PathBuf;

pub use node_set::*;

pub const AMARELEO_HOME_DIR: &str = ".amareleo";
pub const AMARELEO_CHAIN_DIR: &str = "chain";

pub const NODE0_START_COMPLETE: &str = "No connected validators";
pub const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
pub const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
pub const NODE3_START_COMPLETE: &str = "Advanced to block";

pub struct NodeSet<'a> {
    // Path to chain storage
    chain_path: PathBuf,

    // List of node instances
    nodes: Vec<SnarkNode<'a>>,
}
