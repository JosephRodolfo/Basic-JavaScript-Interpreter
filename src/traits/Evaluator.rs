

use crate::{helper_funcs::{rem_first_and_last, str_to_type}, interpreter_types::Interpreter::Interpreter};

pub trait Evaluator {

    fn evaluate_with_scope(&self, scope: &Interpreter)->String;
    fn evaluate(&self) -> &str;
//simple eval takes, left, right, and operator string slices. It determines the correct return type
//by testing the types of the left and right values, then matches those to correct function;
    fn simple_eval(left: &str, operator: &str, right: &str)->String {
        let operation_type = Self::infer_types(left, operator, right);

      let simple_eval_result =  match operation_type {
            InferTypes::Number => { Self::evaluate_number(left, operator, right)}
            InferTypes::Bool => { Self::evaluate_bool(left, operator, right)}
            InferTypes::String => {Self::evaluate_string(left, right)}
            InferTypes::NaN => {"NaN".to_string()}
        };

        simple_eval_result
    }
    //like javascript, all numbers are f64. In the future I may check if they're integers and do integer math just as something to do, but to my understanding and limited experiments this matches what one expects in javascript. I haven't dug into the rust documentation on testing float equality yet but I know crates exist for that.
    //still need to handle convering true and false boolean into 1 and 0 respectively.
    fn evaluate_number(left: &str, operator: &str, right: &str) -> String {
        let left_number = Self::boolean_to_number(left);
        let right_number = Self::boolean_to_number(right);
        let number_result = match operator {
            "+" => left_number + right_number,
            "*" => left_number * right_number,
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

    fn evaluate_string(left: &str, right: &str) -> String {
        let concat_string = format!(
            "\"{}{}\"",
            rem_first_and_last(left),
            rem_first_and_last(right)
        );
        concat_string
    }
    fn evaluate_bool(left: &str, operator: &str, right: &str) -> String {
        let left_type = str_to_type(left).unwrap();
        let right_type = str_to_type(right).unwrap();
        //deep equality operator returns early with false if types don't match
        if (operator == "===" || operator == "!==") && (left_type != right_type) {
            return "false".to_string();
        };

        if (left_type == "string") && (right_type == "string") {
            let string_comparison = match operator {
                "==" =>  left == right,
                "===" =>  left == right,
                ">=" =>  left == right,
                "<=" =>  left == right,
                ">" =>  false,
                "<" =>  false,
                _ => false,
            };
            return string_comparison.to_string();
        }

        let left_str = {
            let type_of = str_to_type(left).unwrap();
            let result = match type_of {
                "string" => Self::infer_boolean_value(left),
                "bool" => {
                    format!("{}", Self::boolean_to_number(left))
                }
                "number" => left.to_string(),
                _ => {
                    panic!("{} is the wrong type!", left)
                }
            };
            result
        };
        let right_str = {
            let type_of = str_to_type(right).unwrap();
            let result = match type_of {
                "string" => Self::infer_boolean_value(right),
                "bool" => {
                    format!("{}", Self::boolean_to_number(right))
                }
                "number" => right.to_string(),
                _ => {
                    panic!("{} is the wrong type!", right)
                }
            };
            result
        };

        let left_number = left_str.parse::<f64>().unwrap();
        let right_number = right_str.parse::<f64>().unwrap();

        let math_bool = match operator {
            "==" =>  left_number == right_number,
            "===" =>  left_number == right_number,
            ">=" =>  left_number >= right_number,
            "<=" =>  left_number <= right_number,
            ">" =>  left_number > right_number,
            "<" =>  left_number < right_number,
            _ => false,
        };
        math_bool.to_string()
    }
    fn infer_types(left: &str, operator: &str, right: &str) -> InferTypes {
        let left_type = str_to_type(left).unwrap();
        let right_type = str_to_type(right).unwrap();

        let math_or_not =
            if (operator == "+") | (operator == "*") | (operator == "-") | (operator == "/") | (operator == "%") {
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
            //if one side is a bool and one side is a number, evaluate to a number. I'm certain there is a shorter way to do this.
        } else if (((left_type == "bool") && (right_type == "number"))
            | ((left_type == "number") && (right_type == "bool")))
            && (operator == "+")
        {
            InferTypes::Number
        } else if (left_type == "string") && (right_type == "string") && (operator == "+") {
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
//this should be renamed. in cases where a boolean expression is evaluated as if it's a number
//it takes a string type, and performs JavaScript Boolean() operation,
//but if it catches that the string is actually a number, it returns that number string,
//so shallow equality will work, e.g., "2" == 2.
    fn infer_boolean_value(string: &str) -> String {

        let test_if_num = rem_first_and_last(string);

        if test_if_num.parse::<f64>().is_ok() {
            return test_if_num.to_string();
        }


        if (string == "true") | (string != "") | (string != "0") {
            return "true".to_string();
        }

        "false".to_string()
    }
    fn boolean_to_number(string: &str) -> f64 {
        let number = match string {
            "true" => 1 as f64,
            "false" => 0 as f64,
            _ => string.parse::<f64>().unwrap(),
        };
        number
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
    use crate::traits::Evaluator::{Evaluator, InferTypes};
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
        assert_eq!(type_of, InferTypes::Number);
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
    #[test]
    fn test_evaluate_string() {
        let sum = BinaryExpression::evaluate_string("\"one\"", "\"two\"");
        assert_eq!("\"onetwo\"", sum);
    }
    #[test]
    fn test_evaluate_boolean_two_strings() {
        let result = BinaryExpression::evaluate_bool("\"one\"", "==", "\"two\"");
        assert_eq!("false", result);
    }
    #[test]
    fn test_evaluate_boolean_two_strings_equal() {
        let result = BinaryExpression::evaluate_bool("\"one\"", "==", "\"one\"");
        assert_eq!("true", result);
    }
    #[test]
    fn test_evaluate_boolean_deep_equality_different_types() {
        let result = BinaryExpression::evaluate_bool("2", "===", "\"2\"");
        assert_eq!("false", result);
    }
    #[test]
    fn test_evaluate_boolean_shallow_equality_different_types() {
        let result = BinaryExpression::evaluate_bool("2", "==", "\"2\"");
        assert_eq!("true", result);
    }
    #[test]
    fn test_evaluate_boolean_two_bools() {
        let result = BinaryExpression::evaluate_bool("true", ">", "false");
        assert_eq!("true", result);
    }
    #[test]
    fn test_evaluate_boolean_two_numbers() {
        let result = BinaryExpression::evaluate_bool("9", "<", "10");
        assert_eq!("true", result);
    }

    #[test]
    fn test_simple_eval_two_numbers() {
        let result = BinaryExpression::simple_eval("1", "+", "1");
        assert_eq!("2", result);
    }
    #[test]
    fn test_simple_eval_simple_subtraction() {
        let sum = BinaryExpression::simple_eval("2", "-", "2");
        assert_eq!("0", sum);
    }
    #[test]
    fn test_simple_eval_bools_as_numbers() {
        let sum = BinaryExpression::simple_eval("true", "-", "false");
        assert_eq!("1", sum);
    }
    #[test]
    fn test_simple_eval_string_addition() {
        let sum = BinaryExpression::simple_eval("\"one\"", "+","\"two\"");
        assert_eq!("\"onetwo\"", sum);
    }
    #[test]
    fn test_simple_eval_unequal_strings() {
        let result = BinaryExpression::simple_eval("\"one\"", "==", "\"two\"");
        assert_eq!("false", result);
    }
    #[test]
    fn test_simple_eval_equal_strings() {
        let result = BinaryExpression::simple_eval("\"one\"", "==", "\"one\"");
        assert_eq!("true", result);
    }
    #[test]
    fn test_simple_eval_deep_equality() {
        let result = BinaryExpression::simple_eval("2", "===", "\"2\"");
        assert_eq!("false", result);
    }
    #[test]
    fn test_simple_eval_shallow_equality() {
        let result = BinaryExpression::simple_eval("2", "==", "\"2\"");
        assert_eq!("true", result);
    }
    #[test]
    fn test_simple_eval_two_bools() {
        let result = BinaryExpression::simple_eval("true", ">", "false");
        assert_eq!("true", result);
    }
    #[test]
    fn test_simple_eval_two_numbers_comparison() {
        let result = BinaryExpression::simple_eval("9", "<", "10");
        assert_eq!("true", result);
    }
    #[test]
    fn test_simple_eval_not_a_number() {
        let result = BinaryExpression::simple_eval("100", "-", "true");
        assert_eq!("NaN", result);
    }
    #[test]
    fn test_simple_eval_division() {
        let result = BinaryExpression::simple_eval("10", "/", "4");
        assert_eq!("2.5", result);
    }
}
