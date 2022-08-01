
#[derive(Debug)]

pub enum VariableInitTypes {
    String(String),
    Number(f64),
    // Array(Params),
    Bool(bool),
}

 impl VariableInitTypes {
    fn match_type_from_str(string_to_match: &str) -> Option<VariableInitTypes> {
        let type_match = match string_to_match {
            "string" => Some(VariableInitTypes::String(string_to_match.to_string())),
            "number" => Some(VariableInitTypes::Number(
                string_to_match.parse::<f64>().unwrap(),
            )),
            "bool" => {
                let result = if string_to_match == "true" {
                    true
                } else {
                    false
                };
                Some(VariableInitTypes::Bool(result))
            }
            _ => None,
        };
        type_match
    }
}