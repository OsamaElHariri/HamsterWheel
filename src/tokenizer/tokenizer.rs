use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex = "(?i)loop"]
    Loop,

    #[regex = "(?i)output"]
    Output,

    #[regex = "(?i)import"]
    Import,

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

    #[token = ":"]
    Colon,

    #[token = ";"]
    SemiColon,

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
    peek_index: usize,
    current_info: InfoToken,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer {
        let lexer = Token::lexer(text);
        Tokenizer {
            peeks: vec![],
            peek_index: 1,
            current_info: InfoToken {
                token: lexer.token.clone(),
                slice: lexer.slice().to_string(),
                start: lexer.range().start,
                end: lexer.range().end,
            },
            lexer: lexer,
        }
    }

    pub fn info(&mut self) -> &InfoToken {
        if self.peeks.len() > 0 {
            &self.peeks[0]
        } else {
            self.get_current_info()
        }
    }

    fn get_current_info(&mut self) -> &InfoToken {
        self.current_info = self.get_info_at_lexer();
        &self.current_info
    }

    fn get_info_at_lexer(&self) -> InfoToken {
        InfoToken {
            token: self.lexer.token.clone(),
            slice: self.lexer.slice().to_string(),
            start: self.lexer.range().start,
            end: self.lexer.range().end,
        }
    }

    pub fn advance(&mut self) {
        self.reset_peek();
        if self.peeks.len() == 0 {
            self.lexer.advance();
        } else {
            self.peeks.remove(0);
        }
    }

    pub fn peek(&mut self) -> &InfoToken {
        let current_peek_index = self.peek_index;
        self.peek_index += 1;
        if current_peek_index < self.peeks.len() {
            &self.peeks[current_peek_index]
        } else if current_peek_index == self.peeks.len() {
            self.get_current_info()
        } else {
            self.peeks.push(self.get_info_at_lexer());
            self.lexer.advance();
            self.get_current_info()
        }
    }

    pub fn reset_peek(&mut self) {
        self.peek_index = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_checks_next_token() {
        let mut tokenizer = Tokenizer::new("thing as something else");
        assert_eq!(tokenizer.info().token, Token::Variable);
        assert_eq!(tokenizer.peek().token, Token::As);
    }

    #[test]
    fn peek_preserves_current_info() {
        let mut tokenizer = Tokenizer::new("thing as something else");
        assert_eq!(tokenizer.peek().token, Token::As);
        assert_eq!(tokenizer.info().token, Token::Variable);
    }

    #[test]
    fn multiple_peeks() {
        let mut tokenizer = Tokenizer::new("thing as loop end");
        assert_eq!(tokenizer.peek().token, Token::As);
        assert_eq!(tokenizer.peek().token, Token::Loop);
        assert_eq!(tokenizer.peek().token, Token::End);
    }

    #[test]
    fn peek_after_advance() {
        let mut tokenizer = Tokenizer::new("thing as loop end");
        assert_eq!(tokenizer.peek().token, Token::As);
        tokenizer.advance();
        assert_eq!(tokenizer.peek().token, Token::Loop);
    }

    #[test]
    fn two_peeks_after_advance() {
        let mut tokenizer = Tokenizer::new("thing as loop end");
        assert_eq!(tokenizer.peek().token, Token::As);
        assert_eq!(tokenizer.peek().token, Token::Loop);
        tokenizer.advance();
        assert_eq!(tokenizer.peek().token, Token::Loop);
    }

    #[test]
    fn peeks_and_advances() {
        let mut tokenizer = Tokenizer::new("thing as loop end {{ }}");
        assert_eq!(tokenizer.info().token, Token::Variable);
        assert_eq!(tokenizer.peek().token, Token::As);
        assert_eq!(tokenizer.peek().token, Token::Loop);
        assert_eq!(tokenizer.peek().token, Token::End);
        assert_eq!(tokenizer.peek().token, Token::LeftMustache);
        assert_eq!(tokenizer.peek().token, Token::RightMustache);
        tokenizer.advance();
        assert_eq!(tokenizer.info().token, Token::As);
        assert_eq!(tokenizer.peek().token, Token::Loop);
        assert_eq!(tokenizer.peek().token, Token::End);
        assert_eq!(tokenizer.peek().token, Token::LeftMustache);
        assert_eq!(tokenizer.peek().token, Token::RightMustache);
        tokenizer.advance();
        assert_eq!(tokenizer.info().token, Token::Loop);
        assert_eq!(tokenizer.peek().token, Token::End);
        assert_eq!(tokenizer.peek().token, Token::LeftMustache);
        assert_eq!(tokenizer.peek().token, Token::RightMustache);
        tokenizer.advance();
        assert_eq!(tokenizer.info().token, Token::End);
        assert_eq!(tokenizer.peek().token, Token::LeftMustache);
        assert_eq!(tokenizer.peek().token, Token::RightMustache);
        tokenizer.advance();
        assert_eq!(tokenizer.info().token, Token::LeftMustache);
        assert_eq!(tokenizer.peek().token, Token::RightMustache);
    }

    #[test]
    fn eof_peek() {
        let mut tokenizer = Tokenizer::new("thing");
        assert_eq!(tokenizer.peek().token, Token::EOF);
        assert_eq!(tokenizer.peek().token, Token::EOF);
        assert_eq!(tokenizer.peek().token, Token::EOF);
        tokenizer.advance();
        assert_eq!(tokenizer.info().token, Token::EOF);
        assert_eq!(tokenizer.peek().token, Token::EOF);
    }
}
