use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event::Key, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, Clear, ClearType},
};

const VERSION: &str = "0.0.1";

pub struct Editor {
    screen_rows: usize,
    screen_cols: usize,
    row_off: usize,
    col_off: usize,
    cursor_x: usize,
    cursor_y: usize,
    rows: Vec<String>,
    filename: Option<String>,
}

impl Editor {
    pub fn new() -> Self {
        let (screen_cols, screen_rows) = crossterm::terminal::size().unwrap();
        Self {
            screen_rows: (screen_rows - 1) as usize,
            screen_cols: screen_cols as usize,
            row_off: 0,
            col_off: 0,
            cursor_x: 0,
            cursor_y: 0,
            rows: Vec::new(),
            filename: None,
        }
    }

    pub fn refresh_screen(&mut self) {
        self.scroll();
        let _ = execute!(stdout(), Hide, MoveTo(0, 0), Clear(ClearType::Purge));

        let mut buffer = String::new();
        self.draw_rows(&mut buffer);
        self.draw_status_bar(&mut buffer);
        stdout().write_all(buffer.as_bytes()).unwrap();

        let _ = execute!(
            stdout(),
            MoveTo(
                (self.cursor_x - self.col_off) as u16,
                (self.cursor_y - self.row_off) as u16
            ),
            Show
        );
    }

    pub fn draw_rows(&self, buffer: &mut String) {
        for i in 0..self.screen_rows {
            let file_row = i + self.row_off;
            if file_row >= self.rows.len() {
                if self.rows.is_empty() && i == self.screen_rows / 3 {
                    let message = "rust-edit v.".to_string() + VERSION + " - Press Ctrl-Q to quit";
                    let len = message.len();
                    let padding = (self.screen_cols - len) / 2;
                    if padding > 0 {
                        buffer.push('~');
                        for _ in 0..padding - 1 {
                            buffer.push(' ');
                        }
                        buffer.push_str(&message);
                    }
                } else {
                    buffer.push('~');
                }
            } else if self.col_off > self.rows[file_row].len() {
                buffer.push('~');
            } else {
                let row = &&self.rows[file_row][self.col_off..];
                let len = row.len();
                if len > self.screen_cols {
                    buffer.push_str(&row[..self.screen_cols]);
                } else {
                    buffer.push_str(row);
                }
            }

            buffer.push_str(&format!("{}", Clear(ClearType::UntilNewLine)));

            buffer.push_str("\r\n");
        }
    }

    fn draw_status_bar(&self, buffer: &mut String) {
        let mut status = format!(
            "{} - line {} of {}",
            self.filename.as_deref().unwrap_or("Untitled"),
            self.cursor_y + 1,
            self.rows.len()
        );

        status.truncate(self.screen_cols);
        let len = status.len();
        for _ in len..self.screen_cols {
            status.push(' ');
        }
        buffer.push_str(status.reverse().to_string().as_str());
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

    pub fn purge(&self) {
        let _ = execute!(stdout(), MoveTo(0, 0), Clear(ClearType::Purge));
    }

    pub fn open(&mut self, _filename: Option<&String>) {
        if let Some(filename) = _filename {
            if let Ok(file) = File::open(filename) {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    self.rows.push(line);
                }
                self.filename = Some(filename.clone());
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
