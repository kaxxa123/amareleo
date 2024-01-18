use crossterm::{cursor, execute, style, style::Color, style::Print, terminal};

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

    pub fn status(&mut self, status: &str) {
        let mut stdout = std::io::stdout();
        let _ = execute!(
            stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, self.size + 1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::SetBackgroundColor(Color::DarkRed),
            style::SetForegroundColor(Color::White),
            Print(format!(" {:<500}", status)),
            style::ResetColor,
            cursor::RestorePosition,
        );
    }
}

impl Drop for ConsoleManager {
    fn drop(&mut self) {
        if self.raw_mode {
            // Try to cleanup.
            // In case of failure we cannot do much...
            let _ = execute!(
                std::io::stdout(),
                cursor::MoveTo(0, self.size + 2),
                terminal::EnableLineWrap
            );

            let _ = terminal::disable_raw_mode();
            self.raw_mode = false;
        }
    }
}
