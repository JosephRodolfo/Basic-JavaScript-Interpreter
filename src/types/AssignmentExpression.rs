use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::types;
use regex::Regex;
use substring::Substring;
use types::{ExpressionStatement::ExpressionStatement, ExpressionType::ExpressionType};

#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentExpression {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub left: Box<ExpressionType>,
    pub operator: String,
    pub right: Box<ExpressionType>,
}

impl AssignmentExpression {
    pub fn create_assignment_expression(string: &str) -> AssignmentExpression {
        let mat = Regex::new("(=|\\+=|\\-=)").unwrap().find(string).unwrap();

        let left = string.substring(0, mat.start());
        let operator = string.substring(mat.start(), mat.end());
        let right = string.substring(mat.end(), string.len());
        let expression_types_vec: Vec<ExpressionType> = vec![left, right]
            .iter()
            .map(|e| {
                let expression_type = ExpressionType::check_expression_type(e);
                ExpressionStatement::create_expression_statement(expression_type, e)
            })
            .collect();

        let new_assignment_expression = AssignmentExpression {
            type_of: "AssignmentExpression".to_string(),
            start: 0,
            end: 0,
            left: Box::new(expression_types_vec[0].clone()),
            right: Box::new(expression_types_vec[1].clone()),
            operator: operator.to_string(),
        };
        new_assignment_expression
    }
}

#[cfg(test)]
mod test {
    use crate::types;
    use types::{
        AssignmentExpression::AssignmentExpression, ExpressionType::ExpressionType,
        Identifier::Identifier, Literal::Literal,
    };
    #[test]
    fn test_create_assignment_expression_identifiers() {
        let test_assignment_expression = AssignmentExpression::create_assignment_expression("a=b");
        let left = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "a".to_string(),
        };
        let right = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "b".to_string(),
        };
        let new_assignment_expression = AssignmentExpression {
            start: 0,
            end: 0,
            type_of: "AssignmentExpression".to_string(),
            left: Box::new(ExpressionType::Identifier(left)),
            right: Box::new(ExpressionType::Identifier(right)),
            operator: "=".to_string(),
        };

        assert_eq!(test_assignment_expression, new_assignment_expression)
    }
    #[test]
    fn test_create_assignment_expression_identifiers_literals_plus_equal() {
        let test_assignment_expression = AssignmentExpression::create_assignment_expression("a+=1");
        let left = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "a".to_string(),
        };
        let right = Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "1".to_string(),
        };
        let new_assignment_expression = AssignmentExpression {
            start: 0,
            end: 0,
            type_of: "AssignmentExpression".to_string(),
            left: Box::new(ExpressionType::Identifier(left)),
            right: Box::new(ExpressionType::Literal(right)),
            operator: "+=".to_string(),
        };

        assert_eq!(test_assignment_expression, new_assignment_expression)
    }
}
