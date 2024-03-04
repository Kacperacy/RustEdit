use crossterm::{
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let _ = enable_raw_mode();
    while editor_process_keypress() {}
    let _ = disable_raw_mode();
}

fn die<S: Into<String>>(message: S) {
    let _ = disable_raw_mode();
    eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
    std::process::exit(1);
}

fn editor_read_key() -> Result<KeyEvent, ()> {
    if let Ok(Key(key)) = read() {
        Ok(key)
    } else {
        die("Read error");
        Err(())
    }
}

fn editor_process_keypress() -> bool {
    match editor_read_key() {
        Ok(c) => {
            println!("{c:?}\r");
            !(c.code == KeyCode::Char('q') && KeyModifiers::CONTROL == c.modifiers)
        }
        Err(_) => false,
    }
}
