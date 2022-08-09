use crate::{
    helper_funcs::{rem_first_and_last, str_to_type},
    types,
};
use regex::Regex;
use substring::Substring;
use types::Identifier::Identifier;
use types::Literal::Literal;

#[derive(PartialEq, Debug, Clone)]
pub struct CallExpression {
    args_literal: Vec<Literal>,
    args_identifier: Vec<Identifier>,
    start: usize,
    end: usize,
    callee: Identifier,
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
        let sorted_tuple = Self::sort_identifier_literal(params_str);
        //creates call expression identifier
        let new_identifier = Identifier {
            start: 0,
            end: match_call_expression.end(),
            type_of: "Identifier".to_string(),
            name: function_name.to_string(),
        };
        //creates literals and identifiers vectors, Vec<Literal> or Vec<Identifier>
        let identifiers_vec = Self::create_identifiers_vec(sorted_tuple.0);
        let literals_vec = Self::create_literals_vec(sorted_tuple.1);
        //creates call expression
        let new_call_expression = CallExpression {
            type_of: "CallExpression".to_string(),
            callee: new_identifier,
            start: 0,
            end: program.len(),
            args_literal: literals_vec,
            args_identifier: identifiers_vec,
        };
        //returns call expression argument
        new_call_expression
    }

    //takes str, returns tuple of Strings, 0 representing literals, 1 representing identifiers
    fn sort_identifier_literal(string: &str) -> (Vec<String>, Vec<String>) {
        let args_count = string.matches(',').count() + 1;
        let mut vec_literals = Vec::new();
        let mut vec_identifiers = Vec::new();

        let mut temp_string = rem_first_and_last(string);
        for i in 0..args_count {
            let formatted = format!("{})", temp_string);
            let match_params = if i == args_count - 1 {
                let x = format!("{}", "(^.*$)");
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

            let value = temp_string.substring(0, match_params.end() - 1).to_string();

            let type_of = str_to_type(&value).unwrap();
            if type_of == "lookup" {
                vec_identifiers.push(value);
            } else {
                vec_literals.push(value);
            }
            let next_string = temp_string.substring(match_params.end(), temp_string.len() + 1);

            temp_string = next_string;
        }
        (vec_identifiers, vec_literals)
    }

    fn create_literals_vec(vec: Vec<String>) -> Vec<Literal> {
        let mut literals_vec = Vec::new();
        for i in 0..vec.len() {
            let new_literal = Literal {
                type_of: "Literal".to_string(),
                start: 0,
                end: 0,
                value: vec[i].to_string(),
            };
            literals_vec.push(new_literal);
        }
        literals_vec
    }
    fn create_identifiers_vec(vec: Vec<String>) -> Vec<Identifier> {
        let mut identifier_vec = Vec::new();
        for i in 0..vec.len() {
            let new_identifier = Identifier {
                type_of: "Identifier".to_string(),
                start: 0,
                end: 0,
                name: vec[i].to_string(),
            };
            identifier_vec.push(new_identifier);
        }
        identifier_vec
    }
}
#[test]
fn test_create_identifiers_arrays() {
    let test_vec: Vec<Identifier> = vec![
        Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "x".to_string(),
        },
        Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "y".to_string(),
        },
        Identifier {
            type_of: "Identifier".to_string(),
            start: 0,
            end: 0,
            name: "z".to_string(),
        },
    ];
    let string = CallExpression::create_identifiers_vec(
        vec!["x", "y", "z"].iter().map(|e| e.to_string()).collect(),
    );
    assert_eq!(string, test_vec);
}
#[test]
fn test_sort_identifier_literal() {
    let test_vec_literal = vec!["2"].into_iter().map(|e| e.to_string()).collect();
    let test_vec_identifier: Vec<String> = vec!["z"].into_iter().map(|e| e.to_string()).collect();
    let string = CallExpression::sort_identifier_literal("(2,z)");
    assert_eq!(string, (test_vec_identifier, test_vec_literal));
}
#[test]
fn test_sort_all_identifier() {
    let test_vec_literal = vec![];
    let test_vec_identifier: Vec<String> = vec!["z", "dogs", "cats"]
        .into_iter()
        .map(|e| e.to_string())
        .collect();
    let string = CallExpression::sort_identifier_literal("(z,dogs,cats)");
    assert_eq!(string, (test_vec_identifier, test_vec_literal));
}
#[test]
fn test_sort_all_literals() {
    let test_vec_literal = vec!["\"z\"", "123", "\"cats\""]
        .into_iter()
        .map(|e| e.to_string())
        .collect();
    let test_vec_identifier: Vec<String> = vec![];
    let string = CallExpression::sort_identifier_literal("(\"z\",123,\"cats\")");
    assert_eq!(string, (test_vec_identifier, test_vec_literal));
}

#[test]
fn create_literals_vec() {
    let test_vec_literal: Vec<Literal> = vec![
        Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "\"x\"".to_string(),
        },
        Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "2".to_string(),
        },
        Literal {
            type_of: "Literal".to_string(),
            start: 0,
            end: 0,
            value: "32".to_string(),
        },
    ];
    let string = CallExpression::create_literals_vec(
        vec!["\"x\"", "2", "32"]
            .iter()
            .map(|e| e.to_string())
            .collect(),
    );
    assert_eq!(string, test_vec_literal);
}
