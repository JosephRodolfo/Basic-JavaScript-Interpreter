use std::arch::x86_64::_mm_stream_pd;

use crate::{
    helper_funcs::{rem_first_and_last, str_to_type},
    types, Identifier,
};
use regex::Regex;
use substring::Substring;
use types::Literal::Literal;
#[derive(PartialEq, Debug, Clone)]
enum BinaryExpressionOptions {
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(Box<BinaryExpression>),
    None(String),
}
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    type_of: String,
    start: usize,
    end: usize,
    left: BinaryExpressionOptions,
    operator: String,
    right: BinaryExpressionOptions,
}

impl BinaryExpression {
    fn loop_through_operators(string: &str) -> BinaryExpression {
        let mut temp_string = string;
        let mut new_binary_expression = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: temp_string.len(),
            left: BinaryExpressionOptions::None("placeholder".to_string()),
            operator: "".to_string(),
            right: BinaryExpressionOptions::None("placeholder".to_string()),
        };
        let mut mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .find(temp_string)
            .expect("no operators found");

        let current = temp_string.substring(0, mat.start());
        let operator = temp_string.substring(mat.start(), mat.end());

        let parens_present = BinaryExpression::check_for_parens(current);

        let (left_result, mut temp_string) = if !parens_present {
            let left_result = BinaryExpression::create_node(current);
            (left_result, temp_string)
        } else {
            let find_right_parens = Regex::new("(\\))")
                .unwrap()
                .find(temp_string)
                .expect("did not find right parenetheses");
            let removed = temp_string.substring(0, find_right_parens.end());
            let new_string = rem_first_and_last(removed);

            let new_op_exists = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
                .unwrap()
                .is_match(temp_string);

            mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
                .unwrap()
                .find(temp_string)
                .expect("no operators found");
            temp_string = temp_string.substring(mat.end(), temp_string.len());

            (
                BinaryExpressionOptions::BinaryExpression(Box::new(
                    BinaryExpression::loop_through_operators(new_string),
                )),
                temp_string,
            )
        };

        new_binary_expression.left = left_result;

        new_binary_expression.operator = operator.to_string();

        let text_for_next_op = temp_string.substring(mat.end(), temp_string.len());
        let mut next_mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .is_match(text_for_next_op);
        println!("NEXT{}", next_mat);

