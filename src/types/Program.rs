use regex::Regex;
use substring::Substring;
use types::VariableInitTypes::VariableInitTypes;

use crate::{
    helper_funcs::{self, find_start_end, str_to_type},
    types, ExpressionStatement, FunctionDeclaration, Identifier, Params, VariableDeclaration, BlockStatement, Body,
};

use super::ExpressionStatement::CallExpression;

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
        if match_find == (0, 0) {
              let expression_statement = ExpressionStatement::check_expression_type(program);
              let result = match expression_statement {
                Err(e)=> e,
                Ok(result)=> result

              };

            }
              
            
        //       let parsed_expression_statement: ExpressionStatement = match result {




        //         "call_expression"=>{ let result = ExpressionStatement::create_call_expression(program);},
        //         "binary_expression"=>{let result = ExpressionStatement::create_binary_expression(program);}







        //       }
            // return Err("Out of things to parse in this item!".to_string());
        // }

        let string_for_match = program.substring(match_find.0, match_find.1);
        let end_position = if string_for_match == "const" || string_for_match == "let" || string_for_match == "var" {
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
            let result = function_declaration.end;

            self.add_function_declaration(function_declaration);
            // println!("string: {}", result);

            Some(result)

        }
        //these three (and others probably) will be there own statements
        else if string_for_match == "if"
       
        {
            None
        }
        else if string_for_match == "for"
       
        {
            None
        } else if string_for_match == "while"
       
        {
            None
        }  
        else {
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
                let result = Program::parse_variables(self, program_string, whole_program);
                Ok(result)
            }
            "let" => {
                let result = Program::parse_variables(self, program_string, whole_program);
                Ok(result)
            }
            "var" => {
                let result = Program::parse_variables(self, program_string, whole_program);
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
                    let result = parse_functions(program_string);
                    Ok(result)
                }
                _ => Err("No Function Declarations Found!".to_string()),
            };
        function_declaration
    }

    fn match_expression_statement_start_parse(
        type_of_string_for_match: &str,
        program_string: String,
    ) {
        match type_of_string_for_match {
            "if" => {
                let function = parse_functions(program_string);
            }
            "for" => {
                let function = parse_functions(program_string);
            }
            "console.log" => {
                let function = parse_functions(program_string);
            }
            _ => {
                // println!("{}", "NO EXPRESSION FOUND!");
            }
        }
    }

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
        for i in 0..program_vec.len() {
            let mut mutable_program_string = program_vec[i].clone();

            let result = Program::parse_program(self, &mutable_program_string, &program_vec[i]);
        }
    }

    pub fn lookup_var(&self, str_to_lookup: &str) -> Result<Option<VariableInitTypes>, String> {
        let mut test_vec: Vec<VariableInitTypes> = Vec::new();
        for i in 0..self.VariableDeclaration.len() {
            if self.VariableDeclaration[i].identifier.name == str_to_lookup {
                let found = self.VariableDeclaration[i].init.clone();
                test_vec.push(found)
            }
        }

        if test_vec.len() == 0 {
            let none_found = Err("No variable with that name found!".to_string());
            return none_found;
        }
        Ok(Some(test_vec[0].clone()))

        //I couldn't figure out how to implement FromIter trait on VariableInitTypes to be able to use collect() due to not knowing the field in advance. I suspect unfortunately it may
        //be because using an enum for this wasn't the best choice to begin with.
        // let var_type = self.VariableDeclaration.iter().filter(|e| e.identifier.name == str_to_lookup).collect();
    }
    //parse expressions, takes self, program (current string item in vector), and whole_program vector (which for now does nothing)
    fn parse_expressions(&self, program: String, whole_program: &String)-> ExpressionStatement{
        let mat = Regex::new("([<>]=?|=+|-|*|%|==|===|)")
            .unwrap()
            .find(&program)
            .expect("No expressions found!");




        unimplemented!()
    }

    fn parse_variables(&self, program: String, whole_program: &String) -> VariableDeclaration {
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

        //ignore this, completely broken post switching to using vecs vs. 1 long string, will revist.
        let start_end_whole = find_start_end(whole_program.to_string(), name);

        //get var value, what follows assignment operator
        let value = program.substring(after_equal.end(), program.len());
        //returns str of type of var ("number", "bool", etc., takes value as param(or what's after assignment operator))
        let type_of_var = str_to_type(value).unwrap();

        //if type return was lookup, it means there was a variable found. Program uses it's lookup method
        //to see if var is present. If so, returns it. Else, returns None. If a real type was found,
        //uses Program's match_type_from_str method to return a VariableInitType enum object with appropriate type value
        let var_value: VariableInitTypes = if type_of_var == "lookup" {
            let result = Program::lookup_var(self, value);
            let found_or_not_var: Result<VariableInitTypes, String> = match result {
                Ok(e) => Ok(e.unwrap()),
                Err(_e) => Err("No variable with that name found!".to_string()),
            };
            //handle variable not found error here in the future
            found_or_not_var.unwrap()
        } else {
            let result = types::VariableInitTypes::VariableInitTypes::match_type_from_str(
                type_of_var,
                value,
            )
            .unwrap();
            result
        };
        //finds the end of the current variable declaration, by finding the start of the next statement or declaration;
        //if nothing comes after current variable declaration, returns 0, though I guess that would mean you could
        //abort current operation because variable would be unreachable?
        // fn find_next_statement_expression(program: &str, start_position: usize) -> (usize) {
        //     let first_cut = program.substring(start_position, program.len());
        //     let mat = Regex::new("(const|let|function|while|if|console.log|for)")
        //         .unwrap()
        //         .find(first_cut);
        //     // println!("{:?}", mat);

        //     let result = match mat {
        //         Some(x) => x.start(),
        //         None => 0,
        //     };

        //     let difference = program.len() - first_cut.len();
        //     // if result !=0{
        //     // println!("result: {}, string {} ", result, first_cut);
        //     // }
        //     result + difference
        // }

        let new_var_declaration_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: start_end_whole.0,
            end: start_end_whole.1,
            name: name.to_string(),
        };

        let new_var_declaration = VariableDeclaration {
            type_of: "VariableDeclaration".to_string(),
            start: mat.start(), 
            end: program.len(),
            kind: var_keyword.to_string(), 
            identifier: new_var_declaration_identifier,
            init: var_value, 
        };
        new_var_declaration
    }
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
fn parse_functions(program: String) -> FunctionDeclaration {
    let program = helper_funcs::skip_space(&program);
    //returns position everything after function
    let mat = Regex::new("(function)")
        .unwrap()
        .find(&program)
        .expect("found no function");
    //string of everything following function keyword
    let rest = program.substring(mat.end(), program.len());

    let func_length_match = Regex::new("(\\})").unwrap().find(&program).unwrap();

    let match_params = Regex::new("(\\()").unwrap().find(&program).unwrap();

    let func_name = program.substring(mat.end(), match_params.end() - 1);

    // params position following function
    let match_params = Regex::new("(\\(.*\\))").unwrap().find(&rest).unwrap();

    //string of params
    let rest_params = rest.substring(match_params.start() + 1, match_params.end() - 1);

    let params_arr = create_params_array_declaration(rest_params).unwrap();

    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: mat.end(),
        end: match_params.end(),
        name: func_name.to_string(),
    };

    let mut new_function_body = Body {
        ..Default::default()
    };

    let new_block_statement = BlockStatement{
        type_of: "BlockStatement".to_string(),
        start: 0,
        end: program.len(),
        body: new_function_body
    };

    let new_func = FunctionDeclaration {
        type_of: "FunctionDeclaration".to_string(),
        start: mat.start(),
        end: func_length_match.end(),
        params: params_arr,
        identifier: new_identifier,
        body: new_block_statement
    };
    return new_func;
}
//these two create_param_array ... functions are for the moment just copies of each other, need to be reworked.
//the idea is to return a Params object consisting of vectors of types
fn create_params_array_declaration(string: &str) -> Result<Params, String> {
    let args_count = string.matches(',').count() + 1;

    let mut new_params = Params {
        number: Vec::new(),
        string: Vec::new(),
        bool: Vec::new(),
    };
    let mut temp_string = string;
    for i in 0..args_count {
        let formatted = format!("{})", temp_string);

        let match_params = if i == args_count - 1 {
            let x = format!("{}", "(^*$)");
            let match_return = Regex::new(&x).unwrap().find(&formatted).expect("not found");
            match_return
        } else {
            let x = format!("{}", "(,)");
            let match_return = Regex::new(&x)
                .unwrap()
                .find(&temp_string)
                .expect("not found");
            match_return
        };

        let first = temp_string.substring(0, match_params.end() - 1).to_string();
        let start_bool = first.chars().next().unwrap_or_default() == '"';
        let end_bool = first.chars().last().unwrap_or_default() == '"';
        if start_bool && end_bool {
            new_params.string.push(first.to_string());
        }

        if (start_bool && !end_bool) || (!start_bool && end_bool) {
            return Err("Malformed string!".to_string());
        }

        if first.parse::<f64>().is_ok() {
            new_params.number.push(first.parse::<f64>().unwrap());
        } else if first == true.to_string() || first == false.to_string() {
            new_params.bool.push(first.parse::<bool>().unwrap());
        }
        let next_string = temp_string.substring(match_params.end(), temp_string.len() + 1);

        temp_string = next_string;
    }

    Ok(new_params)
}

