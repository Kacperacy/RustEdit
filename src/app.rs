use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub content: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            content: String::new(),
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
    }

    pub fn pop_char(&mut self) {
        self.content.pop();
    }
}
