use std::collections::HashMap;

enum Json {
    Null,
    True,
    False,
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

struct Parser {
    chars: Vec<char>,
    i: usize,
    len: usize,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let chars: Vec<char> = input.into().trim().chars().collect();
        Self {
            len: chars.len(),
            i: 0,
            chars,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.get(self.i).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.i += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.next();
        }
    }

    pub fn parse_and_fix(&mut self) -> Json {
        self.parse_value()
    }

    fn parse_value(&mut self) -> Json {
        self.skip_whitespace();

        if let Some(c) = self.peek() {
            match c {
                't' | 'T' => Json::True,

                _ => Json::Null,
            }
        } else {
            Json::Null
        }
    }
}
