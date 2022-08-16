use crate::{types};
use types::{BinaryTree::BinaryExpression, Literal::Literal, Identifier::Identifier, ArrayExpression::ArrayExpression};




#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitTypes {
    Identifier(Identifier),
    Literal(Literal),
    ArrayExpression(ArrayExpression),
    BinaryExpression(BinaryExpression)
}

impl VariableInitTypes {


    
}


