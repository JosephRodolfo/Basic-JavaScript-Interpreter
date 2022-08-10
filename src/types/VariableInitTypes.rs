use crate::{types};
use types::{Literal::Literal, Identifier::Identifier, ArrayExpression::ArrayExpression};


#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitTypes {
    Identifier(Identifier),
    Literal(Literal),
    ArrayExpression(ArrayExpression)
}

impl VariableInitTypes {


    
}


