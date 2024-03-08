use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

use crossterm::{
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap},
};

use crate::screen::*;

pub struct Editor {
    screen: Screen,
    screen_rows: usize,
    screen_cols: usize,
    row_off: usize,
    col_off: usize,
    cursor_x: usize,
    cursor_y: usize,
    status: String,
    status_time: Duration,
    rows: Vec<String>,
    filename: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        let (screen_cols, mut screen_rows) = crossterm::terminal::size().unwrap();
        screen_rows -= 2;
        Self {
            screen: Screen::new(None, screen_rows as usize),
            screen_rows: screen_rows as usize,
            screen_cols: screen_cols as usize,
            row_off: 0,
            col_off: 0,
            cursor_x: 0,
            cursor_y: 0,
            rows: Vec::new(),
            filename: None,
            status: String::new(),
            status_time: Duration::new(0, 0),
        }
    }

    pub fn run(&mut self) {
        let _ = enable_raw_mode();
        let _ = DisableLineWrap;

        loop {
            self.scroll();
            self.screen.refresh_screen(
                &self.rows,
                self.row_off,
                self.col_off,
                self.cursor_x,
                self.cursor_y,
                &self.status,
            );

            if !self.process_keypress() {
                break;
            }
        }

        self.screen.purge();

        let _ = EnableLineWrap;
        let _ = disable_raw_mode();
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status = message;
        self.status_time = Duration::new(0, 0)
    }

    pub fn read_key(&mut self) -> Result<KeyEvent, ()> {
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

    pub fn die<S: Into<String>>(&mut self, message: S) {
        let _ = disable_raw_mode();
        self.screen.purge();
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
                if self.rows.is_empty() {
                    if self.cursor_y < self.screen_rows {
                        self.cursor_y += 1;
                    }
                } else if self.cursor_y < self.rows.len() {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    if let Some(row) = self.rows.get(self.cursor_y - 1) {
                        self.cursor_y -= 1;
                        self.cursor_x = row.len();
                    }
                }
            }
            KeyCode::Right => {
                if let Some(row) = self.rows.get(self.cursor_y) {
                    if self.cursor_x < row.len() {
                        self.cursor_x += 1;
                    } else if self.cursor_y < self.rows.len() {
                        self.cursor_y += 1;
                        self.cursor_x = 0;
                    }
                }
            }
            KeyCode::PageUp => {
                self.cursor_y = self.row_off;
            }
            KeyCode::PageDown => {
                self.cursor_y = self.row_off + self.screen_rows - 1;
            }
            KeyCode::Home => {
                self.cursor_x = 0;
            }
            KeyCode::End => {
                if let Some(row) = self.rows.get(self.cursor_y) {
                    self.cursor_x = row.len();
                }
            }
            _ => {}
        }

        if let Some(row) = self.rows.get(self.cursor_y) {
            if self.cursor_x > row.len() {
                self.cursor_x = row.len();
            }
        }
    }

    pub fn open(&mut self, _filename: Option<&String>) {
        if let Some(filename) = _filename {
            if let Ok(file) = File::open(filename) {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    self.rows.push(line);
                }
                self.filename = Some(filename.clone());
                self.screen.set_filename(Some(filename.clone()));
            }
        }
    }

    pub fn scroll(&mut self) {
        if self.cursor_y < self.row_off {
            self.row_off = self.cursor_y;
        }
        if self.cursor_y >= self.row_off + self.screen_rows {
            self.row_off = self.cursor_y - self.screen_rows + 1;
        }
        if self.cursor_x < self.col_off {
            self.col_off = self.cursor_x;
        }
        if self.cursor_x >= self.col_off + self.screen_cols {
            self.col_off = self.cursor_x - self.screen_cols + 1;
        }
    }
}
