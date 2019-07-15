use crate::tokenizer::tokenizer;
use crate::tokenizer::tokenizer::Token;
use crate::tree_nodes::tree_nodes::ArraySliceExpr;
use crate::tree_nodes::tree_nodes::ArraySliceIndexExpr;
use crate::tree_nodes::tree_nodes::Expr;
use logos::Logos;
use std::error::Error;

pub struct Parser<'a> {
    pub text: &'a str,
    pub lexer: logos::Lexer<Token, &'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser {
        Parser {
            text,
            lexer: tokenizer::tokenize(text),
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.array_slice()
    }

    fn array_slice(&mut self) -> Expr {
        Expr::ArraySlice(Box::new(ArraySliceExpr {
            leftParen: self.consume(Token::LeftBracket),
            start_index: self.array_slice_index(),
            comma: self.consume(Token::Comma),
            end_index: self.array_slice_index(),
            rightParen: self.consume(Token::RightBracket),
        }))
    }

    fn array_slice_index(&mut self) -> Expr {
        match self.lexer.token {
            Token::Number => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::Number),
            }),
            Token::DoubleDot => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::DoubleDot),
            }),
            Token::Variable => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::Variable),
            }),
            _ => panic!("Error parsing the string!"),
        }
    }

    fn consume(&mut self, next: Token) -> Token {
        println!("{:?}", next);
        if self.lexer.token == next {
            let token = self.lexer.token.clone();
            self.lexer.advance();
            token
        } else {
            panic!("Error parsing the string!")
        }
    }
}
