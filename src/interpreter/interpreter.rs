use crate::interpreter::importer::Importer;
use crate::interpreter::interpreter_result::InterpreterResult;
use crate::interpreter::loop_iterator::LoopIterator;
use crate::parser::parser::ParseError;
use crate::parser::parser::Parser;
use crate::parser::scope::Scope;
use crate::parser::var_type::Var;
use crate::parser::var_type::VarType;
use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tree_nodes::tree_nodes::*;
use std::fmt;

pub struct Interpreter<'a> {
    pub text: &'a str,
    parser: Parser<'a>,
    output_file: String,
}

impl<'a> Interpreter<'a> {
    pub fn new(text: &str) -> Interpreter {
        Interpreter {
            text,
            parser: Parser::new(text),
            output_file: String::from(""),
        }
    }

    pub fn interpret(&mut self) -> Result<InterpreterResult, GeneralError> {
        let expr = self.parser.parse()?;
        let mut base_scope = Scope::new();
        Ok(InterpreterResult {
            text: self.visit_expr(expr, &mut base_scope)?,
            output_file: self.output_file.clone(),
        })
    }

    pub fn visit_expr(
        &mut self,
        expr: Expr,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        match expr {
            Expr::Start(node) => self.visit_start(node, scope),
            Expr::Anything(node) => Ok(self.visit_anything(node)),
            Expr::Block(node) => self.visit_block(node, scope),
            Expr::MustacheAccessor(node) => self.visit_mustache_accessor(node, scope),
            Expr::Loop(node) => self.visit_loop(node, scope),
        }
    }

    fn visit_start(
        &mut self,
        start_expr: Box<StartExpr>,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        self.output_file = start_expr.output.file_path.slice;
        self.visit_expr(start_expr.expr, scope)
    }

    fn visit_block(
        &mut self,
        block_expr: Box<BlockExpr>,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        let exprs = block_expr.blocks;
        let mut strings: Vec<String> = vec![];

        Importer::update_scope(scope, block_expr.imports);

        for expr in exprs {
            let mut child_scope = Scope::with_parent(scope);
            strings.push(self.visit_expr(expr, &mut child_scope)?);
        }
        Ok(strings.join(""))
    }

    fn visit_anything(&self, anything_expr: Box<AnythingExpr>) -> String {
        if anything_expr.tokens.len() > 0 {
            self.text[anything_expr.tokens[0].start
                ..anything_expr.tokens[anything_expr.tokens.len() - 1].end]
                .to_string()
        } else {
            String::from("")
        }
    }

    fn visit_mustache_accessor(
        &mut self,
        mustache_accessor_expr: MustacheAccessorExpr,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        self.visit_accessor(mustache_accessor_expr.accessor, scope)
    }

    fn visit_accessor(
        &mut self,
        accessor_expr: AccessorExpr,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        let mut variable = self.lookup(scope, accessor_expr.variable.clone())?.clone();
        for indexer in accessor_expr.indexes {
            variable = self.visit_array_bracket(indexer, scope, variable)?;
        }

        match variable {
            VarType::Value(variable) => Ok(variable.data.to_string()),
            VarType::Number(variable) => Ok(variable.data.to_string()),
            _ => Err(InterpreterError {
                msg: format!("Cannot convert {} to String", accessor_expr.variable.slice),
                line_number: self.get_line_number_for_token(accessor_expr.variable),
            }),
        }
    }

    fn visit_loop(
        &mut self,
        loop_expr: Box<LoopExpr>,
        scope: &mut Scope,
    ) -> Result<String, InterpreterError> {
        let mut strings: Vec<String> = vec![];
        let loop_iterator = self.visit_loop_start(loop_expr.loop_start, scope)?;

        for mut scope in loop_iterator {
            let output = self.visit_expr(*loop_expr.block.clone(), &mut scope)?;
            strings.push(output);
        }

        Ok(strings.join(" "))
    }

