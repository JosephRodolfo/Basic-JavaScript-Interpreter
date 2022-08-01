mod helper_funcs;
use helper_funcs::{find_start_end, read_file_line_by_line, skip_space, str_to_type};
use substring::Substring;
mod types;
use types::VariableInitTypes::VariableInitTypes;
use regex::Regex;

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

#[derive(Debug, Clone)]
struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Params,
}




#[derive(Debug)]
struct VariableDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    kind: String,
    init: types::VariableInitTypes::VariableInitTypes,
}
#[derive(Debug)]
struct ExpressionStatement {}

impl Program {
    //eventually the main parsing loop that will go through program string and turn it into AST and nodes, using
    //looping
    fn parse_program(
        &mut self,
        program: &String,
        whole_program: &String,
    ) -> Result<Option<usize>, String> {
        let mat = Regex::new("(function|const|let|if|for|console.log(\\(*)\\))")
            .unwrap()
            .find(&program);

        let match_find = match mat {
            Some(x) => (x.start(), x.end()),
            None => (0, 0),
        };

        if match_find == (0, 0) {
            return Err("Out of things to parse in this item!".to_string());
        }

        let string_for_match = program.substring(match_find.0, match_find.1);
        let end_position = if string_for_match == "const" || string_for_match == "let" {
            let variable_declaration = Program::match_var_declaration_start_parse(
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
        } else if string_for_match == "if"
            || string_for_match == "for"
            || string_for_match == "console.log()"
        {
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
        type_of_string_for_match: &str,
        program_string: String,
        whole_program: &String,
    ) -> Result<VariableDeclaration, String> {
        match type_of_string_for_match {
            "const" => {
                let result = parse_variables(program_string, whole_program);
                Ok(result)
            }
            "let" => {
                let result = parse_variables(program_string, whole_program);
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
    fn loop_to_parse_program(&mut self, program_vec: Vec<String>) {
        for i in 0..program_vec.len() {
            let mut mutable_program_string = program_vec[i].clone();

            let result = Program::parse_program(self, &mutable_program_string, &program_vec[i]);
        }
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
    let length = file_string.len();
    let file_vec = string_array_to_vec(file_string);

    let mut program = Program {
        end: length,
        ..Default::default()
    };
    Program::loop_to_parse_program(&mut program, file_vec);
    println!("{:#?}", program)
}

fn parse_variables(program: String, whole_program: &String) -> VariableDeclaration {
    let mat = Regex::new("(const|let)")
        .unwrap()
        .find(&program)
        .expect("found no variables");

    let after_equal = Regex::new("(=)")
        .unwrap()
        .find(&program)
        .expect("found no assignment operator");
    let name = program.substring(mat.end(), after_equal.start());

    //ignore this.
    let start_end_whole = find_start_end(whole_program, name);

    //VariableDeclaration End position, mat.start() is the beginning
    let end_current_var = find_next_statement_expression(&program, mat.end());

    let value = program.substring(after_equal.end(), end_current_var);

    let type_of_var = str_to_type(value).unwrap_or_default();

    //finds the end of the current variable declaration, by finding the start of the next statement or declaration;
    //if nothing comes after current variable declaration, returns 0, though I guess that would mean you could
    //abort current operation because variable would be unreachable?
    fn find_next_statement_expression(program: &str, start_position: usize) -> (usize) {
        let first_cut = program.substring(start_position, program.len());
        let mat = Regex::new("(const|let|function|while|if|console.log|for)")
            .unwrap()
            .find(first_cut);
        println!("{:?}", mat);

        let result = match mat {
            Some(x) => x.start(),
            None => 0,
        };

        let difference = program.len() - first_cut.len();
        // if result !=0{
        // println!("result: {}, string {} ", result, first_cut);
        // }
        result + difference
    }

    let new_var_declaration_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: start_end_whole.0,
        end: start_end_whole.1,
        name: name.to_string(),
    };

    let new_var_declaration = VariableDeclaration {
        type_of: "VariableDeclaration".to_string(),
        start: mat.start(),
        end: end_current_var,
        kind: "let".to_string(), //placeholder
        identifier: new_var_declaration_identifier,
        init: types::VariableInitTypes::VariableInitTypes::Number(9.0), //placeholder
    };
    new_var_declaration
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
    //string of everything following function
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

    let new_func = FunctionDeclaration {
        type_of: "FunctionDeclaration".to_string(),
        start: mat.start(),
        end: func_length_match.end(),
        params: params_arr,
        identifier: new_identifier,
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

//takes program string, converts it to array of statements. It skips what's inside blocks
//that functinoality will be moved out to be used when parsing blocks;
//I know the way I did it with chars causes issues I will need to address later, but I don't think they'll be a problem given the limited scope of this project.
fn string_array_to_vec(string: String) -> Vec<String> {
    let mut trimmed = skip_space(&string);
    let mut new_string = trimmed.to_string();

    let mut count: (i32, bool) = (0, false);

    let left_curly: &str = "{";
    let right_curly: &str = "}";

    let left_curly_char = left_curly.chars().next().unwrap();
    let right_curly_char = right_curly.chars().next().unwrap();
    let mut semicolon_matches_vec: Vec<usize> = Vec::new();

    for (i, c) in trimmed.chars().enumerate() {
        if c == left_curly_char {
            count.0 = count.0 + 1;
            count.1 = true;
        }
        if c == right_curly_char {
            count.0 = count.0 - 1;
        }

        if count.0 == 0 {
            count.1 = false;
        }

        if count.1 && c == ";".chars().next().unwrap() {
            new_string.replace_range(i..i + 1, "~");
            semicolon_matches_vec.push(i);
        }
    }
    let result: Vec<String> = new_string
        .split(";")
        .map(|e| e.replace("~", ";").to_string())
        .collect();

    result
}
