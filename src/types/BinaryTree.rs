use core::panic;

use crate::traits;
use crate::{
    helper_funcs::{str_to_type},
    interpreter_types, types,
};
use interpreter_types::{Interpreter::Interpreter, VarsEnum::VarsEnum};
use regex::Regex;
use substring::Substring;
use traits::Evaluator::Evaluator;
use types::{Identifier::Identifier, Literal::Literal};

#[derive(PartialEq, Debug, Clone)]
enum BinaryExpressionOptions {
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(Box<BinaryExpression>),
    None(String),
}
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    type_of: String,
    start: usize,
    end: usize,
    left: BinaryExpressionOptions,
    operator: String,
    right: BinaryExpressionOptions,
}

impl BinaryExpression {
    //takes expression full string, returns current item, operator, and entire string that follows operator
    //there's a lot of room for in this function in particular; the mutable variables and them
    //being changed in a loop/if statement that returns something else is bad.
    // fn loop_through_operators(string: &str) -> (&str, Option<&str>, &str) {
    //     //finds operator
    //     let mat = Regex::new("([<>]=?|-|\\*|%|==|===|\\+|\\?|:)")
    //         .unwrap()
    //         .find(string);
    //     //gets ( as char, and first char of string
    //     let left_paren = "(".chars().next().unwrap();
    //     let c = string.chars().next().unwrap();

    //     //if the operator find match returns None, returns (0, 0), elsewise returns (x, y) tuple of operator start end
    //     let (mat_start, mat_end) = match mat {
    //         Some(mat) => (mat.start(), mat.end()),
    //         None => (0, 0),
    //     };
    //     //if there's no operator, returns early triple of string as current item (since it is the only, thus current, item) None for operator  and empty str for next;
    //     if mat_start == 0 && mat_end == 0 {
    //         return (string, None, "");
    //     }

    //     let mut resultant = "";
    //     let mut new_string = "";
    //     //if the full string starts with ( char, it means this is a parenthetical.
    //     //loops through full string until it finds matching char and returns item, so it doesn't end the return result at an inner parentheses;
    //     let item = if left_paren == c {
    //         let mut count: (i32, bool) = (0, false);

    //         let left_curly: &str = "(";
    //         let right_curly: &str = ")";

    //         let left_curly_char = left_curly.chars().next().unwrap();
    //         let right_curly_char = right_curly.chars().next().unwrap();

    //         for (i, c) in string.chars().enumerate() {
    //             if c == left_curly_char {
    //                 count.0 = count.0 + 1;
    //                 count.1 = true;
    //             };
    //             if c == right_curly_char {
    //                 count.0 = count.0 - 1;
    //             }

    //             if count.0 == 0 {
    //                 resultant = string.substring(0, i + 1);
    //                 new_string = string.substring(i + 2, string.len());

    //                 break;
    //             }
    //         }

    //         let current = rem_first_and_last(resultant);
    //         // println!("current: {}", current);
    //         current
    //     } else {
    //         let current = string.substring(0, mat_start);
    //         let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
    //             .unwrap()
    //             .find(string)
    //             .expect("no operators foudn");
    //         new_string = string.substring(mat.end(), string.len());

    //         current
    //     };
    //     if resultant == string {
    //         return (rem_first_and_last(resultant), None, "");
    //     }

    //     // let current = string.substring(0, mat_start);
    //     let operator = string.substring(mat_start, mat_end);

    //     (item, Some(operator), new_string)
    // }
    //takes full binary expression string, returns  BinaryExpression object, supports theoretically infinite nested parenthetical statements
    // pub fn create_generic_expression(string: &str) -> BinaryExpression {
    //     let (current, operator, next) = BinaryExpression::loop_through_operators(string);
    //     // println!("string: {}, current: {}, operator: {}, next: {}", string, current, operator.unwrap(), next);
    //     let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
    //         .unwrap()
    //         .is_match(current);
    //     //sets left leaf, if a a parenthetical, recursively loops through it and returns binaryexpression object, else creates a node
    //     let left_result = if mat {
    //         // println!("creating binary expressin: {}", current);
    //         let left_result = BinaryExpressionOptions::BinaryExpression(Box::new(
    //             BinaryExpression::create_generic_expression(current),
    //         ));
    //         // println!("left_result: {:?}", left_result);

