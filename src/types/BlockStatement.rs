use crate::Body;

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub body: Body
}