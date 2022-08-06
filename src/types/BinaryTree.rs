use crate::{
    helper_funcs::{find_start_end, str_to_type},
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
//10 + x + 9
fn loop_through_operators(string: &str) -> BinaryExpression {
    let temp_string = string;

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
    let mut counter = 0;
    let mut current = temp_string.substring(0, mat.start());
    let mut operator = temp_string.substring(mat.start(), mat.end());
    let mut next = temp_string.substring(mat.end(), temp_string.len());
    let type_of = self::str_to_type_inc_parentheses(current);

    let result = match type_of {
        "left_parens" => {
            let new_string = temp_string.substring(1, temp_string.len());
            let result = loop_through_operators(new_string);
            BinaryExpressionOptions::BinaryExpression(Box::new(result))
        }

        "right_parens" => BinaryExpressionOptions::None("End".to_string()),
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

    new_binary_expression.left = result;
    new_binary_expression.operator = operator.to_string();
// println!("{:?}", new_binary_expression);
    let temp_string = temp_string.substring(mat.end(), temp_string.len());

    let find_match = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
        .unwrap()
        .is_match(temp_string);
        // println!("{:?}", find_match);

    let binary_tree = if !find_match {
        // let find_end = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
        //     .unwrap()
        //     .find(temp_string)
        //     .expect("no match foudn");
        new_binary_expression.right = self::create_node(
            temp_string.substring(0, temp_string.len()),
            next
        );
        new_binary_expression
    } else {
        new_binary_expression.right = BinaryExpressionOptions::BinaryExpression(Box::new(
            self::loop_through_operators(temp_string),
        ));
        new_binary_expression
    };
    binary_tree
}

fn create_node(string: &str, current: &str) -> BinaryExpressionOptions {
    println!("string: {}, current: {}", string, current);
    let type_of = str_to_type_inc_parentheses(current);
    let result = match type_of {
        "left_parens" => {
            let new_string = string.substring(1, string.len());
            let result = loop_through_operators(new_string);
            BinaryExpressionOptions::BinaryExpression(Box::new(result))
        }

        "right_parens" => BinaryExpressionOptions::None("End".to_string()),
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
fn str_to_type_inc_parentheses(string: &str) -> &str {
    let left_paren = "(".chars().next().unwrap();
    let right_paren = ")".chars().next().unwrap();

    let c = string.chars().next().unwrap();
    let last_c = string.chars().last().unwrap();
    let result = if c == left_paren {
        "left_parens"
    } else if last_c == right_paren {
        "right_parens"
    } else {
        let result = str_to_type(string);

        let type_match = match result {
            Ok("lookup") => "identifier",
            Ok("bool") => "literal",
            Ok("string") => "literal",
            Ok("number") => "literal",
            _ => "Malformed!",
        };
        type_match
    };
    result
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
        operator: "+".to_string()
    };
    assert_eq!(binary_expression_test, loop_through_operators("2+2"));
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
        operator: "+".to_string()
    };
    assert_eq!(binary_expression_test, loop_through_operators("x+x"));
}





#[test]
fn test_str_to_type_incuding_parens_string() {
    let string = str_to_type_inc_parentheses("\"dogs\"");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_numeric() {
    let string = str_to_type_inc_parentheses("4");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_identifier() {
    let string = str_to_type_inc_parentheses("de4");
    assert_eq!(string, "identifier");
}
#[test]
fn test_str_to_type_incuding_parens_parens() {
    let string = str_to_type_inc_parentheses("(de4+3)");
    assert_eq!(string, "left_parens");
}
#[test]
fn test_str_to_type_incuding_parens_right() {
    let string = str_to_type_inc_parentheses("de4+3)");
    assert_eq!(string, "right_parens");
}
#[test]
fn test_str_to_type_incuding_parens_malformed() {
    let string = str_to_type_inc_parentheses("\"de4+3");
    assert_eq!(string, "Malformed!");
}
