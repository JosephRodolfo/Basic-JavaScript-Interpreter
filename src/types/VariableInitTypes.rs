use crate::{types};
use types::{Literal::Literal, Identifier::Identifier};


#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitTypes {
    Identifier(Identifier),
    Literal(Literal)
}

impl VariableInitTypes {


    
}


