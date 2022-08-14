use crate::{
    helper_funcs::{rem_first_and_last, str_to_type},
    types};
use regex::Regex;
use substring::Substring;
use types::{Literal::Literal, Identifier::Identifier};

#[derive(PartialEq, Debug, Clone)]
enum BinaryExpressionOptions {
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(Box<BinaryExpression>),
    None(String),
}
#[derive(PartialEq, Debug, Clone)]
pub struct BinaryExpression {
    type_of: String,
    start: usize,
    end: usize,
    left: BinaryExpressionOptions,
    operator: String,
    right: BinaryExpressionOptions,
}

impl BinaryExpression {
    //takes expression full string, returns current item, operator, and entire string that follows operator
    //there's a lot of room for in this function in particular; the mutable variables and them
    //being changed in a loop/if statement that returns something else is bad.
    fn loop_through_operators(string: &str) -> (&str, Option<&str>, &str) {
        //finds operator
        let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .find(string);
        //gets ( as char, and first char of string
        let left_paren = "(".chars().next().unwrap();
        let c = string.chars().next().unwrap();

        //if the operator find match returns None, returns (0, 0), elsewise returns (x, y) tuple of operator start end
        let (mat_start, mat_end) = match mat {
            Some(mat) => (mat.start(), mat.end()),
            None => (0, 0),
        };
        //if there's no operator, returns early triple of string as current item (since it is the only, thus current, item) None for operator  and empty str for next;
        if mat_start == 0 && mat_end == 0 {
            return (string, None, "");
        }

        let mut resultant = "";
        let mut new_string = "";
        //if the full string starts with ( char, it means this is a parenthetical.
        //loops through full string until it finds matching char and returns item, so it doesn't end the return result at an inner parentheses;
        let item = if left_paren == c {
            let mut count: (i32, bool) = (0, false);

            let left_curly: &str = "(";
            let right_curly: &str = ")";

            let left_curly_char = left_curly.chars().next().unwrap();
            let right_curly_char = right_curly.chars().next().unwrap();

            for (i, c) in string.chars().enumerate() {
                if c == left_curly_char {
                    count.0 = count.0 + 1;
                    count.1 = true;
                };
                if c == right_curly_char {
                    count.0 = count.0 - 1;
                }

                if count.0 == 0 {
                    resultant = string.substring(0, i + 1);
                    new_string = string.substring(i + 2, string.len());

                    break;
                }
            }

            let current = rem_first_and_last(resultant);
            current
        } else {
            let current = string.substring(0, mat_start);
            let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
                .unwrap()
                .find(string)
                .expect("no operators foudn");
            new_string = string.substring(mat.end(), string.len());

            current
        };
        if resultant == string {
            return (rem_first_and_last(resultant), None, "");
        }

        // let current = string.substring(0, mat_start);
        let operator = string.substring(mat_start, mat_end);

        (item, Some(operator), new_string)
    }
    //takes full binary expression string, returns  BinaryExpression object, supports theoretically infinite nested parenthetical statements
   pub fn create_generic_expression(string: &str) -> BinaryExpression {
        let (current, operator, next) = BinaryExpression::loop_through_operators(string);

        let mat = Regex::new("([<>]=?|=+|-|\\*|%|==|===|\\+|\\?|:)")
            .unwrap()
            .is_match(current);
        //sets left leaf, if a a parenthetical, recursively loops through it and returns binaryexpression object, else creates a node
        let left_result = if mat {
            let left_result = BinaryExpressionOptions::BinaryExpression(Box::new(
                BinaryExpression::create_generic_expression(current),
            ));
            left_result
        } else {
            BinaryExpression::create_node(current)
        };

        let mut new_binary_expression = BinaryExpression {
            type_of: "BinaryExpression".to_string(),
            start: 0,
            end: string.len(),
            left: BinaryExpressionOptions::None("placeholder".to_string()),
            operator: "".to_string(),
            right: BinaryExpressionOptions::None("placeholder".to_string()),
        };
        //gets next set of current, operator, and following string, to see if the recursive loop needs to end after the final one
        let (test, test_op, _test_string) = BinaryExpression::loop_through_operators(next);
        //if the test operator from last line returns None, it's time to end recursive loop and return the binary_expression.
        if test_op == None {
            new_binary_expression.left = left_result;
            //if the next item  is a parenthetical statement, starts a new loop using the test result, otherwise, just creates a node
            let right_result = if next.contains("(") {
                BinaryExpressionOptions::BinaryExpression(Box::new(
                    BinaryExpression::create_generic_expression(test),
                ))
            //if it's not a parenthetical expression, just creates node with next
            } else {
                BinaryExpression::create_node(next)
            };

            new_binary_expression.right = right_result;
            new_binary_expression.operator = operator.unwrap().to_string();
            //returns early
            return new_binary_expression;
        }
        // else if there is another operator, creates a node for left, and starts recursive loop off again for right.
        new_binary_expression.left = BinaryExpression::create_node(current);
        new_binary_expression.right = BinaryExpressionOptions::BinaryExpression(Box::new(
            BinaryExpression::create_generic_expression(next),
        ));
        new_binary_expression.operator = operator.unwrap().to_string();

        new_binary_expression
    }
    //just takes string of current item, returns an enum BinaryExpressionOptions literal or identifier
    fn create_node(current: &str) -> BinaryExpressionOptions {
        let type_of = BinaryExpression::str_to_type_inc_parentheses(current);
        let result = match type_of {
            "identifier" => {
                let new_identifier = Identifier {
                    type_of: "Identifier".to_string(),
                    start: 0,
                    end: 0,
                    name: current.to_string(),
                };

                BinaryExpressionOptions::Identifier(new_identifier)
            }
            "literal" => {
                let new_literal = Literal {
                    type_of: "Literal".to_string(),
                    start: 0,
                    end: 0,
                    value: current.to_string(),
                };
                BinaryExpressionOptions::Literal(new_literal)
            }
            _ => BinaryExpressionOptions::None("End".to_string()),
        };
        result
    }


