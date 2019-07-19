pub enum VarType {
    Number(usize),
    Value(String),
    Row(Vec<String>),
    Table(Vec<Vec<String>>),
}
