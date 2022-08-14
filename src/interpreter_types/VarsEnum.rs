
use crate::interpreter_types::{Vars::Vars};
use crate::types::{VariableInitTypes::VariableInitTypes};
#[derive(PartialEq, Debug, Clone)]
pub enum VarsEnum {
    Prim(String),
    Obj(VariableInitTypes),
    Pointer(String)
    
}