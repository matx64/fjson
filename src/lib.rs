use std::collections::HashMap;

pub fn fix(input: impl Into<String>) -> String {
    let json = Parser::new(input).parse_and_fix();
    json.deserialize_all().stringify(0)
}

pub fn fix_without_formatting(input: impl Into<String>) -> String {
    let json = Parser::new(input).parse_and_fix();
    json.deserialize_all().stringify_without_formatting()
}

enum Json {
    Null,
    True,
    False,
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object((HashMap<String, Json>, Vec<String>)),
}

struct Parser {
    chars: Vec<char>,
    i: usize,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            chars: input.into().trim().chars().collect(),
            i: 0,
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
                'n' | 'N' | 't' | 'T' | 'f' | 'F' => self.parse_static(),

                val if val.is_ascii_digit() || val == '-' || val == '.' => self.parse_number(),

                '"' => self.parse_string(),

                '[' => self.parse_array(),

                '{' => self.parse_object(),

                _ => {
                    self.next();
                    Json::Null
                }
            }
        } else {
            Json::Null
        }
    }

    fn parse_static(&mut self) -> Json {
        match self.next().unwrap().to_ascii_lowercase() {
            'n' => Json::Null,
            't' => Json::True,
            _ => Json::False,
        }
    }

    fn parse_number(&mut self) -> Json {
        let mut lex = String::new();

        if let Some('-') = self.peek() {
            lex.push('-');
            self.next();
        }

        if let Some('.') = self.peek() {
            lex.push('0');
        }

        // trailing 0
        if let Some('0') = self.peek() {
            lex.push('0');
            self.next();

            while let Some(c) = self.peek() {
                if c == '0' {
                    self.next();
                } else {
                    if c.is_ascii_digit() {
                        lex.pop();
                    }
                    break;
                }
            }
        }

        // integer
        while let Some(c) = self.peek()
            && c.is_ascii_digit()
        {
            lex.push(c);
            self.next();
        }

        // float
        if let Some('.') = self.peek() {
            lex.push('.');
            self.next();

            let mut count = 0;
            while let Some(c) = self.peek()
                && c.is_ascii_digit()
            {
                lex.push(c);
                self.next();
                count += 1;
            }

            if count == 0 {
                lex.push('0');
            }
        }

        // expoent
        if let Some(c) = self.peek()
            && (c == 'e' || c == 'E')
        {
            lex.push(c);
            self.next();

            if let Some(sign) = self.peek()
                && (sign == '-' || sign == '+')
            {
                lex.push(sign);
                self.next();
            }

            let mut count = 0;
            while let Some(c) = self.peek()
                && c.is_ascii_digit()
            {
                lex.push(c);
                self.next();
                count += 1;
            }

            if count == 0 {
                if lex.ends_with('-') || lex.ends_with('+') {
                    lex.pop();
                }
                lex.pop();
            }
        }

        if lex == "-" {
            lex.push('0');
        }

        Json::Number(lex)
    }

    fn parse_string(&mut self) -> Json {
        let mut lex = String::new();

        self.next();
        while let Some(c) = self.next() {
            match c {
                '"' => {
                    break;
                }

                '\\' => {
                    if let Some(esc) = self.next() {
                        match esc {
                            '"' => lex.push('"'),
                            '\\' => lex.push('\\'),
                            '/' => lex.push('/'),
                            'b' => lex.push('\u{0008}'),
                            'f' => lex.push('\u{000C}'),
                            'n' => lex.push('\n'),
                            'r' => lex.push('\r'),
                            't' => lex.push('\t'),

                            'u' => {
                                if let Some(c) = self.parse_unicode_escape() {
                                    lex.push(c);
                                }
                            }

                            ch => {
                                lex.push(ch);
                            }
                        }
                    }
                }

                _ => {
                    lex.push(c);
                }
            }
        }

        Json::String(lex)
    }

    fn parse_unicode_escape(&mut self) -> Option<char> {
        let mut hex = String::new();
        for _ in 0..4 {
            if let Some(c) = self.next() {
                hex.push(c);
            } else {
                return None;
            }
        }

        let code = u32::from_str_radix(&hex, 16).ok()?;

        if let Some(ch) = char::from_u32(code) {
            Some(ch)
        } else if (0xD800..=0xDBFF).contains(&code) {
            if let (Some('\\'), Some('u')) = (self.next(), self.next()) {
                let mut low_hex = String::new();
                for _ in 0..4 {
                    if let Some(c) = self.next() {
                        low_hex.push(c);
                    } else {
                        return None;
                    }
                }

                let low_code = u32::from_str_radix(&low_hex, 16).ok()?;

                if (0xDC00..=0xDFFF).contains(&low_code) {
                    let full_code = 0x10000 + ((code - 0xD800) << 10) + (low_code - 0xDC00);

                    char::from_u32(full_code)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_array(&mut self) -> Json {
        let mut arr = Vec::new();

        self.next();

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some(']') => {
                    self.next();
                    break;
                }

                Some(',') => {
                    self.next();
                    continue;
                }

                Some(_) => {
                    arr.push(self.parse_value());

                    self.skip_whitespace();

                    while let Some(c) = self.peek() {
                        match c {
                            ']' | ',' => {
                                break;
                            }

                            _ => {
                                self.next();
                            }
                        }
                    }
                }

                None => {
                    break;
                }
            }
        }

        Json::Array(arr)
    }

    fn parse_object(&mut self) -> Json {
        let mut obj = HashMap::new();
        let mut order = Vec::new();

        self.next();

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('}') => {
                    self.next();
                    break;
                }

                Some('"') => {
                    let key = match self.parse_string() {
                        Json::String(s) => s,
                        _ => unreachable!(),
                    };

                    self.skip_whitespace();

                    if self.peek() == Some(':') {
                        self.next();
                    }

                    if let Some(pos) = order.iter().position(|k| k == &key) {
                        order.remove(pos);
                    }

                    obj.insert(key.clone(), self.parse_value());
                    order.push(key);
                }

                None => {
                    break;
                }

                _ => {
                    self.next();
                }
            }
        }

        Json::Object((obj, order))
    }
}

