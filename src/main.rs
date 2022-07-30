use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use substring::Substring;
use uuid::Uuid;

use regex::{Match, Regex};

#[derive(Debug)]
struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Params,
}
#[derive(Debug)]
struct Identifier {
    type_of: String,
    start: usize,
    end: usize,
    name: String,
}
#[derive(Debug)]
struct Params {
    number: Vec<f64>,
    string: Vec<String>,
    bool: Vec<bool>,
}

struct Program {
    type_of: String,
    start: usize,
    end: usize,
    VariableDeclaration: Vec<VariableDeclaration>,
    ExpressionDeclaration: Vec<ExpressionDeclaration>,
    ExpressionStatement: Vec<ExpressionStatement>,
}

struct VariableDeclaration {}
struct ExpressionDeclaration{}
struct ExpressionStatement{}

fn main() {
    let file_string = read_file_line_by_line("src/test/test.txt");
    let trimmed = skip_space(&file_string);
    parse_functions(&trimmed);
    // println!("{}", trimmed);
}

fn create_id() -> Uuid {
    let id = Uuid::new_v4();
    id
}



fn parse_functions(program: &str) -> FunctionDeclaration {
    let program = skip_space(program);
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
    println!("{:?}", new_func);
    return new_func;
}

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
        println!("next string: {}", string);

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
        println!("{}", first);

        if first.parse::<f64>().is_ok() {
            new_params.number.push(first.parse::<f64>().unwrap());
        } else if first == true.to_string() || first == false.to_string() {
            new_params.bool.push(first.parse::<bool>().unwrap());
        }
        let next_string = temp_string.substring(match_params.end(), temp_string.len() + 1);
        println!("next string: {}", string);

        temp_string = next_string;
    }

    Ok(new_params)
}

fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}

fn read_file_line_by_line(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let string = lines.collect();
    string
}
