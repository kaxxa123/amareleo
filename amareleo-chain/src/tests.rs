use std::sync::Arc;
use std::sync::Mutex;

use homedir::get_my_home;

use crate::console::ConsoleManager;
use crate::node::SnarkNode;
use crate::node_batch;

#[test]
fn test_start_node0() {
    let base_console = ConsoleManager::start(10);
    let console = Arc::new(Mutex::new(base_console));
    let mut one_node: SnarkNode =
        SnarkNode::new("node0", node_batch::NODE0_START_COMPLETE, &console);

    if let Some(start_path) = get_my_home().unwrap().take() {
        one_node
            .start(&start_path, node_batch::default_node_args(0), 300u64)
            .unwrap();
    }

    let has_monitor = one_node.has_stdout_monitor();
    assert!(has_monitor);
}