    //         left_result
    //     } else {
    //         BinaryExpression::create_node(current)
    //     };

    //     let mut new_binary_expression = BinaryExpression {
    //         type_of: "BinaryExpression".to_string(),
    //         start: 0,
    //         end: string.len(),
    //         left: left_result,
    //         operator: "".to_string(),
    //         right: BinaryExpressionOptions::None("placeholder".to_string()),
    //     };
    //     //gets next set of current, operator, and following string, to see if the recursive loop needs to end after the final one
    //     let (test, test_op, _test_string) = BinaryExpression::loop_through_operators(next);
    //     //if the test operator from last line returns None, it's time to end recursive loop and return the binary_expression.
    //     if test_op == None {
    //         // new_binary_expression.left = left_result;
    //         //if the next item  is a parenthetical statement, starts a new loop using the test result, otherwise, just creates a node
    //         let right_result = if next.contains("(") {
    //             BinaryExpressionOptions::BinaryExpression(Box::new(
    //                 BinaryExpression::create_generic_expression(test),
    //             ))
    //         //if it's not a parenthetical expression, just creates node with next
    //         } else {
    //             // println!("next: {}", next);
    //             BinaryExpression::create_node(next)
    //         };

    //         new_binary_expression.right = right_result;
    //         new_binary_expression.operator = operator.unwrap().to_string();
    //         //returns early
    //         return new_binary_expression;
    //     }
    //     // else if there is another operator, creates a node for left, and starts recursive loop off again for right.
    //     new_binary_expression.left = BinaryExpression::create_node(current);
    //     new_binary_expression.right = BinaryExpressionOptions::BinaryExpression(Box::new(
    //         BinaryExpression::create_generic_expression(next),
    //     ));
    //     new_binary_expression.operator = operator.unwrap().to_string();

    //     new_binary_expression
    // }
    //just takes string of current item, returns an enum BinaryExpressionOptions literal or identifier
    fn create_node(current: &str) -> BinaryExpressionOptions {
        // println!("node current: {}", current);
        let type_of = BinaryExpression::str_to_type_inc_parentheses(current);
        let result = match type_of {
            "identifier" => {
                let new_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: current.to_string(),
                };

                BinaryExpressionOptions::Identifier(new_identifier)
            }
            "literal" => {
                let new_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: current.to_string(),
                };
                BinaryExpressionOptions::Literal(new_literal)
            }
            _ => BinaryExpressionOptions::None("End".to_string()),
        };
        result
    }

    fn str_to_type_inc_parentheses(string: &str) -> &str {
        let result = str_to_type(string);

        let type_match = match result {
            Ok("lookup") => "identifier",
            Ok("bool") => "literal",
            Ok("string") => "literal",
            Ok("number") => "literal",
            _ => "Malformed!",
        };
        // type_match
        // };
        type_match
    }
