use crossterm::{
    cursor::MoveTo,
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

fn main() {
    let _ = enable_raw_mode();

    editor_refresh_screen();
    loop {
        editor_refresh_screen();
        if !editor_process_keypress() {
            break;
        }
    }
    editor_refresh_screen();

    let _ = disable_raw_mode();
}

fn die<S: Into<String>>(message: S) {
    let _ = disable_raw_mode();
    editor_refresh_screen();
    eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
    std::process::exit(1);
}

fn editor_refresh_screen() {
    print!("{} {}", Clear(ClearType::All), MoveTo(0, 0));
    editor_draw_rows();
    print!("{}", MoveTo(0, 0));
}

fn editor_draw_rows() {
    for _ in 0..24 {
        println!("~\r");
    }
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
