use crate::tokenizer::tokenizer::InfoToken;

pub enum Expr {
    ArraySliceIndex(ArraySliceIndexExpr),
    ArrayBracket(ArrayBracketExpr),
    ArraySlice(Box<ArraySliceExpr>),
    DotVariable(DotVariableExpr),
    Accessor(Box<AccessorExpr>),
    ArrayAccessor(Box<ArrayAccessorExpr>),
    Block(Box<BlockExpr>),
    LoopEnd(LoopEndExpr),
    LoopStart(Box<LoopStartExpr>),
}

pub struct LoopStartExpr {
    left_mustache: InfoToken,
    loop_variable: Box<Option<Expr>>,
    array_accessor: Box<Expr>,
    as_variable: Box<Option<Expr>>,
    rightt_mustache: InfoToken,
}

pub struct ParenVariableParen {
    left_paren: InfoToken,
    variable: InfoToken,
    right_paren: InfoToken,
}

pub struct AsStatementExpr {
    pub r#as: InfoToken,
    pub variable: InfoToken,
}

pub struct LoopEndExpr {
    left_mustache: InfoToken,
    end: InfoToken,
    right_mustache: InfoToken,
}

pub struct BlockExpr {
    block: Box<Expr>,
}

pub struct BlockAnythingBlock {
    pub blocks: Vec<Expr>,
}

pub struct MustacheAccessor {
    left_mustache: InfoToken,
    accessor: Box<Expr>,
    right_mustache: InfoToken,
}

pub struct ArrayAccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<Option<(Expr, Expr)>>,
    pub array_slice: Option<Expr>,
}

pub struct AccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<Option<(Expr, Expr)>>,
}

pub struct DotVariableExpr {
    pub dot: InfoToken,
    pub variable: InfoToken,
}

pub struct ArraySliceExpr {
    pub left_paren: InfoToken,
    pub start_index: Expr,
    pub comma: InfoToken,
    pub end_index: Expr,
    pub right_paren: InfoToken,
}

pub struct ArrayBracketExpr {
    pub left_paren: InfoToken,
    pub variable: InfoToken,
    pub right_paren: InfoToken,
}

pub struct ArraySliceIndexExpr {
    pub token: InfoToken,
}
