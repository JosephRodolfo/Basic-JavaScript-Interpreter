use crate::{interpreter_types::VarsEnum::VarsEnum};
use crate::types;
use types::{VariableInitTypes::VariableInitTypes, Identifier::Identifier, Literal::Literal, ArrayExpression::ArrayExpression};
use crate::traits;
use traits::{
    ExpressionTypes::ExpressionTypes
};
#[derive(PartialEq, Debug, Clone)]
pub struct Vars {
    pub value: VarsEnum,
    pub kind: String,
}


