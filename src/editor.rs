use std::io::{self, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyEvent, KeyModifiers, read};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue, terminal};

struct RawMode;

impl RawMode {
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;

        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), Show);
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    fn render(&self) -> io::Result<()> {
        let (width, height) = terminal::size()?;

        let mut stdout = io::stdout();

        queue!(stdout, Hide)?;

        for row in 0..height {
            queue!(stdout, MoveTo(0, row))?;

            let mut text = Self::get_row_content(row, width, height);
            text.truncate(width as usize);

            queue!(stdout, Print(text), Clear(ClearType::UntilNewLine))?;
        }

        queue!(stdout, MoveTo(0, 0), Show)?;

        stdout.flush()?;

        Ok(())
    }

    fn get_row_content(row: u16, width: u16, height: u16) -> String {
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let code = key_event.code;
        let modifiers = key_event.modifiers;

        if code.is_char('q') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
        }

        // println!("{}", code);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let _guard = RawMode::enter()?;

        execute!(stdout, Clear(ClearType::All))?;

        while !self.should_quit {
            self.render()?;

            if let Event::Key(key_event) = read()? {
                self.handle_key_event(key_event)
            }
        }

        Ok(())
    }
}
