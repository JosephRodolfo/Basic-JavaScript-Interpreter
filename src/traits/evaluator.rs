use core::num;
use std::ffi::OsString;

use regex::Regex;

use crate::helper_funcs::str_to_type;

pub trait evaluator {
    fn evaluate(&self) -> &str;

    fn simple_eval(left: &str, operator: &str, right: &str) {
        let operation_type = Self::infer_math_operator_types(left, operator, right);
        match operation_type {
            InferTypes::Number => {}
            InferTypes::Bool => {}
            InferTypes::String => {}
            InferTypes::NaN => {}
        }

        unimplemented!()
    }
    //like javascript, all numbers are f64. In the future I may check if they're integers and do integer math just as something to do, but to my understanding and limited experiments this matches what one expects in javascript. I haven't dug into the rust documentation on testing float equality yet but I know crates exist for that.
    //still need to handle convering true and false boolean into 1 and 0 respectively.
    fn evaluate_number(left: &str, operator: &str, right: &str) -> String {
        let left_number = match left {
            "true" => 1 as f64,
            "false" => 0 as f64,
            _ => left.parse::<f64>().unwrap(),
        };

        let right_number = match right {
            "true" => 1 as f64,
            "false" => 0 as f64,
            _ => right.parse::<f64>().unwrap(),
        };
        println!("{}, {}", left_number, right_number);
        let number_result = match operator {
            "+" => left_number + right_number,
            "-" => left_number - right_number,
            "/" => left_number / right_number,
            "%" => left_number % right_number,
            _ => {
                panic!("{} is an incorrect operator!", operator)
            }
        };
        let result_string = format!("{}", number_result);
        result_string
    }

    fn evaluate_string(left: &str, operator: &str, right: &str) -> String {
        let left_number = left.parse::<f64>().unwrap();
        let right_number = right.parse::<f64>().unwrap();

        let number_result = match operator {
            "+" => left_number + right_number,
            "-" => left_number - right_number,
            "/" => left_number / right_number,
            "%" => left_number % right_number,
            _ => {
                panic!("{} is an incorrect operator!", operator)
            }
        };
        let result_string = format!("{}", number_result);
        result_string
    }
    fn infer_types(left: &str, operator: &str, right: &str) -> InferTypes {
        let left_type = str_to_type(left).unwrap();
        let right_type = str_to_type(right).unwrap();

        let math_or_not =
            if (operator == "+") | (operator == "-") | (operator == "/") | (operator == "%") {
                true
            } else {
                false
            };

        let inferred_type = if math_or_not {
            let return_type = Self::infer_math_operator_types(left_type, operator, right_type);
            return_type
        } else {
            InferTypes::Bool
        };
        inferred_type
    }

    fn infer_math_operator_types(left_type: &str, operator: &str, right_type: &str) -> InferTypes {
        let return_type = if (left_type == "number") && (right_type == "number") {
            InferTypes::Number
        } else if (left_type == "string") && (right_type == "string") && (operator=="+"){
            InferTypes::String
        } else if (left_type == "bool") && (right_type == "bool") {
            InferTypes::Number
        } else if (left_type != right_type) && (operator == "+") {
            InferTypes::String
        } else {
            InferTypes::NaN
        };
        return_type
    }

    fn infer_boolean_value(string: &str) -> bool {
        if (string == "true") | (string != "") | (string != "0") {
            return true;
        }

        false
    }
}
#[derive(Debug, PartialEq)]
pub enum InferTypes {
    Number,
    String,
    Bool,
    NaN,
}

#[cfg(test)]
mod test {
    use crate::traits::evaluator::{evaluator, InferTypes};
    // use evaluator::{InferTypes};
    use crate::types::BinaryTree::BinaryExpression;
    #[test]
    fn test_infer_types_two_numbers() {
        let type_of = BinaryExpression::infer_types("2", "+", "2");
        assert_eq!(type_of, InferTypes::Number);
    }
    #[test]
    fn test_infer_types_comparison() {
        let type_of = BinaryExpression::infer_types("2", "===", "2");
        assert_eq!(type_of, InferTypes::Bool);
    }
    #[test]
    fn test_infer_types_mixed_string_plus() {
        let type_of = BinaryExpression::infer_types("2", "+", "true");
        assert_eq!(type_of, InferTypes::String);
    }
    #[test]
    fn test_infer_types_mixed_string_minux_nan() {
        let type_of = BinaryExpression::infer_types("2", "-", "\"false\"");
        assert_eq!(type_of, InferTypes::NaN);
    }
    #[test]
    fn test_infer_types_mixed_math_two_bools() {
        let type_of = BinaryExpression::infer_types("true", "+", "false");
        assert_eq!(type_of, InferTypes::Number);
    }
    #[test]
    fn test_evaluate_number_expression_addition() {
        let sum = BinaryExpression::evaluate_number("2", "+", "2");
        assert_eq!("4", sum);
    }
    #[test]
    fn test_evaluate_number_expression_subtraction() {
        let sum = BinaryExpression::evaluate_number("2", "-", "2");
        assert_eq!("0", sum);
    }
    #[test]
    fn test_evaluate_number_bools_numbers() {
        let sum = BinaryExpression::evaluate_number("true", "-", "false");
        assert_eq!("1", sum);
    }
}
