pub fn fix(input: impl Into<String>) -> String {
    Parser::new(input).parse_and_fix()
}

struct Parser {
    input: String,
    result: String,
    state: ParserState,
    need_close: Vec<char>,
}

enum ParserState {
    ValueStart,
    Object,
    True,
    False,
    Null,
    Number,
    String,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            result: String::new(),
            state: ParserState::ValueStart,
            need_close: Vec::new(),
        }
    }

    pub fn parse_and_fix(&mut self) -> String {
        self.input = self.input.trim().to_string();

        for ch in self.input.chars().collect::<Vec<char>>() {
            match self.state {
                ParserState::ValueStart => self.value_start(ch),

                _ => todo!(),
            }
        }

        self.result.to_owned()
    }

    fn value_start(&mut self, ch: char) {
        match ch {
            '{' => {
                self.state = ParserState::Object;
            }

            't' => {
                self.state = ParserState::True;
            }

            'f' => {
                self.state = ParserState::False;
            }

            'n' => {
                self.state = ParserState::Null;
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
}
