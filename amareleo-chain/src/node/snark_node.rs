use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{ChildStdout, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use crate::chain_errors::ChainErrors;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;

impl<'a> SnarkNode<'a> {
    pub fn new(name: &str, ready: &str, console: &'a Arc<Mutex<ConsoleManager>>) -> SnarkNode<'a> {
        SnarkNode {
            name: name.to_string(),
            ready_phrase: ready.to_string(),
            process: None,
            stdout_reader: None,
            stdout_thread: None,
            stdout_silent: Arc::new(AtomicBool::new(true)),
            console,
        }
    }

    fn start_process(
        &mut self,
        start_path: &PathBuf,
        params: Vec<String>,
        time_limit_secs: u64,
    ) -> anyhow::Result<()> {
        if self.process.is_some() {
            return Err(ChainErrors::ProcessAlreadyStarted.into());
        }

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
                self.report("killed");
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
                self.report_err(&format!("Error reading line: {}", error));
                break;
            } else if line.to_ascii_lowercase().contains(&ready_pharse_low) {
                self.report(&line);
                ready = true;
                break;
            } else if start_time.elapsed() > Duration::from_secs(time_limit_secs) {
                self.report_err("Timeout reading line");
                break;
            } else {
                self.report(&line);
            }
        }

        if !ready {
            snarkos.kill()?;
            self.report("killed");
            return Err(ChainErrors::CannotFindReady.into());
        }

        self.process = Some(snarkos);
        self.stdout_reader = Some(reader);

        Ok(())
    }

    fn consume_stdout(&mut self) -> anyhow::Result<()> {
        // Clone the Arc for the thread
        let shared_stdout_silent = Arc::clone(&self.stdout_silent);
        let shared_console = Arc::clone(self.console);

        // Consume the reader
        let reader = self
            .stdout_reader
            .take()
            .ok_or::<ChainErrors>(ChainErrors::NoStdout)?;

        let thread_name = String::from(&self.name);

        // Spawn a thread to read from the stdout
        let handle = thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(chunk) => {
                        // Process each chunk (line) as needed
                        let silent = shared_stdout_silent.load(Ordering::Relaxed);

                        // Display line if not silent
                        if !silent {
                            let console = shared_console.lock();
                            let _ = console.map(|mut obj| obj.report(&thread_name, &chunk));
                        }
                    }
                    Err(err) => {
                        report_err(&thread_name, &format!("Error reading line: {}", err));
                        report_err(&thread_name, "terminating stdout monitor.");
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

        // Get process and set it to None to block freeing it twice.
        // ...and kill the process.
        match self.process.take() {
            Some(proc) => runner = proc,
            None => return Err(ChainErrors::ProcessNotRunning.into()),
        }

        let kill_res = runner.kill();
        self.report("killed");

        // Wait for stdout thread to exit.
        if let Some(handle) = self.stdout_thread.take() {
            let _ = handle.join();
            self.report("monitor stopped");
        }

        // Check if process killing returned any error
        kill_res?;

        // Wait for the process to finish and get the exit status
        let exit_status = runner.wait()?;
        self.report(&format!("exit code {exit_status}"));
        Ok(exit_status)
    }

    #[cfg(test)]
    pub fn has_stdout_monitor(&self) -> bool {
        self.stdout_thread.is_some()
    }

    pub fn stdout_show(&mut self, show: bool) {
        self.stdout_silent.store(!show, Ordering::Relaxed);
    }

    fn report(&self, line: &str) {
        let console_a = self.console.lock();
        let _ = console_a.map(|mut obj| obj.report(&self.name, line));
    }

    fn report_err(&self, line: &str) {
        report_err(&self.name, line);
    }
}

impl<'a> Drop for SnarkNode<'a> {
    fn drop(&mut self) {
        let _ = self.end();
    }
}

fn report_err(name: &str, line: &str) {
    eprintln!("{} | {}", name, line.trim_end());
}
