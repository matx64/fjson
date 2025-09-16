pub fn fix(input: impl Into<String>) -> String {
    Parser::new(input).parse_and_fix()
}

struct Parser {
    input: String,
    result: String,
    current: char,
    state: ParserState,
    need_close: Vec<char>,
}

enum ParserState {
    ValueStart,
    True,
    False,
    Null,
    Object,
    Number,
    String,
    ValueEnd,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            result: String::new(),
            current: '\0',
            state: ParserState::ValueStart,
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

            '{' => {
                self.state = ParserState::Object;
            }

            '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.state = ParserState::Number;
            }

            '"' => {
                self.state = ParserState::String;
                self.need_close.push('"');
            }

            '[' => {
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
}
