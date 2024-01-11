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

const NODE0_START_COMPLETE: &str = "INFO No connected validators";
const NODE1_START_COMPLETE: &str = "INFO No connected validators";
// const NODE2_START_COMPLETE: &str = "INFO No connected validators";
// const NODE3_START_COMPLETE: &str = "INFO No connected validators";

// snarkos supports --dev-num-validators parameter
// however this cannot be less than 4
const NODEO_ARG: [&str; 5] = ["start", "--nodisplay", "--dev", "0", "--validator"];

const NODE1_ARG: [&str; 6] = [
    "start",
    "--nodisplay",
    "--dev",
    "1",
    "--validator",
    "--norest",
];

pub struct SnarkNode {
    //Child process handle
    name: String,
    process: Child,
    stdout_reader: Option<BufReader<ChildStdout>>,
    stdout_thread: Option<JoinHandle<()>>,
    stdout_silent: Arc<Mutex<bool>>,
}

impl SnarkNode {
    pub fn new(
        name: &str,
        start_path: &PathBuf,
        params: Vec<&str>,
        time_limit_secs: u64,
        ready_phrase: &str,
    ) -> anyhow::Result<SnarkNode> {
        let start_time = Instant::now();

        let mut snarkos = Command::new("snarkos")
            .args(params)
            .stdout(Stdio::piped())
            .current_dir(start_path)
            .spawn()?;

        let mut ready: bool = false;
        let ready_pharse_low = ready_phrase.to_ascii_lowercase();

        // Get a handle to the stdout of the spawned process
        let stdout = snarkos
            .stdout
            .take()
            .ok_or::<ChainErrors>(ChainErrors::NoProcessStdout)?;

        // Create a buffered reader to read lines from stdout
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

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
            return Err(ChainErrors::CannotFindReady.into());
        }

        Ok(SnarkNode {
            name: name.to_string(),
            process: snarkos,
            stdout_reader: Some(reader),
            stdout_thread: None,
            stdout_silent: Arc::new(Mutex::new(true)),
        })
    }

    pub fn consume_stdout(&mut self) -> anyhow::Result<()> {
        // Clone the Arc for the thread
        let shared_stdout_silent = Arc::clone(&self.stdout_silent);

        // Consume the reader
        let reader = self
            .stdout_reader
            .take()
            .ok_or::<ChainErrors>(ChainErrors::NoProcessStdout)?;

        // Spawn a thread to read from the stdout
        let handle: JoinHandle<()> = thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(chunk) => {
                        // Process each chunk (line) as needed
                        let silent = shared_stdout_silent.lock().expect("Failed to lock Mutex");
                        if !(*silent) {
                            println!("{}", chunk);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading line: {}", err);
                        break; // Stop reading on error
                    }
                }
            }
        });

        self.stdout_thread = Some(handle);
        Ok(())
    }

    pub fn has_stdout_monitor(&self) -> bool {
        self.stdout_thread.is_some()
    }

    pub fn end(&mut self) -> anyhow::Result<ExitStatus> {
        // Kill the process after the specified duration
        let kill_res = self.process.kill();
        println!("{} killed", &self.name);

        if let Some(handle) = self.stdout_thread.take() {
            let _ = handle.join();
            println!("{} monitor stopped", &self.name);
        }

        kill_res?;

        // Wait for the process to finish and get the exit status
        let exit_status = self.process.wait()?;
        println!("{} exit code {exit_status}", &self.name);
        Ok(exit_status)
    }
}

fn main() {
    //Cross-platform compatible retreival of the user's home dir
    let home_dir = get_my_home()
        .expect("Failed to get home directory")
        .unwrap();
    println!("Home dir: {:?}", home_dir);

    //Start a snarkos instance
    let mut node0 = SnarkNode::new(
        "node0",
        &home_dir,
        NODEO_ARG.to_vec(),
        300u64,
        NODE0_START_COMPLETE,
    )
    .expect("Failed to start node0");

    let mut node1 = SnarkNode::new(
        "node1",
        &home_dir,
        NODE1_ARG.to_vec(),
        300u64,
        NODE1_START_COMPLETE,
    )
    .expect("Failed to start node0");

    let _ = node0.consume_stdout();
    let _ = node1.consume_stdout();

    if node0.has_stdout_monitor() {
        thread::sleep(Duration::from_secs(60));
    }

    node1.end().expect("Failed to get node1 exit status");
    node0.end().expect("Failed to get node0 exit status");
}
