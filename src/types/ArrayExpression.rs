use std::array;

use crate::traits::CommaSeperatedList::CommaSeperatedList;
use crate::{traits::ExpressionTypes::ExpressionTypes, types};
use types::{ExpressionStatement::ExpressionStatement, ExpressionType::ExpressionType, Literal::Literal};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpression {
    type_of: String,
    start: usize,
    end: usize,
    elements: Vec<ExpressionType>,
}

impl ArrayExpression {
    pub fn create_array_expression(string: &str)->ArrayExpression {
        let vec = ArrayExpression::create_string_vec(string);
        let result_vec = ArrayExpression::create_comma_seperated_array(vec);
        let elements_vec = match result_vec {
            Ok(result_vec) => result_vec,
            Err(_e) => panic!("Error with array expression"),
        };

        let new_array_expression = ArrayExpression {
            type_of: "ArrayExpression".to_string(),
            start: 0,
            end: 0,
            elements: elements_vec,
        };
        new_array_expression
    }
}

impl CommaSeperatedList<ExpressionType> for ArrayExpression {
    fn create_comma_seperated_array(string_vec: Vec<&str>) -> Result<Vec<ExpressionType>, String> {
        let result = string_vec
            .iter()
            .map(|e| {
                let expression_type = ExpressionType::check_expression_type(e);
                ExpressionStatement::create_expression_statement(expression_type, e)
            })
            .collect::<Vec<ExpressionType>>();
        Ok(result)
    }
}

#[test]
fn test_create_array_expression_literals() {

    let one = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "2".to_string()
    };let two = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };let three = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "4".to_string(),
    };let four = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "5".to_string(),
    };
    let vec = vec![one, two, three, four].iter().map(|e|ExpressionType::Literal(e.to_owned())).collect();

    let test_array = ArrayExpression {
        type_of: "ArrayExpression".to_string(),
        start: 0,
        end: 0,
        elements: vec,
    };
let array = ArrayExpression::create_array_expression("2,3,4,5");
    assert_eq!(array, test_array);
}
