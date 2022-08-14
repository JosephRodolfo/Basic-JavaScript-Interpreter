use crate::{traits::ExpressionTypes::ExpressionTypes, types};
use regex::Regex;
use substring::Substring;
use types::ExpressionType::ExpressionType;
use types::ExpressionStatement::ExpressionStatement;
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    type_of: String,
    start: usize,
    end: usize,
    argument: ExpressionType,
}

impl ReturnStatement {
    pub fn create_return_statement(string: &str)->ReturnStatement {
        let mat = Regex::new("^(return)")
            .unwrap()
            .find(string)
            .expect("no return statement found");
        let return_argument = string.substring(mat.end(), string.len());

        let expression_type = ExpressionType::check_expression_type(return_argument);
        let expression = ExpressionStatement::create_expression_statement(expression_type, return_argument);

        let new_return_statement = ReturnStatement {
            type_of: "ReturnStatement".to_string(),
            start: 0,
            end: 0,
            argument: expression,
        };
        new_return_statement
    }
}

#[cfg(test)]
mod test {
    use crate::types;
    use types::{
        ExpressionType::ExpressionType, Identifier::Identifier, ReturnStatement::ReturnStatement, ExpressionStatement::ExpressionStatement
    };


#[test]
fn test_create_return_statement_identifier() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };
 
    let test_return_statement = ReturnStatement {
        type_of: "ReturnStatement".to_string(),
        start: 0,
        end: 0,
        argument: ExpressionType::Identifier(new_identifier)
    };
    let return_statement: ReturnStatement = ReturnStatement::create_return_statement("returnx");
    assert_eq!(return_statement, test_return_statement);
}
#[test]
fn test_create_return_statement_binary_expression(){
    let test_expression_statement = ExpressionStatement::create_expression_statement(Ok("binary_expression"), "3==2");
     
    let test_return_statement = ReturnStatement {
        type_of: "ReturnStatement".to_string(),
        start: 0,
        end: 0,
        argument: test_expression_statement
    };
    let return_statement: ReturnStatement = ReturnStatement::create_return_statement("return3==2");
    assert_eq!(return_statement, test_return_statement);
}
}