    fn str_to_type_inc_parentheses(string: &str) -> &str {
        let result = str_to_type(string);

        let type_match = match result {
            Ok("lookup") => "identifier",
            Ok("bool") => "literal",
            Ok("string") => "literal",
            Ok("number") => "literal",
            _ => "Malformed!",
        };
        // type_match
        // };
        type_match
    }
}

#[cfg(test)]
mod test {
    use crate::types;
    use types::{
        Literal::Literal, Identifier::Identifier, BinaryTree::BinaryExpression, BinaryTree::BinaryExpressionOptions
    };


#[test]
fn test_create_binary_tree_three_items_parentheses_last() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Literal(new_literal),
    };

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 7,
        type_of: "BinaryExpression".to_string(),
        left: BinaryExpressionOptions::Identifier(cloned_identifier),
        right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::create_generic_expression("x+(x+3)")
    );
}

#[test]
fn test_create_binary_tree_three_items_parentheses_first() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Identifier(cloned_identifier),
    };

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 7,
        type_of: "BinaryExpression".to_string(),
        left: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        right: BinaryExpressionOptions::Literal(new_literal),
        operator: "+".to_string(),
    };

    assert_eq!(
        binary_expression_test,
        BinaryExpression::create_generic_expression("(x+x)+3")
    );
}

#[test]

fn test_create_binary_tree_nested_parentheses() {
    let a = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "a".to_string(),
    };
    let b = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "b".to_string(),
    };
    let c = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "c".to_string(),
    };
    let d = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "d".to_string(),
    };
    let e = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "e".to_string(),
    };
    let c_d_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(c),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Identifier(d),
    };
    let cd_e_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 7,
        left: BinaryExpressionOptions::BinaryExpression(Box::new(c_d_binary_expression)),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Identifier(e),
    };

    let b_cd_e_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 11,
        left: BinaryExpressionOptions::Identifier(b),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::BinaryExpression(Box::new(cd_e_binary_expression)),
    };

    let a_b_cd_e_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 15,
        left: BinaryExpressionOptions::Identifier(a),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::BinaryExpression(Box::new(b_cd_e_binary_expression)),
    };






    assert_eq!(
        a_b_cd_e_binary_expression,
        BinaryExpression::create_generic_expression("a+(b+((c+d)+e))")
    );
}

