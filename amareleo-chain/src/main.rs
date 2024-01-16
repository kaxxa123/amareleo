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
    let base_console = ConsoleManager::start(10);
    let console = Arc::new(Mutex::new(base_console));

    let mut nodes = NodeSet::new(&console);

    let res = nodes.start();

    if res.is_ok() {
        {
            let console_a = console.lock();
            let _ = console_a.map(|mut obj| {
                obj.report("main", "");
                obj.report("main", "All nodes started!");
                obj.report("main", "");
            });
        }

        loop {
            if let Ok(event::Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('q') => {
                        let console_a = console.lock();
                        let _ = console_a.map(|mut obj| obj.report("main", "Quitting..."));

                        break;
                    }

                    KeyCode::Char('s') => {
                        {
                            let console_a = console.lock();
                            let _ = console_a.map(|mut obj| obj.report("main", "Silent..."));
                        }
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
}
