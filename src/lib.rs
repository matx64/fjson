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
    True,
    False,
    Null,
    Number,
    Float,
    Object,
    String,
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
                ParserState::ValueStart => self.value_start(),

                ParserState::True => self.handle_true(),
                ParserState::False => self.handle_false(),
                ParserState::Null => self.handle_null(),

                ParserState::Number => self.handle_number(),
                ParserState::Float => self.handle_float(),

                _ => todo!(),
            }
        }

        self.result.to_owned()
    }

    fn value_start(&mut self) {
        match self.current {
            't' => {
                self.state = ParserState::True;
            }

            'f' => {
                self.state = ParserState::False;
            }

            'n' => {
                self.state = ParserState::Null;
            }

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                self.state = ParserState::Number;
                self.lexeme = String::from(self.current);
            }

            '{' => {
                self.state = ParserState::Object;
                self.result.push(self.current);
                self.need_close.push('}');
            }

            '"' => {
                self.state = ParserState::String;
                self.lexeme = String::from(self.current);
                self.need_close.push('"');
            }

            '[' => {
                self.result.push(self.current);
                self.need_close.push(']');
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
            self.state = ParserState::ValueEnd;
        }
    }
}
