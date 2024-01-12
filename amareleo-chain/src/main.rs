mod chain_errors;

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use chain_errors::ChainErrors;
use homedir::get_my_home;

const NODE0_START_COMPLETE: &str = "No connected validators";
const NODE1_START_COMPLETE: &str = "Connected to 1 validators";
const NODE2_START_COMPLETE: &str = "Connected to 2 validators";
const NODE3_START_COMPLETE: &str = "Advanced to block";

pub struct SnarkNode {
    //Child process handle
    name: String,
    ready_phrase: String,
    process: Option<Child>,
    stdout_reader: Option<BufReader<ChildStdout>>,
    stdout_thread: Option<JoinHandle<()>>,
    stdout_silent: Arc<Mutex<bool>>,
}

impl SnarkNode {
    pub fn new(name: &str, ready: &str) -> SnarkNode {
        SnarkNode {
            name: name.to_string(),
            ready_phrase: ready.to_string(),
            process: None,
            stdout_reader: None,
            stdout_thread: None,
            stdout_silent: Arc::new(Mutex::new(true)),
        }
    }

    fn start_process(
        &mut self,
        start_path: &PathBuf,
        params: Vec<String>,
        time_limit_secs: u64,
    ) -> anyhow::Result<()> {
        let start_time = Instant::now();

        let mut snarkos = Command::new("snarkos")
            .args(params)
            .stdout(Stdio::piped())
            .current_dir(start_path)
            .spawn()?;

        // Get a handle to the stdout of the spawned process
        let stdout_opt: Option<ChildStdout> = snarkos.stdout.take();
        let stdout: ChildStdout = match stdout_opt {
            None => {
                snarkos.kill()?;
                println!("{} killed", &self.name);
                return Err(ChainErrors::NoStdout.into());
            }
            Some(stream) => stream,
        };

        let mut ready: bool = false;
        let mut line = String::new();
        let mut reader = BufReader::new(stdout);
        let ready_pharse_low = self.ready_phrase.to_ascii_lowercase();

        // Print each line from stdout to the console...
        // ...and check if it contains the key phrase we are looking for.
        loop {
            line.clear();
            let read_res = reader.read_line(&mut line);

            if let Err(error) = read_res {
                eprintln!("Error reading line: {}", error);
                break;
            } else if line.to_ascii_lowercase().contains(&ready_pharse_low) {
                print!("{}", &line);
                ready = true;
                break;
            } else if start_time.elapsed() > Duration::from_secs(time_limit_secs) {
                eprintln!("Timeout reading line");
                break;
            }

            print!("{}", &line);
        }

        if !ready {
            snarkos.kill()?;
            println!("{} killed", &self.name);
            return Err(ChainErrors::CannotFindReady.into());
        }

        self.process = Some(snarkos);
        self.stdout_reader = Some(reader);

        Ok(())
    }

    fn consume_stdout(&mut self) -> anyhow::Result<()> {
        // Clone the Arc for the thread
        let shared_stdout_silent = Arc::clone(&self.stdout_silent);

        // Consume the reader
        let reader = self
            .stdout_reader
            .take()
            .ok_or::<ChainErrors>(ChainErrors::NoStdout)?;

        let thread_name = String::from(&self.name);

        // Spawn a thread to read from the stdout
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut start_time = Instant::now();
            let mut silent = shared_stdout_silent.lock().expect("Failed to lock Mutex");

            for line in reader.lines() {
                match line {
                    Ok(chunk) => {
                        // Update the silent status every 5 seconds to limit
                        // the amount of locking.
                        if start_time.elapsed() > Duration::from_secs(5) {
                            silent = shared_stdout_silent.lock().expect("Failed to lock Mutex");
                            start_time = Instant::now();
                        }

                        // Display line if not silent
                        if !(*silent) {
                            println!("{thread_name} | {}", chunk);
                        }
                    }
                    Err(err) => {
                        eprintln!("{thread_name} | Error reading line: {}", err);
                        eprintln!("{thread_name} | terminating stdout monitor.");
                        break; // Stop reading on error
                    }
                }
            }
        });

        self.stdout_thread = Some(handle);
        Ok(())
    }

    pub fn start(
        &mut self,
        start_path: &PathBuf,
        params: Vec<String>,
        time_limit_secs: u64,
    ) -> anyhow::Result<()> {
        self.start_process(start_path, params, time_limit_secs)?;
        self.consume_stdout()
    }

    pub fn end(&mut self) -> anyhow::Result<ExitStatus> {
        let mut runner;

        match self.process.take() {
            Some(proc) => runner = proc,
            None => return Err(ChainErrors::ProcessNotRunning.into()),
        }

        let kill_res = runner.kill();
        println!("{} killed", &self.name);

        if let Some(handle) = self.stdout_thread.take() {
            let _ = handle.join();
            println!("{} monitor stopped", &self.name);
        }

        kill_res?;

        // Wait for the process to finish and get the exit status
        let exit_status = runner.wait()?;
        println!("{} exit code {exit_status}", &self.name);
        Ok(exit_status)
    }

    pub fn has_stdout_monitor(&self) -> bool {
        self.stdout_thread.is_some()
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
