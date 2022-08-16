use std::collections::HashMap;

use crate::interpreter_types::{Interpreter::Interpreter, Vars::Vars};
use crate::types::VariableInitTypes::VariableInitTypes;

#[derive(PartialEq, Debug, Clone)]
pub enum VarsEnum {
    Prim(String),
    Obj(VariableInitTypes),
    Pointer(String),
}

impl VarsEnum {
    pub fn retrieve_pointed_value(self, scope_mem: &Interpreter) -> Result<(String, Vars), String> {
        let pointer_value: Result<String, String> = match self {
            VarsEnum::Pointer(value) => Ok(value),
            _ => Err("not defined".to_string()),
        };

        let key: String = match pointer_value {
            Ok(value) => value,
            Err(e) => {
                panic!("{} is not defined!", e);
            }
        };

        let pointed_presence = scope_mem.hash_heap.contains_key(&key);
        if !pointed_presence {
            let error_messge = format!("{} does not exist", key);
            return Err(error_messge);
        }
        let result = scope_mem.hash_heap.get_key_value(&key).unwrap();

        Ok((result.0.to_string(), result.1.to_owned()))
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
    fn test_retrieve_pointed_to_value_and_after_changing_reference_value() {
        //create interpreted var to test against
        let mut new_interpreter = Interpreter::default();

        let new_var = VariableDeclaration::create_variable_declaration(
            "leta=[1,2,3]".to_string(),
            &"".to_string(),
        );

        let interpreted_test_var = VariableDeclaration::create_interpreter_var(
            new_var,
            &new_interpreter.hash_stack,
            &new_interpreter.hash_heap,
            &new_interpreter.pointers,
        )
        .unwrap();
        //insert var to test against into heap
        new_interpreter.hash_heap.insert(
            interpreted_test_var.0.to_string(),
            interpreted_test_var.1.to_owned(),
        );
        //create the interpreted var that is a poitner to that heap reference
        let new_pointer_var =
            VariableDeclaration::create_variable_declaration("letx=a".to_string(), &"".to_string());

        let interpreted_pointer_var = VariableDeclaration::create_interpreter_var(
            new_pointer_var,
            &new_interpreter.hash_stack,
            &new_interpreter.hash_heap,
            &new_interpreter.pointers,
        )
        .unwrap();

        let retrieved_value = interpreted_pointer_var
            .1
            .value
            .retrieve_pointed_value(&new_interpreter)
            .unwrap();

        assert_eq!(interpreted_test_var, retrieved_value);

        new_interpreter.hash_heap.insert(
            interpreted_test_var.0.to_string(),
            Vars {
                value: VarsEnum::Obj(VariableInitTypes::ArrayExpression(
                    ArrayExpression::create_array_expression("3,2,1"),
                )),
                kind: "let".to_string(),
            },
        );

        let updated_test_value = new_interpreter
            .hash_heap
            .get_key_value(&interpreted_test_var.0)
            .unwrap();
        let new_tuple = (
            &updated_test_value.0.to_string(),
            &updated_test_value.1.to_owned(),
        );
      

        //change the pointed to value on the heap;
        //then test if the pointer will return the correct updated value;
        assert_eq!(new_tuple, updated_test_value);
    }
}
