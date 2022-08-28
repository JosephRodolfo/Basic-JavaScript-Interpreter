use crate::{types, traits};
use types::{BinaryTree::BinaryExpression, Literal::Literal, Identifier::Identifier, ArrayExpression::ArrayExpression, FunctionDeclaration::FunctionDeclaration};
use traits::{ExpressionTypes::ExpressionTypes};





#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitTypes {
    Identifier(Identifier),
    Literal(Literal),
    ArrayExpression(ArrayExpression),
    BinaryExpression(BinaryExpression),
    FunctionExpression(FunctionDeclaration)
}

impl ExpressionTypes for VariableInitTypes {}


