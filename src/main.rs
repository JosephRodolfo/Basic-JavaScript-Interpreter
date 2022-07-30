use std::array;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseFloatError;
use std::path::Path;
use substring::Substring;

use regex::Regex;

struct FunctionDeclaration {
    typeOf: String,
    start: i32,
    end: i32,
    identifier: Identifier,
    params: Vec<String>,
}

struct Identifier {
    type_of: String,
    start: i32,
    end: i32,
    name: String,
}

struct Params {
    number: Vec<f64>,
    string: Vec<String>,
    bool: Vec<bool>,
}

fn main() {
    let file_string = read_file_line_by_line("src/test/test.txt");
    let trimmed = skip_space(&file_string);
    parse_functions(&trimmed);
    // println!("{}", trimmed);
}

fn parse_functions(program: &str) {
    let program = skip_space(program);
    //returns position everything after function
    let mat = Regex::new("(function)")
        .unwrap()
        .find(&program)
        .expect("found no function");
    //string of everything following function
    let rest = program.substring(mat.end(), program.len());
    // params position following function
    let match_params = Regex::new("(\\(.*\\))").unwrap().find(&rest).unwrap();

    //string of params
    let rest_params = rest.substring(match_params.start() + 1, match_params.end() - 1);

    create_params_array(rest_params, "params");

    // println!("{}", rest_params)
}

fn create_params_array(string: &str, name: &str) -> Params {
    let args_count = string.matches(',').count();
    let match_params = Regex::new("(,)")
        .unwrap()
        .find(&string)
        .expect("found no params");

    let mut new_params = Params {
        number: Vec::new(),
        string: Vec::new(),
        bool: Vec::new(),
    };

    for i in 0..args_count {
        let first = string.substring(0, match_params.end() - 1).to_string();

        if first.chars().next().unwrap() == '"' && first.chars().last().unwrap() == '"' {
            new_params.string.push(first.to_string());
        }

        if first.parse::<f64>().is_ok() {
            new_params.number.push(first.parse::<f64>().unwrap());
        } else if first == "true" || first == "false" {
            new_params.bool.push(first.parse::<bool>().unwrap());
        }
    }
    return new_params;
}

// function parseExpression(program) {
//     program = skipSpace(program);
//     let match, expr;
//     if (match = /^"([^"]*)"/.exec(program)) {
//       expr = {type: "value", value: match[1]};
//     } else if (match = /^\d+\b/.exec(program)) {
//       expr = {type: "value", value: Number(match[0])};
//     } else if (match = /^[^\s(),#"]+/.exec(program)) {
//       expr = {type: "word", name: match[0]};
//     } else {
//       throw new SyntaxError("Unexpected syntax: " + program);
//     }

//     return parseApply(expr, program.slice(match[0].length));
//   }

fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}

fn read_file_line_by_line(filepath: &str) -> String {
    let mut vec: Vec<String> = Vec::new();
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let string = lines.collect();
    string
}
