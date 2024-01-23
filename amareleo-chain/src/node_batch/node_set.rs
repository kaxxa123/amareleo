use crate::config::*;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;
use crate::node_batch::*;

use std::sync::Arc;
use std::sync::Mutex;

use std::path::PathBuf;

impl<'a> NodeSet<'a> {
    pub fn new(chain_cfg: ChainArgs, console: &Arc<Mutex<ConsoleManager>>) -> NodeSet {
        let mut node_list = Vec::<SnarkNode>::new();

        for (idx, node_cfg) in chain_cfg.snarkos.into_iter().enumerate() {
            node_list.push(SnarkNode::new(node_cfg, &format!("node{idx}"), console));
        }

        NodeSet {
            chain_path: PathBuf::new(),
            clear: false,
            nodes: node_list,
        }
    }

    pub fn start(&mut self, ledger: &Option<String>) -> anyhow::Result<()> {
        self.chain_path = if let Some(path) = ledger {
            folder_exists(path)?
        } else {
            self.clear = true;
            create_ledger_dir()?
        };

        for node in self.nodes.iter_mut() {
            node.start(&self.chain_path, 300u64)?;
        }

        Ok(())
    }

    pub fn end(&mut self) {
        // We do not check if a node has already been terminated
        // because the node instances will take care for this themselves.
        for (_, node) in self.nodes.iter_mut().enumerate().rev() {
            let _ = node.end();
        }

        // Only clear if user didn't override the ledger storge path
        if self.clear {
            let _ = clear_ledger_dir(&self.chain_path);
        }
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
        self.end();
    }
}
