use crate::chain_errors::ChainErrors;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;
use crate::node_batch::NodeSet;

use std::sync::Arc;
use std::sync::Mutex;

use homedir::get_my_home;
use std::path::PathBuf;

const NODE0_START_COMPLETE: &str = "No connected validators";
const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
const NODE3_START_COMPLETE: &str = "Advanced to block";

impl<'a> NodeSet<'a> {
    pub fn new(console: &Arc<Mutex<ConsoleManager>>) -> NodeSet {
        NodeSet {
            nodes: vec![
                SnarkNode::new("node0", NODE0_START_COMPLETE, console),
                SnarkNode::new("node1", NODE1_START_COMPLETE, console),
                SnarkNode::new("node2", NODE2_START_COMPLETE, console),
                SnarkNode::new("node3", NODE3_START_COMPLETE, console),
            ],
        }
    }

    fn node_args(num: usize) -> Vec<String> {
        let mut args: Vec<String> = vec![
            String::from("start"),
            String::from("--dev"),
            num.to_string(),
            String::from("--nodisplay"),
            String::from("--validator"),
        ];

        if num != 0 {
            args.push(String::from("--norest"));
        }

        args
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        //Cross-platform compatible retreival of the user's home dir
        let start_path: PathBuf = match get_my_home()?.take() {
            None => return Err(ChainErrors::NoHomeDir.into()),
            Some(path) => path,
        };

        for (idx, node) in self.nodes.iter_mut().enumerate() {
            node.start(&start_path, NodeSet::node_args(idx), 300u64)?;
        }

        Ok(())
    }

    pub fn end(&mut self) -> anyhow::Result<()> {
        // We do not check if a node has already been terminated
        // because the node instances will take care for this themselves.
        for (_, node) in self.nodes.iter_mut().enumerate().rev() {
            let _ = node.end();
        }

        Ok(())
    }

    pub fn stdout_silent(&mut self) {
        for (_, node) in self.nodes.iter_mut().enumerate() {
            node.stdout_show(false);
        }
    }

    pub fn stdout_show(&mut self, idx: usize) {
        for (cnt, node) in self.nodes.iter_mut().enumerate() {
            node.stdout_show(cnt == idx);
        }
    }
}

impl<'a> Drop for NodeSet<'a> {
    fn drop(&mut self) {
        let _ = self.end();
    }
}
