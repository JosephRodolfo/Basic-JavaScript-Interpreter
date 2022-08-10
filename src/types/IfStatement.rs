use crate::helper_funcs::rem_first_and_last;
use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::{types};
use core::panic;
use regex::Regex;
use substring::Substring;
use types::ExpressionStatement::ExpressionStatement;
use types::BlockStatement::BlockStatement;
use types::Literal::Literal;
use types::ExpressionType::ExpressionType;
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
    pub fn create_if_statement(string: &str) -> IfStatement {
        println!("{}", string);

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
        
        let expression = ExpressionStatement::create_expression_statement(type_of_test, parens_removed_test);
        let new_consequent_block_statement = BlockStatement {
            type_of: "BlockStatement".to_string(),
            start: 0,
            end: 0,
            body:Vec::new()
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
        body:Vec::new()
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
