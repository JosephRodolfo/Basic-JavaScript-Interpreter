use regex::Regex;
use substring::Substring;
use types::Literal::Literal;
use crate::helper_funcs::str_to_type_inc_parentheses;
use crate::types::BlockStatement::BlockStatement;
use crate::{
    types, Identifier, helper_funcs, Body,
};




#[derive(Debug)]
pub struct FunctionDeclaration {
    type_of: String,
    start: usize,
    end: usize,
    identifier: Identifier,
    params: Vec<Identifier>,
    body: BlockStatement
}
impl FunctionDeclaration{

    fn check_function_type(
        type_of_string_for_match: &str,
        program_string: String,
    ) -> Result<&str, &str> {
        let function_declaration: Result<&str, &str> =
            match type_of_string_for_match {
                "function" =>   Ok("function"),
                
                _ => panic!("Problem with variable declaration!"),
            };
        function_declaration
    }

    
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
    
        let params_arr = FunctionDeclaration::create_params_array_declaration(rest_params).unwrap();
    
        let new_identifier = Identifier {
            type_of: "Identifier".to_string(),
            start: mat.end(),
            end: match_params.end(),
            name: func_name.to_string(),
        };
    
        let mut new_function_body = Body {
            ..Default::default()
        };
    
        let new_block_statement = BlockStatement {
            type_of: "BlockStatement".to_string(),
            start: 0,
            end: program.len(),
            body: new_function_body,
        };
    
        let new_func = FunctionDeclaration {
            type_of: "FunctionDeclaration".to_string(),
            start: mat.start(),
            end: func_length_match.end(),
            params: params_arr,
            identifier: new_identifier,
            body: new_block_statement,
        };
        return new_func;
    }

    fn create_params_array_declaration(string: &str) -> Result<Vec<Identifier>, String> {
        let args_count = string.matches(',').count() + 1;
    
           let mut params_vec: Vec<Identifier>=Vec::new();
        
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
    
            let first = temp_string.substring(0, match_params.end() - 1);
            
            let param = str_to_type_inc_parentheses(first);
          
            match param {
            "identifier"=>{
                
                let new_param = Identifier {
                    type_of:"Identifier".to_string(),
                    start:0,
                    end:0,
                    name: first.to_string()
                };
                params_vec.push(new_param)},
            _=>{panic!("Param malformed!")}
            }
            temp_string = temp_string.substring(match_params.end(), temp_string.len() + 1);
    
        }
    
        Ok(params_vec)
    }

}













