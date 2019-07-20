use crate::tokenizer::tokenizer::InfoToken;

pub enum Expr {
    ArraySliceIndex(ArraySliceIndexExpr),
    ArrayBracketIndex(ArrayBracketIndexExpr),
    ArrayBracket(Box<ArrayBracketExpr>),
    ArraySlice(Box<ArraySliceExpr>),
    DotVariable(DotVariableExpr),
    Accessor(Box<AccessorExpr>),
    ArrayAccessor(Box<ArrayAccessorExpr>),
    MustacheAccessor(Box<MustacheAccessorExpr>),
    Block(Box<BlockExpr>),
    LoopEnd(LoopEndExpr),
    AsVariable(AsVariableExpr),
    ParenVariableParen(ParenVariableParenExpr),
    LoopStart(Box<LoopStartExpr>),
    Loop(Box<LoopExpr>),
}

pub struct LoopExpr {
    pub loop_start: Box<Expr>,
    pub block: Box<Expr>,
    pub loop_end: Box<Expr>,
}

pub struct LoopStartExpr {
    pub left_mustache: InfoToken,
    pub r#loop: InfoToken,
    pub loop_variable: Box<Option<Expr>>,
    pub array_accessor: Box<Expr>,
    pub as_variable: Box<Option<Expr>>,
    pub right_mustache: InfoToken,
}

pub struct ParenVariableParenExpr {
    pub left_paren: InfoToken,
    pub variable: InfoToken,
    pub right_paren: InfoToken,
}

pub struct AsVariableExpr {
    pub r#as: InfoToken,
    pub variable: InfoToken,
}

pub struct LoopEndExpr {
    pub left_mustache: InfoToken,
    pub end: InfoToken,
    pub right_mustache: InfoToken,
}

pub struct BlockExpr {
    pub blocks: Vec<Expr>,
}

pub struct MustacheAccessorExpr {
    pub left_mustache: InfoToken,
    pub accessor: Box<Expr>,
    pub right_mustache: InfoToken,
}

pub struct ArrayAccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<Expr>,
    pub array_slice: Option<Expr>,
}

pub struct AccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<Expr>,
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
    pub variable: Expr,
    pub right_paren: InfoToken,
}

pub struct ArraySliceIndexExpr {
    pub token: InfoToken,
}

pub struct ArrayBracketIndexExpr {
    pub token: InfoToken,
}
