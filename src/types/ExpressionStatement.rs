use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::types;
use types::BinaryTree::BinaryExpression;
use types::CallExpression::CallExpression;
use types::ExpressionType::ExpressionType;


#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    type_of: String,
    start: usize,
    end: usize,
    expression: ExpressionType,
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
#[test]
fn test_check_if_valid_expression_identifier() {
    let string = ExpressionStatement::check_expression_type("x").unwrap();
    assert_eq!(string, "identifier");
}
#[test]
fn test_check_if_valid_expression_literal() {
    let string = ExpressionStatement::check_expression_type("\"x\"").unwrap();
    assert_eq!(string, "literal");
}

impl ExpressionStatement {
 
 

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
}

impl ExpressionTypes for ExpressionStatement {}
