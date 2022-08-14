use crate::{
    helper_funcs::{rem_first_and_last, str_to_type},
    types};
use regex::Regex;
use substring::Substring;
use types::{Literal::Literal, Identifier::Identifier, VariableInitTypes::VariableInitTypes};


#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentExpression {
    type_of: String,
    start: usize,
    end: usize,
    left: VariableInitTypes,
    operator: String,
    right: VariableInitTypes,
}


impl AssignmentExpression {
   pub fn create_assignment_expression(string: &str) {

        let mat = Regex::new("(=|\\+=|\\-=)")
        .unwrap()
        .find(string);
    }
}