#[test]
fn test_create_binary_tree_three_items_mixed() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };

    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "3".to_string(),
    };
    let cloned_identifier = new_identifier.clone();
    let new_binary_expression = BinaryExpression {
        type_of: "BinaryExpression".to_string(),
        start: 0,
        end: 3,
        left: BinaryExpressionOptions::Identifier(new_identifier),
        operator: "+".to_string(),
        right: BinaryExpressionOptions::Literal(new_literal),
    };

    let identifier = BinaryExpressionOptions::Identifier(cloned_identifier);

    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 5,
        type_of: "BinaryExpression".to_string(),
        left: identifier,
        right: BinaryExpressionOptions::BinaryExpression(Box::new(new_binary_expression)),
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::create_generic_expression("x+x+3")
    );
}

#[test]
fn test_create_binary_tree_two_literals() {
    let new_literal = Literal {
        type_of: "Literal".to_string(),
        start: 0,
        end: 0,
        value: "2".to_string(),
    };
    let literal = BinaryExpressionOptions::Literal(new_literal);
    let cloned_lit = literal.clone();
    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 3,
        type_of: "BinaryExpression".to_string(),
        left: literal,
        right: cloned_lit,
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::create_generic_expression("2+2")
    );
}
#[test]
fn test_create_binary_tree_two_identifiers() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };
    let identifier = BinaryExpressionOptions::Identifier(new_identifier);
    let cloned_id = identifier.clone();
    let binary_expression_test = BinaryExpression {
        start: 0,
        end: 3,
        type_of: "BinaryExpression".to_string(),
        left: identifier,
        right: cloned_id,
        operator: "+".to_string(),
    };
    assert_eq!(
        binary_expression_test,
        BinaryExpression::create_generic_expression("x+x")
    );
}

#[test]
fn test_create_binary_expression_one_arg() {
    let new_identifier = Identifier {
        type_of: "Identifier".to_string(),
        start: 0,
        end: 0,
        name: "x".to_string(),
    };
    let string = BinaryExpression::create_node("x");
    assert_eq!(string, BinaryExpressionOptions::Identifier(new_identifier));
}
#[test]
fn test_str_to_type_incuding_parens_string() {
    let string = BinaryExpression::str_to_type_inc_parentheses("\"dogs\"");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_numeric() {
    let string = BinaryExpression::str_to_type_inc_parentheses("4");
    assert_eq!(string, "literal");
}
#[test]
fn test_str_to_type_incuding_parens_identifier() {
    let string = BinaryExpression::str_to_type_inc_parentheses("de4");
    assert_eq!(string, "identifier");
}
#[test]
fn test_str_to_type_incuding_parens_malformed() {
    let string = BinaryExpression::str_to_type_inc_parentheses("\"de4+3");
    assert_eq!(string, "Malformed!");
}

#[test]
fn test_loops_through_operator_two_simple() {
    let string = BinaryExpression::loop_through_operators("x+x");
    assert_eq!(string, ("x", Some("+"), "x"));
}
#[test]

fn test_loops_through_operator_two_literal() {
    let string = BinaryExpression::loop_through_operators("1+2");
    assert_eq!(string, ("1", Some("+"), "2"));
}
#[test]

fn test_loops_through_operator_reaches_end() {
    let string = BinaryExpression::loop_through_operators("1");
    assert_eq!(string, ("1", None, ""));
}
#[test]

fn test_loops_through_operator_parens_end() {
    let string = BinaryExpression::loop_through_operators("1+(x+2)");
    assert_eq!(string, ("1", Some("+"), "(x+2)"));
}
#[test]

fn test_loops_through_operator_parens_beginning() {
    let string = BinaryExpression::loop_through_operators("(x+2)+2");
    assert_eq!(string, ("x+2", Some("+"), "2"));
}
#[test]

fn test_loops_through_operators_parens_string() {
    let string = BinaryExpression::loop_through_operators("(x+2)");
    assert_eq!(string, ("x+2", None, ""));
}

}