//returns the vec in RPN, from vector composed of operands and operators in order of string
    fn create_vec(operator_num_vec: Vec<&str>) -> Vec<&str> {
        let mut output: Vec<&str> = Vec::new();
        let mut operator_stack: Vec<&str> = Vec::new();

        for i in 0..operator_num_vec.len() {
            let current = operator_num_vec[i];
            let operator_two_left_parens = if operator_stack.len() > 0 {
                Self::check_top_stack_not_left_paren(&operator_stack)
            } else {
                false
            };

            let mat = Regex::new("([<>]=?|-|\\*|%|===|\\^|\\(|\\)|/|==|\\+|\\?|:)")
                .unwrap()
                .is_match(current);

            if mat {
                match current {
                    "+" | "-" | "/" | "%" | "^" | "*" | "==" | "===" | ">" | "<" | ">=" | "<=" => {
                        let mut o_one_precedence = Self::check_operator_precedence(current);
                        let mut o_two_precedence = if operator_stack.len() > 0 {
                            Self::check_operator_precedence(
                                operator_stack[operator_stack.len() - 1],
                            )
                        } else {
                            (0, false)
                        };
                        //if top of operator stack is not a left parentheses
                        if !operator_two_left_parens {
                            while (o_two_precedence.0 > o_one_precedence.0)
                                || (o_two_precedence.0 == o_one_precedence.0 && o_one_precedence.1)
                            {
                                let popped_op = operator_stack.pop().unwrap();
                                output.push(popped_op);

                                o_one_precedence = Self::check_operator_precedence(current);
                                o_two_precedence = if operator_stack.len() > 0 {
                                    Self::check_operator_precedence(
                                        operator_stack[operator_stack.len() - 1],
                                    )
                                } else {
                                    (0, false)
                                };
                            }
                        }
                        operator_stack.push(current);
                    }

                    "(" => {
                        operator_stack.push(current);
                    }
                    ")" => {
                        if !operator_two_left_parens {
                            let popped_op = operator_stack.pop().unwrap();
                            output.push(popped_op);
                        };
                        operator_stack.pop();
                    }
                    _ => {
                        panic!("Error! Current is {}", current);
                    }
                }
            } else {
                output.push(current);
            }
        }
        operator_stack.reverse();
        output.append(&mut operator_stack);
        output
    }

    pub fn check_top_stack_not_left_paren(vector: &Vec<&str>) -> bool {
        let to_check = vector[vector.len() - 1];
        if to_check == "(" {
            return true;
        }
        false
    }
