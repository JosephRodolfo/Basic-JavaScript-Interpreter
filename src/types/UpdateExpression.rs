use crate::{interpreter_types, traits, types};
use interpreter_types::Interpreter::Interpreter;
use regex::Regex;
use substring::Substring;
use traits::Evaluator::Evaluator;
use types::Identifier::Identifier;

#[derive(PartialEq, Debug, Clone)]
pub struct UpdateExpression {
    prefix: bool,
    type_of: String,
    start: usize,
    end: usize,
    operator: String,
  pub  argument: Identifier,
}

impl UpdateExpression {
    pub fn create_update_expression(string: &str) -> UpdateExpression {
        let mat = Regex::new("(\\+\\+|--)")
            .unwrap()
            .find(string)
            .expect("no update expression found");

        let operator = string.substring(mat.start(), mat.end());
        let (identifier_name, prefix_bool) = if mat.start() == 0 {
            (string.substring(mat.end(), string.len()), true)
        } else {
            (string.substring(0, mat.start()), false)
        };
        let new_identifier = Identifier {
            name: identifier_name.to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: prefix_bool,
            operator: operator.to_string(),
            argument: new_identifier,
        }
    }
}

impl Evaluator for UpdateExpression {
    //I'm realizing these evaluates need to be able to return an error. also multiple values
    //so they will eventually need to be Result<(String, Vars), String>
    fn evaluate_with_scope(&self, scope: &Interpreter) -> String {
        let name = self.argument.evaluate();
        let value = scope.lookup_for_eval(name);

        let result: Result<String, String> = match value {
            interpreter_types::VarsEnum::VarsEnum::Prim(value) => {

                let value_numeric = if value.parse::<f64>().is_ok(){

                    Ok(value)

                } else{
                    Err("NaN!".to_string())
                };  

                let result = match value_numeric {
                    Ok(value)=> value,
                    Err(e)=> panic!("NaN!: {}", e)
                };
                Ok(result)
                
            }
            interpreter_types::VarsEnum::VarsEnum::Obj(_) => Err("NaN!".to_string()),
            interpreter_types::VarsEnum::VarsEnum::Pointer(_) => Err("NaN!".to_string()),
        };

        let var_value = match result {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };

        let operator = match self.prefix {
            true => panic!("Prefix is not yet implemented!"),
            false => &self.operator,
        };

        let binary_eval = if operator == "++" {
            let sum = Self::simple_eval(&var_value, "+", "1");
            sum
        } else if operator == "--" {
            let difference = Self::simple_eval(&var_value, "-", "1");
            difference
        } else {
            panic!("Something went wrong in your update expression!")
        };
        binary_eval
    }

    fn evaluate(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{types, traits::Evaluator::Evaluator, interpreter_types};
    use types::{Identifier::Identifier, UpdateExpression::UpdateExpression};
    use interpreter_types::{Interpreter::Interpreter, Vars::Vars};

    #[test]
    fn test_update_expresssion_prefix_false() {
        let new_identifier = Identifier {
            name: "a".to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        let new_update_expression = UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: false,
            operator: "++".to_string(),
            argument: new_identifier,
        };

        let test_update_expression = UpdateExpression::create_update_expression("a++");
        assert_eq!(test_update_expression, new_update_expression);
    }
    #[test]
    fn test_update_expresssion_prefix_true() {
        let new_identifier = Identifier {
            name: "test".to_string(),
            start: 0,
            end: 0,
            type_of: "Identifier".to_string(),
        };
        let new_update_expression = UpdateExpression {
            type_of: "UpdateExpression".to_string(),
            start: 0,
            end: 0,
            prefix: true,
            operator: "--".to_string(),
            argument: new_identifier,
        };

        let test_update_expression = UpdateExpression::create_update_expression("--test");
        assert_eq!(test_update_expression, new_update_expression);
    }

    // #[test]

    // fn test_evaluate_update_expression(){
    //     let new_identifier = Identifier {
    //         name: "a".to_string(),
    //         start: 0,
    //         end: 0,
    //         type_of: "Identifier".to_string(),
    //     };
    //     let new_update_expression = UpdateExpression {
    //         type_of: "UpdateExpression".to_string(),
    //         start: 0,
    //         end: 0,
    //         prefix: false,
    //         operator: "++".to_string(),
    //         argument: new_identifier,
    //     };

    //    let interpreter = Interpreter::default();

    //    let vars = Vars{

    //    }
    //    interpreter.hash_stack.insert("a".to_string(), );
    //     new_update_expression.evaluate_with_scope(&interpreter);

    //     assert_eq!(, new_update_expression);

    // }
}
