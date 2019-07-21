#[derive(Clone)]
pub enum VarType {
    Number(Var<usize>),
    Value(Var<String>),
    Row(Var<Vec<String>>),
    Table(Var<Vec<Vec<String>>>),
}

#[derive(Clone)]
pub struct Var<T> {
    pub row: usize,
    pub col: usize,
    pub data: T,
}

impl<T> Var<T> {
    pub fn new(data: T) -> Var<T> {
        Var {
            row: 0,
            col: 0,
            data,
        }
    }
}
