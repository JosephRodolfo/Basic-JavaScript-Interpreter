
use crate::traits;  
use traits::{Evaluator::Evaluator};


#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub type_of: String,
    pub start: usize,
    pub end: usize,
    pub value: String,
}

impl Evaluator for Literal {
     fn evaluate(&self)-> &str {
        
        &self.value
    }

}

