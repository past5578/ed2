use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};

use cursor_pos::{CursorPos, Direction};
use terminal::Terminal;

mod cursor_pos;
mod terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: CursorPos,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        Ok(Editor {
            should_quit: false,
            terminal: Terminal::new()?,
            cursor_pos: CursorPos::default(),
        })
    }

    fn render(&self) -> io::Result<()> {
        let (width, height) = self.terminal.size()?;

        self.terminal.hide_cursor()?;

        for row in 0..height {
            self.terminal.move_cursor_to(0, row)?;

            let mut text = self.get_row_content(row, width, height);
            text.truncate(width);

            self.terminal.print(&text)?;
            self.terminal.clear_until_new_line()?;
        }

        self.terminal
            .move_cursor_to(self.cursor_pos.x, self.cursor_pos.y)?;

        self.terminal.show_cursor()?;

        self.terminal.flush()?;

        Ok(())
    }

    fn get_row_content(&self, row: usize, width: usize, height: usize) -> String {
        let welcome_row = height / 3;

        if row == welcome_row {
            let message = "ed2 0.1.0";
            let padding = width
                .saturating_sub(message.chars().count())
                .saturating_sub(1);

            format!("~{}{}", " ".repeat(padding / 2), message)
        } else {
            "~".to_string()
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        let (width, height) = self.terminal.size()?;

        let code = key_event.code;
        let modifiers = key_event.modifiers;

        if code.is_char('q') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
        }

        if key_event.is_press() {
            let direction = match code {
                KeyCode::Up => Some(Direction::Up),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Left => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                _ => None,
            };

            if let Some(direction) = direction {
                self.cursor_pos.move_in(
                    direction,
                    width.saturating_sub(1),
                    height.saturating_sub(1),
                )
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.clear_screen()?;

        while !self.should_quit {
            self.render()?;

            if let Event::Key(key_event) = read()? {
                self.handle_key_event(key_event)?;
            }
        }

        Ok(())
    }
}
