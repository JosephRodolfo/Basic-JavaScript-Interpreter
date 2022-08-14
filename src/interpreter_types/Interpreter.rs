use std::collections::HashMap;
use std::hash::Hash;
use crate::traits::{evaluator::evaluator};
use crate::types;
use crate::interpreter_types;
use interpreter_types::{Vars::Vars, VarsEnum::VarsEnum};
use types::{VariableInitTypes::VariableInitTypes, ExpressionStatement::ExpressionStatement, VariableDeclaration::VariableDeclaration, Program::Program, BodyTypes::BodyTypes};
pub struct Interpreter {
    stack: HashMap<String, String>,

}


impl Interpreter {
  pub  fn loop_through_body_types(program: Program,  mut scope_stack_vars: HashMap<String, Vars>, mut scope_heap_vars: HashMap<String, Vars>, mut scope_pointers: HashMap<String, Vars>){
    for i in 0..program.body.len(){

        let result =    match &program.body[i]{


                BodyTypes::VariableDeclaration(value)=>{
                    
                let result =    VariableDeclaration::create_interpreter_var(value.clone(), &scope_stack_vars, &scope_heap_vars, &scope_pointers);
                    let new_var = result.unwrap();
                    let key = new_var.0;
                    let value = new_var.1.clone();
                   match new_var.1.value {
                        VarsEnum::Prim(_prim_value)=>{
                            scope_stack_vars.insert(key, value);
                        },
                        VarsEnum::Obj(_obj_value)=>{
                            scope_heap_vars.insert(key, value);
                        },
                        VarsEnum::Pointer(_pointer_value)=>{
                            scope_pointers.insert(key, value);

                        }
                    }

                },
                BodyTypes::ExpressionStatement(value)=>{



                },
                _=>{}


            };

        }
println!("{:?}", scope_stack_vars);
    
    }
}