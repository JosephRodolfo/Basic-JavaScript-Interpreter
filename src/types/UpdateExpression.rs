use crate::types;
use regex::Regex;
use substring::Substring;
use types::Identifier::Identifier;
#[derive(PartialEq, Debug, Clone)]
pub struct UpdateExpression {
    prefix: bool,
    type_of: String,
    start: usize,
    end: usize,
    operator: String,
    argument: Identifier,
}

impl UpdateExpression {
    pub fn create_update_expression(string: &str) -> UpdateExpression {
        let mat = Regex::new("(\\+\\+|--)")
            .unwrap()
            .find(string)
            .expect("no update expression found");

        let operator = string.substring(mat.start(), mat.end());
        let (identifier_name, prefix_bool) = if mat.start() == 0 {
            (string.substring(mat.end(), string.len()), true)
        } else {
            (string.substring(0, mat.start()), false)
        };
        let new_identifier = Identifier {
            name: identifier_name.to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: prefix_bool,
            operator: operator.to_string(),
            argument: new_identifier,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types;
    use types::{Identifier::Identifier, UpdateExpression::UpdateExpression};

    #[test]
    fn test_update_expresssion_prefix_false() {
        let new_identifier = Identifier {
            name: "a".to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        let new_update_expression = UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: false,
            operator: "++".to_string(),
            argument: new_identifier,
        };

        let test_update_expression = UpdateExpression::create_update_expression("a++");
        assert_eq!(test_update_expression, new_update_expression);
    }
    #[test]
    fn test_update_expresssion_prefix_true() {
        let new_identifier = Identifier {
            name: "test".to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        let new_update_expression = UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: true,
            operator: "--".to_string(),
            argument: new_identifier,
        };

        let test_update_expression = UpdateExpression::create_update_expression("--test");
        assert_eq!(test_update_expression, new_update_expression);
    }
}
