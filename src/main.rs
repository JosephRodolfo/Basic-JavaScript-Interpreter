use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use substring::Substring;
use uuid::Uuid;

use regex::{Match, Regex};

#[derive(Debug, Clone)]
struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Params,
}
#[derive(Debug, Clone)]
struct Identifier {
    type_of: String,
    start: usize,
    end: usize,
    name: String,
}
#[derive(Debug, Clone)]
struct Params {
    number: Vec<f64>,
    string: Vec<String>,
    bool: Vec<bool>,
}
#[derive(Debug)]
struct Program {
    type_of: String,
    start: usize,
    end: usize,
    VariableDeclaration: Vec<VariableDeclaration>,
    FunctionDeclaration: Vec<FunctionDeclaration>,
    ExpressionStatement: Vec<ExpressionStatement>,
}
#[derive(Debug)]
struct VariableDeclaration {}
#[derive(Debug)]
struct ExpressionStatement {}

impl Program {
    //eventually the main loop that will go through program string and turn it into AST and nodes, using 
    //looping
    fn parse_program(&mut self, program: String) {
        let mat = Regex::new("(function|const|let|if|for|console.log(\\(*)\\))")
            .unwrap()
            .find(&program)
            .expect("found no program");
        let string_for_match = program.substring(mat.start(), mat.end());
        if string_for_match == "const" || string_for_match == "let" {
        } else if string_for_match == "function" {
            let function_declaration =
                Program::match_function_declaration_start_parse("function", program)
                    .unwrap()
                    .clone();
            self.add_function_declaration(function_declaration);
            println!("{:?}", self)
        } else if string_for_match == "if"
            || string_for_match == "for"
            || string_for_match == "console.log()"
        {
        }
        //This one will check for existing variables to determine if valid expression.
    }
    //These three match functions will take the already decided type (func_dec, var_dec, expression_statement and call parser functions, returning the node)
    fn match_var_declaration_start_parse(type_of_string_for_match: &str, program_string: String) {
        match type_of_string_for_match {
            "const" => {
                // println!("found {}!", type_of_string_for_match)
            }
            "let" => {
                // println!("found {}!", type_of_string_for_match)
            }
            _ => {
                return;
            }
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
    fn add_function_declaration(&mut self, data_to_add: FunctionDeclaration) {
        self.FunctionDeclaration.push(data_to_add);
    }
    fn add_variable_declaration(&mut self, data_to_add: VariableDeclaration) {
        self.VariableDeclaration.push(data_to_add);
    }
    fn add_expression_statement(&mut self, data_to_add: ExpressionStatement) {
        self.ExpressionStatement.push(data_to_add);
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
fn main() {
    let file_string = read_file_line_by_line("src/test/test.txt");

    let trimmed = skip_space(&file_string);
    let mut program = Program {
        end: trimmed.len(),
        ..Default::default()
    };

    Program::parse_program(&mut program, trimmed);

    // parse_functions(&trimmed);
    // println!("{}", trimmed);
}

fn create_id() -> Uuid {
    let id = Uuid::new_v4();
    id
}
//parses function. right now takes whole program, starts at 0 and as far as I've gotten which is handling parameters, everything in the block should be reusable from main code
//
fn parse_functions(program: String) -> FunctionDeclaration {
    let program = skip_space(&program);
    //returns position everything after function
    let mat = Regex::new("(function)")
        .unwrap()
        .find(&program)
        .expect("found no function");
    //string of everything following function
    let rest = program.substring(mat.end(), program.len());

    let func_length_match = Regex::new("(\\})").unwrap().find(&rest).unwrap();

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

    let new_func = FunctionDeclaration {
        type_of: "FunctionDeclaration".to_string(),
        start: mat.start(),
        end: func_length_match.end(),
        params: params_arr,
        identifier: new_identifier,
    };
    // println!("{:?}", new_func);
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
            // println!("{}", "string found")
        }

        if (start_bool && !end_bool) || (!start_bool && end_bool) {
            return Err("Malformed string!".to_string());
        }
        println!("{}", first);

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
            // println!("{}", "string found")
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
//deletes whitespace
fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}
//opens file, returns in string format
fn read_file_line_by_line(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let string = lines.collect();
    string
}
