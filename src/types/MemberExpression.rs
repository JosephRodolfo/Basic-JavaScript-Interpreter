use crate::types;
use types::Identifier::Identifier;
struct MemberExpression {
    type_of: String,
    start: usize,
    end: usize,
    callee: Identifier,
    property: Identifier,
}

impl MemberExpression {
    fn create_member_expression() {}
}
