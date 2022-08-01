use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

//deletes whitespace, returns string
pub fn skip_space(slice: &str) -> String {
    slice.split_whitespace().collect()
}
//takes string, and str to find, returns tuple containing start and end usize position
pub fn find_start_end(whole_program: &String, name_to_find: &str) -> (usize, usize) {
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

pub fn str_to_type(string: &str)->Result<&str, &str> {
    let start_bool = string.chars().next().unwrap_or_default() == '"';
        let end_bool = string.chars().last().unwrap_or_default() == '"';
        if start_bool && end_bool {
           return Ok("string");
        }

        if (start_bool && !end_bool) || (!start_bool && end_bool) {
            return Err("Malformed string!");
        }

        if string.parse::<f64>().is_ok() {

          return   Ok("number");

        } else if string == true.to_string() || string == false.to_string() {
            return Ok("bool");
        }
        else {
            return Err("Something went wrong!");

        }
        

    }


