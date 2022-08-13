use crate::types;
use crate::traits::ExpressionTypes::ExpressionTypes;
use types::{ArrayExpression::ArrayExpression, UpdateExpression::UpdateExpression, Identifier::Identifier, Literal::Literal};


use super::{BinaryTree::BinaryExpression, CallExpression::CallExpression};

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType {
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    Literal(Literal),
    Identifier(Identifier),
    ArrayExpression(ArrayExpression),
    UpdateExpression(UpdateExpression)
}

impl ExpressionType {
    
}

impl ExpressionTypes for ExpressionType {}