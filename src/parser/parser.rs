use crate::parser::scope::Scope;
use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tree_nodes::tree_nodes::ArraySliceExpr;
use crate::tree_nodes::tree_nodes::ArraySliceIndexExpr;
use crate::tree_nodes::tree_nodes::AsStatementExpr;
use crate::tree_nodes::tree_nodes::Expr;
use std::error::Error;

pub struct Parser<'a> {
    pub text: &'a str,
    pub lexer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser {
        Parser {
            text,
            lexer: Tokenizer::new(text),
        }
    }

    pub fn parse(&mut self) -> Expr {
        // let a = Scope::new();
        self.array_slice()
    }

    fn array_slice(&mut self) -> Expr {
        Expr::ArraySlice(Box::new(ArraySliceExpr {
            left_paren: self.consume(Token::LeftBracket),
            start_index: self.array_slice_index(),
            comma: self.consume(Token::Comma),
            end_index: self.array_slice_index(),
            right_paren: self.consume(Token::RightBracket),
        }))
    }

    fn array_slice_index(&mut self) -> Expr {
        match self.lexer.info().token {
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

    fn consume(&mut self, next: Token) -> InfoToken {
        println!("{:?}", next);
        let info = self.lexer.info();
        if info.token == next {
            self.lexer.advance();
            info
        } else {
            panic!("Error parsing the string!")
        }
    }
}
