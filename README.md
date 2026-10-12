# ed2

A bare-bones text editor to use as a learning project for Rust. It is not yet ready for everyday use, but it’s being worked on almost every day.

## Usage

- Use the arrow keys for cursor movement.
- Quit the editor using Ctrl-Q.

## Build & Run

```sh
cargo build --release
./target/release/ed2 # or ".\target\release\ed2.exe" on windows
```

## Planned

- File opening and displaying
- Scrolling in the display
- Unicode support
- Editing the buffer
- Saving buffer edits
- Status bar (could be sort of like Vim)
- Searching buffer