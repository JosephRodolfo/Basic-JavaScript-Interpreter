#![warn(clippy::pedantic)]
use crate::interpreter_types;
use crate::types;
use interpreter_types::{Vars::Vars, VarsEnum::VarsEnum};
use std::collections::HashMap;
use types::{
    BodyTypes::BodyTypes, ExpressionStatement::ExpressionStatement,
    FunctionDeclaration::FunctionDeclaration, Program::Program,
    VariableDeclaration::VariableDeclaration,
};
#[derive(Debug)]
pub struct Interpreter {
    pub hash_stack: HashMap<String, Vars>,
    pub hash_heap: HashMap<String, Vars>,
    pub pointers: HashMap<String, Vars>,
}

impl Interpreter {
    pub fn loop_through_body_types(mut self, program: Program) {
        for i in 0..program.body.len() {
            let result = match &program.body[i] {
                BodyTypes::VariableDeclaration(value) => {
                    let result = VariableDeclaration::create_interpreter_var(
                        value.clone(),
                        &self.hash_stack,
                        &self.hash_heap,
                        &self.pointers,
                    );
                    let expression_result = match result {
                        Ok(result) => result,
                        Err(e) => panic!("{}", e),
                    };

                    expression_result
                }
                BodyTypes::ExpressionStatement(value) => {
                    let result = ExpressionStatement::create_evaulator_expression(
                        value.clone(),
                        &self.hash_stack,
                        &self.hash_heap,
                        &self.pointers,
                    );
                    let expression_result = match result {
                        Ok(result) => result,
                        Err(e) => panic!("{}", e),
                    };

                    expression_result
                }
                BodyTypes::FunctionDeclaration(value) => {
                    let result = FunctionDeclaration::create_interpreter_var(value);
                    let function_result = match result {
                        Ok(result) => result,
                        Err(e) => panic!("{}", e),
                    };

                    function_result
                }

                _ => {
                    todo!()
                }
            };
            self.insert_to_memory(result);
        }
        // println!("{:#?}", self);
    }
//this should really be called something more like look up for declaration or reassignment
    pub fn lookup_for_eval(&self, key: &str) -> VarsEnum {
        let result: Result<VarsEnum, String> = if self.hash_heap.contains_key(key) {
            let vars = self.hash_heap.get_key_value(key).unwrap().1.to_owned();
            let const_check = if vars.kind == "const" {
                Err("consts cannot be reassigned!".to_string())
            } else {
                Ok(vars.value)
            };
            const_check
        } else if self.hash_stack.contains_key(key) {
            let vars = self.hash_stack.get_key_value(key).unwrap().1.to_owned();
            let const_check = if vars.kind == "const" {
                Err("consts cannot be reassigned!".to_string())
            } else {
                Ok(vars.value)
            };
            const_check
        } else if self.pointers.contains_key(key) {
            let vars = self.pointers.get_key_value(key).unwrap().1.to_owned();
            let const_check = if vars.kind == "const" {
                Err("consts cannot be reassigned!".to_string())
            } else {
                Ok(vars.value)
            };

            const_check
        } else {
            let error_message = format!("Undefined variable! {}", key);
            Err(error_message.to_string())
        };

        match result {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn insert_to_memory(&mut self, mem_tuple: (String, Vars)) {
        let key = mem_tuple.0;
        let value = mem_tuple.1.clone();
        match mem_tuple.1.value {
            VarsEnum::Prim(_prim_value) => {
                self.hash_stack.insert(key, value);
            }
            VarsEnum::Obj(_obj_value) => {
                self.hash_heap.insert(key, value);
            }
            VarsEnum::Pointer(_pointer_value) => {
                self.pointers.insert(key, value);
            }
        }
    }

    pub fn default() -> Interpreter {
        let hash_stack: HashMap<String, Vars> = HashMap::new();
        let hash_heap: HashMap<String, Vars> = HashMap::new();
        let pointers: HashMap<String, Vars> = HashMap::new();
        Interpreter {
            hash_stack,
            hash_heap,
            pointers,
        }
    }
}
