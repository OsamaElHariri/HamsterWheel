use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex = "(?i)loop"]
    Loop,

    #[regex = "(?i)end"]
    End,

    #[regex = "(?i)as"]
    As,

    #[token = "("]
    LeftParentheses,

    #[token = ")"]
    RightParentheses,

    #[token = "["]
    LeftBracket,

    #[token = "]"]
    RightBracket,

    #[token = "{{"]
    LeftMustache,

    #[token = "}}"]
    RightMustache,

    #[token = ".."]
    DoubleDot,

    #[token = "."]
    Dot,

    #[token = ","]
    Comma,

    #[regex = "[0-9]+"]
    Number,

    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Variable,

    #[end]
    EOF,

    #[error]
    Error,
}

#[derive(Clone)]
pub struct InfoToken {
    pub token: Token,
    pub slice: String,
    pub start: usize,
    pub end: usize,
}

pub struct Tokenizer<'a> {
    lexer: logos::Lexer<Token, &'a str>,
    peeks: Vec<InfoToken>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer {
        Tokenizer {
            lexer: Token::lexer(text),
            peeks: vec![],
        }
    }

    pub fn info(&mut self) -> InfoToken {
        if self.peeks.len() > 0 {
            self.peeks[0].clone()
        } else {
            self.get_current_info()
        }
    }

    fn get_current_info(&self) -> InfoToken {
        InfoToken {
            token: self.lexer.token.clone(),
            slice: self.lexer.slice().to_string(),
            start: self.lexer.range().start,
            end: self.lexer.range().end,
        }
    }

    pub fn advance(&mut self) {
        if self.peeks.len() == 0 {
            self.lexer.advance();
        } else {
            self.peeks.remove(0);
        }
    }

    pub fn peek(&mut self) -> InfoToken {
        self.peeks.push(self.get_current_info());
        self.lexer.advance();
        self.get_current_info()
    }
}
