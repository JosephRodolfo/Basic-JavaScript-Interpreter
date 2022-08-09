mod helper_funcs;
use helper_funcs::{read_file_line_by_line, string_array_to_vec};
mod types;
use types::{Program::Program, ExpressionStatement::ExpressionStatement, FunctionDeclaration::FunctionDeclaration, VariableDeclaration::VariableDeclaration, Identifier::Identifier};




#[derive(Debug)]
pub struct Body {
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
    println!("{:#?}", program)
}

