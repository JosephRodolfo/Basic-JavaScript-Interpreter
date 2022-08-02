mod helper_funcs;
use helper_funcs::{read_file_line_by_line, skip_space};
mod types;
use types::Program::Program;

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

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Params,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    kind: String,
    init: types::VariableInitTypes::VariableInitTypes,
}
#[derive(Debug)]
pub struct ExpressionStatement {}

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
