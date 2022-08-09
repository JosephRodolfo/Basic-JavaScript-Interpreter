use regex::Regex;
use substring::Substring;
use types::VariableInitTypes::VariableInitTypes;

use crate::{
    helper_funcs::{self, find_start_end, str_to_type},
    types, Body, ExpressionStatement, FunctionDeclaration, Identifier, VariableDeclaration,
};

#[derive(Debug)]
pub struct Program {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub VariableDeclaration: Vec<VariableDeclaration>,
    pub FunctionDeclaration: Vec<FunctionDeclaration>,
    pub ExpressionStatement: Vec<ExpressionStatement>,
}

impl Program {
    //eventually the main parsing loop that will go through program string and turn it into AST and nodes, using
    //looping
    fn parse_program(
        &mut self,
        program: &String,
        whole_program: &String,
    ) -> Result<Option<usize>, String> {
        //this needs to be changed to match exact, beginning of string
        let mat = Regex::new("(function|const|let|var|if|for|while|console.log(\\(*)\\))")
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
                _ => ExpressionStatement::create_binary_expression(program),
            };

            self.add_expression_statement(parsed_expression_statement);
            return Err("Out of things to parse in this item!".to_string());
        }

        let string_for_match = program.substring(match_find.0, match_find.1);
        let end_position = if string_for_match == "const"
            || string_for_match == "let"
            || string_for_match == "var"
        {
            let variable_declaration = Program::match_var_declaration_start_parse(
                self,
                string_for_match,
                program.to_string(),
                whole_program,
            )
            .unwrap();
            let result = variable_declaration.end;

            self.add_variable_declaration(variable_declaration);

            Some(result)
        } else if string_for_match == "function" {
            let function_declaration = Program::match_function_declaration_start_parse(
                string_for_match,
                program.to_string(),
            )
            .unwrap();

            self.add_function_declaration(function_declaration);
            // println!("string: {}", result);

            None
        }
        //these three (and others probably) will be there own statements
        else if string_for_match == "if" {
            None
        } else if string_for_match == "for" {
            None
        } else if string_for_match == "while" {
            None
        } else {
            None
        };
        Ok(end_position)
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
            "const" => {
                let result =
                    VariableDeclaration::create_variable_declaration(program_string, whole_program);
                Ok(result)
            }
            "let" => {
                let result =
                    VariableDeclaration::create_variable_declaration(program_string, whole_program);
                Ok(result)
            }
            "var" => {
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

    // fn match_expression_statement_start_parse(
    //     type_of_string_for_match: &str,
    //     program_string: String,
    // ) {
    //     match type_of_string_for_match {
    //         "if" => {
    //             let function = parse_functions(program_string);
    //         }
    //         "for" => {
    //             let function = parse_functions(program_string);
    //         }
    //         "console.log" => {
    //             let function = parse_functions(program_string);
    //         }
    //         _ => {
    //             // println!("{}", "NO EXPRESSION FOUND!");
    //         }
    //     }
    // }

    //I think this (and probably some other things) will be better outside of program later, possibly in a trait, since I'll use something similar for parsing function declaration and other statement blocks
    //I also suspect these three can be a generic
    fn add_function_declaration(&mut self, data_to_add: FunctionDeclaration) {
        self.FunctionDeclaration.push(data_to_add);
    }
    fn add_variable_declaration(&mut self, data_to_add: VariableDeclaration) {
        self.VariableDeclaration.push(data_to_add);
    }
    fn add_expression_statement(&mut self, data_to_add: ExpressionStatement) {
        self.ExpressionStatement.push(data_to_add);
    }
    pub fn loop_to_parse_program(&mut self, program_vec: Vec<String>) {
        for i in 0..program_vec.len() - 1 {
            let mutable_program_string = program_vec[i].clone();

            let result = Program::parse_program(self, &mutable_program_string, &program_vec[i]);
        }
    }

    // pub fn lookup_var(&self, str_to_lookup: &str) -> Result<Option<VariableInitTypes>, String> {
    //     let mut test_vec: Vec<VariableInitTypes> = Vec::new();
    //     for i in 0..self.VariableDeclaration.len() {
    //         if self.VariableDeclaration[i]::Identifier.name == str_to_lookup {
    //             let found = self.VariableDeclaration[i].init.clone();
    //             test_vec.push(found)
    //         }
    //     }

    //     if test_vec.len() == 0 {
    //         let none_found = Err("No variable with that name found!".to_string());
    //         return none_found;
    //     }
    //     Ok(Some(test_vec[0].clone()))

    //     //I couldn't figure out how to implement FromIter trait on VariableInitTypes to be able to use collect() due to not knowing the field in advance. I suspect unfortunately it may
    //     //be because using an enum for this wasn't the best choice to begin with.
    //     // let var_type = self.VariableDeclaration.iter().filter(|e| e.identifier.name == str_to_lookup).collect();
    // }
    //parse expressions, takes self, program (current string item in vector), and whole_program vector (which for now does nothing)
    // fn parse_expressions(&self, program: String, whole_program: &String) -> ExpressionStatement {
    //     let mat = Regex::new("([<>]=?|=+|-|*|%|==|===|)")
    //         .unwrap()
    //         .find(&program)
    //         .expect("No expressions found!");

    //     unimplemented!()
    // }
}

impl Default for Program {
    fn default() -> Program {
        Program {
            type_of: "Program".to_string(),
            start: 0,
            end: 0,
            VariableDeclaration: Vec::new(),
            FunctionDeclaration: Vec::new(),
            ExpressionStatement: Vec::new(),
        }
    }
}

//parses function. right now takes whole program, starts at 0 and as far as I've gotten which is handling parameters, everything in the block should be reusable from main Program
//returns FunctionDeclaration node.

