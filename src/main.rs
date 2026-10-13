use std::io;

use editor::Editor;

mod editor;

fn main() -> io::Result<()> {
    let mut editor = Editor::new()?;

    editor.run()
}
