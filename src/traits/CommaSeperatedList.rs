use regex::Regex;
use substring::Substring;

//a trait for handling lists seperated by something like commas, using generics and lifetimes
pub trait CommaSeperatedList<T> {
    fn create_string_vec<'a>(string: &'a str, character: &str) -> Vec<&'a str> {
        let args_count = string.matches(character).count() + 1;

        let mut string_vec: Vec<&str> = Vec::new();

        let mut temp_string = string;
        for i in 0..args_count {
            let formatted = format!("{})", temp_string);

            let match_params = if i == args_count - 1 {
                let x = format!("{}", "(^*$)");
                let match_return = Regex::new(&x).unwrap().find(&formatted).expect("not found");
                match_return
            } else {
                let x = format!("({})", character);
                let match_return = Regex::new(&x)
                    .unwrap()
                    .find(&temp_string)
                    .expect("not found");
                match_return
            };

            let first = temp_string.substring(0, match_params.end() - 1);
            
            string_vec.push(first);
            temp_string = temp_string.substring(match_params.end(), temp_string.len())
        }
        string_vec
    }

    fn create_comma_seperated_array(string_vec: Vec<&str>)->Result<Vec<T>, String>;

}
