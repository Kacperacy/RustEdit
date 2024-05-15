use std::error;

use ratatui::layout::Rect;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub content: Vec<String>,
    pub cursor_position: Position,
    pub cursor_offset: Position,
    pub opened_filename: String,
    pub window_size: Rect,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            content: vec![String::new()],
            cursor_position: Position { x: 0, y: 0 },
            cursor_offset: Position { x: 0, y: 0 },
            opened_filename: String::new(),
            window_size: Rect::new(0, 0, 0, 0),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn insert_char(&mut self, c: char) {
        while self.cursor_position.y >= self.content.len() {
            self.content.push(String::new());
        }

        self.content[self.cursor_position.y + self.cursor_offset.y]
            .insert(self.cursor_position.x + self.cursor_offset.x, c);
        self.move_cursor(Direction { x: 1, y: 0 });
    }

    pub fn add_new_line(&mut self) {
        while self.cursor_position.y >= self.content.len() {
            self.content.push(String::new());
        }

        self.content.insert(
            self.cursor_position.y + self.cursor_offset.y + 1,
            String::new(),
        );
        self.move_cursor(Direction { x: 0, y: -1 });
    }

    pub fn pop_char(&mut self) {
        if self.content.len() == 0 {
            return;
        }

        if self.content[self.cursor_position.y + self.cursor_offset.y].len() == 0 {
            self.content
                .remove(self.cursor_position.y + self.cursor_offset.y);

            self.move_cursor(Direction { x: 0, y: 1 });
        } else {
            self.content[self.cursor_position.y + self.cursor_offset.y]
                .remove(self.cursor_position.x + self.cursor_offset.x - 1);

            self.move_cursor(Direction { x: -1, y: 0 });
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if direction.x < 0 && self.cursor_position.x + self.cursor_offset.x > 0 {
            if self.cursor_position.x == 0 {
                self.cursor_offset.x -= 1;
            } else {
                self.cursor_position.x -= 1;
            }
        } else if direction.x > 0 {
            if let Some(line) = self.content.get(self.cursor_position.y) {
                if line.len() > self.cursor_position.x + self.cursor_offset.x {
                    if self.window_size.width.saturating_sub(1) > self.cursor_position.x as u16 {
                        self.cursor_position.x += 1;
                    } else {
                        self.cursor_offset.x += 1;
                    }
                }
            }
        }

        if direction.y > 0 && self.cursor_position.y + self.cursor_offset.y > 0 {
            if self.cursor_position.y == 0 {
                self.cursor_offset.y -= 1;
            } else {
                self.cursor_position.y -= 1;
            }

            if self.cursor_position.x > self.content[self.cursor_position.y].len() {
                self.cursor_position.x = self.content[self.cursor_position.y].len();
                self.cursor_offset.x = 0;
            }
        } else if direction.y < 0
            && self.cursor_position.y + self.cursor_offset.y < self.content.len().saturating_sub(1)
        {
            if self.window_size.height.saturating_sub(4) > self.cursor_position.y as u16 {
                self.cursor_position.y += 1;
            } else {
                self.cursor_offset.y += 1;
            }

            if self.cursor_position.x > self.content[self.cursor_position.y].len() {
                self.cursor_position.x = self.content[self.cursor_position.y].len();
                self.cursor_offset.x = 0;
            }
        }
    }
}
