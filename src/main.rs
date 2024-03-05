mod editor;
use editor::*;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    let _ = enable_raw_mode();
    let editor = Editor::new();

    editor.refresh_screen();
    loop {
        editor.refresh_screen();
        if !editor.process_keypress() {
            break;
        }
    }
    editor.refresh_screen();

    let _ = disable_raw_mode();
}
