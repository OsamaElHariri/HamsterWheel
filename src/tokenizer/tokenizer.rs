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

pub fn tokenize<'a>(text: &'a str) -> logos::Lexer<Token, &'a str> {
    Token::lexer(text)
}
