mod chain_errors;
mod node;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use homedir::get_my_home;

use chain_errors::ChainErrors;

use node::SnarkNode;

const NODE0_START_COMPLETE: &str = "No connected validators";
const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
const NODE3_START_COMPLETE: &str = "Advanced to block";

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

fn start_batch(nodes: &mut [SnarkNode]) -> anyhow::Result<()> {
    //Cross-platform compatible retreival of the user's home dir
    let start_path: PathBuf = match get_my_home()?.take() {
        None => return Err(ChainErrors::NoHomeDir.into()),
        Some(path) => path,
    };

    for (idx, node) in nodes.iter_mut().enumerate() {
        node.start(&start_path, node_args(idx), 300u64)?;
    }

    Ok(())
}

fn end_batch(nodes: &mut [SnarkNode]) -> anyhow::Result<()> {
    for (_, node) in nodes.iter_mut().enumerate().rev() {
        let _ = node.end();
    }

    Ok(())
}

fn main() {
    let mut nodes: Vec<SnarkNode> = vec![
        SnarkNode::new("node0", NODE0_START_COMPLETE),
        SnarkNode::new("node1", NODE1_START_COMPLETE),
        SnarkNode::new("node2", NODE2_START_COMPLETE),
        SnarkNode::new("node3", NODE3_START_COMPLETE),
    ];

    let _ = start_batch(&mut nodes);

    if nodes[3].has_stdout_monitor() {
        println!();
        println!("All nodes started!");
        println!();
        thread::sleep(Duration::from_secs(60));
    }

    let _ = end_batch(&mut nodes);
}
