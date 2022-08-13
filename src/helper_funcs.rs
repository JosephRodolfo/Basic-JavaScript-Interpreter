use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

//deletes whitespace, returns string
pub fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}
//takes string, and str to find, returns tuple containing start and end usize position

//opens file, returns  String of file
pub fn read_file_line_by_line(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let string = lines.collect();
    string
}

//takes str value and determines the type; returns str of type for matching
pub fn str_to_type(string: &str) -> Result<&str, &str> {
    let start_bool = string.chars().next().unwrap_or_default() == '"';
    let end_bool = string.chars().last().unwrap_or_default() == '"';
    let array_expression_regex = "^\\[.*\\]$";
    let match_array_expression = Regex::new(&array_expression_regex)
        .unwrap()
        .is_match(string);
    if match_array_expression {
        return Ok("array_expression");
    }

    if start_bool && end_bool {
        return Ok("string");
    }
    //logical xor, if input has one quotation mark on either side but not both, return malformed string error
    if (start_bool && !end_bool) || (!start_bool && end_bool) {
        return Err("Malformed string!");
    }

    if string.trim().parse::<f64>().is_ok() {
        return Ok("number");
    } else if string == true.to_string() || string == false.to_string() {
        return Ok("bool");
    } else {
        //if str input is none of these types, initiate a lookup of variables to determine if it's an existing variable;
        return Ok("lookup");
    }
}
//removes first and last chars from string
pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

//find outside brackets/parentheses; 
//I will further abstrct this to make it more reusable since right now it's mutating and returning a string rather than an index or something like that

pub fn find_outside_brackets(char_left: char, char_right: char, string: &str) -> String {
    let mut count: (i32, bool) = (0, false);
    let mut new_string = string.to_string();

    for (i, c) in string.chars().enumerate() {
        if c == char_left {
            count.0 += 1;
            count.1 = true;
        }
        if c == char_right {
            count.0 -= 1;
        }

        if count.0 == 0 {
            count.1 = false;
        }

        if count.1 && c == ';' {
            new_string.replace_range(i..i + 1, "~");
        }
    }
    new_string
}
//adds a semicolon for purposes of parsing at the end of block statements,
//javascript doesn't require semicolons after block statements so I wanted to emulate that. 
//"offset" increases which each hit to account for increasing string length
//as semicolons are added.
pub fn find_ending_bracket_no_semicolon_needed(string: &str) -> String {
    let mut count: (i32, bool) = (0, false);
    let mut new_string = string.to_string();
    let mut offset = 0;

    let left_curly_char = '{';
    let right_curly_char = '}';
    for (i, c) in string.chars().enumerate() {
        if c == left_curly_char {
            count.0 += 1;
            count.1 = true;
        }
        if c == right_curly_char {
            count.0 -= 1;
        }

        if count.0 == 0 {
            count.1 = false;
        }

        if !count.1 && c == '}' {
            offset += 1;
            new_string.insert_str(i + offset, ";");
        }
    }
    new_string
}

//takes program string, converts it to array of statements. It skips what's inside blocks
//that functinoality will be moved out to be used when parsing blocks;
//I know the way I did it with chars causes issues I will need to address later, but I don't think they'll be a problem given the limited scope of this project.
pub fn string_array_to_vec(string: String) -> Vec<String> {
    let trimmed = skip_space(&string);
    let rem = find_ending_bracket_no_semicolon_needed(&trimmed);
    let first = find_outside_brackets('{', '}', &rem);
    let second = find_outside_brackets('(', ')', &first);

    let result: Vec<String> = second
        .split(";")
        .map(|e| e.replace("~", ";").to_string())
        .collect();
    println!("{:?}", result);

    result
}

pub fn str_to_type_inc_parentheses(string: &str) -> &str {
    let result = str_to_type(string);

    let type_match = match result {
        Ok("lookup") => "identifier",
        Ok("bool") => "literal",
        Ok("string") => "literal",
        Ok("number") => "literal",
        Ok("array_expression") => "array_expression",
        _ => "Malformed!",
    };
    type_match
}

#[cfg(test)]
mod test {
    use crate::helper_funcs;
    use helper_funcs::str_to_type;
    #[test]
    fn test_str_to_type_number() {
        let string = str_to_type("660").unwrap();
        assert_eq!(string, "number");
    }
    #[test]
    fn test_str_to_type_decimal() {
        let string = str_to_type("1001.01010").unwrap();
        assert_eq!(string, "number");
    }
    #[test]
    fn test_str_to_type_string() {
        let string = str_to_type("\"dogs\"").unwrap();
        assert_eq!(string, "string");
    }
    #[test]
    fn test_str_to_type_malformed_string_err() {
        let string = str_to_type("\"dogs");
        assert!(string.is_err());
    }

    #[test]
    fn test_str_to_type_bool() {
        let string = str_to_type("true").unwrap();
        assert_eq!(string, "bool");
    }

    #[test]
    fn test_str_to_type_lookup_variable() {
        let string = str_to_type("x").unwrap();
        assert_eq!(string, "lookup");
    }
}
