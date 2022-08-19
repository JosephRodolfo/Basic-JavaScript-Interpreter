use crate::{types, traits};
use types::{BinaryTree::BinaryExpression, Literal::Literal, Identifier::Identifier, ArrayExpression::ArrayExpression};
use traits::{ExpressionTypes::ExpressionTypes};



#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitTypes {
    Identifier(Identifier),
    Literal(Literal),
    ArrayExpression(ArrayExpression),
    BinaryExpression(BinaryExpression)
}

impl ExpressionTypes for VariableInitTypes {}


