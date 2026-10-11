use std::io;

use editor::Editor;

mod editor;

fn main() -> io::Result<()> {
    Editor::default().run()
}
