use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

//deletes whitespace, returns string
pub fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}
//takes string, and str to find, returns tuple containing start and end usize position
pub fn find_start_end(whole_program: String, name_to_find: &str) -> (usize, usize) {
    let formatted_name_to_find = format!("{}", name_to_find);

    let mat = Regex::new(&formatted_name_to_find)
        .unwrap()
        .find(&whole_program)
        .expect("found no function");

    (mat.start(), mat.end())
}

//opens file, returns  String of file
pub fn read_file_line_by_line(filepath: &str) -> String {
    let path = Path::new(filepath);
    let file = File::open(path).expect("Cannot open file.txt");
    let reader = BufReader::new(&file);
    let lines = reader.lines().map(|l| l.expect("Couldn't read line"));
    let string = lines.collect();
    string
}


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

//takes str value and determines the type; returns str of type for matching
pub fn str_to_type(string: &str)->Result<&str, &str> {
    let start_bool = string.chars().next().unwrap_or_default() == '"';
        let end_bool = string.chars().last().unwrap_or_default() == '"';
        if start_bool && end_bool {
           return Ok("string");
        }
        //logical xor, if input has one quotation mark on either side but not both, return malformed string error
        if (start_bool && !end_bool) || (!start_bool && end_bool) {
            return Err("Malformed string!");
        }

        if string.trim().parse::<f64>().is_ok() {
          return   Ok("number");

        } else if string == true.to_string() || string == false.to_string() {
            return Ok("bool");
        }
        else {
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
//takes program string, converts it to array of statements. It skips what's inside blocks
//that functinoality will be moved out to be used when parsing blocks;
//I know the way I did it with chars causes issues I will need to address later, but I don't think they'll be a problem given the limited scope of this project.
   pub fn string_array_to_vec(string: String) -> Vec<String> {
        let trimmed = skip_space(&string);
        let mut new_string = trimmed.to_string();
    
        let mut count: (i32, bool) = (0, false);
    
        let left_curly: &str = "{";
        let right_curly: &str = "}";
    
        let left_curly_char = left_curly.chars().next().unwrap();
        let right_curly_char = right_curly.chars().next().unwrap();
        let mut semicolon_matches_vec: Vec<usize> = Vec::new();
    
        for (i, c) in trimmed.chars().enumerate() {
            if c == left_curly_char {
                count.0 = count.0 + 1;
                count.1 = true;
            }
            if c == right_curly_char {
                count.0 = count.0 - 1;
            }
    
            if count.0 == 0 {
                count.1 = false;
            }
    
            if count.1 && c == ";".chars().next().unwrap() {
                new_string.replace_range(i..i + 1, "~");
                semicolon_matches_vec.push(i);
            }
        }
        let result: Vec<String> = new_string
            .split(";")
            .map(|e| e.replace("~", ";").to_string())
            .collect();
    
        result
    }
    
   pub fn str_to_type_inc_parentheses(string: &str) -> &str {
        let result = str_to_type(string);

        let type_match = match result {
            Ok("lookup") => "identifier",
            Ok("bool") => "literal",
            Ok("string") => "literal",
            Ok("number") => "literal",
            _ => "Malformed!",
        };
        type_match
    }
