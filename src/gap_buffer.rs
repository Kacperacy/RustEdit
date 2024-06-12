struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    fn new(capacity: usize) -> GapBuffer {
        GapBuffer {
            buffer: vec![' '; capacity],
            gap_start: 0,
            gap_end: capacity,
        }
    }

    fn insert(&mut self, c: char) {
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

    fn move_cursor(&mut self, index: usize) {
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
}
