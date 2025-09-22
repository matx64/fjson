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
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let chars: Vec<char> = input.into().trim().chars().collect();
        Self { i: 0, chars }
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
                'f' | 'F' => Json::False,
                'n' | 'N' => Json::Null,

                val if val.is_ascii_digit() || val == '-' => self.parse_number(),
                '.' => self.parse_float("0"),

                '"' => self.parse_string(),

                '[' => self.parse_array(),

                '{' => self.parse_object(),

                _ => Json::Null,
            }
        } else {
            Json::Null
        }
    }

    fn parse_number(&mut self) -> Json {
        let mut lex = String::from(self.next().unwrap());

        while let Some(c) = self.next() {
            match c {
                val if val.is_ascii_digit() => {
                    lex.push(c);
                }

                'e' | 'E' => {
                    if !lex.contains('e') {
                        lex.push('e');
                    }
                }

                '+' | '-' => {
                    if let Some(last) = lex.chars().last()
                        && last == 'e'
                    {
                        lex.push(c);
                    }
                }

                '.' => {
                    if let Some(last) = lex.chars().last()
                        && last == '-'
                    {
                        lex.push('0');
                    }
                    return self.parse_float(&lex);
                }

                _ => {}
            }
        }

        Json::Number(self.normalize_number(lex))
    }

    fn parse_float(&mut self, int_part: &str) -> Json {
        let mut lex = format!("{}{}", int_part, self.next().unwrap());

        while let Some(c) = self.next() {
            match c {
                val if val.is_ascii_digit() => {
                    lex.push(c);
                }

                'e' | 'E' => {
                    if !lex.contains('e') {
                        lex.push('e');
                    }
                }

                '+' | '-' => {
                    if let Some(last) = lex.chars().last()
                        && last == 'e'
                    {
                        lex.push(c);
                    }
                }

                '.' => {}

                _ => {}
            }
        }

        Json::Number(self.normalize_number(lex))
    }

    fn normalize_number(&mut self, lex: String) -> String {
        let mut result = {
            if lex.starts_with('-') {
                lex.strip_prefix('-').unwrap().to_string()
            } else {
                lex.clone()
            }
        };

        while result.starts_with('0') {
            result.remove(0);
        }

        if result.starts_with('.') {
            result.insert(0, '0');
        }

        let mut check_end = true;
        while check_end {
            if result.ends_with('.') {
                result.push('0');
            } else if result.ends_with('e') || result.ends_with("-") || result.ends_with("+") {
                result.pop();
            } else {
                check_end = false;
            }
        }

        if lex.starts_with('-') {
            result.insert(0, '-');
        }

        if result.is_empty() || result == "-" {
            result = String::from('0');
        }

        result
    }

    fn parse_string(&mut self) -> Json {
        let mut lex = String::new();

        self.next();
        while let Some(c) = self.next() {
            match c {
                '"' => {
                    if let Some(last) = lex.chars().last()
                        && last == '\\'
                    {
                        lex.push(c);
                    } else {
                        break;
                    }
                }

                _ => {
                    lex.push(c);
                }
            }
        }

        Json::String(lex)
    }

    fn parse_array(&mut self) -> Json {
        let mut arr = Vec::new();

        self.next();
        self.skip_whitespace();

        if self.peek() == Some(']') {
            return Json::Array(arr);
        }

        loop {
            arr.push(self.parse_value());

            self.skip_whitespace();
            match self.peek() {
                Some(',') => {
                    self.next();
                }

                Some(']') | None => {
                    self.next();
                    break;
                }

                _ => {}
            }
        }

        Json::Array(arr)
    }

    fn parse_object(&mut self) -> Json {
        todo!()
    }
}
