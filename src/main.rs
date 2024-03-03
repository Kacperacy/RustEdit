use std::time::Duration;

use crossterm::{
    event::{poll, read, Event::Key, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let _ = enable_raw_mode();
    loop {
        let mut c: Option<KeyEvent> = None;

        if let Ok(true) = poll(Duration::from_millis(100)) {
            if let Ok(Key(key)) = read() {
                c = Some(key);
            }
        }

        if let Some(c) = c {
            println!("{c:?}\r");
            if c.code == KeyCode::Char('q') {
                break;
            }
        } else {
            println!("No input\r");
        }
    }
    let _ = disable_raw_mode();
}