    fn visit_loop_start<'b>(
        &mut self,
        loop_start_expr: LoopStartExpr,
        scope: &'b mut Scope,
    ) -> Result<LoopIterator<'b>, InterpreterError> {
        let (variable, min, max) =
            self.visit_array_accessor(loop_start_expr.array_accessor.clone(), scope)?;
        let as_variable_name: Option<String> = match loop_start_expr.as_variable {
            Some(as_variable) => Some(as_variable.variable.slice),
            None => None,
        };

        let loop_variable_name: Option<String> = match &loop_start_expr.loop_variable {
            Some(loop_variable) => Some(loop_variable.variable.slice.clone()),
            None => None,
        };

        let collection_variable_name: Option<String> = match &loop_start_expr.loop_variable {
            Some(loop_variable) => match &loop_variable.second_variable {
                Some(second_variable) => Some(second_variable.variable.slice.clone()),
                None => None,
            },
            None => None,
        };

        match &variable {
            VarType::Table(_var) => (),
            VarType::Row(_var) => (),
            _ => {
                return Err(InterpreterError {
                    msg: String::from("Attempt to loop on a non-iterable"),
                    line_number: self
                        .get_line_number_for_token(loop_start_expr.array_accessor.variable.clone()),
                })
            }
        };

        Ok(LoopIterator::new(
            scope,
            variable,
            min,
            max,
            loop_variable_name,
            collection_variable_name,
            as_variable_name,
        ))
    }

    fn visit_array_accessor(
        &mut self,
        array_accessor_expr: ArrayAccessorExpr,
        scope: &mut Scope,
    ) -> Result<(VarType, usize, usize), InterpreterError> {
        let mut variable = self
            .lookup(scope, array_accessor_expr.variable.clone())?
            .clone();
        for indexer in array_accessor_expr.indexes {
            variable = self.visit_array_bracket(indexer, scope, variable)?;
        }

        let (min, max) = self.visit_array_slice(
            array_accessor_expr.variable,
            array_accessor_expr.array_slice,
            scope,
            &variable,
        )?;

        Ok((variable, min, max))
    }

    fn visit_array_slice(
        &mut self,
        variable_info_token: InfoToken,
        array_slice: Option<ArraySliceExpr>,
        scope: &mut Scope,
        collection: &VarType,
    ) -> Result<(usize, usize), InterpreterError> {
        let mut min_index = 0;
        let mut _max_index = 0;
        match array_slice {
            Some(array_slice) => match collection {
                VarType::Table(var) => {
                    min_index = match array_slice.start_index.token.token {
                        Token::DoubleDot => 0,
                        _ => self.get_number_from_token(scope, array_slice.start_index.token)?,
                    };
                    _max_index = match array_slice.end_index.token.token {
                        Token::DoubleDot => var.data.len(),
                        _ => self.get_number_from_token(scope, array_slice.end_index.token)?,
                    };
                }
                VarType::Row(var) => {
                    min_index = match array_slice.start_index.token.token {
                        Token::DoubleDot => 0,
                        _ => self.get_number_from_token(scope, array_slice.start_index.token)?,
                    };
                    _max_index = match array_slice.end_index.token.token {
                        Token::DoubleDot => var.data.len(),
                        _ => self.get_number_from_token(scope, array_slice.end_index.token)?,
                    };
                }
                _ => {
                    return Err(InterpreterError {
                        msg: String::from("Attempt to slice a non-iterable"),
                        line_number: self.get_line_number_for_token(variable_info_token),
                    })
                }
            },
            None => match collection {
                VarType::Table(var) => {
                    _max_index = var.data.len();
                }
                VarType::Row(var) => {
                    _max_index = var.data.len();
                }
                _ => {
                    return Err(InterpreterError {
                        msg: String::from("Attempt to loop on a non-iterable"),
                        line_number: self.get_line_number_for_token(variable_info_token),
                    })
                }
            },
        };
        Ok((min_index, _max_index))
    }

    fn visit_array_bracket(
        &mut self,
        array_bracket_expr: ArrayBracketExpr,
        scope: &mut Scope,
        collection: VarType,
    ) -> Result<VarType, InterpreterError> {
        let index = self.visit_array_bracket_index(array_bracket_expr.clone().variable, scope)?;
        match collection {
            VarType::Table(var) => {
                let mut value = var.data.get(index);
                let otherwise_value: &Vec<String> = &vec![];
                let value = value.get_or_insert(otherwise_value);
                Ok(VarType::Row(Var::new(value.clone())))
            }
            VarType::Row(var) => {
                let mut value = var.data.get(index);
                let otherwise_value = &String::from("");
                let value = value.get_or_insert(otherwise_value);
                Ok(VarType::Value(Var::new(value.clone())))
            }
            _ => Err(InterpreterError {
                msg: String::from("Attempt to index a non-iterable"),
                line_number: self
                    .get_line_number_for_token(array_bracket_expr.variable.token.clone()),
            }),
        }
    }

    fn visit_array_bracket_index(
        &mut self,
        array_bracket_index_expr: ArrayBracketIndexExpr,
        scope: &mut Scope,
    ) -> Result<usize, InterpreterError> {
        self.get_number_from_token(scope, array_bracket_index_expr.token)
    }

    fn get_number_from_token(
        &mut self,
        scope: &mut Scope,
        info_token: InfoToken,
    ) -> Result<usize, InterpreterError> {
        match info_token.token {
            Token::Number => match info_token.slice.parse::<usize>() {
                Err(_) => Err(InterpreterError {
                    msg: format!("Cannot index using the variable {}", info_token.slice),
                    line_number: self.get_line_number_for_token(info_token),
                }),
                Ok(val) => Ok(val),
            },
            Token::Variable => {
                if let VarType::Number(x) = self.lookup(scope, info_token.clone())? {
                    Ok(x.data)
                } else {
                    return Err(InterpreterError {
                        msg: format!("Cannot index using the variable {}", info_token.slice),
                        line_number: self.get_line_number_for_token(info_token),
                    });
                }
            }
            _ => Err(InterpreterError {
                msg: format!("Cannot index arrays using {}", info_token.slice),
                line_number: self.get_line_number_for_token(info_token),
            }),
        }
    }

    fn lookup<'b>(
        &mut self,
        scope: &'b mut Scope,
        info_token: InfoToken,
    ) -> Result<&'b VarType, InterpreterError> {
        match scope.lookup(&info_token.slice) {
            Ok(val) => Ok(val),
            Err(e) => Err(InterpreterError {
                msg: format!("{}", e),
                line_number: self.get_line_number_for_token(info_token),
            }),
        }
    }

    fn get_line_number_for_token(&mut self, info_token: InfoToken) -> usize {
        self.parser.get_line_count_at_index(info_token.start)
    }
}

pub struct InterpreterError {
    pub msg: String,
    line_number: usize,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at line number {}\n{}", self.line_number, self.msg)
    }
}

impl fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub struct GeneralError {
    pub msg: String,
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl From<ParseError> for GeneralError {
    fn from(error: ParseError) -> Self {
        GeneralError {
            msg: format!("{}", error),
        }
    }
}

impl From<InterpreterError> for GeneralError {
    fn from(error: InterpreterError) -> Self {
        GeneralError {
            msg: format!("{}", error),
        }
    }
}
