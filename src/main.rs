use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod editor;
use editor::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let _ = enable_raw_mode();
    let mut editor = Editor::new();
    editor.open(args.get(1));

    loop {
        editor.refresh_screen();
        if !editor.process_keypress() {
            break;
        }
    }
    editor.purge();

    let _ = disable_raw_mode();
}
