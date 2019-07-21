use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tree_nodes::tree_nodes::*;
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
        // println!("parse fn");
        // let a = Scope::new();
        self.block()
    }

    fn block(&mut self) -> Expr {
        // println!("block");
        let mut blocks: Vec<Expr> = vec![];
        while self.lexer.info().token != Token::EOF {
            if self.lexer.info().token == Token::LeftMustache {
                let next_info = self.lexer.peek();
                match next_info.token {
                    Token::Loop => blocks.push(self.r#loop()),
                    Token::End => break,
                    _ => blocks.push(self.mustache_accessor()),
                };
            } else {
                blocks.push(self.anything());
            };
        }

        Expr::Block(Box::new(BlockExpr { blocks }))
    }

    fn anything(&mut self) -> Expr {
        // println!("anything");
        let mut tokens: Vec<InfoToken> = vec![];
        while self.lexer.info().token != Token::EOF
            && self.lexer.info().token != Token::LeftMustache
        {
            let info = self.lexer.info();
            let mut start = info.start - 1;
            while self.text.chars().nth(start).expect("").is_whitespace() && start > 0 {
                start -= 1
            }
            tokens.push(InfoToken {
                token: info.token.clone(),
                slice: info.slice.clone(),
                start: start + 1,
                end: info.end,
            });
            self.lexer.advance();
        }

        Expr::Anything(Box::new(AnythingExpr { tokens }))
    }

    fn mustache_accessor(&mut self) -> Expr {
        // println!("mustache_accessor");
        Expr::MustacheAccessor(MustacheAccessorExpr {
            left_mustache: self.consume(Token::LeftMustache),
            accessor: self.accessor(),
            right_mustache: self.consume(Token::RightMustache),
        })
    }

    fn r#loop(&mut self) -> Expr {
        // println!("loop");
        Expr::Loop(Box::new(LoopExpr {
            loop_start: self.loop_start(),
            block: Box::new(self.block()),
            loop_end: self.loop_end(),
        }))
    }

    fn loop_start(&mut self) -> LoopStartExpr {
        // println!("loop_start");
        let left_mustache = self.consume(Token::LeftMustache);
        let r#loop = self.consume(Token::Loop);
        let mut loop_variable: Option<ParenVariableParenExpr> = None;
        if self.lexer.info().token == Token::LeftParentheses {
            loop_variable = Some(self.loop_variable());
        };
        let array_accessor = self.array_accessor();
        let mut as_variable: Option<AsVariableExpr> = None;
        if self.lexer.info().token == Token::As {
            as_variable = Some(self.as_variable());
        };
        LoopStartExpr {
            left_mustache,
            r#loop,
            loop_variable,
            array_accessor,
            as_variable,
            right_mustache: self.consume(Token::RightMustache),
        }
    }

    fn loop_variable(&mut self) -> ParenVariableParenExpr {
        // println!("loop_variable");
        ParenVariableParenExpr {
            left_paren: self.consume(Token::LeftParentheses),
            variable: self.consume(Token::Variable),
            right_paren: self.consume(Token::RightParentheses),
        }
    }

    fn as_variable(&mut self) -> AsVariableExpr {
        // println!("as_variable");
        AsVariableExpr {
            r#as: self.consume(Token::As),
            variable: self.consume(Token::Variable),
        }
    }

    fn loop_end(&mut self) -> LoopEndExpr {
        // println!("loop_end");
        LoopEndExpr {
            left_mustache: self.consume(Token::LeftMustache),
            end: self.consume(Token::End),
            right_mustache: self.consume(Token::RightMustache),
        }
    }

    fn array_accessor(&mut self) -> ArrayAccessorExpr {
        // println!("array_accessor");
        let variable = self.consume(Token::Variable);
        let mut indexers: Vec<ArrayBracketExpr> = vec![];
        while self.lexer.info().token == Token::LeftBracket || self.lexer.info().token == Token::Dot
        {
            if self.lexer.info().token == Token::LeftBracket {
                let var_peek = self.lexer.peek();
                match var_peek.token {
                    Token::Variable | Token::DoubleDot | Token::Number => {
                        let comma_peek = self.lexer.peek();
                        if comma_peek.token == Token::Comma {
                            break;
                        };
                    }
                    _ => (),
                };

                indexers.push(self.array_bracket());
            };

            // if self.lexer.info().token == Token::Dot {
            //     indexers.push(self.dot_variable());
            // };
        }

        let array_slice = if self.lexer.info().token == Token::LeftBracket {
            Some(self.array_slice())
        } else {
            None
        };

        ArrayAccessorExpr {
            variable,
            indexes: indexers,
            array_slice,
        }
    }

    fn accessor(&mut self) -> AccessorExpr {
        // println!("accessor");
        let variable = self.consume(Token::Variable);
        let mut indexers: Vec<ArrayBracketExpr> = vec![];
        while self.lexer.info().token == Token::LeftBracket || self.lexer.info().token == Token::Dot
        {
            if self.lexer.info().token == Token::LeftBracket {
                indexers.push(self.array_bracket());
            };

            // if self.lexer.info().token == Token::Dot {
            //     indexers.push(self.dot_variable());
            // };
        }

        AccessorExpr {
            variable,
            indexes: indexers,
        }
    }

    // fn dot_variable(&mut self) -> Expr {
    //     // println!("dot_variable");
    //     Expr::DotVariable(DotVariableExpr {
    //         dot: self.consume(Token::Dot),
    //         variable: self.consume(Token::Variable),
    //     })
    // }

    fn array_slice(&mut self) -> ArraySliceExpr {
        // println!("array_slice");
        ArraySliceExpr {
            left_paren: self.consume(Token::LeftBracket),
            start_index: self.array_slice_index(),
            comma: self.consume(Token::Comma),
            end_index: self.array_slice_index(),
            right_paren: self.consume(Token::RightBracket),
        }
    }

    fn array_slice_index(&mut self) -> ArraySliceIndexExpr {
        // println!("array_slice_index");
        match self.lexer.info().token {
            Token::DoubleDot => ArraySliceIndexExpr {
                token: self.consume(Token::DoubleDot),
            },
            Token::Variable => ArraySliceIndexExpr {
                token: self.consume(Token::Variable),
            },
            _ => ArraySliceIndexExpr {
                token: self.consume(Token::Number),
            },
        }
    }

    fn array_bracket(&mut self) -> ArrayBracketExpr {
        // println!("array_bracket");
        ArrayBracketExpr {
            left_paren: self.consume(Token::LeftBracket),
            variable: self.array_bracket_index(),
            right_paren: self.consume(Token::RightBracket),
        }
    }

    fn array_bracket_index(&mut self) -> ArrayBracketIndexExpr {
        // println!("array_bracket_index");
        match self.lexer.info().token {
            Token::Variable => ArrayBracketIndexExpr {
                token: self.consume(Token::Variable),
            },
            _ => ArrayBracketIndexExpr {
                token: self.consume(Token::Number),
            },
        }
    }

    fn consume(&mut self, next: Token) -> InfoToken {
        println!("{:?}", next);
        let info = self.lexer.info();
        if info.token == next {
            let info = info.clone();
            self.lexer.advance();
            info.clone()
        } else {
            panic!(
                "Expected {:?}, found {:?}.\nCurrent slice reads {}.\n The slice runs from {} to {}",
                 next, info.token, info.slice, info.start, info.end
            )
        }
    }
}
