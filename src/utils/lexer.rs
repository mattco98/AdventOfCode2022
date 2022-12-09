use std::str::FromStr;

pub struct Lexer {
    cursor: usize,
    source: Vec<char>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new<T: AsRef<str>>(source: T) -> Self {
        Self { cursor: 0, source: source.as_ref().chars().collect() }
    }

    pub fn ch(&self) -> char {
        self.source[self.cursor]
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn done(&self) -> bool {
        self.cursor >= self.source.len()
    }

    pub fn has(&self, n: usize) -> bool {
        self.cursor + n < self.source.len()
    }

    pub fn consume_word(&mut self) -> Option<String> {
        let start = self.cursor;

        while !self.done() && !self.ch().is_whitespace() {
            self.advance();
        }

        if start == self.cursor {
            None
        } else {
            Some(self.source[start..self.cursor].iter().collect())
        }
    }

    pub fn consume_number<T: FromStr>(&mut self, base: u32) -> Option<T> {
        let start = self.cursor;

        while !self.done() && self.ch().is_digit(base) {
            self.advance();
        }

        if start == self.cursor {
            None
        } else {
            match self.source[start..self.cursor].iter().collect::<String>().parse::<T>() {
                Err(..) => {
                    self.cursor = start;
                    None
                } 
                Ok(v) => Some(v)
            }
        }
    }

    pub fn skip_until(&mut self, ch: char) {
        while !self.done() && self.ch() != ch {
            self.advance();
        }
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let start = self.cursor;

        while !self.done() && self.ch().is_whitespace() {
            self.advance();
        }

        self.cursor != start
    }

    pub fn consume(&mut self) -> char {
        let ch = self.ch();
        self.advance();
        ch
    }

    pub fn matches_str<T: AsRef<str>>(&self, s: T) -> bool {
        let r = s.as_ref();

        if !self.has(r.len()) {
            return false;
        }

        for (i, ch) in r.chars().enumerate() {
            if ch != self.source[self.cursor + i] {
                return false;
            }
        }

        true
    }

    pub fn consume_char(&mut self, ch: char) -> bool {
        if self.ch() == ch {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume_str<T: AsRef<str>>(&mut self, s: T) -> bool {
        let r = s.as_ref();

        if !self.has(r.len()) {
            return false;
        }

        let start = self.cursor;

        for ch in r.chars() {
            if self.ch() == ch {
                self.advance();
            } else {
                self.cursor = start;
                return false;
            }
        }

        true
    }
}
