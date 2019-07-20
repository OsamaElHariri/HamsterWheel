use crate::parser::scope::Scope;
use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tree_nodes::tree_nodes::AccessorExpr;
use crate::tree_nodes::tree_nodes::ArrayAccessorExpr;
use crate::tree_nodes::tree_nodes::ArrayBracketExpr;
use crate::tree_nodes::tree_nodes::ArrayBracketIndexExpr;
use crate::tree_nodes::tree_nodes::ArraySliceExpr;
use crate::tree_nodes::tree_nodes::ArraySliceIndexExpr;
use crate::tree_nodes::tree_nodes::AsVariableExpr;
use crate::tree_nodes::tree_nodes::BlockExpr;
use crate::tree_nodes::tree_nodes::DotVariableExpr;
use crate::tree_nodes::tree_nodes::Expr;
use crate::tree_nodes::tree_nodes::LoopEndExpr;
use crate::tree_nodes::tree_nodes::LoopExpr;
use crate::tree_nodes::tree_nodes::LoopStartExpr;
use crate::tree_nodes::tree_nodes::MustacheAccessorExpr;
use crate::tree_nodes::tree_nodes::ParenVariableParenExpr;
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
        println!("parse fn");
        // let a = Scope::new();
        self.block()
    }

    fn block(&mut self) -> Expr {
        println!("block");
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
                self.lexer.advance();
            };
        }

        Expr::Block(Box::new(BlockExpr { blocks }))
    }

    fn mustache_accessor(&mut self) -> Expr {
        println!("mustache_accessor");
        Expr::MustacheAccessor(Box::new(MustacheAccessorExpr {
            left_mustache: self.consume(Token::LeftMustache),
            accessor: Box::new(self.accessor()),
            right_mustache: self.consume(Token::RightMustache),
        }))
    }

    fn r#loop(&mut self) -> Expr {
        println!("loop");
        Expr::Loop(Box::new(LoopExpr {
            loop_start: Box::new(self.loop_start()),
            block: Box::new(self.block()),
            loop_end: Box::new(self.loop_end()),
        }))
    }

    fn loop_start(&mut self) -> Expr {
        println!("loop_start");
        let left_mustache = self.consume(Token::LeftMustache);
        let r#loop = self.consume(Token::Loop);
        let mut loop_variable: Box<Option<Expr>> = Box::new(None);
        if self.lexer.info().token == Token::LeftParentheses {
            loop_variable = Box::new(Some(self.loop_variable()));
        };
        let array_accessor = Box::new(self.array_accessor());
        let mut as_variable: Box<Option<Expr>> = Box::new(None);
        if self.lexer.info().token == Token::As {
            as_variable = Box::new(Some(self.as_variable()));
        };
        Expr::LoopStart(Box::new(LoopStartExpr {
            left_mustache,
            r#loop,
            loop_variable,
            array_accessor,
            as_variable,
            right_mustache: self.consume(Token::RightMustache),
        }))
    }

    fn loop_variable(&mut self) -> Expr {
        println!("loop_variable");
        Expr::ParenVariableParen(ParenVariableParenExpr {
            left_paren: self.consume(Token::LeftParentheses),
            variable: self.consume(Token::Variable),
            right_paren: self.consume(Token::RightParentheses),
        })
    }

    fn as_variable(&mut self) -> Expr {
        println!("as_variable");
        Expr::AsVariable(AsVariableExpr {
            r#as: self.consume(Token::As),
            variable: self.consume(Token::Variable),
        })
    }

    fn loop_end(&mut self) -> Expr {
        println!("loop_end");
        Expr::LoopEnd(LoopEndExpr {
            left_mustache: self.consume(Token::LeftMustache),
            end: self.consume(Token::End),
            right_mustache: self.consume(Token::RightMustache),
        })
    }

    fn array_accessor(&mut self) -> Expr {
        println!("array_accessor");
        let variable = self.consume(Token::Variable);
        let mut indexers: Vec<Expr> = vec![];
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

            if self.lexer.info().token == Token::Dot {
                indexers.push(self.dot_variable());
            };
        }

        let array_slice = if self.lexer.info().token == Token::LeftBracket {
            Some(self.array_slice())
        } else {
            None
        };

        Expr::ArrayAccessor(Box::new(ArrayAccessorExpr {
            variable,
            indexes: indexers,
            array_slice,
        }))
    }

    fn accessor(&mut self) -> Expr {
        println!("accessor");
        let variable = self.consume(Token::Variable);
        let mut indexers: Vec<Expr> = vec![];
        while self.lexer.info().token == Token::LeftBracket || self.lexer.info().token == Token::Dot
        {
            if self.lexer.info().token == Token::LeftBracket {
                indexers.push(self.array_bracket());
            };

            if self.lexer.info().token == Token::Dot {
                indexers.push(self.dot_variable());
            };
        }

        Expr::Accessor(Box::new(AccessorExpr {
            variable,
            indexes: indexers,
        }))
    }

    fn dot_variable(&mut self) -> Expr {
        println!("dot_variable");
        Expr::DotVariable(DotVariableExpr {
            dot: self.consume(Token::Dot),
            variable: self.consume(Token::Variable),
        })
    }

    fn array_slice(&mut self) -> Expr {
        println!("array_slice");
        Expr::ArraySlice(Box::new(ArraySliceExpr {
            left_paren: self.consume(Token::LeftBracket),
            start_index: self.array_slice_index(),
            comma: self.consume(Token::Comma),
            end_index: self.array_slice_index(),
            right_paren: self.consume(Token::RightBracket),
        }))
    }

    fn array_slice_index(&mut self) -> Expr {
        println!("array_slice_index");
        match self.lexer.info().token {
            Token::DoubleDot => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::DoubleDot),
            }),
            Token::Variable => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::Variable),
            }),
            _ => Expr::ArraySliceIndex(ArraySliceIndexExpr {
                token: self.consume(Token::Number),
            }),
        }
    }

    fn array_bracket(&mut self) -> Expr {
        println!("array_bracket");
        Expr::ArrayBracket(Box::new(ArrayBracketExpr {
            left_paren: self.consume(Token::LeftBracket),
            variable: self.array_bracket_index(),
            right_paren: self.consume(Token::RightBracket),
        }))
    }

    fn array_bracket_index(&mut self) -> Expr {
        println!("array_bracket_index");
        match self.lexer.info().token {
            Token::Variable => Expr::ArrayBracketIndex(ArrayBracketIndexExpr {
                token: self.consume(Token::Variable),
            }),
            _ => Expr::ArrayBracketIndex(ArrayBracketIndexExpr {
                token: self.consume(Token::Number),
            }),
        }
    }

    fn consume(&mut self, next: Token) -> InfoToken {
        // println!("{:?}", next);
        let info = self.lexer.info();
        if info.token == next {
            let info = info.clone();
            self.lexer.advance();
            info.clone()
        } else {
            panic!(
                "{:?} is not a {:?}.\nCurrent slice reads {}.\n The slice runs from {} to {}",
                info.token, next, info.slice, info.start, info.end
            )
        }
    }
}
