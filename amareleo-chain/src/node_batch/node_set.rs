use crate::chain_errors::ChainErrors;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;
use crate::node_batch::*;

use std::fs::{create_dir, remove_dir_all};
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::{Context, Ok};

use homedir::get_my_home;
use std::path::PathBuf;

impl<'a> NodeSet<'a> {
    pub fn new(console: &Arc<Mutex<ConsoleManager>>) -> NodeSet {
        NodeSet {
            chain_path: PathBuf::new(),
            nodes: vec![
                SnarkNode::new("node0", NODE0_START_COMPLETE, console),
                SnarkNode::new("node1", NODE1_START_COMPLETE, console),
                SnarkNode::new("node2", NODE2_START_COMPLETE, console),
                SnarkNode::new("node3", NODE3_START_COMPLETE, console),
            ],
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        //Cross-platform compatible retreival of the user's home dir
        self.chain_path = create_ledger_dir()?;

        for (idx, node) in self.nodes.iter_mut().enumerate() {
            node.start(&self.chain_path, default_node_args(idx), 300u64)?;
        }

        Ok(())
    }

    pub fn end(&mut self) {
        // We do not check if a node has already been terminated
        // because the node instances will take care for this themselves.
        for (_, node) in self.nodes.iter_mut().enumerate().rev() {
            let _ = node.end();
        }

        let _ = clear_ledger_dir(&self.chain_path);
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

pub fn default_node_args(num: usize) -> Vec<String> {
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

pub fn create_ledger_dir() -> anyhow::Result<PathBuf> {
    //Cross-platform compatible retreival of the user's home dir
    let home_path: PathBuf = match get_my_home()?.take() {
        None => return Err(ChainErrors::NoHomeDir.into()),
        Some(path) => path,
    };

    if !home_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home path not found",
        ))
        .context("Failed to create ledger directory");
    }

    if !home_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Home path is not a directory",
        ))
        .context("Failed to create ledger directory");
    }

    let amareleo_path = home_path.join(AMARELEO_HOME_DIR);
    if !amareleo_path.exists() {
        create_dir(&amareleo_path)?;
    }

    let chain_path = amareleo_path.join(AMARELEO_CHAIN_DIR);
    if chain_path.exists() {
        remove_dir_all(&chain_path)?;
    }
    create_dir(&chain_path)?;

    Ok(chain_path)
}

pub fn clear_ledger_dir(path: &PathBuf) -> anyhow::Result<()> {
    if !path.exists() || !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Chain Path not found",
        ))
        .context("Failed to clear ledger.");
    }

    remove_dir_all(path)?;
    Ok(())
}
