mod chain_errors;
mod console;
mod node;
mod node_batch;

use std::sync::Arc;
use std::sync::Mutex;

use console::ConsoleManager;
use crossterm::event::{self, KeyCode};
use node_batch::NodeSet;

fn main() {
    let mut base_console = ConsoleManager::start(20);
    base_console.status("Starting nodes...");

    let console = Arc::new(Mutex::new(base_console));

    let mut nodes = NodeSet::new(&console);
    match nodes.start() {
        Ok(_) => {
            report!(
                console,
                "main",
                Some("q - quit | (0, 1, 2, 3) - node dump | s - silent"),
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

#[cfg(test)]
mod tests;
