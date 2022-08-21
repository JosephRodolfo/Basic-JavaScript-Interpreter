use crate::interpreter_types::Interpreter::Interpreter;
use crate::interpreter_types::{Vars::Vars, VarsEnum::VarsEnum};
use crate::traits::Evaluator::Evaluator;
use crate::traits::ExpressionTypes::ExpressionTypes;

use crate::{
    helper_funcs::{rem_first_and_last},
    types,
};
use regex::Regex;
use std::collections::HashMap;
use substring::Substring;
use types::{
    ArrayExpression::ArrayExpression, Identifier::Identifier, Literal::Literal,
    VariableInitTypes::VariableInitTypes, BinaryTree::BinaryExpression
};


#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub declarations: VariableDeclarator,
    pub kind: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    type_of: String,
    start: usize,
    end: usize,
    id: Identifier,
    init: VariableInitTypes,
}

impl VariableDeclaration {
    pub fn create_variable_declaration(
        program: String,
        whole_program: &String,
    ) -> VariableDeclaration {
        let mat = Regex::new("(const|let|var)")
            .unwrap()
            .find(&program)
            .expect("found no variables");
        //returns keyword (let, const, var)
        let var_keyword = program.substring(mat.start(), mat.end());
        //returns match or everything after assignment operator
        let after_equal = Regex::new("(=)")
            .unwrap()
            .find(&program)
            .expect("found no assignment operator");
        //get var name
        let name = program.substring(mat.end(), after_equal.start());

        //get var value, what follows assignment operator
        let value = program.substring(after_equal.end(), program.len());
        //returns str of type of var ("number", "bool", etc., takes value as param(or what's after assignment operator))
        let type_of_var = VariableInitTypes::check_expression_type(value).unwrap();
        // println!("type: {}, value:{}", type_of_var, value);
        let variable_init_value: Result<VariableInitTypes, String> = match type_of_var {
            "identifier" => {
                let new_var_declaration_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: value.to_string(),
                };
                Ok(VariableInitTypes::Identifier(
                    new_var_declaration_identifier,
                ))
            }
            "literal" => {
                let new_var_declaration_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: value.to_string(),
                };
                Ok(VariableInitTypes::Literal(new_var_declaration_literal))
            }
            "array_expression" => {
                let square_brackets_removed = rem_first_and_last(value);
                Ok(VariableInitTypes::ArrayExpression(
                    ArrayExpression::create_array_expression(square_brackets_removed),
                ))
            }
            "binary_expression" => {
                Ok(VariableInitTypes::BinaryExpression(
                    BinaryExpression::create_binary_expression(value),
                ))
            }
            _ => panic!("Problem with variable declaration! {}", value),
        };

        let new_var_declaration_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: name.to_string(),
        };

        let new_var_declarator = VariableDeclarator {
            type_of: "VariableDeclarator".to_string(),
            start: 0,
            end: 0,
            id: new_var_declaration_identifier,
            init: variable_init_value.unwrap(),
        };

        let new_var_declaration = VariableDeclaration {
            type_of: "VariableDeclaration".to_string(),
            start: mat.start(),
            end: program.len(),
            kind: var_keyword.to_string(),
            declarations: new_var_declarator,
        };
        new_var_declaration
    }

    pub fn create_interpreter_var(
        var: VariableDeclaration,
        scope_stack: &HashMap<String, Vars>,
        scope_heap: &HashMap<String, Vars>,
        scope_pointers: &HashMap<String, Vars>,
    ) -> Result<(String, Vars), String> {
        let name = var.declarations.id.name;

        if scope_heap.contains_key(&name)
            | scope_stack.contains_key(&name)
            | scope_pointers.contains_key(&name)
        {
            let error_message = format!("Error, {} is already declared as a variable!", name);
            return Err(error_message);
        }
        let result = match var.declarations.init {
            VariableInitTypes::Literal(value) => {
                let hash_value = value.evaluate().to_owned();
                let new_var = Vars {
                    kind: var.kind,
                    value: VarsEnum::Prim(hash_value),
                };

                Ok((name, new_var))
            }

            VariableInitTypes::Identifier(value) => {
                let hash_value = value.evaluate().to_owned();
                // println!("name: {}, hash_value: {}", name, hash_value);

                let value = if scope_heap.contains_key(&hash_value) {
                    let new_var = Vars {
                        kind: var.kind,
                        value: VarsEnum::Pointer(hash_value),
                    };

                    Ok(new_var)
                } else if scope_stack.contains_key(&hash_value) {
                    let result = scope_stack.get_key_value(&hash_value).unwrap();
                    let prim_value = result.1.value.clone();
                    // println!("result: {:?}", result);

                    let new_var = Vars {
                        kind: var.kind,
                        value: prim_value,
                    };

                    Ok(new_var)
                } else {
                    let error_message = format!("{} is undefined", hash_value);
                    Err(error_message)
                };

                Ok((name, value.unwrap()))
            }
            VariableInitTypes::ArrayExpression(value) => {
                let new_var = Vars {
                    kind: var.kind,
                    value: VarsEnum::Obj(VariableInitTypes::ArrayExpression(value)),
                };

                Ok((name, new_var))
            },
            VariableInitTypes::BinaryExpression(value)=>{

                let mut new_interpreter = Interpreter::default();
                new_interpreter.hash_heap=scope_heap.clone();
                new_interpreter.hash_stack=scope_stack.clone();
                new_interpreter.pointers=scope_pointers.clone();
                let new_var = Vars {
                    kind: var.kind,
                    value:VarsEnum::Prim(value.evaluate_with_scope(&new_interpreter))
                };
                Ok((name, new_var))
            }
        };
        result
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter_types;
    use crate::types;
    use interpreter_types::{Interpreter::Interpreter, Vars::Vars, VarsEnum::VarsEnum};
    use types::{
        ArrayExpression::ArrayExpression, VariableDeclaration::VariableDeclaration,
        VariableInitTypes::VariableInitTypes,
    };

    #[test]
    fn test_create_interpreter_var_literal() {
        let new_interpreter = Interpreter::default();
        let new_var =
            VariableDeclaration::create_variable_declaration("letx=2".to_string(), &"".to_string());

        let interpreted_test_var = VariableDeclaration::create_interpreter_var(
            new_var,
            &new_interpreter.hash_stack,
            &new_interpreter.hash_heap,
            &new_interpreter.pointers,
        );

        let var = Vars {
            kind: "let".to_string(),
            value: VarsEnum::Prim("2".to_string()),
        };
        let test_var = ("x".to_string(), var);
        assert_eq!(interpreted_test_var.unwrap(), test_var);
    }
    #[test]

    fn test_create_interpreter_var_redeclared_error() {
        let mut new_interpreter = Interpreter::default();
        let var = Vars {
            kind: "let".to_string(),
            value: VarsEnum::Prim("2".to_string()),
        };
        new_interpreter.hash_stack.insert("y".to_string(), var);

        let new_var =
            VariableDeclaration::create_variable_declaration("lety=1".to_string(), &"".to_string());

        let interpreted_test_var = VariableDeclaration::create_interpreter_var(
            new_var,
            &new_interpreter.hash_stack,
            &new_interpreter.hash_heap,
            &new_interpreter.pointers,
        );

        assert!(interpreted_test_var.is_err())
    }
    #[test]

    fn test_create_interpreter_var_array_obj_type() {
        let new_interpreter = Interpreter::default();
        let new_array_expression = ArrayExpression::create_array_expression("1,2,3");

        let var = Vars {
            kind: "let".to_string(),
            value: VarsEnum::Obj(VariableInitTypes::ArrayExpression(new_array_expression)),
        };

        let test_interpreted_expression = ("a".to_string(), var);
        let new_var = VariableDeclaration::create_variable_declaration(
            "leta=[1,2,3]".to_string(),
            &"".to_string(),
        );

        let interpreted_test_var = VariableDeclaration::create_interpreter_var(
            new_var,
            &new_interpreter.hash_stack,
            &new_interpreter.hash_heap,
            &new_interpreter.pointers,
        );

        assert_eq!(interpreted_test_var.unwrap(), test_interpreted_expression);
    }
   
}
