use crate::helper_funcs::rem_first_and_last;
use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::{types, Body, Identifier};
use core::panic;
use regex::Regex;
use substring::Substring;
use types::BinaryTree::BinaryExpression;
use types::BlockStatement::BlockStatement;
use types::CallExpression::CallExpression;
use types::ExpressionType::ExpressionType;
use types::Literal::Literal;
#[derive(PartialEq, Debug)]
pub struct IfStatement {
    type_of: String,
    start: usize,
    end: usize,
    test: ExpressionType,
    consequent: BlockStatement,
    alternate: Option<BlockStatement>,
}

impl IfStatement {
    //should be taking str confirmed to start with "if("
    fn create_if_statement(string: &str) -> IfStatement {
        let mat = Regex::new("^(if\\(.*\\))").unwrap().is_match(string);

        if !mat {
            panic!("Malformed if statement!")
        };

        let test_position = Regex::new("(\\(.*\\))")
            .unwrap()
            .find(string)
            .expect("no if statement test found!");

        let test_information = string.substring(test_position.start(), test_position.end());
        let parens_removed_test = rem_first_and_last(test_information);
        let type_of_test = ExpressionType::check_expression_type(parens_removed_test);

        let expression: ExpressionType = match type_of_test {
            Ok("call_expression") => ExpressionType::CallExpression(
                CallExpression::create_generic_expression(parens_removed_test),
            ),
            Ok("binary_expression") => ExpressionType::BinaryExpression(
                BinaryExpression::create_generic_expression(parens_removed_test),
            ),
            Ok("literal") => {
                let new_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: parens_removed_test.to_string(),
                };
                ExpressionType::Literal(new_literal)
            }
            Ok("identifier") => {
                let new_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: parens_removed_test.to_string(),
                };
                ExpressionType::Identifier(new_identifier)
            }
            _ => {
                panic!("Error",)
            }
        };

        let new_consequent_block_statement = BlockStatement {
            type_of: "BlockStatement".to_string(),
            start: 0,
            end: 0,
            body: Body::default(),
        };

        let new_if_statement = IfStatement {
            type_of: "IfStatement".to_string(),
            start: 0,
            end: 0,
            test: expression,
            alternate: None,

            consequent: new_consequent_block_statement,
        };
        new_if_statement
    }
}

#[test]
fn test_create_if_statement() {
    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "2".to_string(),
    };
    let new_consequent_block_statement = BlockStatement {
        type_of: "BlockStatement".to_string(),
        start: 0,
        end: 0,
        body: Body::default(),
    };
    let test_if_statement = IfStatement {
        type_of: "IfStatement".to_string(),
        start: 0,
        end: 0,
        test: ExpressionType::Literal(new_literal),
        alternate: None,

        consequent: new_consequent_block_statement,
    };
    let string: IfStatement = IfStatement::create_if_statement("if(2){return x}");
    println!("{:#?}", string);
    assert_eq!(string, test_if_statement);
}
