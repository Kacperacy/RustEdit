use std::io::BufRead;

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, Clear, ClearType},
};

const VERSION: &str = "0.0.1";

pub struct Editor {
    screen_rows: usize,
    screen_cols: usize,
    cursor_x: usize,
    cursor_y: usize,
    rows: Vec<String>,
}

impl Editor {
    pub fn new() -> Self {
        let (screen_cols, screen_rows) = crossterm::terminal::size().unwrap();
        Self {
            screen_rows: screen_rows as usize,
            screen_cols: screen_cols as usize,
            cursor_x: 0,
            cursor_y: 0,
            rows: Vec::new(),
        }
    }

    pub fn refresh_screen(&self) {
        print!("{} {}", Hide, MoveTo(0, 0));
        self.draw_rows();
        print!("{}", MoveTo(self.cursor_x as u16, self.cursor_y as u16));
    }

    pub fn draw_rows(&self) {
        for i in 0..self.screen_rows {
            if i >= self.rows.len() {
                if self.rows.is_empty() && i == self.screen_rows / 3 {
                    let message = "rust-edit v.".to_string() + VERSION + " - Press Ctrl-Q to quit";
                    let len = message.len();
                    let padding = (self.screen_cols - len) / 2;
                    if padding > 0 {
                        print!("~");
                        for _ in 0..padding - 1 {
                            print!(" ");
                        }
                        print!("{}", message);
                    }
                } else {
                    print!("~");
                }
            } else if self.rows[i].len() > self.screen_cols {
                print!("{}", &self.rows[i][0..self.screen_cols]);
            } else {
                print!("{}", &self.rows[i]);
            }

            print!("{}", Clear(ClearType::UntilNewLine));
            if i < self.screen_rows - 1 {
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

    pub fn process_keypress(&mut self) -> bool {
        match self.read_key() {
            Ok(c) => {
                println!("{c:?}\r");

                if c.code == KeyCode::Char('q') && KeyModifiers::CONTROL == c.modifiers {
                    false
                } else if c.code == KeyCode::Up
                    || c.code == KeyCode::Down
                    || c.code == KeyCode::Left
                    || c.code == KeyCode::Right
                    || c.code == KeyCode::PageUp
                    || c.code == KeyCode::PageDown
                    || c.code == KeyCode::Home
                    || c.code == KeyCode::End
                {
                    self.move_cursor(c);
                    true
                } else {
                    true
                }
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

    pub fn move_cursor(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < self.screen_rows {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.screen_cols {
                    self.cursor_x += 1;
                }
            }
            KeyCode::PageUp => {
                self.cursor_y = 0;
            }
            KeyCode::PageDown => {
                self.cursor_y = self.screen_rows;
            }
            KeyCode::Home => {
                self.cursor_x = 0;
            }
            KeyCode::End => {
                self.cursor_x = self.screen_cols;
            }
            _ => {}
        }
    }

    pub fn purge(&self) {
        print!("{}", Clear(ClearType::Purge));
    }

    pub fn open(&mut self, _filename: Option<&String>) {
        if let Some(filename) = _filename {
            let file = std::fs::File::open(filename).unwrap();
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                self.rows.push(line.unwrap());
            }
        }
    }
}
