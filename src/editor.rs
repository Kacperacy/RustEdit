use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    time::Duration,
};

use crossterm::{
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap},
};

use crate::screen::*;

const QUIT_TIMES: u8 = 3;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

pub struct Editor {
    screen: Screen,
    screen_size: Size,
    offset: Size,
    cursor: Position,
    status: String,
    status_time: Duration,
    rows: Vec<String>,
    filename: Option<String>,
    dirty: bool,
    quit_times: u8,
}

impl Editor {
    pub fn new() -> Self {
        let (screen_cols, mut screen_rows) = crossterm::terminal::size().unwrap();
        screen_rows -= 2;
        Self {
            screen: Screen::new(None, screen_rows as usize),
            screen_size: Size {
                rows: screen_rows as usize,
                cols: screen_cols as usize,
            },
            offset: Size { rows: 0, cols: 0 },
            cursor: Position { x: 0, y: 0 },
            rows: Vec::new(),
            filename: None,
            status: String::new(),
            status_time: Duration::new(0, 0),
            dirty: false,
            quit_times: QUIT_TIMES,
        }
    }

    fn die<S: Into<String>>(&mut self, message: S) {
        let _ = disable_raw_mode();
        self.screen.purge();
        eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
        std::process::exit(1);
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
            } else {
                self.filename = Some(filename.clone());
                self.screen.set_filename(Some(filename.clone()));
            }
            self.dirty = false;
        }
    }

    pub fn run(&mut self) {
        let _ = enable_raw_mode();
        let _ = DisableLineWrap;

        loop {
            self.scroll();
            self.screen.refresh_screen(
                &self.rows,
                self.offset,
                self.cursor,
                &self.status,
                self.dirty,
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

    fn read_key(&mut self) -> KeyEvent {
        if let Ok(Key(key)) = read() {
            key
        } else {
            self.die("Read error");
            unreachable!()
        }
    }

    fn process_keypress(&mut self) -> bool {
        let c = self.read_key();

        if KeyModifiers::CONTROL == c.modifiers {
            if c.code == KeyCode::Char('q') {
                if self.dirty {
                    self.quit_times -= 1;
                    self.set_status_message(format!(
                        "WARNING!!! File has unsaved changes. Press Ctrl-Q {} more time(s) to quit.",
                        self.quit_times
                    ));
                    return self.quit_times != 0;
                } else {
                    return false;
                }
            } else if c.code == KeyCode::Char('s') {
                self.save();
            } else if c.code == KeyCode::Char('f') {
                self.find();
            }
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
        } else if c.code == KeyCode::Backspace {
            self.delete_char();
        } else if c.code == KeyCode::Enter {
            self.rows.insert(self.cursor.y, String::new());
            self.cursor.x = 0;
            self.cursor.y += 1;
        } else if let KeyCode::Char(c) = c.code {
            self.insert_char(c);
        }
        self.quit_times = QUIT_TIMES;
        true
    }

    fn move_cursor(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                }
            }
            KeyCode::Down => {
                if self.rows.is_empty() {
                    if self.cursor.y < self.screen_size.rows {
                        self.cursor.y += 1;
                    }
                } else if self.cursor.y < self.rows.len() {
                    self.cursor.y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                } else if self.cursor.y > 0 {
                    if let Some(row) = self.rows.get(self.cursor.y - 1) {
                        self.cursor.y -= 1;
                        self.cursor.x = row.len();
                    }
                }
            }
            KeyCode::Right => {
                if let Some(row) = self.rows.get(self.cursor.y) {
                    if self.cursor.x < row.len() {
                        self.cursor.x += 1;
                    } else if self.cursor.y < self.rows.len() {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            KeyCode::PageUp => {
                self.cursor.y = self.offset.rows;
            }
            KeyCode::PageDown => {
                self.cursor.y = self.offset.rows + self.screen_size.rows - 1;
            }
            KeyCode::Home => {
                self.cursor.x = 0;
            }
            KeyCode::End => {
                if let Some(row) = self.rows.get(self.cursor.y) {
                    self.cursor.x = row.len();
                }
            }
            _ => {}
        }

        if let Some(row) = self.rows.get(self.cursor.y) {
            if self.cursor.x > row.len() {
                self.cursor.x = row.len();
            }
        } else {
            self.cursor.x = 0;
        }
    }

    fn scroll(&mut self) {
        if self.cursor.y < self.offset.rows {
            self.offset.rows = self.cursor.y;
        }
        if self.cursor.y >= self.offset.rows + self.screen_size.rows {
            self.offset.rows = self.cursor.y - self.screen_size.rows + 1;
        }
        if self.cursor.x < self.offset.cols {
            self.offset.cols = self.cursor.x;
        }
        if self.cursor.x >= self.offset.cols + self.screen_size.cols {
            self.offset.cols = self.cursor.x - self.screen_size.cols + 1;
        }
    }

    fn insert_char(&mut self, c: char) {
        while self.cursor.y >= self.rows.len() {
            self.rows.push(String::new());
        }
        self.rows[self.cursor.y].insert(self.cursor.x, c);
        self.cursor.x += 1;
        self.dirty = true;
    }

    fn delete_char(&mut self) {
        if self.cursor.y >= self.rows.len() {
            return;
        }
        if self.cursor.x > 0 {
            self.rows[self.cursor.y].remove(self.cursor.x - 1);
            self.cursor.x -= 1;
            self.dirty = true;
        } else if self.cursor.y > 0 {
            let row = self.rows.remove(self.cursor.y);
            self.cursor.y -= 1;
            self.cursor.x = self.rows[self.cursor.y].len();
            self.rows[self.cursor.y].push_str(&row);
            self.dirty = true;
        }
    }

    fn save(&mut self) {
        if let Some(filename) = &self.filename {
            let mut file = File::create(filename).unwrap();
            for row in &self.rows {
                file.write_all(row.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
            self.set_status_message(format!(
                "{} bytes written to disk",
                file.metadata().unwrap().len()
            ));
            self.dirty = false;
        } else if let Some(filename) = self.prompt("Save as: ") {
            self.filename = Some(filename.clone());
            self.screen.set_filename(Some(filename.clone()));
            self.save();
        }
    }

    fn find(&mut self) {
        if let Some(query) = self.prompt("Search: ") {
            for i in 0..self.rows.len() {
                if let Some(pos) = self.rows[i].find(&query) {
                    self.cursor.y = i;
                    self.cursor.x = pos;
                    self.offset.rows = self.rows.len();
                    break;
                }
            }
        }
    }

    fn prompt(&mut self, prompt: &str) -> Option<String> {
        self.set_status_message(prompt.to_string());
        let mut input = String::new();
        loop {
            self.set_status_message(format!("{}{}", prompt, input));
            self.scroll();
            self.screen.refresh_screen(
                &self.rows,
                self.offset,
                self.cursor,
                &self.status,
                self.dirty,
            );
            let c = self.read_key();
            if c.code == KeyCode::Esc {
                self.set_status_message(String::new());
                return None;
            } else if c.code == KeyCode::Enter {
                if !input.is_empty() {
                    self.set_status_message(String::new());
                    return Some(input);
                }
            } else if c.code == KeyCode::Backspace {
                input.pop();
            } else if let KeyCode::Char(c) = c.code {
                input.push(c);
            }
        }
    }
}
