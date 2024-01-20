mod snark_node;

use std::io::BufReader;
use std::process::{Child, ChildStdout};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

use crate::console::ConsoleManager;

// SnarkOS process wrapper
// This object will start, monitor and terminate
// a single snarkos instance.
pub struct SnarkNode<'a> {
    // Friendly process name
    name: String,

    // snarkos arguments
    args: Vec<String>,

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
    stdout_silent: Arc<AtomicBool>,

    // Thread-safe access to console object
    console: &'a Arc<Mutex<ConsoleManager>>,
}
