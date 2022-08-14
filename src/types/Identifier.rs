#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub name: String,
}
impl Identifier {
    pub fn evaluate(&self) -> &str {
        &self.name
    }
}
