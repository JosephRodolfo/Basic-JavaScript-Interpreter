use regex::Regex;
use substring::Substring;
use types::Literal::Literal;
use types::VariableInitTypes::VariableInitTypes;

use crate::{
    helper_funcs::{str_to_type_inc_parentheses},
    types, Identifier,
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
        let type_of_var = str_to_type_inc_parentheses(value);
        let variable_init_value: Result<VariableInitTypes, String> = match type_of_var {
            "lookup" => {
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
            _ => panic!("Problem with variable declaration!"),
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
}