fn create_params_array_expression(string: &str, name: &str) -> Result<Params, String> {
    let args_count = string.matches(',').count() + 1;

    let mut new_params = Params {
        number: Vec::new(),
        string: Vec::new(),
        bool: Vec::new(),
    };
    let mut temp_string = string;
    for i in 0..args_count {
        let formatted = format!("{})", temp_string);

        let match_params = if i == args_count - 1 {
            let x = format!("{}", "(^*$)");
            let match_return = Regex::new(&x).unwrap().find(&formatted).expect("not found");
            match_return
        } else {
            let x = format!("{}", "(,)");
            let match_return = Regex::new(&x)
                .unwrap()
                .find(&temp_string)
                .expect("not found");
            match_return
        };

        let first = temp_string.substring(0, match_params.end() - 1).to_string();

        let start_bool = first.chars().next().unwrap_or_default() == '"';
        let end_bool = first.chars().last().unwrap_or_default() == '"';
        if start_bool && end_bool {
            new_params.string.push(first.to_string());
        }

        if (start_bool && !end_bool) || (!start_bool && end_bool) {
            return Err("Malformed string!".to_string());
        }

        if first.parse::<f64>().is_ok() {
            new_params.number.push(first.parse::<f64>().unwrap());
        } else if first == true.to_string() || first == false.to_string() {
            new_params.bool.push(first.parse::<bool>().unwrap());
        }
        let next_string = temp_string.substring(match_params.end(), temp_string.len() + 1);
        // println!("next string: {}", string);

        temp_string = next_string;
    }

    Ok(new_params)
}
