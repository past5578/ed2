use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue, terminal};

use cursor_pos::{CursorPos, Direction};
use raw_mode::RawMode;

mod cursor_pos;
mod raw_mode;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor_pos: CursorPos,
}

impl Editor {
    fn render(&self) -> io::Result<()> {
        let (width, height) = terminal::size()?;

        let mut stdout = io::stdout();

        queue!(stdout, Hide)?;

        for row in 0..height {
            queue!(stdout, MoveTo(0, row))?;

            let mut text = self.get_row_content(row, width, height);
            text.truncate(width as usize);

            queue!(stdout, Print(text), Clear(ClearType::UntilNewLine))?;
        }

        queue!(
            stdout,
            MoveTo(self.cursor_pos.x as u16, self.cursor_pos.y as u16),
            Show
        )?;

        stdout.flush()?;

        Ok(())
    }

    fn get_row_content(&self, row: u16, width: u16, height: u16) -> String {
        let welcome_row = height / 3;

        if row == welcome_row {
            let message = "ed2 0.1.0";
            let padding = (width as usize)
                .saturating_sub(message.chars().count())
                .saturating_sub(1);

            format!("~{}{}", " ".repeat(padding / 2), message)
        } else {
            "~".to_string()
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        let (width, height) = terminal::size()?;

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
        let mut stdout = io::stdout();
        let _guard = RawMode::enter()?;

        execute!(stdout, Clear(ClearType::All))?;

        while !self.should_quit {
            self.render()?;

            if let Event::Key(key_event) = read()? {
                self.handle_key_event(key_event)?;
            }
        }

        Ok(())
    }
}
