use crate::{
    types, Identifier
};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub identifier: Identifier,
    pub kind: String,
    pub init: types::VariableInitTypes::VariableInitTypes,
}