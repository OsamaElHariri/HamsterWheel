use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tree_nodes::tree_nodes::*;
use std::fmt;

pub struct Parser<'a> {
    pub text: &'a str,
    pub lexer: Tokenizer<'a>,
    pub current_line: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Parser {
        Parser {
            text,
            lexer: Tokenizer::new(text),
            current_line: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.start()
    }

    fn start(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::Start(Box::new(StartExpr {
            output: self.output()?,
            expr: self.block()?,
        })))
    }

    fn output(&mut self) -> Result<OutputExpr, ParseError> {
        let left_mustache = self.consume(Token::LeftMustache)?;
        let output = self.consume(Token::Output)?;
        let mut vars = vec![];
        while self.lexer.info().token != Token::EOF
            && self.lexer.info().token != Token::RightMustache
            && self.lexer.info().token != Token::LeftMustache
        {
            let token = self.lexer.info().token.clone();
            vars.push(self.consume(token)?);
        }
        if vars.len() == 0 {
            return Err(ParseError {
                msg: String::from("An output file must be specified at the top of the file"),
                line_number: self.current_line,
            });
        }
        let value_text = self.text[vars[0].start..vars[vars.len() - 1].end].to_string();
        let value = InfoToken {
            token: Token::Variable,
            slice: value_text,
            start: vars[0].start,
            end: vars.last().expect("non-empty").end,
        };
        Ok(OutputExpr {
            left_mustache,
            output,
            file_path: value,
            right_mustache: self.consume(Token::RightMustache)?,
        })
    }

    fn block(&mut self) -> Result<Expr, ParseError> {
        let mut blocks: Vec<Expr> = vec![];
        let mut imports = vec![];
        while self.lexer.info().token != Token::EOF {
            if self.lexer.info().token == Token::LeftMustache {
                let next_info = self.lexer.peek();
                match next_info.token {
                    Token::Loop => blocks.push(self.r#loop()?),
                    Token::Import => imports.push(self.import_stmt()?),
                    Token::End => break,
                    _ => blocks.push(self.mustache_accessor()?),
                };
            } else {
                blocks.push(self.anything());
            };
        }

        Ok(Expr::Block(Box::new(BlockExpr { imports, blocks })))
    }

    fn anything(&mut self) -> Expr {
        let mut tokens: Vec<InfoToken> = vec![];
        while self.lexer.info().token != Token::EOF
            && self.lexer.info().token != Token::LeftMustache
        {
            let info = self.lexer.info();
            let mut start = info.start - 1;
            while start > 0 && self.text.chars().nth(start).expect("").is_whitespace() {
                start -= 1
            }

            let mut end = info.end;
            while end < self.text.len() && self.text.chars().nth(end).expect("").is_whitespace() {
                end += 1;
            }
            tokens.push(InfoToken {
                token: info.token.clone(),
                slice: info.slice.clone(),
                start: start + 1,
                end: end,
            });
            self.lexer.advance();
        }

        Expr::Anything(Box::new(AnythingExpr { tokens }))
    }

    fn mustache_accessor(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::MustacheAccessor(MustacheAccessorExpr {
            left_mustache: self.consume(Token::LeftMustache)?,
            accessor: self.accessor()?,
            right_mustache: self.consume(Token::RightMustache)?,
        }))
    }

    fn r#loop(&mut self) -> Result<Expr, ParseError> {
        Ok(Expr::Loop(Box::new(LoopExpr {
            loop_start: self.loop_start()?,
            block: Box::new(self.block()?),
            loop_end: self.loop_end()?,
        })))
    }

    fn import_stmt(&mut self) -> Result<ImportExpr, ParseError> {
        let left_mustache = self.consume(Token::LeftMustache)?;
        let config = self.consume(Token::Import)?;
        let right_mustache = self.consume(Token::RightMustache)?;
        let mut configs = vec![];
        while self.lexer.info().token != Token::LeftMustache {
            configs.push(self.import_option()?);
        }

        Ok(ImportExpr {
            left_mustache,
            config,
            right_mustache,
            configs: configs,
            end: self.loop_end()?,
        })
    }

    fn import_option(&mut self) -> Result<ImportConfigOptionExpr, ParseError> {
        let variable = self.consume(Token::Variable)?;
        let colon = self.consume(Token::Colon)?;
        let mut vars = vec![];
        while self.lexer.info().token != Token::EOF
            && self.lexer.info().token != Token::SemiColon
            && self.lexer.info().token != Token::LeftMustache
        {
            let token = self.lexer.info().token.clone();
            vars.push(self.consume(token)?);
        }
        if vars.len() == 0 {
            return Err(ParseError {
                msg: String::from("Configs must be in the form of: \"name: value;\""),
                line_number: self.current_line,
            });
        }
        let value_text = self.text[vars[0].start..vars[vars.len() - 1].end].to_string();
        let value = InfoToken {
            token: Token::Variable,
            slice: value_text,
            start: vars[0].start,
            end: vars.last().expect("non-empty").end,
        };

        Ok(ImportConfigOptionExpr {
            variable,
            colon,
            value: value,
            semicolon: self.consume(Token::SemiColon)?,
        })
    }

    fn loop_start(&mut self) -> Result<LoopStartExpr, ParseError> {
        let left_mustache = self.consume(Token::LeftMustache)?;
        let r#loop = self.consume(Token::Loop)?;
        let mut loop_variable: Option<ParenVariableParenExpr> = None;
        if self.lexer.info().token == Token::LeftParentheses {
            loop_variable = Some(self.loop_variable()?);
        };
        let array_accessor = self.array_accessor()?;
        let mut as_variable: Option<AsVariableExpr> = None;
        if self.lexer.info().token == Token::As {
            as_variable = Some(self.as_variable()?);
        };
        Ok(LoopStartExpr {
            left_mustache,
            r#loop,
            loop_variable,
            array_accessor,
            as_variable,
            right_mustache: self.consume(Token::RightMustache)?,
        })
    }

    fn loop_variable(&mut self) -> Result<ParenVariableParenExpr, ParseError> {
        let left_paren = self.consume(Token::LeftParentheses)?;
        let variable = self.consume(Token::Variable)?;
        let second_variable = match self.lexer.info().token {
            Token::Comma => Some(self.comma_variable()?),
            _ => None,
        };

        Ok(ParenVariableParenExpr {
            left_paren,
            variable,
            second_variable,
            right_paren: self.consume(Token::RightParentheses)?,
        })
    }

    fn comma_variable(&mut self) -> Result<CommaVariableExpr, ParseError> {
        Ok(CommaVariableExpr {
            comma: self.consume(Token::Comma)?,
            variable: self.consume(Token::Variable)?,
        })
    }

    fn as_variable(&mut self) -> Result<AsVariableExpr, ParseError> {
        Ok(AsVariableExpr {
            r#as: self.consume(Token::As)?,
            variable: self.consume(Token::Variable)?,
        })
    }

    fn loop_end(&mut self) -> Result<EndExpr, ParseError> {
        Ok(EndExpr {
            left_mustache: self.consume(Token::LeftMustache)?,
            end: self.consume(Token::End)?,
            right_mustache: self.consume(Token::RightMustache)?,
        })
    }

    fn array_accessor(&mut self) -> Result<ArrayAccessorExpr, ParseError> {
        let variable = self.consume(Token::Variable)?;
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

                indexers.push(self.array_bracket()?);
            };

            // if self.lexer.info().token == Token::Dot {
            //     indexers.push(self.dot_variable());
            // };
        }

        let array_slice = if self.lexer.info().token == Token::LeftBracket {
            Some(self.array_slice()?)
        } else {
            None
        };

        Ok(ArrayAccessorExpr {
            variable,
            indexes: indexers,
            array_slice,
        })
    }

    fn accessor(&mut self) -> Result<AccessorExpr, ParseError> {
        let variable = self.consume(Token::Variable)?;
        let mut indexers: Vec<ArrayBracketExpr> = vec![];
        while self.lexer.info().token == Token::LeftBracket || self.lexer.info().token == Token::Dot
        {
            if self.lexer.info().token == Token::LeftBracket {
                indexers.push(self.array_bracket()?);
            };

            // if self.lexer.info().token == Token::Dot {
            //     indexers.push(self.dot_variable());
            // };
        }

        Ok(AccessorExpr {
            variable,
            indexes: indexers,
        })
    }

    // fn dot_variable(&mut self) -> Expr {
    //     Expr::DotVariable(DotVariableExpr {
    //         dot: self.consume(Token::Dot),
    //         variable: self.consume(Token::Variable),
    //     })
    // }

    fn array_slice(&mut self) -> Result<ArraySliceExpr, ParseError> {
        Ok(ArraySliceExpr {
            left_paren: self.consume(Token::LeftBracket)?,
            start_index: self.array_slice_index()?,
            comma: self.consume(Token::Comma)?,
            end_index: self.array_slice_index()?,
            right_paren: self.consume(Token::RightBracket)?,
        })
    }

    fn array_slice_index(&mut self) -> Result<ArraySliceIndexExpr, ParseError> {
        match self.lexer.info().token {
            Token::DoubleDot => Ok(ArraySliceIndexExpr {
                token: self.consume(Token::DoubleDot)?,
            }),
            Token::Variable => Ok(ArraySliceIndexExpr {
                token: self.consume(Token::Variable)?,
            }),
            _ => Ok(ArraySliceIndexExpr {
                token: self.consume(Token::Number)?,
            }),
        }
    }

    fn array_bracket(&mut self) -> Result<ArrayBracketExpr, ParseError> {
        Ok(ArrayBracketExpr {
            left_paren: self.consume(Token::LeftBracket)?,
            variable: self.array_bracket_index()?,
            right_paren: self.consume(Token::RightBracket)?,
        })
    }

    fn array_bracket_index(&mut self) -> Result<ArrayBracketIndexExpr, ParseError> {
        match self.lexer.info().token {
            Token::Variable => Ok(ArrayBracketIndexExpr {
                token: self.consume(Token::Variable)?,
            }),
            _ => Ok(ArrayBracketIndexExpr {
                token: self.consume(Token::Number)?,
            }),
        }
    }

    fn consume(&mut self, next: Token) -> Result<InfoToken, ParseError> {
        // println!("{:?}", next);
        let start_index = self.lexer.info().start;
        self.current_line = self.get_line_count_at_index(start_index);
        let info = self.lexer.info();
        if info.token == next {
            let info = info.clone();
            self.lexer.advance();
            Ok(info.clone())
        } else {
            Err(ParseError {
                msg: format!("Expected {:?}, found {:?}.\nCurrent slice reads {}.\n The slice runs from {} to {}",
                 next, info.token, info.slice, info.start, info.end),
                 line_number: self.current_line,
            })
        }
    }

    pub fn get_line_count_at_index(&mut self, index: usize) -> usize {
        let text = &self.text[0..index];
        text.matches("\n").count() + 1
    }
}

pub struct ParseError {
    line_number: usize,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("Error at line number {}\n{}", self.line_number, self.msg)
        )
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
