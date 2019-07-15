use crate::tokenizer::tokenizer;

pub enum Expr {
    ArraySlice(Box<ArraySliceExpr>),
    ArraySliceIndex(ArraySliceIndexExpr),
}

pub struct ArraySliceExpr {
    pub leftParen: tokenizer::Token,
    pub start_index: crate::tree_nodes::tree_nodes::Expr,
    pub comma: tokenizer::Token,
    pub end_index: crate::tree_nodes::tree_nodes::Expr,
    pub rightParen: tokenizer::Token,
}

pub struct ArraySliceIndexExpr {
    pub token: tokenizer::Token,
}
