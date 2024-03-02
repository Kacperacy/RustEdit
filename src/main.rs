use crossterm::{
    event::{read, Event::Key, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let _ = enable_raw_mode();
    loop {
        if let Ok(Key(key)) = read() {
            if key.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{:?}\r", key);
            }
        }
    }
    let _ = disable_raw_mode();
}