//bool is right-left associativity
    pub fn check_operator_precedence(string: &str) -> (i8, bool) {
        let result = match string {
            "^" => (4, false),
            "*" => (3, true),
            "/" => (3, true),
            "+" => (2, true),
            "-" => (2, true),
            "===" | "=="|"<="|">="|"<"|">"=>(1,true),
            _ => (100, true),
        };
        result
    }
    //this creates a vec with no parsing, one unit per vec item, the title isn't exactly accurate yet
    pub fn create_shunting_yard_vec(string: &str) -> Vec<&str> {
        let mut new_string = string;

        let mut final_vec: Vec<&str> = Vec::new();
        if string.len() <= 0 {
            return final_vec;
        }
        //the idea here was to do a first pass to find == or === then cut off letters if present in second pass since I couldn't find a way to match only === or == in regex and instead got this: e===f,
        //I forgot about it and it ended up working anyway testing shallow and deep equality. I need to investigate later. 
        //finds operator
        // let first_mat =
        //     Regex::new("([<>]=?|-|\\*|%|\\^|\\(|\\)|/|[^=](==)[^=]|[^=](===)[^=]|\\+|\\?|:)")
        //         .unwrap()
        //         .find(string);

        let mat = Regex::new("([<>]=?|-|\\*|%|\\^|\\(|\\)|/|===|==|\\+|\\?|:)")
            .unwrap()
            .find(string);
        let mat_result = match mat {
            Some(value) => value,
            None => {
                final_vec.push(string);
                return final_vec;
            }
        };
        if mat_result.start() == 0 {
            final_vec.push(new_string.substring(mat_result.start(), mat_result.end()));
            new_string = new_string.substring(mat_result.end(), new_string.len())
        } else {
            final_vec.push(new_string.substring(0, mat_result.start()));
            final_vec.push(new_string.substring(mat_result.start(), mat_result.end()));
            new_string = new_string.substring(mat_result.end(), new_string.len())
        }
        final_vec.append(&mut Self::create_shunting_yard_vec(new_string));

        final_vec
    }

    pub fn create_combined_shunted_vec(string: &str) -> Vec<&str> {
        let pre_vec = Self::create_shunting_yard_vec(string);
        let parse_vec = Self::create_vec(pre_vec);
        parse_vec
    }

    // pub fn create_binary_expression_old(string: &str) -> Vec<&str> {
    //     let mut new_string = "";
    //     let mut final_vec: Vec<&str> = Vec::new();
    //     //finds operator
    //     let mat = Regex::new("([<>]=?|-|\\*|%|==|===|\\+|\\?|:)")
    //         .unwrap()
    //         .find(string);
    //     //gets ( as char, and first char of string
    //     let left_paren = "(".chars().next().unwrap();
    //     let c = string.chars().next().unwrap();

    //     //if the operator find match returns None, returns (0, 0), elsewise returns (x, y) tuple of operator start end
    //     let (mat_start, mat_end) = match mat {
    //         Some(mat) => (mat.start(), mat.end()),
    //         None => (0, 0),
    //     };
        //if there's no operator, returns early triple of string as current item (since it is the only, thus current, item) None for operator  and empty str for next;
        // if mat_start == 0 && mat_end == 0 {
        //     final_vec.push(string);
        //     return final_vec;
        // }

        // //if the full string starts with ( char, it means this is a parenthetical.
        // //loops through full string until it finds matching char and returns item, so it doesn't end the return result at an inner parentheses;
        // if left_paren == c {
        //     let mut count: (i32, bool) = (0, false);

        //     let left_curly: &str = "(";
        //     let right_curly: &str = ")";

    //         let left_curly_char = left_curly.chars().next().unwrap();
    //         let right_curly_char = right_curly.chars().next().unwrap();

    //         for (i, c) in string.chars().enumerate() {
    //             if c == left_curly_char {
    //                 count.0 = count.0 + 1;
    //                 count.1 = true;
    //             };
    //             if c == right_curly_char {
    //                 count.0 = count.0 - 1;
    //             }

    //             if count.0 == 0 {
    //                 final_vec.push(string.substring(0, i + 1));
    //                 new_string = string.substring(i + 2, string.len());
    //                 break;
    //             }
    //         }
    //     } else {
    //         final_vec.push(string.substring(0, mat_start));
    //         new_string = string.substring(mat_end, string.len());
    //     }
    //     let operator = string.substring(mat_start, mat_end);

    //     final_vec.push(operator);

    //     final_vec.append(&mut Self::create_binary_expression_old(new_string));
    //     final_vec
    // }

    pub fn create_binary_expression(string: &str) -> BinaryExpression {
        let mut shunted_vec = Self::create_combined_shunted_vec(string);
        let binary_expression = Self::create_binary_expression_from_rpn(&mut shunted_vec);
        binary_expression
    }

    //takes an array in reverse polish notation format, and turns it into a parsed binary expression that can be evaluated by evaluator trait
    pub fn create_binary_expression_from_rpn<'a>(
        string_vec: &mut Vec<&'a str>,
    ) -> BinaryExpression {
        let mut stack: Vec<&str> = Vec::new();
        string_vec.reverse();
        let mut current = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: 0,
            left: BinaryExpressionOptions::None("placeholder".to_string()),
            operator: "+".to_string(),
            right: BinaryExpressionOptions::None("placeholder".to_string()),
        };
        //loops through RPN vector. Note RPN vector has been reversed so I can more cleanly use pop() for stack
        for _i in 0..string_vec.len() {
            //if string_vec pops off a none value indicating there are no values left, return current binary expression
            let popped = match string_vec.pop() {
                Some(value) => value,
                None => return current,
            };

            let mat = Regex::new("([<>]=?|-|\\*|%|==|\\^|\\(|\\)|/|===|\\+|\\?|:)")
                .unwrap()
                .is_match(popped);
            //match current item from RPN string_vec.
            match mat {
                //if not an operator push to stack
                false => stack.push(popped),
                //if an operator
                true => {
                    //match length of stack. if greater than two, create a new binary expression. if after creating binary expression
                    //there is still more items in string vec, start while loop nesting binary expressions. the same thing
                    //happens with one item in stack. I will DRY this up later. when RPN string_vec ==0 break for loop.
                    let expression = match stack.len() {
                        2..=1000 => {
                            let mut result_expression_one = BinaryExpression {
                                type_of: "BinaryExpression".to_string(),
                                start: 0,
                                end: 0,
                                left: Self::create_node(stack[stack.len() - 2]),
                                operator: popped.to_string(),
                                right: Self::create_node(stack[stack.len() - 1]),
                            };
                            stack.pop();
                            stack.pop();

                            while string_vec.len() > 0 && stack.len() > 1 {
                                let popped_two = string_vec.pop().unwrap();

                                let result_expression = BinaryExpression {
                                    type_of: "BinaryExpression".to_string(),
                                    start: 0,
                                    end: 0,
                                    left: BinaryExpressionOptions::BinaryExpression(Box::new(
                                        result_expression_one.clone(),
                                    )),
                                    operator: popped_two.to_string(),
                                    right: Self::create_node(stack[stack.len() - 1]),
                                };
                                stack.pop();

                                result_expression_one = result_expression;
                            }

                            result_expression_one
                        }
                        1 => {
                            let result_expression = BinaryExpression {
                                type_of: "BinaryExpression".to_string(),
                                start: 0,
                                end: 0,
                                right: BinaryExpressionOptions::BinaryExpression(Box::new(
                                    current.clone(),
                                )),
                                operator: popped.to_string(),
                                left: Self::create_node(stack[stack.len() - 1]),
                            };
                            stack.pop();

                            result_expression
                        }
                        _ => {
                            if string_vec.len() > 0 {
                                continue;
                            } else {
                                break;
                            }
                        }
                    };
                    current = expression;
                }
            }
        }
        current
    }
}

