use crate::types;
use crate::traits::ExpressionTypes::ExpressionTypes;
use types::Literal::Literal;
use types::Identifier::Identifier;
use types::{ArrayExpression::ArrayExpression};


use super::{BinaryTree::BinaryExpression, CallExpression::CallExpression};

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType {
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    Literal(Literal),
    Identifier(Identifier),
    ArrayExpression(ArrayExpression)
}

impl ExpressionTypes for ExpressionType {}