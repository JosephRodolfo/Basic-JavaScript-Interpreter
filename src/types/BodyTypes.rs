use crate::types;
use types::{
    ExpressionStatement::ExpressionStatement, FunctionDeclaration::FunctionDeclaration,
    IfStatement::IfStatement, ReturnStatement::ReturnStatement,
    VariableDeclaration::VariableDeclaration, ForStatement::ForStatement
};


#[derive(Debug, PartialEq)]
pub enum BodyTypes {
    ExpressionStatement(ExpressionStatement),
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    VariableDeclaration(VariableDeclaration),
    ReturnStatement(ReturnStatement),
    ForStatement(ForStatement),
}