impl Evaluator for BinaryExpression {
    fn evaluate(&self) -> &str {
        ""
    }
    fn evaluate_with_scope(&self, scope: &Interpreter) -> String {
        let left = match &self.left {
            BinaryExpressionOptions::Literal(value) => value.evaluate().to_string(),
            BinaryExpressionOptions::Identifier(value) => {
                let var_to_match = VarsEnum::create_vars_enum(value.evaluate(), scope);
                // let result = var_to_match.retrieve_pointed_value(scope).unwrap();

                let string_value = match var_to_match {
                    VarsEnum::Prim(prim_value) => prim_value.to_string(),
                    VarsEnum::Obj(obj_value) => format!("{:?}", obj_value).to_string(),
                    VarsEnum::Pointer(_) => {
                        panic!("There shouldn't be a pointer pointing to another pointer!")
                    }
                };
                string_value
            }
            BinaryExpressionOptions::BinaryExpression(binary_expression) => {
                binary_expression.evaluate_with_scope(scope)
            }
            BinaryExpressionOptions::None(_) => todo!(),
        };
        let right = match &self.right {
            BinaryExpressionOptions::Literal(value) => value.evaluate().to_string(),
            BinaryExpressionOptions::Identifier(value) => {
                let var_to_match = VarsEnum::create_vars_enum(value.evaluate(), scope);
                // let result = var_to_match.retrieve_pointed_value(scope).unwrap();

                let string_value = match var_to_match {
                    VarsEnum::Prim(prim_value) => prim_value.to_string(),
                    VarsEnum::Obj(obj_value) => format!("{:?}", obj_value).to_string(),
                    VarsEnum::Pointer(_) => {
                        panic!("There shouldn't be a pointer pointing to another pointer!")
                    }
                };
                string_value
            }

            BinaryExpressionOptions::BinaryExpression(binary_expression) => {
                binary_expression.evaluate_with_scope(scope)
            }
            BinaryExpressionOptions::None(_) => todo!(),
        };

        let eval_result = Self::simple_eval(&left, &self.operator, &right).to_string();
        eval_result
    }
}

