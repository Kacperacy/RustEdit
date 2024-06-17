#[derive(Debug, Clone)]
pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new(capacity: usize) -> GapBuffer {
        GapBuffer {
            buffer: vec![' '; capacity],
            gap_start: 0,
            gap_end: capacity,
        }
    }

    pub fn push(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.resize();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    fn resize(&mut self) {
        let new_capacity = self.buffer.len() * 2;
        let mut new_buffer = vec![' '; new_capacity];
        let gap_size = self.gap_end - self.gap_start;

        new_buffer[..self.gap_start].copy_from_slice(&self.buffer[..self.gap_start]);
        new_buffer[new_capacity - gap_size..].copy_from_slice(&self.buffer[self.gap_end..]);

        self.gap_end = new_capacity - gap_size;
        self.buffer = new_buffer;
    }

    fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    fn move_gap(&mut self, index: usize) {
        if index < self.gap_start {
            let move_size = self.gap_start - index;
            self.buffer
                .copy_within(index..self.gap_start, self.gap_end - move_size);
            self.gap_start = index;
            self.gap_end -= move_size;
        } else if index > self.gap_start {
            let move_size = index - self.gap_start;
            self.buffer
                .copy_within(self.gap_end..self.gap_end + move_size, self.gap_start);
            self.gap_start += move_size;
            self.gap_end += move_size;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result: Vec<char> =
            Vec::with_capacity(self.buffer.len() - (self.gap_end - self.gap_start));
        result.extend_from_slice(&self.buffer[..self.gap_start]);
        result.extend_from_slice(&self.buffer[self.gap_end..]);
        result.into_iter().collect()
    }

    pub fn insert_at(&mut self, index: usize, c: char) {
        if index > self.buffer.len() {
            return;
        }
        self.move_gap(index);
        self.push(c);
    }

    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push(c);
        }
    }

    pub fn remove_at(&mut self, index: usize) {
        if index > self.buffer.len() - (self.gap_end - self.gap_start) {
            return;
        }
        self.move_gap(index + 1);
        self.delete();
    }

    pub fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }

    pub fn split_off(&mut self, at: usize) -> GapBuffer {
        if at > self.len() {
            panic!("Index out of bounds");
        }
        self.move_gap(at);

        let new_capacity = self.buffer.len() - self.gap_end;
        let mut new_buffer = vec![' '; new_capacity];

        new_buffer[..new_capacity].copy_from_slice(&self.buffer[self.gap_end..]);

        self.gap_end = self.buffer.len();

        GapBuffer {
            buffer: new_buffer,
            gap_start: 0,
            gap_end: new_capacity,
        }
    }
}
