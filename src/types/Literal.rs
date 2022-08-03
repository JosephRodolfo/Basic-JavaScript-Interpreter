

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub value: String,
}