pub enum VarType {
    Value(String),
    Row(Vec<String>),
    Table(Vec<Vec<String>>),
}