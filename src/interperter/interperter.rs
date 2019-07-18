use crate::parser::parser::Parser;
use crate::tree_nodes::tree_nodes::Expr;
use crate::tree_nodes::tree_nodes::ArraySliceIndexExpr;

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
    
    fn visit_array_slice_index(&self, node: ArraySliceIndexExpr) {
        
    }
}
