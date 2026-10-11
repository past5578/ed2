use std::io;

use crossterm::event::{Event, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
    }
}

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let code = key_event.code;
        let modifiers = key_event.modifiers;

        if code.is_char('q') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;

            return;
        }

        println!("{}", code);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let _guard = RawMode::enter()?;

        while !self.should_quit {
            match read()? {
                Event::Key(key_event) => self.handle_key_event(key_event),
                _ => {}
            }
        }

        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}
