
use crate::traits;  
use crate::interpreter_types;
use traits::{Evaluator::Evaluator};
use interpreter_types::{Interpreter::Interpreter};


#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub value: String,
}


impl Evaluator for Literal {
    fn evaluate_with_scope(&self, _scope: &Interpreter)->String {
        "".to_string()
    }
     fn evaluate(&self)-> &str {
        
        &self.value
    }

}

