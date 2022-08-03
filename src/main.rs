mod helper_funcs;
use helper_funcs::{read_file_line_by_line, string_array_to_vec};
mod types;
use types::{Program::Program, ExpressionStatement::ExpressionStatement, VariableDeclaration::VariableDeclaration};
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {

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
pub struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Params,
    body: BlockStatement
}
#[derive(Debug)]
pub struct BlockStatement {
    type_of: String,
    start: usize,
    end: usize,
    body: Body
}
#[derive(Debug)]
struct Body {
    FunctionDeclaration: Vec<FunctionDeclaration>,
    VariableDeclaration: Vec<VariableDeclaration>,
    ExpressionStatement: Vec<ExpressionStatement>,
  }

  impl Default for Body {
    fn default() -> Body {
        Body {
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
    // println!("{:#?}", program)
}