#[cfg(test)]
mod test {
    use crate::{
        interpreter_types,
        traits::Evaluator::Evaluator,
        types::{self},
    };
    use interpreter_types::Interpreter::Interpreter;
    use types::{
        BinaryTree::BinaryExpression, BinaryTree::BinaryExpressionOptions, Identifier::Identifier,
        Literal::Literal,
    };

    #[test]
    fn test_create_shunting_yard_vec() {
        let result = BinaryExpression::create_combined_shunted_vec("7+6*2^3+2");
        assert_eq!(vec!["7", "6", "2", "3", "^", "*", "+", "2", "+"], result);
    }

    #[test]
    fn check_operator_precedence() {
        let result = BinaryExpression::check_operator_precedence("*");

        assert_eq!((3, true), result);
    }
    #[test]
    fn check_for_left_paren_top_stack() {
        let result = BinaryExpression::check_top_stack_not_left_paren(&vec![")", "(", "*", ")"]);

        assert_eq!(false, result);
    }
    #[test]
    fn check_for_left_paren_top_stack_true() {
        let result = BinaryExpression::check_top_stack_not_left_paren(&vec![")", "(", "*", "("]);

        assert_eq!(true, result);
    }

    #[test]
    fn test_create_pre_shunting_yard_vec() {
        let result = BinaryExpression::create_shunting_yard_vec("3+4*2/(1-5)^2^3");

        assert_eq!(
            vec!["3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3"],
            result
        );
    }

    #[test]
    fn test_evaulate_with_scope_order_ops() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("6*8+1");

