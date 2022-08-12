use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::types;
use types::ArrayExpression::ArrayExpression;
use types::{
    BinaryTree::BinaryExpression, CallExpression::CallExpression, ExpressionType::ExpressionType,
    Identifier::Identifier, Literal::Literal, UpdateExpression::UpdateExpression,
};
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    type_of: String,
    start: usize,
    end: usize,
    expression: ExpressionType,
}

impl ExpressionStatement {

    // pub fn create_full_expression_statement_(type_of_test: Re){
    //     ExpressionStatement::create_expression_statement(type_of_test, expression_string)
    // }
    pub fn create_expression_statement(
        type_of_test: Result<&str, &str>,
        expression_string: &str,
    ) -> ExpressionType {
        let expression: ExpressionType = match type_of_test {
            Ok("call_expression") => ExpressionType::CallExpression(
                CallExpression::create_generic_expression(expression_string),
            ),
            Ok("update_expression") => ExpressionType::UpdateExpression(
                UpdateExpression::create_update_expression(expression_string),
            ),
            Ok("binary_expression") => ExpressionType::BinaryExpression(
                BinaryExpression::create_generic_expression(expression_string),
            ),
            Ok("array_expression") => ExpressionType::ArrayExpression(
                ArrayExpression::create_array_expression(expression_string),
            ),
            Ok("literal") => {
                let new_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: expression_string.to_string(),
                };
                ExpressionType::Literal(new_literal)
            }
            Ok("identifier") => {
                let new_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: expression_string.to_string(),
                };
                ExpressionType::Identifier(new_identifier)
            }
            _ => {
                panic!("Error",)
            }
        };
        
        expression
    }
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
    pub fn create_update_expression(string: &str) -> ExpressionStatement {
        let result = UpdateExpression::create_update_expression(string);
        let new_expression_statement = ExpressionStatement { 
            type_of: "ExpressionStatement".to_string(),
            start: 0,
            end: 0,
            expression: ExpressionType::UpdateExpression(result),
        };
        new_expression_statement
    }

}

impl ExpressionTypes for ExpressionStatement {}

#[cfg(test)]
mod test {
    use crate::traits::ExpressionTypes::ExpressionTypes;
    use crate::types;
    use types::{
        ExpressionStatement::ExpressionStatement
    };
    
    #[test]
    fn test_check_if_valid_expression_starts_operator() {
        let string = ExpressionStatement::check_expression_type("+-=9999992");
        assert_eq!(string, Err("unrecognized_expression"));
    }
    #[test]
    fn test_check_if_valid_expression_starts_literal() {
        let string = ExpressionStatement::check_expression_type("0+2").unwrap();
        assert_eq!(string, "binary_expression");
    }
    #[test]
    fn test_check_if_valid_expression_ends_single_operator() {
        let string = ExpressionStatement::check_expression_type("2===999999999-?");
        assert_eq!(string, Err("unrecognized_expression"));
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
}
