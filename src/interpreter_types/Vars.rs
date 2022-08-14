use crate::interpreter_types::{VarsEnum::VarsEnum};
#[derive(PartialEq, Debug, Clone)]
pub struct Vars {
    pub value: VarsEnum,
    pub kind: String,
}