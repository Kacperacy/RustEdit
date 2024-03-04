use crossterm::{
    event::{poll, read, Event::Key, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

fn main() {
    let _ = enable_raw_mode();
    loop {
        let mut c: Option<KeyEvent> = None;

        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Ok(Key(key)) = read() {
                    c = Some(key);
                } else {
                    die("Read error")
                }
            }
            Ok(false) => (),
            Err(_) => die("Poll error"),
        }

        if let Some(c) = c {
            println!("{c:?}\r");
            if c.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    let _ = disable_raw_mode();
}

fn die<S: Into<String>>(message: S) {
    let _ = disable_raw_mode();
    eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
    std::process::exit(1);
}
