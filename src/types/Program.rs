use crate::traits::ExpressionTypes::ExpressionTypes;
use crate::types;
use regex::Regex;
use substring::Substring;
use types::{BodyTypes::BodyTypes, IfStatement::IfStatement, ReturnStatement::ReturnStatement};

use crate::{ExpressionStatement, FunctionDeclaration, VariableDeclaration};

#[derive(Debug)]
pub struct Program {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub body: Vec<BodyTypes>,
}

impl Program {
    //eventually the main parsing loop that will go through program string and turn it into AST and nodes, using
    //looping
    //This was one of the first things I did so I need to redo it, ExpresionStatement handling in particular;
    //and create a function for matching line to BodyTypes type. Then match that to
    //creation functions.
    fn parse_program(&mut self, program: &String, whole_program: &String) {
        self.type_of = "test".to_string();

        //this needs to be changed to match exact, beginning of string,
        //also needs conditional logic for block statement vs. whole program,
        //as return statements shouldn't be in main program
        let mat = Regex::new("(function|return|const|let|var|if|for|while|console.log(\\(*)\\))")
            .unwrap()
            .find(&program);

        let match_find = match mat {
            Some(x) => (x.start(), x.end()),
            None => (0, 0),
        };
        //if the first  of the program string doesn't match any of the reserved keywords, it checks to see if it's an expression.
        if match_find.0 == 0 && match_find.1 == 0 {
            let expression_statement = ExpressionStatement::check_expression_type(program);
            let result: &str = match expression_statement {
                Err(e) => panic!("Problem parsing expression statement: {:?}", e),
                Ok(result) => result,
            };

            let parsed_expression_statement: ExpressionStatement = match result {
                "call_expression" => ExpressionStatement::create_call_expression(program),
                "update_expression" => ExpressionStatement::create_update_expression(program),
                _ => ExpressionStatement::create_binary_expression(program),
            };
            self.self_add_body_types(BodyTypes::ExpressionStatement(parsed_expression_statement));
            // self.add_expression_statement(parsed_expression_statement);
        }

        let string_for_match = program.substring(match_find.0, match_find.1);
        if string_for_match == "const" || string_for_match == "let" || string_for_match == "var" {
            let variable_declaration = Program::match_var_declaration_start_parse(
                self,
                string_for_match,
                program.to_string(),
                whole_program,
            )
            .unwrap();
            let result = variable_declaration.end;
            self.self_add_body_types(BodyTypes::VariableDeclaration(variable_declaration));

            // self.add_variable_declaration(variable_declaration);

            Some(result)
        } else if string_for_match == "function" {
            let function_declaration = Program::match_function_declaration_start_parse(
                string_for_match,
                program.to_string(),
            )
            .unwrap();

            self.self_add_body_types(BodyTypes::FunctionDeclaration(function_declaration));

            None
        } else if string_for_match == "if" {
            let new_if_statement = IfStatement::create_if_statement(program);
            self.self_add_body_types(BodyTypes::IfStatement(new_if_statement));

            None
        } else if string_for_match == "return" {
            let new_if_statement = ReturnStatement::create_return_statement(program);
            self.self_add_body_types(BodyTypes::ReturnStatement(new_if_statement));

            None
        } else if string_for_match == "for" {
            None
        } else if string_for_match == "while" {
            None
        } else {
            None
        };
        //This one will check for existing variables to determine if valid expression.
    }
    //These three match functions will take the already decided type (func_dec, var_dec, expression_statement and call parser functions, returning the node);
    //I'm pretty sure they can combined in one generic
    fn match_var_declaration_start_parse(
        &self,
        type_of_string_for_match: &str,
        program_string: String,
        whole_program: &String,
    ) -> Result<VariableDeclaration, String> {
        match type_of_string_for_match {
            "const" | "let" | "var" => {
                let result =
                    VariableDeclaration::create_variable_declaration(program_string, whole_program);
                Ok(result)
            }
            _ => Err("No Variable Declarations Found!".to_string()),
        }
    }
    fn match_function_declaration_start_parse(
        type_of_string_for_match: &str,
        program_string: String,
    ) -> Result<FunctionDeclaration, String> {
        let function_declaration: Result<FunctionDeclaration, String> =
            match type_of_string_for_match {
                "function" => {
                    let result = FunctionDeclaration::create_function_declaration(program_string);
                    Ok(result)
                }
                _ => Err("No Function Declarations Found!".to_string()),
            };
        function_declaration
    }

    //I think this (and probably some other things) will be better outside of program later, possibly in a trait, since I'll use something similar for parsing function declaration and other statement blocks
    //I also suspect these three can be a generic

    fn self_add_body_types(&mut self, data_to_add: BodyTypes) {
        self.body.push(data_to_add);
    }
    pub fn loop_to_parse_program(&mut self, program_vec: Vec<String>) {
        for i in 0..program_vec.len() - 1 {
            let mutable_program_string = program_vec[i].clone();

            self.parse_program(&mutable_program_string, &program_vec[i]);
        }
    }
}

impl Default for Program {
    fn default() -> Program {
        Program {
            type_of: "Program".to_string(),
            start: 0,
            end: 0,
            body: Vec::new(),
        }
    }
}
