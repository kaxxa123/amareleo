use crossterm::{cursor, execute, style::Print, terminal};

use crate::console::*;

impl ConsoleManager {
    pub fn start(size: u16) -> ConsoleManager {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        execute!(
            std::io::stdout(),
            terminal::DisableLineWrap,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();

        ConsoleManager {
            raw_mode: true,
            position: 0,
            size,
        }
    }

    fn end(&mut self) {
        if self.raw_mode {
            execute!(
                std::io::stdout(),
                cursor::MoveTo(0, self.size),
                terminal::EnableLineWrap
            )
            .unwrap();

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            self.raw_mode = false;
        }
    }

    pub fn report(&mut self, name: &str, line: &str) {
        let mut stdout = std::io::stdout();

        let _ = execute!(
            stdout,
            cursor::MoveTo(0, self.position),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(format!("{} | {}", name, line.trim_end())),
            cursor::MoveTo(0, self.position + 1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        );

        self.position = (self.position + 1) % self.size;
    }
}

impl Drop for ConsoleManager {
    fn drop(&mut self) {
        self.end();
    }
}
