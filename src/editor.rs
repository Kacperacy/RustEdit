use crossterm::{
    cursor::MoveTo,
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, Clear, ClearType},
};

pub struct Editor {
    screen_rows: usize,
    screen_cols: usize,
}

impl Editor {
    pub fn new() -> Self {
        let (screen_cols, screen_rows) = crossterm::terminal::size().unwrap();
        Self {
            screen_rows: screen_rows as usize,
            screen_cols: screen_cols as usize,
        }
    }

    pub fn refresh_screen(&self) {
        print!("{} {}", Clear(ClearType::All), MoveTo(0, 0));
        self.draw_rows();
        print!("{}", MoveTo(0, 0));
    }

    pub fn draw_rows(&self) {
        for _ in 0..self.screen_rows {
            print!("~");
            if self.screen_rows > 1 {
                print!("\r\n");
            }
        }
    }

    pub fn read_key(&self) -> Result<KeyEvent, ()> {
        if let Ok(Key(key)) = read() {
            Ok(key)
        } else {
            self.die("Read error");
            Err(())
        }
    }

    pub fn process_keypress(&self) -> bool {
        match self.read_key() {
            Ok(c) => {
                println!("{c:?}\r");
                !(c.code == KeyCode::Char('q') && KeyModifiers::CONTROL == c.modifiers)
            }
            Err(_) => false,
        }
    }

    pub fn die<S: Into<String>>(&self, message: S) {
        let _ = disable_raw_mode();
        self.refresh_screen();
        eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
        std::process::exit(1);
    }

    pub fn get_cursor_position(&self) -> Result<(u16, u16), ()> {
        crossterm::cursor::position().map_err(|_| ())
    }
}
