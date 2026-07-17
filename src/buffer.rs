pub struct GapBuffer {
    data: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new() -> Self {
        Self {
            data: vec!['\0'; 1024],
            gap_start: 0,
            gap_end: 1024,
        }
    }

    pub fn from_string(content: &str) -> Self {
        let chars: Vec<char> = content.chars().collect();
        let len = chars.len();
        let gap_size = 1024;
        let mut data = Vec::with_capacity(len + gap_size);
        data.extend_from_slice(&chars);
        data.resize(len + gap_size, '\0');
        Self {
            data,
            gap_start: len,
            gap_end: len + gap_size,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - (self.gap_end - self.gap_start)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.len());
        s.extend(&self.data[..self.gap_start]);
        s.extend(&self.data[self.gap_end..]);
        s
    }

    pub fn insert(&mut self, idx: usize, c: char) {
        if self.gap_start == self.gap_end {
            self.grow();
        }
        self.move_gap(idx);
        self.data[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn delete(&mut self, idx: usize) -> Option<char> {
        if idx >= self.len() {
            return None;
        }
        self.move_gap(idx);
        let c = self.data[self.gap_end];
        self.gap_end += 1;
        Some(c)
    }

    pub fn get_range_string(&self, start: usize, end: usize) -> String {
        let mut s = String::new();
        let limit = std::cmp::min(end, self.len());
        for i in start..limit {
            s.push(self.get_char(i));
        }
        s
    }

    pub fn line_start_idx(&self, idx: usize) -> usize {
        let mut i = idx;
        while i > 0 {
            if self.get_char(i - 1) == '\n' {
                break;
            }
            i -= 1;
        }
        i
    }

    pub fn line_end_idx(&self, idx: usize) -> usize {
        let mut i = idx;
        let len = self.len();
        while i < len {
            if self.get_char(i) == '\n' {
                break;
            }
            i += 1;
        }
        i
    }

    pub fn idx_at_line(&self, line_num: usize) -> usize {
        let target_line = line_num.saturating_sub(1);
        let mut current_line = 0;
        let mut idx = 0;
        let len = self.len();
        while idx < len && current_line < target_line {
            if self.get_char(idx) == '\n' {
                current_line += 1;
            }
            idx += 1;
        }
        idx
    }

    pub fn line_col_at_idx(&self, idx: usize) -> (usize, usize) {
        let mut line = 0;
        let mut col = 0;
        for i in 0..std::cmp::min(idx, self.len()) {
            if self.get_char(i) == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    pub fn idx_at_line_col(&self, line: usize, col: usize) -> usize {
        let mut current_line = 0;
        let mut idx = 0;
        let len = self.len();
        while idx < len && current_line < line {
            if self.get_char(idx) == '\n' {
                current_line += 1;
            }
            idx += 1;
        }
        let mut current_col = 0;
        while idx < len && current_col < col {
            if self.get_char(idx) == '\n' {
                break;
            }
            current_col += 1;
            idx += 1;
        }
        idx
    }

    pub fn total_lines(&self) -> usize {
        let mut count = 1;
        let len = self.len();
        for i in 0..len {
            if self.get_char(i) == '\n' {
                count += 1;
            }
        }
        count
    }

    pub fn get_line_string(&self, line_idx: usize) -> String {
        let mut current_line = 0;
        let mut idx = 0;
        let len = self.len();
        while idx < len && current_line < line_idx {
            if self.get_char(idx) == '\n' {
                current_line += 1;
            }
            idx += 1;
        }
        let mut line_str = String::new();
        while idx < len {
            let c = self.get_char(idx);
            if c == '\n' {
                break;
            }
            line_str.push(c);
            idx += 1;
        }
        line_str
    }

    pub fn search_all(&self, query: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        if query.is_empty() {
            return matches;
        }
        let query_chars: Vec<char> = query.chars().collect();
        let len = self.len();
        if len < query_chars.len() {
            return matches;
        }
        for i in 0..=(len - query_chars.len()) {
            let mut matched = true;
            for j in 0..query_chars.len() {
                if self.get_char(i + j) != query_chars[j] {
                    matched = false;
                    break;
                }
            }
            if matched {
                matches.push(i);
            }
        }
        matches
    }

    fn get_char(&self, idx: usize) -> char {
        if idx < self.gap_start {
            self.data[idx]
        } else {
            self.data[idx + (self.gap_end - self.gap_start)]
        }
    }

    fn move_gap(&mut self, pos: usize) {
        if pos == self.gap_start {
            return;
        }
        if pos < self.gap_start {
            let shift = self.gap_start - pos;
            for i in 0..shift {
                self.data[self.gap_end - 1 - i] = self.data[self.gap_start - 1 - i];
            }
            self.gap_start -= shift;
            self.gap_end -= shift;
        } else {
            let shift = pos - self.gap_start;
            for i in 0..shift {
                self.data[self.gap_start + i] = self.data[self.gap_end + i];
            }
            self.gap_start += shift;
            self.gap_end += shift;
        }
    }

    fn grow(&mut self) {
        let current_gap_size = self.gap_end - self.gap_start;
        let grow_by = if current_gap_size == 0 { 1024 } else { current_gap_size * 2 };
        let mut new_data = Vec::with_capacity(self.data.len() + grow_by);
        new_data.extend_from_slice(&self.data[..self.gap_start]);
        new_data.resize(new_data.len() + grow_by, '\0');
        new_data.extend_from_slice(&self.data[self.gap_end..]);
        self.gap_end = self.gap_start + grow_by;
        self.data = new_data;
    }
}