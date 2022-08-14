use regex::Regex;

use crate::helper_funcs::str_to_type_inc_parentheses;

pub trait ExpressionTypes {
    fn check_expression_type(string: &str) -> Result<&str, &str> {
        let operators = "([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)";
        let mat = Regex::new("^([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .is_match(string);
        let match_update_expression_end = Regex::new("((--|\\+\\+)$)").unwrap().is_match(string);
        let match_update_expression_start = Regex::new("(^(--|\\+\\+))").unwrap().is_match(string);
        if (match_update_expression_end && !match_update_expression_start)
            || (!match_update_expression_end && match_update_expression_start)
        {
            return Ok("update_expression");
        }

        if mat {
            return Err("unrecognized_expression");
        }

        let assignment_match = Regex::new("^[^=]*(=|\\+=|-=)[^=]*$")
            .unwrap()
            .is_match(string);
        if assignment_match {
            return Ok("assignment_expression");

        }
        let match_end = format!("{}$", operators);
        let match_end_match = Regex::new(&match_end).unwrap().is_match(&string);
        if match_end_match {
            return Err("unrecognized_expression");
        }
        let array_expression_regex =  "^\\[.*\\]$";
        let match_array_expression = Regex::new(&array_expression_regex)
            .unwrap()
            .is_match(string);
        if match_array_expression {
            return Ok("array_expression");
        }

        let call_expression_regex = "(\\(.*\\))$";
        let match_call_expression = Regex::new(&call_expression_regex).unwrap().is_match(string);
        if match_call_expression {
            return Ok("call_expression");
        }

        let mat_operator = Regex::new(operators).unwrap().is_match(string);
        if !mat_operator {
            let result = str_to_type_inc_parentheses(string);
            let lit_id: &str = match result {
                "literal" => "literal",
                "identifier" => "identifier",
                _ => panic!("Error"),
            };
            return Ok(lit_id);
        }

        Ok("binary_expression")
    }
}
#[cfg(test)]
mod test {
    use crate::traits::ExpressionTypes::ExpressionTypes;
    use crate::types;
    use types::ExpressionType::ExpressionType;
    #[test]
    fn test_check_array_expression() {
        let array = ExpressionType::check_expression_type("[2,3,4,5]");
        assert_eq!(array.unwrap(), "array_expression");
    }

    #[test]
    fn test_update_expression_prefix() {
        let array = ExpressionType::check_expression_type("--a");
        assert_eq!(array.unwrap(), "update_expression");
    }
    #[test]
    fn test_update_expression_common_for_statement() {
        let array = ExpressionType::check_expression_type("i++");
        assert_eq!(array.unwrap(), "update_expression");
    }
    #[test]

    fn test_update_binary_expression() {
        let array = ExpressionType::check_expression_type("0+2");
        assert_eq!(array.unwrap(), "binary_expression");
    }
    #[test]
    fn test_update_expresion_broken() {
        let array = ExpressionType::check_expression_type("++a++");
        assert_eq!(array, Err("unrecognized_expression"));
    }

    #[test]
    fn test_update_expression_not_prefix() {
        let array = ExpressionType::check_expression_type("--a");
        assert_eq!(array.unwrap(), "update_expression");
    }
    #[test]
    fn test_assignment_plus_equal() {
        let array = ExpressionType::check_expression_type("a+=8");
        assert_eq!(array.unwrap(), "assignment_expression");
    }
    #[test]
    fn test_assignment_minus_equal() {
        let array = ExpressionType::check_expression_type("a-=8");
        assert_eq!(array.unwrap(), "assignment_expression");
    }
    #[test]
    fn test_assignment_equal() {
        let array = ExpressionType::check_expression_type("a=8");
        assert_eq!(array.unwrap(), "assignment_expression");
    }
}
