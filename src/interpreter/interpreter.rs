use crate::interpreter::loop_iterator::LoopIterator;
use crate::parser::parser::Parser;
use crate::parser::scope::Scope;
use crate::parser::var_type::Var;
use crate::parser::var_type::VarType;
use crate::tokenizer::tokenizer::InfoToken;
use crate::tokenizer::tokenizer::Token;
use crate::tree_nodes::tree_nodes::*;

pub struct Interpreter<'a> {
    pub text: &'a str,
    parser: Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(text: &str) -> Interpreter {
        Interpreter {
            text,
            parser: Parser::new(text),
        }
    }

    pub fn interpret(&mut self) -> String {
        let expr = self.parser.parse();
        let mut base_scope = Scope::new();
        base_scope.insert(
            String::from("rows"),
            VarType::Row(Var::new(vec![
                String::from("Okay1"),
                String::from("Okay2"),
                String::from("Okay3"),
            ])),
        );
        self.visit_expr(expr, &mut base_scope)
    }

    pub fn visit_expr(&self, expr: Expr, scope: &mut Scope) -> String {
        match expr {
            Expr::Anything(node) => self.visit_anything(node),
            Expr::Block(node) => self.visit_block(node, scope),
            Expr::MustacheAccessor(node) => self.visit_mustache_accessor(node, scope),
            Expr::Loop(node) => self.visit_loop(node, scope),
        }
    }

    fn visit_block(&self, block_expr: Box<BlockExpr>, scope: &mut Scope) -> String {
        let exprs = block_expr.blocks;
        let mut strings: Vec<String> = vec![];

        for expr in exprs {
            let mut child_scope = Scope::with_parent(scope);
            strings.push(self.visit_expr(expr, &mut child_scope));
        }
        strings.join(" ")
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
        &self,
        mustache_accessor_expr: MustacheAccessorExpr,
        scope: &mut Scope,
    ) -> String {
        self.visit_accessor(mustache_accessor_expr.accessor, scope)
    }

    fn visit_accessor(&self, accessor_expr: AccessorExpr, scope: &mut Scope) -> String {
        let mut variable = scope.lookup(&accessor_expr.variable.slice).clone();
        for indexer in accessor_expr.indexes {
            variable = self.visit_array_bracket(indexer, scope, variable);
        }

        match variable {
            VarType::Value(variable) => variable.data.clone(),
            VarType::Number(variable) => variable.data.to_string(),
            _ => panic!("Cannot convert {} to String", accessor_expr.variable.slice),
        }
    }

    fn visit_loop(&self, loop_expr: Box<LoopExpr>, scope: &mut Scope) -> String {
        let mut strings: Vec<String> = vec![];
        let loop_iterator = self.visit_loop_start(loop_expr.loop_start, scope);
        for mut scope in loop_iterator {
            let output = self.visit_expr(*loop_expr.block.clone(), &mut scope);
            strings.push(output);
        }

        strings.join(" ")
    }

    fn visit_loop_start<'b>(
        &self,
        loop_start_expr: LoopStartExpr,
        scope: &'b mut Scope,
    ) -> LoopIterator<'b> {
        let (variable, min, max) = self.visit_array_accessor(loop_start_expr.array_accessor, scope);
        let as_variable_name: Option<String> = match loop_start_expr.as_variable {
            Some(as_variable) => Some(as_variable.variable.slice),
            None => None,
        };

        let loop_variable_name: Option<String> = match loop_start_expr.loop_variable {
            Some(loop_variable) => Some(loop_variable.variable.slice),
            None => None,
        };

        LoopIterator::new(
            scope,
            variable,
            min,
            max,
            loop_variable_name,
            as_variable_name,
        )
    }

    fn visit_array_accessor(
        &self,
        array_accessor_expr: ArrayAccessorExpr,
        scope: &mut Scope,
    ) -> (VarType, usize, usize) {
        let mut variable = scope.lookup(&array_accessor_expr.variable.slice).clone();
        for indexer in array_accessor_expr.indexes {
            variable = self.visit_array_bracket(indexer, scope, variable);
        }

        let (min, max) = self.visit_array_slice(array_accessor_expr.array_slice, scope, &variable);

        (variable, min, max)
    }

    fn visit_array_slice(
        &self,
        array_slice: Option<ArraySliceExpr>,
        scope: &mut Scope,
        collection: &VarType,
    ) -> (usize, usize) {
        let mut min_index = 0;
        let mut max_index = 0;
        match array_slice {
            Some(array_slice) => match collection {
                VarType::Table(var) => {
                    min_index = match array_slice.start_index.token.token {
                        Token::DoubleDot => 0,
                        _ => self.get_number_from_token(scope, array_slice.start_index.token),
                    };
                    max_index = match array_slice.end_index.token.token {
                        Token::DoubleDot => var.data.len(),
                        _ => self.get_number_from_token(scope, array_slice.end_index.token),
                    };
                }
                VarType::Row(var) => {
                    min_index = match array_slice.start_index.token.token {
                        Token::DoubleDot => 0,
                        _ => self.get_number_from_token(scope, array_slice.start_index.token),
                    };
                    max_index = match array_slice.end_index.token.token {
                        Token::DoubleDot => var.data.len(),
                        _ => self.get_number_from_token(scope, array_slice.end_index.token),
                    };
                }
                _ => panic!("Attempted to slice a non-iterable"),
            },
            None => match collection {
                VarType::Table(var) => {
                    max_index = var.data.len();
                }
                VarType::Row(var) => {
                    max_index = var.data.len();
                }
                _ => panic!("Attempt to loop on a non-iterable"),
            },
        };
        (min_index, max_index)
    }

    fn visit_array_bracket(
        &self,
        array_bracket_expr: ArrayBracketExpr,
        scope: &mut Scope,
        collection: VarType,
    ) -> VarType {
        let index = self.visit_array_bracket_index(array_bracket_expr.variable, scope);
        match collection {
            VarType::Table(var) => VarType::Row(Var::new(var.data[index].clone())),
            VarType::Row(var) => VarType::Value(Var::new(var.data[index].clone())),
            _ => panic!("Attempted to index a non-iterable"),
        }
    }

    fn visit_array_bracket_index(
        &self,
        array_bracket_index_expr: ArrayBracketIndexExpr,
        scope: &mut Scope,
    ) -> usize {
        self.get_number_from_token(scope, array_bracket_index_expr.token)
    }

    fn get_number_from_token(&self, scope: &mut Scope, info_token: InfoToken) -> usize {
        match info_token.token {
            Token::Number => info_token.slice.parse::<usize>().unwrap(),
            Token::Variable => {
                if let VarType::Number(x) = scope.lookup(&info_token.slice) {
                    x.data
                } else {
                    panic!("Cannot index using the variable {}", info_token.slice)
                }
            }
            _ => panic!("Cannot index arrays using {}", info_token.slice),
        }
    }
}
