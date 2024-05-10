use std::error;

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            content: Vec::new(),
            cursor_position: Position { x: 0, y: 0 },
            cursor_offset: Position { x: 0, y: 0 },
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
        self.content[self.cursor_position.y].insert(self.cursor_position.x, c);
        self.cursor_position.x += 1;
    }

    pub fn add_new_line(&mut self) {
        self.cursor_position.y += 1;
        self.cursor_position.x = 0;
    }

    pub fn pop_char(&mut self) {
        self.content[self.cursor_position.y].pop();

        if self.cursor_position.x > 0 {
            self.cursor_position.x -= 1;
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if direction.x < 0 && self.cursor_position.x > 0 {
            self.cursor_position.x -= 1;
        } else if direction.x > 0 {
            if let Some(line) = self.content.get(self.cursor_position.y) {
                if line.len() > self.cursor_position.x {
                    self.cursor_position.x += 1;
                }
            }
        }

        if direction.y > 0 && self.cursor_position.y > 0 {
            self.cursor_position.y -= 1;
            if self.cursor_position.x > self.content[self.cursor_position.y].len() {
                self.cursor_position.x = self.content[self.cursor_position.y].len();
            }
        } else if direction.y < 0 && self.cursor_position.y < self.content.len().saturating_sub(1) {
            self.cursor_position.y += 1;
            if self.cursor_position.x > self.content[self.cursor_position.y].len() {
                self.cursor_position.x = self.content[self.cursor_position.y].len();
            }
        }
    }
}
