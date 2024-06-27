use std::{cmp::min, error, fs};

use ratatui::layout::Rect;

use crate::gap_buffer::GapBuffer;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const GAP_BUFFER_DEFAULT_SIZE: usize = 80;
const QUIT_TIMES: i8 = 2;
const DEFAULT_STATUS: &str = "Press Ctrl + C to quit, Ctrl + S to save.";

#[derive(Debug)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct App {
    prompt_cursor_position: Position,
    quit_times: i8,
    pub running: bool,
    pub content: Vec<GapBuffer>,
    pub cursor_position: Position,
    pub cursor_offset: Position,
    pub opened_filename: String,
    pub window_size: Rect,
    pub dirty: bool,
    pub is_prompt: bool,
    pub prompt: String,
    pub status: String,
    pub line_numbers_width: usize,
    pub is_selecting: bool,
    pub selecting_position: Position,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            content: vec![GapBuffer::new(80)],
            cursor_position: Position { x: 0, y: 0 },
            cursor_offset: Position { x: 0, y: 0 },
            opened_filename: String::new(),
            window_size: Rect::new(0, 0, 0, 0),
            dirty: false,
            quit_times: QUIT_TIMES,
            is_prompt: false,
            prompt: String::new(),
            prompt_cursor_position: Position { x: 0, y: 0 },
            status: DEFAULT_STATUS.into(),
            line_numbers_width: 4,
            is_selecting: false,
            selecting_position: Position { x: 0, y: 0 },
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        if !self.dirty {
            self.running = false;
            return;
        }

        self.quit_times -= 1;
        if self.quit_times <= 0 {
            self.running = false;
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
        self.reset_quit();
    }

    pub fn reset_quit(&mut self) {
        self.quit_times = QUIT_TIMES;
    }

    pub fn get_cursor_position(&self) -> Position {
        if self.is_prompt {
            Position {
                x: self.prompt_cursor_position.x + self.cursor_offset.x,
                y: self.prompt_cursor_position.y + self.cursor_offset.y,
            }
        } else {
            Position {
                x: self.cursor_position.x + self.cursor_offset.x,
                y: self.cursor_position.y + self.cursor_offset.y,
            }
        }
    }

    fn update_line_numbers_width(&mut self) {
        self.line_numbers_width =
            std::cmp::max((self.content.len() as f64).log10().ceil() as usize, 4);
    }

    fn push_to_content(&mut self, s: GapBuffer) {
        self.content.push(s);
        self.update_line_numbers_width();
    }

    fn insert_to_content(&mut self, index: usize, s: GapBuffer) {
        self.content.insert(index, s);
        self.update_line_numbers_width();
    }

    fn remove_from_content(&mut self, index: usize) -> String {
        let s = self.content.remove(index);
        self.line_numbers_width =
            std::cmp::max((self.content.len() as f64).log10().ceil() as usize, 4);
        s.to_string()
    }

    pub fn enter_prompt(&mut self) {
        self.is_prompt = true;
        self.prompt_cursor_position = self.cursor_position;
        self.cursor_position.y = self.window_size.height as usize;
        self.cursor_position.x = 0;
    }

    pub fn exit_prompt(&mut self) {
        if self.is_prompt {
            self.is_prompt = false;
            self.prompt = String::new();
            self.cursor_position = self.prompt_cursor_position;
            self.prompt_cursor_position = Position { x: 0, y: 0 };
        }
    }

    pub fn save_to_file(&mut self) {
        if self.opened_filename.is_empty() || self.is_prompt {
            self.enter_prompt();
            return;
        }

        self.dirty = false;
        self.status = format!("Saved to {}", self.opened_filename);
        let _ = fs::write(
            &self.opened_filename,
            self.content
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    pub fn insert_char(&mut self, c: char) {
        if self.is_prompt {
            self.prompt.insert(self.cursor_position.x, c);
            self.move_cursor(Direction { x: 1, y: 0 }, false);
            return;
        }

        while self.cursor_position.y >= self.content.len() {
            self.push_to_content(GapBuffer::new(GAP_BUFFER_DEFAULT_SIZE));
        }

        self.content[self.cursor_position.y + self.cursor_offset.y]
            .insert_at(self.cursor_position.x + self.cursor_offset.x, c);
        self.move_cursor(Direction { x: 1, y: 0 }, false);
    }

    pub fn add_new_line(&mut self) {
        if self.is_prompt {
            self.opened_filename = self.prompt.clone();
            self.exit_prompt();
            self.save_to_file();
            return;
        }

        let pos = self.get_cursor_position();

        while pos.y >= self.content.len() {
            self.push_to_content(GapBuffer::new(GAP_BUFFER_DEFAULT_SIZE));
        }

        let current_line = &mut self.content[pos.y];

        if current_line.len() > pos.x {
            let new_line = current_line.split_off(pos.x);

            self.insert_to_content(pos.y + 1, new_line);
        } else {
            self.insert_to_content(pos.y + 1, GapBuffer::new(GAP_BUFFER_DEFAULT_SIZE));
        }

        self.cursor_position.x = 0;
        self.move_cursor(Direction { x: 0, y: -1 }, false);
    }

    pub fn pop_char(&mut self) {
        if self.is_prompt {
            if self.prompt.len() == 0 {
                return;
            }
            self.prompt.pop();
            self.move_cursor(Direction { x: -1, y: 0 }, false);
            return;
        }

        if self.content.len() == 0 {
            return;
        }

        let pos = self.get_cursor_position();

        if self.content[pos.y].len() == 0 {
            self.remove_from_content(pos.y);

            self.move_cursor(Direction { x: 0, y: 1 }, false);
        } else if self.cursor_position.x == 0 && self.cursor_position.y > 0 {
            let lower_line = self.remove_from_content(pos.y);

            self.cursor_position.x = self.content[pos.y - 1].len();

            self.content[pos.y - 1].push_str(&lower_line);

            self.move_cursor(Direction { x: 0, y: 1 }, false);
        } else if !(self.cursor_position.x == 0 && self.cursor_position.y == 0) {
            self.content[pos.y].remove_at(pos.x - 1);

            self.move_cursor(Direction { x: -1, y: 0 }, false);
        }
    }

    pub fn move_cursor(&mut self, direction: Direction, is_selection: bool) {
        self.reset_quit();

        if self.is_prompt {
            if direction.x < 0 && self.cursor_position.x > 0 {
                self.cursor_position.x -= 1;
            } else if direction.x > 0 && self.prompt.len() > self.cursor_position.x {
                self.cursor_position.x += 1;
            }
            return;
        }

        if is_selection && !self.is_selecting {
            self.is_selecting = true;
            self.selecting_position = self.get_cursor_position();
        }

        if self.status != DEFAULT_STATUS {
            self.status = DEFAULT_STATUS.into();
        }

        let pos = self.get_cursor_position();

        if direction.x < 0 {
            self.move_cursor_left(pos);
        } else if direction.x > 0 {
            self.move_cursor_right(pos);
        }

        if direction.y > 0 && pos.y > 0 {
            self.move_cursor_up(pos);
        } else if direction.y < 0 && pos.y < self.content.len().saturating_sub(1) {
            self.move_cursor_down(pos);
        }
    }

    fn move_cursor_left(&mut self, pos: Position) {
        if pos.x > 0 {
            if self.cursor_position.x == 0 {
                if self.cursor_offset.x > 0 {
                    self.cursor_offset.x -= 1;
                }
            } else {
                self.cursor_position.x -= 1;
            }
        } else if self.cursor_position.y > 0 {
            self.cursor_position.y -= 1;
            let len = self.content[pos.y - 1].len();
            self.cursor_position.x = min(len, self.window_size.width.into());
            self.cursor_offset.x = len - self.cursor_position.x;
        }
    }

    fn move_cursor_right(&mut self, pos: Position) {
        if let Some(line) = self.content.get(pos.y) {
            if line.len() > pos.x {
                if self.window_size.width > (self.cursor_position.x + 1) as u16 {
                    self.cursor_position.x += 1;
                } else {
                    self.cursor_offset.x += 1;
                }
            } else if line.len() == pos.x && pos.y < self.content.len() - 1 {
                self.cursor_offset.x = 0;
                self.cursor_position.x = 0;
                if self.cursor_position.y + 1 > self.window_size.height.into() {
                    self.cursor_offset.y += 1;
                } else {
                    self.cursor_position.y += 1;
                }
            }
        }
    }

    fn move_cursor_up(&mut self, pos: Position) {
        if self.cursor_position.y == 0 {
            if self.cursor_offset.y > 0 {
                self.cursor_offset.y -= 1;
            }
        } else {
            self.cursor_position.y -= 1;
        }

        if self.cursor_position.x > self.content[pos.y - 1].len() {
            self.cursor_position.x = self.content[pos.y - 1].len();
            self.cursor_offset.x = 0;
        }
    }

    fn move_cursor_down(&mut self, pos: Position) {
        if self.window_size.height.saturating_sub(4) > self.cursor_position.y as u16 {
            self.cursor_position.y += 1;
        } else {
            self.cursor_offset.y += 1;
        }

        if self.cursor_position.x > self.content[pos.y + 1].len() {
            self.cursor_position.x = self.content[pos.y + 1].len();
            self.cursor_offset.x = 0;
        }
    }

    pub fn jump_at_end_line(&mut self) {
        let pos = self.get_cursor_position();
        let len = self.content[pos.y].len();
        let width = self.window_size.width.into();

        if len > width {
            self.cursor_position.x = width;
            self.cursor_offset.x = len - width;
        } else {
            self.cursor_position.x = len - self.cursor_offset.x;
        }
    }

    pub fn jump_at_start_line(&mut self) {
        self.cursor_position.x = 0;
        self.cursor_offset.x = 0;
    }
}
