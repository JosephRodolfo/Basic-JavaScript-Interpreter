use regex::Regex;
use substring::Substring;
use crate::traits::CommaSeperatedList::CommaSeperatedList;
use crate::types::BlockStatement::BlockStatement;
use crate::types::Identifier::Identifier;

use crate::{
    helper_funcs
};




#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Vec<Identifier>,
    body: BlockStatement
}

impl FunctionDeclaration{

    
    pub fn create_function_declaration(program: String) -> FunctionDeclaration {
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
    
        let params_string_arr = FunctionDeclaration::create_string_vec(rest_params, ",");
        let params_arr = FunctionDeclaration::create_comma_seperated_array(params_string_arr).unwrap();
    
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: mat.end(),
            end: match_params.end(),
            name: func_name.to_string(),
        };
        let block_statement_string = rest.substring(match_params.end(), program.len());
        let new_block_statement = BlockStatement::create_block_statement(block_statement_string);

    
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

   
      
 

}

impl CommaSeperatedList<Identifier> for FunctionDeclaration {
    fn create_comma_seperated_array(string_vec: Vec<&str>) -> Result<Vec<Identifier>, String> {
        let result = string_vec
            .iter()
            .map(|e| {
                let new_identifier = Identifier {
                    name: e.to_string(),
                    start: 0,
                    end: 0,
                    type_of: "Identifier".to_string(),
                };
                new_identifier
            })
            .collect::<Vec<Identifier>>();

        Ok(result)
    }
}












