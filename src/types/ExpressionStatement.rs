use crate::interpreter_types::{Vars::Vars, VarsEnum::VarsEnum};
use crate::traits::Evaluator::Evaluator;
use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::types;
use crate::HashMap;
use types::{
    ArrayExpression::ArrayExpression, AssignmentExpression::AssignmentExpression,
    BinaryTree::BinaryExpression, CallExpression::CallExpression, ExpressionType::ExpressionType,
    Identifier::Identifier, Literal::Literal, UpdateExpression::UpdateExpression,
    VariableInitTypes::VariableInitTypes,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    type_of: String,
    start: usize,
    end: usize,
    pub expression: ExpressionType,
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
            Ok("assignment_expression") => ExpressionType::AssignmentExpression(
                AssignmentExpression::create_assignment_expression(expression_string),
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
    pub fn create_assignment_expression(string: &str) -> ExpressionStatement {
        let result = AssignmentExpression::create_assignment_expression(string);
        let new_expression_statement = ExpressionStatement {
            start: 0,
            end: 0,
            type_of: "ExpressionStatement".to_string(),
            expression: ExpressionType::AssignmentExpression(result),
        };
        new_expression_statement
    }

    pub fn create_evaulator_expression(
        value: ExpressionStatement,
        scope_stack: &HashMap<String, Vars>,
        scope_heap: &HashMap<String, Vars>,
        scope_pointers: &HashMap<String, Vars>,
    ) -> Result<(String, Vars), String> {
        let expression_type = value.expression;

        let returned_expression = match expression_type {
            ExpressionType::AssignmentExpression(value) => {
                let operator = value.operator;

                if operator != "=" {
                    return Err("that type of assignment not supported yet".to_string());
                }
                let left: Result<String, String> = match *value.left {
                    ExpressionType::Identifier(value) => {
                        let name = value.evaluate();
                        let scope_stack_prescence = scope_stack.contains_key(name);
                        let scope_heap_prescence = scope_heap.contains_key(name);
                        let scope_pointers_presence = scope_pointers.contains_key(name);

                        let result = if scope_heap_prescence {
                            let result = scope_heap.get_key_value(name);
                            let kind = &result.unwrap().1.kind;
                            if kind == "const" {
                                return Err("consts cannot be reassigned!".to_string());
                            } else {
                                result
                            }
                        } else if scope_stack_prescence {
                            let result = scope_stack.get_key_value(name);
                            let kind = &result.unwrap().1.kind;
                            if kind == "const" {
                                return Err("consts cannot be reassigned!".to_string());
                            } else {
                                result
                            }
                        } else if scope_pointers_presence {
                            let result = scope_pointers.get_key_value(name);
                            let kind = &result.unwrap().1.kind;
                            if kind == "const" {
                                return Err("consts cannot be reassigned!".to_string());
                            } else {
                                result
                            }
                        } else {
                            return Err("Undeclared variable! This should actually be permissable and become part of the global scope but that is unimplemented at the moment. You probably didn't want to do this anyways.".to_string());
                        };

                        Ok(name.to_string())
                      
                    }

                    ExpressionType::BinaryExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::CallExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::Literal(literal) => {
                        let literal_value = literal.value.to_owned();

                        if operator == "=" {
                            return Err("Assigning to number literal is wrong!".to_string());
                        }
                        Ok(literal_value)
                    }
                    //obviously sometimes this should work, but I haven't implemented it yet.
                    ExpressionType::ArrayExpression(_) => {
                        return Err("Destructuring not currently supported!".to_string())
                    }
                    ExpressionType::UpdateExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::AssignmentExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                };

                let right: Result<Vars, String> = match *value.right {
                    ExpressionType::Identifier(value) => {
                        let name = value.evaluate();
                        let right_result_identifier = if scope_heap.contains_key(name) {
                            let heap_value =
                                scope_heap.get_key_value(name).unwrap().1.value.clone();
                            let new_var = Vars {
                                kind: "let".to_string(),
                                value: heap_value,
                            };

                            Ok(new_var)
                        } else if scope_stack.contains_key(name) {
                            let stack_value =
                                scope_stack.get_key_value(name).unwrap().1.value.clone();
                            let new_var = Vars {
                                kind: "let".to_string(),
                                value: stack_value,
                            };

                            Ok(new_var)
                        } else if scope_pointers.contains_key(name) {
                            let pointers_value =
                                scope_pointers.get_key_value(name).unwrap().1.value.clone();
                            let new_var = Vars {
                                kind: "let".to_string(),
                                value: pointers_value,
                            };

                            Ok(new_var)
                        } else {
                            let error_message = format!("{} is undefined", name);
                            Err(error_message)
                        };

                        Ok(right_result_identifier.unwrap())
                    }

                    ExpressionType::BinaryExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::CallExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::Literal(literal) => {
                        let new_var = Vars {
                            kind: "let".to_string(),
                            value: VarsEnum::Prim(literal.evaluate().to_string()),
                        };

                        Ok(new_var)
                    }
                    ExpressionType::ArrayExpression(var) => {
                        let new_var = Vars {
                            kind: "let".to_string(),
                            value: VarsEnum::Obj(VariableInitTypes::ArrayExpression(var)),
                        };

                        Ok(new_var)
                    }
                    ExpressionType::UpdateExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                    ExpressionType::AssignmentExpression(_) => {
                        return Err("Assigning to rvalue".to_string())
                    }
                };

                (left, right)
            }

            _ => {
                todo!()
            }
        };

        let left = match returned_expression.0 {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        let right = match returned_expression.1 {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        Ok((left, right))
    }
}

impl ExpressionTypes for ExpressionStatement {}

#[cfg(test)]
mod test {
    use crate::traits::ExpressionTypes::ExpressionTypes;
    use crate::types;
    use types::ExpressionStatement::ExpressionStatement;

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
