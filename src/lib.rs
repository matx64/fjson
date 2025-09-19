pub fn fix(input: impl Into<String>) -> String {
    Parser::new(input).parse_and_fix()
}

struct Parser {
    input: String,
    result: String,
    state: ParserState,
    current: char,
    lexeme: String,
    need_close: Vec<char>,
}

enum ParserState {
    ValueStart,
    Integer,
    Float,
    String,
    ObjectStart,
    ObjectKey,
    ValueEnd,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            result: String::new(),
            state: ParserState::ValueStart,
            current: '\0',
            lexeme: String::new(),
            need_close: Vec::new(),
        }
    }

    pub fn parse_and_fix(&mut self) -> String {
        self.input = self.input.trim().to_string();

        for ch in self.input.chars().collect::<Vec<char>>() {
            self.current = ch;

            match self.state {
                ParserState::ValueStart => self.handle_value_start(),

                ParserState::Integer => self.handle_integer(),
                ParserState::Float => self.handle_float(),

                ParserState::String => self.handle_string(),

                ParserState::ObjectStart => self.handle_object_start(),
                ParserState::ObjectKey => self.handle_object_key(),

                ParserState::ValueEnd => self.handle_value_end(),
            }
        }

        self.handle_end();
        self.result.to_owned()
    }

    fn handle_value_start(&mut self) {
        match self.current {
            't' | 'T' => {
                self.handle_true();
            }

            'f' | 'F' => {
                self.handle_false();
            }

            'n' | 'N' => {
                self.handle_null();
            }

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                self.state = ParserState::Integer;
                self.lexeme = String::from(self.current);
            }

            '.' => {
                self.state = ParserState::Float;
                self.lexeme = String::from("0.");
            }

            '"' => {
                self.state = ParserState::String;
                self.lexeme = String::from(self.current);
                self.need_close.push('"');
            }

            '{' => {
                self.state = ParserState::ObjectStart;
                self.result.push(self.current);
                self.need_close.push('}');
            }

            '[' => {
                self.result.push(self.current);
                self.need_close.push(']');
            }

            ']' => {
                if let Some(last) = self.need_close.last()
                    && *last == ']'
                {
                    self.result.push(self.current);
                    self.need_close.pop();
                }
            }

            _ => {}
        }
    }

    fn handle_true(&mut self) {
        self.result.push_str("true");
        self.state = ParserState::ValueEnd;
    }

    fn handle_false(&mut self) {
        self.result.push_str("false");
        self.state = ParserState::ValueEnd;
    }

    fn handle_null(&mut self) {
        self.result.push_str("null");
        self.state = ParserState::ValueEnd;
    }

    fn handle_integer(&mut self) {
        match self.current {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.lexeme.push(self.current);
            }

            'e' | 'E' => {
                if !self.lexeme.contains('e') {
                    self.lexeme.push('e');
                }
            }

            '+' | '-' => {
                if let Some(last) = self.lexeme.chars().last()
                    && last == 'e'
                {
                    self.lexeme.push(self.current);
                }
            }

            '.' => {
                if let Some(last) = self.lexeme.chars().last()
                    && last == '-'
                {
                    self.lexeme.push('0');
                }
                self.lexeme.push(self.current);
                self.state = ParserState::Float;
            }

            _ => {
                self.normalize_number();
                self.result.push_str(&self.lexeme);
                self.handle_value_end();
            }
        }
    }

    fn handle_float(&mut self) {
        match self.current {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.lexeme.push(self.current);
            }

            'e' | 'E' => {
                if !self.lexeme.contains('e') {
                    self.lexeme.push('e');
                }
            }

            '+' | '-' => {
                if let Some(last) = self.lexeme.chars().last()
                    && last == 'e'
                {
                    self.lexeme.push(self.current);
                }
            }

            '.' => {}

            _ => {
                self.normalize_number();
                self.result.push_str(&self.lexeme);
                self.handle_value_end();
            }
        }
    }

    fn normalize_number(&mut self) {
        let mut result = {
            if self.lexeme.starts_with('-') {
                self.lexeme[1..].to_string()
            } else {
                self.lexeme.clone()
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

        if self.lexeme.starts_with('-') {
            result.insert(0, '-');
        }

        if result.is_empty() || result == "-" {
            result = String::from('0');
        }

        self.lexeme = result;
    }

    fn handle_string(&mut self) {
        self.lexeme.push(self.current);

        if self.current == '"'
            && let Some(last) = self.lexeme.chars().last()
            && last != '\\'
        {
            self.result.push_str(&self.lexeme);
            self.need_close.pop();
            self.state = ParserState::ValueEnd;
        }
    }

    fn handle_object_start(&mut self) {
        match self.current {
            '"' => {
                self.lexeme.push(self.current);
                self.need_close.push('"');
                self.state = ParserState::ObjectKey;
            }

            '}' => {
                self.result.push('}');
                self.need_close.pop();
                self.state = ParserState::ValueEnd;
            }

            _ => {
                self.handle_value_end();
            }
        }
    }

    fn handle_object_key(&mut self) {
        match self.current {
            '"' => {
                self.lexeme.push(self.current);
                self.lexeme.push(':');
                self.result.push_str(&self.lexeme);
                self.need_close.pop();
                self.state = ParserState::ValueStart;
            }

            _ => {
                self.lexeme.push(self.current);
            }
        }
    }

    fn handle_value_end(&mut self) {
        self.state = ParserState::ValueEnd;

        if self.current == ','
            && let Some(last) = self.need_close.last()
        {
            if *last == '}' {
                self.result.push(self.current);
                self.state = ParserState::ObjectKey;
            } else if *last == ']' {
                self.result.push(self.current);
                self.state = ParserState::ValueStart;
            }
        }
    }

    fn handle_end(&mut self) {
        match self.state {
            ParserState::ValueStart => {
                if let Some(last_bracket) = self.need_close.last()
                    && let Some(last_char) = self.result.chars().last()
                {
                    if last_char == ',' {
                        self.result.pop();
                    } else if *last_bracket == '}' {
                        self.result.push_str("null");
                    }
                }
            }

            ParserState::Integer | ParserState::Float => {
                self.normalize_number();
                self.result.push_str(&self.lexeme);
            }

            ParserState::String => {
                self.lexeme.push('"');
                self.result.push_str(&self.lexeme);
                self.need_close.pop();
            }

            ParserState::ObjectStart => {
                self.result.push('}');
                self.need_close.pop();
            }

            ParserState::ObjectKey => {
                self.lexeme.push('"');
                self.result.push_str(&self.lexeme);
                self.need_close.pop();

                self.result.push_str(": null}");
            }

            _ => {}
        }

        while let Some(ch) = self.need_close.pop() {
            self.result.push(ch);
        }
    }
}

#[cfg(test)]
mod tests;
