use std::sync::Arc;
use std::sync::Mutex;

use crate::config;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;

#[test]
fn test_start_node0() {
    let mut chain_cfg = config::ChainArgs::load().unwrap();

    let base_console = ConsoleManager::start(10);
    let console = Arc::new(Mutex::new(base_console));

    let chain_path = config::create_ledger_dir().unwrap();

    let mut one_node: SnarkNode = SnarkNode::new(chain_cfg.snarkos.remove(0), "node0", &console);

    one_node.start(&chain_path, 300u64).unwrap();

    let has_monitor = one_node.has_stdout_monitor();
    assert!(has_monitor);
}
