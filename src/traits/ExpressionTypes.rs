use regex::Regex;

use crate::helper_funcs::str_to_type_inc_parentheses;

pub trait ExpressionTypes {
        fn check_expression_type(string: &str) -> Result<&str, &str> {
            //
            let operators = "([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)";
            let mat = Regex::new("^([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
                .unwrap()
                .is_match(string);
            println!("{:?}", mat);
            if mat {
                return Err("unrecognized expression");
            }
            let match_end = format!("{}$", operators);
            let match_end_match = Regex::new(&match_end).unwrap().is_match(&string);
            if match_end_match {
                return Err("unrecognized expression");
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