impl Json {
    pub fn deserialize_all(self) -> Json {
        match self {
            Self::String(val) => {
                let trimmed = val.trim();

                if trimmed.starts_with('{') || trimmed.starts_with('[') {
                    Parser::new(trimmed).parse_and_fix().deserialize_all()
                } else {
                    Json::String(val)
                }
            }

            Json::Array(arr) => Json::Array(arr.into_iter().map(|v| v.deserialize_all()).collect()),

            Json::Object((obj, order)) => Json::Object((
                obj.into_iter()
                    .map(|(k, v)| (k, v.deserialize_all()))
                    .collect(),
                order,
            )),

            other => other,
        }
    }

    pub fn stringify(&self, tabs: usize) -> String {
        const TAB: &str = "   ";

        match self {
            Self::Null => "null".to_string(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),

            Self::Number(val) => val.clone(),
            Self::String(val) => format!("\"{}\"", val),

            Self::Array(arr) => {
                if arr.is_empty() {
                    return "[]".to_string();
                }

                let mut result = String::from("[\n");
                let tab_str = TAB.repeat(tabs + 1);

                for val in arr {
                    result.push_str(&format!("{}{},\n", tab_str, val.stringify(tabs + 1)));
                }

                result.truncate(result.len() - 2);

                result.push('\n');
                result.push_str(&TAB.repeat(tabs));
                result.push(']');
                result
            }

            Self::Object((obj, order)) => {
                if obj.is_empty() {
                    return "{}".to_string();
                }

                let mut result = String::from("{\n");
                let tab_str = TAB.repeat(tabs + 1);

                for key in order {
                    if let Some(val) = obj.get(key) {
                        result.push_str(&format!(
                            "{}\"{}\": {},\n",
                            tab_str,
                            key,
                            val.stringify(tabs + 1)
                        ));
                    }
                }

                result.truncate(result.len() - 2);

                result.push('\n');
                result.push_str(&TAB.repeat(tabs));
                result.push('}');
                result
            }
        }
    }

    pub fn stringify_without_formatting(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),

            Self::Number(val) => val.clone(),
            Self::String(val) => format!("\"{}\"", val),

            Self::Array(arr) => {
                let mut result = String::from('[');

                for val in arr {
                    result.push_str(&format!("{},", val.stringify_without_formatting()));
                }

                if result.ends_with(',') {
                    result.pop();
                }

                result.push(']');
                result
            }

            Self::Object((obj, order)) => {
                let mut result = String::from('{');

                for key in order {
                    if let Some(val) = obj.get(key) {
                        result.push_str(&format!(
                            "\"{}\":{},",
                            key,
                            val.stringify_without_formatting()
                        ));
                    }
                }

                if result.ends_with(',') {
                    result.pop();
                }

                result.push('}');
                result
            }
        }
    }
}

#[cfg(test)]
mod tests;
