use crate::tokenizer::tokenizer::InfoToken;

#[derive(Clone)]
pub enum Expr {
    Start(Box<StartExpr>),
    Block(Box<BlockExpr>),
    Anything(Box<AnythingExpr>),
    Loop(Box<LoopExpr>),
    MustacheAccessor(MustacheAccessorExpr),
}

#[derive(Clone)]
pub struct StartExpr {
    pub output: OutputExpr,
    pub expr: Expr,
}

#[derive(Clone)]
pub struct OutputExpr {
    pub left_mustache: InfoToken,
    pub output: InfoToken,
    pub file_path: InfoToken,
    pub right_mustache: InfoToken,
}

#[derive(Clone)]
pub struct LoopExpr {
    pub loop_start: LoopStartExpr,
    pub block: Box<Expr>,
    pub loop_end: EndExpr,
}

#[derive(Clone)]
pub struct LoopStartExpr {
    pub left_mustache: InfoToken,
    pub r#loop: InfoToken,
    pub loop_variable: Option<ParenVariableParenExpr>,
    pub array_accessor: ArrayAccessorExpr,
    pub as_variable: Option<AsVariableExpr>,
    pub right_mustache: InfoToken,
}

#[derive(Clone)]
pub struct ParenVariableParenExpr {
    pub left_paren: InfoToken,
    pub variable: InfoToken,
    pub second_variable: Option<CommaVariableExpr>,
    pub right_paren: InfoToken,
}

#[derive(Clone)]
pub struct CommaVariableExpr {
    pub comma: InfoToken,
    pub variable: InfoToken,
}

#[derive(Clone)]
pub struct AsVariableExpr {
    pub r#as: InfoToken,
    pub variable: InfoToken,
}

#[derive(Clone)]
pub struct EndExpr {
    pub left_mustache: InfoToken,
    pub end: InfoToken,
    pub right_mustache: InfoToken,
}

#[derive(Clone)]
pub struct BlockExpr {
    pub imports: Vec<ImportExpr>,
    pub blocks: Vec<Expr>,
}

#[derive(Clone)]
pub struct ImportExpr {
    pub left_mustache: InfoToken,
    pub config: InfoToken,
    pub right_mustache: InfoToken,
    pub configs: Vec<ImportConfigOptionExpr>,
    pub end: EndExpr,
}

impl ImportExpr {
    pub fn value_of(&self, val: &str) -> Option<String> {
        for config in &self.configs {
            if config.variable.slice.trim() == val.trim() {
                return Some(config.value.slice.clone());
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct ImportConfigOptionExpr {
    pub variable: InfoToken,
    pub colon: InfoToken,
    pub value: InfoToken,
    pub semicolon: InfoToken,
}

#[derive(Clone)]
pub struct AnythingExpr {
    pub tokens: Vec<InfoToken>,
}

#[derive(Clone)]
pub struct MustacheAccessorExpr {
    pub left_mustache: InfoToken,
    pub accessor: AccessorExpr,
    pub right_mustache: InfoToken,
}

#[derive(Clone)]
pub struct ArrayAccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<ArrayBracketExpr>,
    pub array_slice: Option<ArraySliceExpr>,
}

#[derive(Clone)]
pub struct AccessorExpr {
    pub variable: InfoToken,
    pub indexes: Vec<ArrayBracketExpr>,
}

#[derive(Clone)]
pub struct DotVariableExpr {
    pub dot: InfoToken,
    pub variable: InfoToken,
}

#[derive(Clone)]
pub struct ArraySliceExpr {
    pub left_paren: InfoToken,
    pub start_index: ArraySliceIndexExpr,
    pub comma: InfoToken,
    pub end_index: ArraySliceIndexExpr,
    pub right_paren: InfoToken,
}

#[derive(Clone)]
pub struct ArrayBracketExpr {
    pub left_paren: InfoToken,
    pub variable: ArrayBracketIndexExpr,
    pub right_paren: InfoToken,
}

#[derive(Clone)]
pub struct ArraySliceIndexExpr {
    pub token: InfoToken,
}

#[derive(Clone)]
pub struct ArrayBracketIndexExpr {
    pub token: InfoToken,
}
