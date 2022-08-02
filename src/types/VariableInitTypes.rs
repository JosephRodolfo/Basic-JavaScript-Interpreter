#[derive(Debug, Clone)]
pub enum VariableInitTypes {
    String(String),
    Number(f64),
    // Array(Params),
    Bool(bool),
    Null(String),
}

impl VariableInitTypes {
    //takes str (type) and str (value), returns selfenum value option
    pub fn match_type_from_str(string_to_match: &str, value: &str) -> Option<VariableInitTypes> {
        let type_match = match string_to_match {
            "string" => Some(VariableInitTypes::String(value.to_string())),
            "number" => {
        
                let float = value.parse::<f64>().unwrap();

                Some(VariableInitTypes::Number(float))
            }
            "bool" => {
                let result = if string_to_match == "true" {
                    true
                } else {
                    false
                };
                Some(VariableInitTypes::Bool(result))
            }
            _ => Some(VariableInitTypes::Null("null".to_string())),
        };
        type_match
    }


    
}


