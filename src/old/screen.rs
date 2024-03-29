use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Print, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

use crate::{Position, Size};

const VERSION: &str = "0.0.1";

pub struct Screen {
    stdout: Stdout,
    width: usize,
    height: usize,
    filename: Option<String>,
}

impl Screen {
    pub fn new(filename: Option<String>, height: usize) -> Self {
        let stdout = stdout();
        let (width, _) = terminal::size().unwrap();

        Self {
            stdout,
            width: width as usize,
            height,
            filename,
        }
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn purge(&mut self) {
        self.stdout
            .queue(MoveTo(0, 0))
            .unwrap()
            .queue(Clear(ClearType::Purge))
            .unwrap();
    }

    fn queue(&mut self, text: &str) {
        self.stdout.queue(Print(text)).unwrap();
    }

    pub fn set_filename(&mut self, filename: Option<String>) {
        self.filename = filename;
    }

    pub fn refresh_screen(
        &mut self,
        rows: &[String],
        offset: Size,
        cursor: Position,
        status: &str,
        dirty: bool,
    ) {
        self.stdout.queue(Hide).unwrap();
        self.purge();

        self.draw_rows(rows, offset);
        self.draw_status_bar(rows, cursor.y, dirty);
        self.draw_status_message(status);

        self.stdout
            .queue(MoveTo(
                (cursor.x - offset.cols) as u16,
                (cursor.y - offset.rows) as u16,
            ))
            .unwrap()
            .queue(Show)
            .unwrap();
        self.flush();
    }

    fn draw_rows(&mut self, rows: &[String], offset: Size) {
        for i in 0..self.height {
            let file_row = i + offset.rows;
            if file_row >= rows.len() {
                if rows.is_empty() && i == self.height / 3 {
                    let message = "rust-edit v.".to_string() + VERSION;
                    let len = message.len();
                    let padding = (self.width - len) / 2;
                    if padding > 0 {
                        self.queue("~");
                        for _ in 0..padding - 1 {
                            self.queue(" ");
                        }
                        self.queue(&message);
                    }
                } else {
                    self.queue("~");
                }
            } else if offset.cols > rows[file_row].len() {
                self.queue("~");
            } else {
                let row = &&rows[file_row][offset.cols..];
                let len = row.len();
                if len > self.width {
                    self.queue(&row[..self.width]);
                } else {
                    self.queue(row);
                }
            }

            self.stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();
            self.queue("\r\n");
        }
    }

    fn draw_status_bar(&mut self, rows: &[String], cursor_y: usize, dirty: bool) {
        let mut status = format!(
            "{} - line {} of {} {}",
            self.filename.as_deref().unwrap_or("Untitled"),
            cursor_y + 1,
            rows.len(),
            if dirty { "(modified)" } else { "" }
        );
        let percentages = format!(
            "{}%",
            ((cursor_y + 1) as f64 / rows.len() as f64 * 100.0).round()
        );

        status.truncate(self.width);
        let len = status.len();
        for _ in len..self.width - percentages.len() {
            status.push(' ');
        }
        status.push_str(percentages.as_str());
        self.queue(status.reverse().to_string().as_str());
        self.queue("\r\n");
    }

    fn draw_status_message(&mut self, status: &str) {
        self.stdout.queue(Clear(ClearType::CurrentLine)).unwrap();
        self.queue(status);
    }
}
