use crate::helper_funcs::{rem_first_and_last, str_to_type};
use crate::{types, Identifier};
use types::BinaryTree::BinaryExpression;
use types::CallExpression::CallExpression;

use regex::Regex;
use substring::Substring;
use types::Literal::Literal;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    type_of: String,
    start: usize,
    end: usize,
    expression: ExpressionType,
}

#[derive(PartialEq, Debug, Clone)]
enum ExpressionType {
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
}

#[test]
fn test_check_if_valid_expression_starts_operator() {
    let string = ExpressionStatement::check_expression_type("+-=9999992");
    assert_eq!(string, Err("unrecognized expression"));
}
#[test]
fn test_check_if_valid_expression_starts_literal() {
    let string = ExpressionStatement::check_expression_type("0+2").unwrap();
    assert_eq!(string, "binary_expression");
}
#[test]
fn test_check_if_valid_expression_ends_single_operator() {
    let string = ExpressionStatement::check_expression_type("2===999999999-?");
    assert_eq!(string, Err("unrecognized expression"));
}

#[test]
fn test_check_if_valid_expression_ends_call_expression_no_params() {
    let string = ExpressionStatement::check_expression_type("testFunc()").unwrap();
    assert_eq!(string, "call_expression");
}
#[test]
fn test_check_if_valid_expression_ends_call_expression_params() {
    let string = ExpressionStatement::check_expression_type("testFunc(x, y, z)").unwrap();
    assert_eq!(string, "call_expression");
}

impl ExpressionStatement {
    //some validation, returns none if an operator is first or last character ( currently ++ at the end will break it);
    //if it ends in (____) returns call expression. At the moment elsewise it's returning binary expression.
    //will need to add much more robust validation later;
    pub fn check_expression_type(string: &str) -> Result<&str, &str> {
        //
        let operators = "([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)";
        let mat = Regex::new("^([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .is_match(&string);
        println!("{:?}", mat);
        if mat {
            return Err("unrecognized expression");
        }
        let match_end = format!("{}$", operators);
        let match_end_match = Regex::new(&match_end).unwrap().is_match(&string);
        if match_end_match {
            return Err("unrecognized expression");
        }
        let call_expression_regex = "(\\(.*\\))$";
        let match_call_expression = Regex::new(&call_expression_regex)
            .unwrap()
            .is_match(&string);
        if match_call_expression {
            return Ok("call_expression");
        }

        Ok("binary_expression")
    }

    pub fn create_binary_expression(string: &str) -> ExpressionStatement {
        let result = BinaryExpression::create_generic_expression(string);
        let new_expression_statement = ExpressionStatement {
            type_of: "ExpressionStatement".to_string(),
            start: 0,
            end: 0,
            expression: ExpressionType::BinaryExpression(result),
        };
        new_expression_statement
    }

    pub fn create_call_expression(string: &str) -> ExpressionStatement {
        let result = CallExpression::create_generic_expression(string);
        let new_expression_statement = ExpressionStatement {
            type_of: "ExpressionStatement".to_string(),
            start: 0,
            end: 0,
            expression: ExpressionType::CallExpression(result),
        };
        new_expression_statement
    }

    //creates a new call expression object
    // pub fn create_call_expression(program: &str) -> CallExpression {

    // }
}
