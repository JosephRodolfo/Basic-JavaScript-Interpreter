use crate::{
    helper_funcs::rem_first_and_last,
    traits::{self, ExpressionTypes::ExpressionTypes},
    types,
};
use regex::Regex;
use substring::Substring;
use traits::CommaSeperatedList::CommaSeperatedList;
use types::{
    BlockStatement::BlockStatement, ExpressionStatement::ExpressionStatement,
    ExpressionType::ExpressionType, VariableDeclaration::VariableDeclaration,
};
#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    type_of: String,
    start: usize,
    end: usize,
    init: VariableDeclaration,
    test: ExpressionType,
    update: ExpressionType,
    body: BlockStatement,
}

impl ForStatement {
   pub fn create_for_statement(string: &str) -> ForStatement {
        let call_expression_regex = "(\\(.*\\))";
        let match_for_statement_args = Regex::new(&call_expression_regex)
            .unwrap()
            .find(string)
            .expect("not found");
        let args = string.substring(
            match_for_statement_args.start(),
            match_for_statement_args.end(),
        );

        let str_vec = ForStatement::create_string_vec(rem_first_and_last(args), ";");

        let variable_declaration = VariableDeclaration::create_variable_declaration(
            str_vec[0].to_string(),
            &"".to_string(),
        );
        let test_type = ExpressionStatement::check_expression_type(str_vec[1]);

        let test_type_binary_only = match test_type {
            Ok("binary_expression") => test_type,
            _ => panic!("Only binary expressions allowed!"),
        };
        let test =
            ExpressionStatement::create_expression_statement(test_type_binary_only, str_vec[1]);
        let update_type = ExpressionStatement::check_expression_type(str_vec[2]);
        let test_type_update_only = match update_type {
            Ok("update_expression") => update_type,
            _ => panic!("Only update expressions allowed!"),
        };
        let update =
            ExpressionStatement::create_expression_statement(test_type_update_only, str_vec[2]);

        let body_string = string.substring(match_for_statement_args.end(), string.len());
        let body = BlockStatement::create_block_statement(body_string);

        let new_for_statement = ForStatement {
            type_of: "ForStatement".to_string(),
            start: 0,
            end: 0,
            init: variable_declaration,
            test,
            update,
            body,
        };
        new_for_statement
    }
}

impl CommaSeperatedList<ExpressionType> for ForStatement {
    fn create_comma_seperated_array(_string_vec: Vec<&str>) -> Result<Vec<ExpressionType>, String> {
        let result: Vec<ExpressionType> = vec![];
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::types;
    use types::{
        BlockStatement::BlockStatement, ExpressionStatement::ExpressionStatement,
        ForStatement::ForStatement,
    };

    #[test]
    fn test_create_for_statement() {
        let test = ExpressionStatement::create_expression_statement(Ok("binary_expression"), "i<2");
        let update =
            ExpressionStatement::create_expression_statement(Ok("update_expression"), "i++");
        let variable_declaration =
            types::VariableDeclaration::VariableDeclaration::create_variable_declaration(
                "leti=2".to_string(),
                &"".to_string(),
            );

        let body = BlockStatement::create_block_statement("{vec+=3}");
        let test_for_statement = ForStatement {
            type_of: "ForStatement".to_string(),
            start: 0,
            end: 0,
            init: variable_declaration,
            test,
            update,
            body,
        };
        let new_for_statement = ForStatement::create_for_statement("for(leti=2;i<2;i++){vec+=3}");

        assert_eq!(test_for_statement, new_for_statement);
    }
}
