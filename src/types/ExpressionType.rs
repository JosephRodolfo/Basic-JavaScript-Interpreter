use regex::Regex;
use crate::types;
use crate::{traits::ExpressionTypes::ExpressionTypes, helper_funcs::str_to_type_inc_parentheses};
use types::Literal::Literal;
use types::Identifier::Identifier;


use super::{BinaryTree::BinaryExpression, CallExpression::CallExpression};

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType {
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    Literal(Literal),
    Identifier(Identifier)

}

impl ExpressionTypes for ExpressionType {}