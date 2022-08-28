use crate::interpreter_types::Interpreter::Interpreter;
use crate::{
    helper_funcs::{rem_first_and_last, str_to_type},
    traits::{
        CommaSeperatedList::CommaSeperatedList, Evaluator::Evaluator,
        ExpressionTypes::ExpressionTypes,
    },
    types,
};
use regex::Regex;
use substring::Substring;
use types::{
    ExpressionStatement::ExpressionStatement, ExpressionType::ExpressionType,
    Identifier::Identifier
};

#[derive(PartialEq, Debug, Clone)]
pub struct CallExpression {
  pub  args: Vec<ExpressionType>,
    start: usize,
    end: usize,
    pub callee: Identifier,
    type_of: String,
}

impl CallExpression {
    pub fn create_generic_expression(program: &str) -> CallExpression {
        //finds parentehses
        let call_expression_regex = "(\\(.*\\))$";
        let match_call_expression = Regex::new(&call_expression_regex)
            .unwrap()
            .find(program)
            .expect("not found");
        let function_name = program.substring(0, match_call_expression.start());
        //gets args str
        let params_str =
            program.substring(match_call_expression.start(), match_call_expression.end());
        //sorts args into literals and identifiers
        let string_vec = Self::create_string_vec(rem_first_and_last(params_str), ",");
        let args_vec = Self::create_comma_seperated_array(string_vec).unwrap();

        //creates call expression identifier
        let new_identifier = Identifier {
            start: 0,
            end: match_call_expression.end(),
            type_of: "Identifier".to_string(),
            name: function_name.to_string(),
        };

        //creates call expression
        let new_call_expression = CallExpression {
            type_of: "CallExpression".to_string(),
            callee: new_identifier,
            start: 0,
            end: program.len(),
            args: args_vec,
        };
        //returns call expression argument
        new_call_expression
    }


}

impl CommaSeperatedList<ExpressionType> for CallExpression {
    fn create_comma_seperated_array(string_vec: Vec<&str>) -> Result<Vec<ExpressionType>, String> {
        let result = string_vec
            .iter()
            .map(|e| {
                let expression_type = ExpressionType::check_expression_type(e);
                ExpressionStatement::create_expression_statement(expression_type, e)
            })
            .collect::<Vec<ExpressionType>>();
        Ok(result)
    }
}

impl Evaluator for CallExpression {
    fn evaluate_with_scope(&self, scope: &Interpreter) -> String {
        let name = self.callee.evaluate();

       let retrieved_value = scope.lookup_for_eval(name);


        todo!()
    }

    fn evaluate(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::traits;
    use crate::types;
    use crate::types::ExpressionType::ExpressionType;
    use traits::CommaSeperatedList::CommaSeperatedList;
    use types::{CallExpression::CallExpression, Identifier::Identifier, Literal::Literal};
    #[test]
    fn test_create_identifiers_arrays() {
        let x = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        };
        let y = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "y".to_string(),
        };
        let z = Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "z".to_string(),
        };
        let test_vec: Vec<ExpressionType> = vec![
            ExpressionType::Identifier(x),
            ExpressionType::Identifier(y),
            ExpressionType::Identifier(z),
        ];
        let string = CallExpression::create_comma_seperated_array(vec!["x", "y", "z"]).unwrap();
        assert_eq!(string, test_vec);
    }
}
