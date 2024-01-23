mod app_args;
mod chain_errors;
mod config;
mod console;
mod node;
mod node_batch;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use app_args::*;
use clap::Parser;

use config::*;
use console::ConsoleManager;
use crossterm::event::{self, KeyCode};
use node_batch::NodeSet;

fn run(args: &RunArgs) {
    let mut base_console = ConsoleManager::start(20);
    base_console.status("Starting nodes...");

    let chain_cfg = match ChainArgs::load(&args.cfg) {
        Err(err) => {
            base_console.batch_report(
                "main",
                None,
                &["ERROR on loading config", &err.to_string(), ""],
            );
            return;
        }
        Ok(cfg) => cfg,
    };

    let console = Arc::new(Mutex::new(base_console));
    let mut nodes = NodeSet::new(chain_cfg, &console);
    match nodes.start(&args.ledger) {
        Ok(_) => {
            report!(
                console,
                "main",
                Some("q - quit | (0, 1, 2...) - node log | s - silent"),
                "",
                "All nodes started!",
                ""
            );

            loop {
                if let Ok(event::Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            report!(console, "main", None, "Quitting...");
                            break;
                        }

                        KeyCode::Char('s') => {
                            report!(console, "main", None, "Silent...");
                            nodes.stdout_silent();
                        }

                        KeyCode::Char('0') => nodes.stdout_show(0),
                        KeyCode::Char('1') => nodes.stdout_show(1),
                        KeyCode::Char('2') => nodes.stdout_show(2),
                        KeyCode::Char('3') => nodes.stdout_show(3),
                        KeyCode::Char('4') => nodes.stdout_show(4),
                        KeyCode::Char('5') => nodes.stdout_show(5),
                        KeyCode::Char('6') => nodes.stdout_show(6),
                        KeyCode::Char('7') => nodes.stdout_show(7),
                        KeyCode::Char('8') => nodes.stdout_show(8),
                        KeyCode::Char('9') => nodes.stdout_show(9),

                        _ => {}
                    }
                }
            }
        }
        Err(err) => {
            report!(
                console,
                "main",
                Some("Failed!"),
                "",
                "Failed on starting nodes!",
                &err.to_string(),
                ""
            );
        }
    }
}

fn init(args: &InitArgs) {
    let mut base_console = ConsoleManager::start(20);
    base_console.status("Initializing config...");

    let cfg_path: anyhow::Result<PathBuf> = match &args.cfg {
        Some(apath) => Ok(PathBuf::from(apath)),
        None => match create_amareleo_dir() {
            Ok(default_dir) => Ok(default_dir.join(AMARELEO_CHAIN_CFG)),
            Err(err) => {
                base_console.batch_report(
                    "main",
                    Some("Failed"),
                    &[
                        "ERROR on initializing default config path",
                        &err.to_string(),
                        "",
                    ],
                );
                Err(err)
            }
        },
    };

    if let Ok(path) = cfg_path {
        if !args.force && path.exists() {
            base_console.batch_report(
                "main",
                Some("Failed"),
                &[
                    "ERROR config already exists. Use --force to overwrite",
                    path.to_str().unwrap(),
                    "",
                ],
            );

            return;
        }

        match ChainArgs::init(&path) {
            Err(err) => base_console.batch_report(
                "main",
                Some("Failed"),
                &[
                    "ERROR on initializing config",
                    &err.to_string(),
                    path.to_str().unwrap(),
                    "",
                ],
            ),
            Ok(_) => {
                base_console.batch_report(
                    "main",
                    Some("Ready"),
                    &["Configuration initialized", path.to_str().unwrap(), ""],
                );
            }
        }
    }
}

fn main() {
    let params = AppCmds::try_parse();

    let params = match params {
        Ok(res) => res.cmd_type.unwrap_or(AppCmdType::Run(RunArgs {
            cfg: None,
            ledger: None,
        })),
        Err(err) => {
            let _ = err.print();
            cmd_usage();
            return;
        }
    };

    match params {
        AppCmdType::Init(args) => init(&args),
        AppCmdType::Run(args) => run(&args),
    }
}

#[cfg(test)]
mod tests;
