use crate::tokenizer::tokenizer::InfoToken;

pub enum Expr {
    ArraySlice(Box<ArraySliceExpr>),
    ArraySliceIndex(ArraySliceIndexExpr),
    AsStatement(AsStatementExpr),
}

pub struct ArraySliceExpr {
    pub left_paren: InfoToken,
    pub start_index: crate::tree_nodes::tree_nodes::Expr,
    pub comma: InfoToken,
    pub end_index: crate::tree_nodes::tree_nodes::Expr,
    pub right_paren: InfoToken,
}

pub struct AsStatementExpr {
    pub r#as: InfoToken,
    pub variable: InfoToken,
}

pub struct ArraySliceIndexExpr {
    pub token: InfoToken,
}