        let binary_tree = if !next_mat {
            let next = text_for_next_op.substring(0, text_for_next_op.len());

            new_binary_expression.right = BinaryExpression::create_node(next);
            new_binary_expression
        } else {
            let parens_present = BinaryExpression::check_for_parens(current);

            if parens_present {
                let find_right_parens = Regex::new("(\\))")
                    .unwrap()
                    .find(temp_string)
                    .expect("did not find right parenetheses");
                let removed = temp_string.substring(0, find_right_parens.end());
                let new_string = rem_first_and_last(removed);
                let result = BinaryExpressionOptions::BinaryExpression(Box::new(
                    BinaryExpression::create_node_binary(new_string),
                ));
                new_binary_expression.right = result;
            } else {
                temp_string = temp_string.substring(mat.end(), temp_string.len());
                new_binary_expression.right = BinaryExpressionOptions::BinaryExpression(Box::new(
                    BinaryExpression::create_node_binary(temp_string),
                ));
            }
            new_binary_expression
        };
        binary_tree
    }

    fn create_node_binary(expression_string: &str) -> BinaryExpression {
        let temp_string = if BinaryExpression::check_for_parens(expression_string) {
            let result = rem_first_and_last(expression_string);

            result
        } else {
            expression_string
        };

        let mut new_binary_expression = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: temp_string.len(),
            left: BinaryExpressionOptions::None("placeholder".to_string()),
            operator: "".to_string(),
            right: BinaryExpressionOptions::None("placeholder".to_string()),
        };
        let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .find(temp_string)
            .expect("no operators found");

        let current = temp_string.substring(0, mat.start());
        let operator = temp_string.substring(mat.start(), mat.end());
        let left_result = BinaryExpression::create_node(current);

        new_binary_expression.left = left_result;

        new_binary_expression.operator = operator.to_string();

        let text_for_next_op = temp_string.substring(mat.end(), temp_string.len());
        let next_mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .is_match(text_for_next_op);
        println!("NEXT{}", next_mat);

        let binary_tree = if !next_mat {
            let next = text_for_next_op.substring(0, text_for_next_op.len());

            new_binary_expression.right = BinaryExpression::create_node(next);
            new_binary_expression
        } else {
            new_binary_expression.right = BinaryExpressionOptions::BinaryExpression(Box::new(
                BinaryExpression::loop_through_operators(temp_string),
            ));
            new_binary_expression
        };
        binary_tree
    }

    fn create_node(current: &str) -> BinaryExpressionOptions {
        let type_of = BinaryExpression::str_to_type_inc_parentheses(current);
        let result = match type_of {
            "left_parens" => {
                let find_right_parens = Regex::new("(\\))")
                    .unwrap()
                    .find(current)
                    .expect("did not find right parenetheses");
                let new_string = rem_first_and_last(current);
                let result = BinaryExpression::loop_through_operators(new_string);
                BinaryExpressionOptions::BinaryExpression(Box::new(result))
            }

            "identifier" => {
                let new_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: current.to_string(),
                };

                BinaryExpressionOptions::Identifier(new_identifier)
            }
            "literal" => {
                let new_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: current.to_string(),
                };
                BinaryExpressionOptions::Literal(new_literal)
            }
            _ => BinaryExpressionOptions::None("End".to_string()),
        };
        result
    }

    fn check_for_parens(string: &str) -> bool {
        let left_paren = "(".chars().next().unwrap();
        let c = string.chars().next();

        let result = match c {
            Some(c) => {
                if c == left_paren {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        result
    }

    fn str_to_type_inc_parentheses(string: &str) -> &str {
    
        let result = str_to_type(string);

        let type_match = match result {
            Ok("lookup") => "identifier",
            Ok("bool") => "literal",
            Ok("string") => "literal",
            Ok("number") => "literal",
            _ => "Malformed!",
        };
        // type_match
        // };
        type_match
    }
}

#[test]
fn test_create_binary_tree_three_items_parentheses_last() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Literal(new_literal),
    };

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 7,
        type_of: "BinaryExpression".to_string(),
        left: BinaryExpressionOptions::Identifier(cloned_identifier),
        right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        operator: "+".to_string(),
    };
    let printed = BinaryExpression::loop_through_operators("x+(z+y)+a+b+c");
    println!("{:#?}", printed);
    assert_eq!(
        binary_expression_test,
        BinaryExpression::loop_through_operators("x+(x+3)")
    );
}

#[test]
fn test_create_binary_tree_three_items_parentheses_first() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Identifier(cloned_identifier),
    };

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 7,
        type_of: "BinaryExpression".to_string(),
        left: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        right: BinaryExpressionOptions::Literal(new_literal),
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::loop_through_operators("(x+x)+3")
    );
}

#[test]
fn test_create_binary_tree_three_items_mixed() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Literal(new_literal),
    };

    let identifier = BinaryExpressionOptions::Identifier(cloned_identifier);

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 5,
        type_of: "BinaryExpression".to_string(),
        left: identifier,
        right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::loop_through_operators("x+x+3")
    );
}

#[test]
fn test_create_binary_tree_two_literals() {
    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "2".to_string(),
    };
    let literal = BinaryExpressionOptions::Literal(new_literal);
    let cloned_lit = literal.clone();
    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 3,
        type_of: "BinaryExpression".to_string(),
        left: literal,
        right: cloned_lit,
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::loop_through_operators("2+2")
    );
}
#[test]
fn test_create_binary_tree_two_identifiers() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };
    let identifier = BinaryExpressionOptions::Identifier(new_identifier);
    let cloned_id = identifier.clone();
    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 3,
        type_of: "BinaryExpression".to_string(),
        left: identifier,
        right: cloned_id,
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::loop_through_operators("x+x")
    );
}

#[test]
fn test_str_to_type_incuding_parens_string() {
    let string = BinaryExpression::str_to_type_inc_parentheses("\"dogs\"");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_numeric() {
    let string = BinaryExpression::str_to_type_inc_parentheses("4");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_identifier() {
    let string = BinaryExpression::str_to_type_inc_parentheses("de4");
    assert_eq!(string, "identifier");
}
#[test]
fn test_str_to_type_incuding_parens_malformed() {
    let string = BinaryExpression::str_to_type_inc_parentheses("\"de4+3");
    assert_eq!(string, "Malformed!");
}
