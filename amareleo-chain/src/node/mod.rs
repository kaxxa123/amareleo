mod snark_node;

use std::io::BufReader;
use std::process::{Child, ChildStdout};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

pub struct SnarkNode {
    // Friendly process name
    name: String,

    // A phrase that indicates that the process started and has
    // completed initialization.
    ready_phrase: String,

    // Child process handle
    process: Option<Child>,

    // Object to stdout reader
    stdout_reader: Option<BufReader<ChildStdout>>,

    // Handle for thread monitoring stdout
    stdout_thread: Option<JoinHandle<()>>,

    // Thread-safe flag for enabling/disabling
    // redirecting stdout to console.
    stdout_silent: Arc<Mutex<bool>>,
}
