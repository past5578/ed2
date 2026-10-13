use std::io::{self, Stdout, Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use super::raw_mode::RawMode;

pub(super) struct Terminal {
    _raw_mode_guard: RawMode,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        Ok(Terminal {
            _raw_mode_guard: RawMode::enter()?,
            stdout: stdout(),
        })
    }

    pub fn size(&self) -> io::Result<(usize, usize)> {
        let (columns, rows) = crossterm::terminal::size()?;

        Ok((columns.into(), rows.into()))
    }

    pub fn flush(&self) -> io::Result<()> {
        (&self.stdout).flush()
    }
}

// Cursor methods
impl Terminal {
    pub fn show_cursor(&self) -> io::Result<()> {
        queue!(&self.stdout, Show)
    }

    pub fn hide_cursor(&self) -> io::Result<()> {
        queue!(&self.stdout, Hide)
    }

    pub fn move_cursor_to(&self, x: usize, y: usize) -> io::Result<()> {
        queue!(&self.stdout, MoveTo(x as u16, y as u16))
    }
}

// Drawing methods
impl Terminal {
    pub fn print(&self, text: &str) -> io::Result<()> {
        queue!(&self.stdout, Print(text))
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        queue!(&self.stdout, Clear(ClearType::All))
    }

    pub fn clear_until_new_line(&self) -> io::Result<()> {
        queue!(&self.stdout, Clear(ClearType::UntilNewLine))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = execute!(&self.stdout, Show);
    }
}
