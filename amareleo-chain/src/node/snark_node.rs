use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{ChildStdout, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use crate::chain_errors::ChainErrors;
use crate::config::NodeArgs;
use crate::console::ConsoleManager;
use crate::node::SnarkNode;
use crate::report;

impl<'a> SnarkNode<'a> {
    pub fn new(
        cfg: NodeArgs,
        name: &str,
        console: &'a Arc<Mutex<ConsoleManager>>,
    ) -> SnarkNode<'a> {
        SnarkNode {
            name: name.to_string(),
            args: cfg.node,
            ready_phrase: cfg.started,
            process: None,
            stdout_reader: None,
            stdout_thread: None,
            stdout_silent: Arc::new(AtomicBool::new(true)),
            console,
        }
    }

    fn start_process(&mut self, start_path: &PathBuf, time_limit_secs: u64) -> anyhow::Result<()> {
        if self.process.is_some() {
            return Err(ChainErrors::ProcessAlreadyStarted.into());
        }

        let start_time = Instant::now();

        let mut snarkos = Command::new("snarkos")
            .args(&self.args)
            .stdout(Stdio::piped())
            .current_dir(start_path)
            .spawn()?;

        // Get a handle to the stdout of the spawned process
        let stdout_opt: Option<ChildStdout> = snarkos.stdout.take();
        let stdout: ChildStdout = match stdout_opt {
            None => {
                snarkos.kill()?;
                report!(self.console, &self.name, None, "killed");
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
                report!(
                    self.console,
                    &self.name,
                    None,
                    "ERROR reading node stdout line:",
                    &error.to_string(),
                    ""
                );
                break;
            } else if line.to_ascii_lowercase().contains(&ready_pharse_low) {
                report!(self.console, &self.name, None, &line);
                ready = true;
                break;
            } else if start_time.elapsed() > Duration::from_secs(time_limit_secs) {
                report!(
                    self.console,
                    &self.name,
                    None,
                    "ERROR timeout reading 'ready phrase'",
                    ""
                );
                break;
            } else {
                report!(self.console, &self.name, None, &line);
            }
        }

        if !ready {
            snarkos.kill()?;
            report!(self.console, &self.name, None, "killed");
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
                        // Display line if not silent
                        let silent = shared_stdout_silent.load(Ordering::Relaxed);
                        if !silent {
                            report!(shared_console, &thread_name, None, &chunk);
                        }
                    }
                    Err(err) => {
                        report!(
                            shared_console,
                            &thread_name,
                            None,
                            "ERROR reading node stdout line:",
                            &err.to_string(),
                            "Terminating stdout monitor.",
                            ""
                        );
                        break; // Stop reading on error
                    }
                }
            }
        });

        self.stdout_thread = Some(handle);
        Ok(())
    }

    pub fn start(&mut self, start_path: &PathBuf, time_limit_secs: u64) -> anyhow::Result<()> {
        self.start_process(start_path, time_limit_secs)?;
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
        report!(self.console, &self.name, None, "killed");

        // Wait for stdout thread to exit.
        if let Some(handle) = self.stdout_thread.take() {
            let _ = handle.join();
            report!(self.console, &self.name, None, "monitor stopped");
        }

        // Check if process killing returned any error
        kill_res?;

        // Wait for the process to finish and get the exit status
        let exit_status = runner.wait()?;
        report!(
            self.console,
            &self.name,
            None,
            &format!("exit code {exit_status}")
        );
        Ok(exit_status)
    }

    #[cfg(test)]
    pub fn has_stdout_monitor(&self) -> bool {
        self.stdout_thread.is_some()
    }

    pub fn stdout_show(&mut self, show: bool) {
        self.stdout_silent.store(!show, Ordering::Relaxed);
    }
}

impl<'a> Drop for SnarkNode<'a> {
    fn drop(&mut self) {
        let _ = self.end();
    }
}
