mod helper_funcs;
use helper_funcs::{read_file_line_by_line, string_array_to_vec};
mod traits;
mod types;
use types::{
    ExpressionStatement::ExpressionStatement, FunctionDeclaration::FunctionDeclaration,
    Identifier::Identifier, Program::Program, VariableDeclaration::VariableDeclaration,
};

fn main() {
    let file_string = read_file_line_by_line("src/test/test.txt");
    let length = file_string.len();
    let file_vec = string_array_to_vec(file_string);

    let mut program = Program {
        end: length,
        ..Default::default()
    };

    program.loop_to_parse_program(file_vec);
    println!("{:#?}", program)
}
