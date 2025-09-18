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
    Number,
    Float,
    String,
    ObjectStart,
    ObjectKey,
    ObjectKeyEnd,
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

                ParserState::Number => self.handle_number(),
                ParserState::Float => self.handle_float(),

                ParserState::String => self.handle_string(),

                ParserState::ObjectStart => self.handle_object_start(),
                ParserState::ObjectKey => self.handle_object_key(),
                ParserState::ObjectKeyEnd => self.handle_object_key_end(),

                ParserState::ValueEnd => self.handle_value_end(),
            }
        }

        self.handle_end();

        self.result.to_owned()
    }

    fn handle_value_start(&mut self) {
        match self.current {
            't' => {
                self.handle_true();
            }

            'f' => {
                self.handle_false();
            }

            'n' => {
                self.handle_null();
            }

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                self.state = ParserState::Number;
                self.lexeme = String::from(self.current);
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

    fn handle_number(&mut self) {
        match self.current {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'e' | 'E' => {
                self.lexeme.push(self.current);
            }

            '.' => {
                self.lexeme.push(self.current);
                self.state = ParserState::Float;
            }

            _ => {
                self.result.push_str(&self.lexeme);
                self.state = ParserState::ValueEnd;
            }
        }
    }

    fn handle_float(&mut self) {
        match self.current {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'e' | 'E' => {
                self.lexeme.push(self.current);
            }

            _ => {
                self.result.push_str(&self.lexeme);
                self.state = ParserState::ValueEnd;
            }
        }
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
                self.state = ParserState::ValueEnd;
            }
        }
    }

    fn handle_object_key(&mut self) {
        match self.current {
            '"' => {
                self.lexeme.push(self.current);
                self.result.push_str(&self.lexeme);
                self.need_close.pop();
                self.state = ParserState::ObjectKeyEnd;
            }

            _ => {
                self.lexeme.push(self.current);
            }
        }
    }

    fn handle_object_key_end(&mut self) {
        if self.current == ':' {
            self.result.push(self.current);
            self.state = ParserState::ValueStart;
        }
    }

    fn handle_value_end(&mut self) {
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
            ParserState::Number | ParserState::Float => {
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

                self.result.push_str(": \"\"}");
            }

            ParserState::ObjectKeyEnd => {
                self.result.push_str(": \"\"}");
            }

            _ => {}
        }

        while let Some(ch) = self.need_close.pop() {
            self.result.push(ch);
        }
    }
}
