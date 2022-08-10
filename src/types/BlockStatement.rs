use crate::helper_funcs::rem_first_and_last;
use crate::types::ExpressionType;
use crate::{helper_funcs::string_array_to_vec, types};
use types::BodyTypes::BodyTypes;
use types::ReturnStatement::ReturnStatement;
use types::Program::Program;
#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub body: Vec<BodyTypes>,
}

impl BlockStatement {
    pub fn create_block_statement(string: &str) -> BlockStatement {



        let removed = rem_first_and_last(string);


        let length = string.len();
        let file_vec = string_array_to_vec(removed.to_string());
    
        let mut program = Program {
            end: length,
            ..Default::default()
        };

        program.loop_to_parse_program(file_vec);









        let new_block_statement = BlockStatement {
            type_of: "BlockStatement".to_string(),
            start: 0,
            end: length,
            body: program.body
        };

        new_block_statement
    }
}


#[test]
fn test_create_block_statement() {

 

   let return_statement = ReturnStatement::create_return_statement("returnx+2");
   let new_body_type = BodyTypes::ReturnStatement(return_statement);
   let new_vec = vec![new_body_type];
   let new_block_statement = BlockStatement {
    type_of: "BlockStatement".to_string(),
    start: 0,
    end: "{returnx+2;}".len(),
    body: new_vec,
};
  let test_block_statement = BlockStatement::create_block_statement("{returnx+2;}");
    assert_eq!(test_block_statement, new_block_statement);
}