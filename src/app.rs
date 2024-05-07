use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub content: String,
    pub cursor_position: Position,
    pub cursor_offset: Position,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            content: String::new(),
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

    pub fn append_char(&mut self, c: char) {
        self.content.push(c);
        self.cursor_position.x += 1;
    }

    pub fn pop_char(&mut self) {
        self.content.pop();
        if let Some(previous_char) = self.cursor_position.x.checked_sub(1) {
            self.cursor_position.x = previous_char;
        }
    }

    pub fn move_cursor(&mut self, direction: i8) {
        if direction < 0 {
            if let Some(previous_char) = self.cursor_position.x.checked_sub(1) {
                self.cursor_position.x = previous_char;
            }
        } else {
            self.cursor_position.x += 1;
        }
    }
}