        let result_string = result.evaluate_with_scope(&new_interpreter);
        assert_eq!("49", result_string);
    }
    #[test]
    fn test_evaulate_with_scope_sum_three_parens() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("1+10*2")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("21", result);
    }
    #[test]
    fn test_evaulate_with_scope_order_operations_1() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("8^2/(9-1)")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("8", result);
    }
    #[test]
    fn test_evaulate_with_scope_order_operations_2() {
        let new_interpreter = Interpreter::default(); //7623^2   // +*+
        let result = BinaryExpression::create_binary_expression("7+6*2^3+2")
            .evaluate_with_scope(&new_interpreter);
        assert_eq!("57", result);
    }

    #[test]
    fn test_evaulate_with_scope_order_operations_nested() {
        let new_interpreter = Interpreter::default(); //7623^2   // +*+
        let result = BinaryExpression::create_binary_expression("1+(2+(3*(4/2)))")
            .evaluate_with_scope(&new_interpreter);
        assert_eq!("9", result);
    }

    #[test]
    fn test_evaulate_with_scope_sum_three_nums() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("1+1+1")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("3", result);
    }
    #[test]
    fn test_evaulate_two_bools_comparison() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("true===false")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("false", result);
    }

    #[test]
    fn test_evaulate_two_bools_comparison_shallow() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("true==false")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("false", result);
    }
    #[test]
    fn test_evaulate_two_bools_comparison_shallow_equal() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("true==true")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("true", result);
    }
    #[test]
    fn test_evaulate_infer_bool_value_deepequality_fail() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("true===1")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("false", result);
    }
    #[test]
    fn test_evaulate_infer_bool_value_shallow_equality_succeed() {
        let new_interpreter = Interpreter::default();
        let result = BinaryExpression::create_binary_expression("true==1")
            .evaluate_with_scope(&new_interpreter);

        assert_eq!("true", result);
    }

    #[test]
    fn test_evaluate_binary_expression_greater_than() {
        let new_interpreter = Interpreter::default();
        let result =
            BinaryExpression::create_binary_expression("2>1").evaluate_with_scope(&new_interpreter);

        assert_eq!("true", result);
    }
    #[test]
    fn test_evaluate_binary_expression_greater_than_parentheses() {
        let new_interpreter = Interpreter::default();
        let result =
            BinaryExpression::create_binary_expression("2>(1*7)").evaluate_with_scope(&new_interpreter);
        assert_eq!("false", result);
    }
    #[test]
    fn test_evaulate_with_scope_sum() {
        let new_interpreter = Interpreter::default();
        let result =
            BinaryExpression::create_binary_expression("1+1").evaluate_with_scope(&new_interpreter);

        assert_eq!("2", result);
    }

    #[test]
    fn test_create_binary_tree_three_items_parentheses_last() {
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "y".to_string(),
        };

        let new_literal = Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "3".to_string(),
        };
        let cloned_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        };
        let new_binary_expression = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: 0,
            left: BinaryExpressionOptions::Identifier(new_identifier),
            operator: "+".to_string(),
            right: BinaryExpressionOptions::Literal(new_literal),
        };

        let binary_expression_test = BinaryExpression {
            start: 0,
            end: 0,
            type_of: "BinaryExpression".to_string(),
            left: BinaryExpressionOptions::Identifier(cloned_identifier),
            right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
            operator: "+".to_string(),
        };
        assert_eq!(
            binary_expression_test,
            BinaryExpression::create_binary_expression("x+(y+3)")
        );
    }

    #[test]
    fn test_create_binary_tree_three_items_parentheses_first() {
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        };

        let new_literal = Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "3".to_string(),
        };
        let cloned_identifier = new_identifier.clone();
        let new_binary_expression = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: 0,
            left: BinaryExpressionOptions::Identifier(new_identifier),
            operator: "+".to_string(),
            right: BinaryExpressionOptions::Identifier(cloned_identifier),
        };

        let binary_expression_test = BinaryExpression {
            start: 0,
            end: 0,
            type_of: "BinaryExpression".to_string(),
            right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
            left: BinaryExpressionOptions::Literal(new_literal),
            operator: "+".to_string(),
        };

        assert_eq!(
            binary_expression_test,
            BinaryExpression::create_binary_expression("(x+x)+3")
        );
    }
  
    #[test]
    fn test_create_binary_tree_two_literals() {
        let new_literal = Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "2".to_string(),
        };
        let literal = BinaryExpressionOptions::Literal(new_literal);
        let cloned_lit = literal.clone();
        let binary_expression_test = BinaryExpression {
            start: 0,
            end: 0,
            type_of: "BinaryExpression".to_string(),
            left: literal,
            right: cloned_lit,
            operator: "+".to_string(),
        };
        assert_eq!(
            binary_expression_test,
            BinaryExpression::create_binary_expression("2+2")
        );
    }
    #[test]
    fn test_create_binary_tree_two_identifiers() {
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        };
        let identifier = BinaryExpressionOptions::Identifier(new_identifier);
        let cloned_id = identifier.clone();
        let binary_expression_test = BinaryExpression {
            start: 0,
            end: 0,
            type_of: "BinaryExpression".to_string(),
            left: identifier,
            right: cloned_id,
            operator: "+".to_string(),
        };
        assert_eq!(
            binary_expression_test,
            BinaryExpression::create_binary_expression("x+x")
        );
    }

    #[test]
    fn test_create_binary_expression_one_arg() {
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        };
        let string = BinaryExpression::create_node("x");
        assert_eq!(string, BinaryExpressionOptions::Identifier(new_identifier));
    }
    #[test]
    fn test_str_to_type_incuding_parens_string() {
        let string = BinaryExpression::str_to_type_inc_parentheses("\"dogs\"");
        assert_eq!(string, "literal");
    }
    #[test]
    fn test_str_to_type_incuding_parens_numeric() {
        let string = BinaryExpression::str_to_type_inc_parentheses("4");
        assert_eq!(string, "literal");
    }
    #[test]
    fn test_str_to_type_incuding_parens_identifier() {
        let string = BinaryExpression::str_to_type_inc_parentheses("de4");
        assert_eq!(string, "identifier");
    }
    #[test]
    fn test_str_to_type_incuding_parens_malformed() {
        let string = BinaryExpression::str_to_type_inc_parentheses("\"de4+3");
        assert_eq!(string, "Malformed!");
    }
}

 