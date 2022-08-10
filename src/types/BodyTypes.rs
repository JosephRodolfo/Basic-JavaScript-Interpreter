use crate::types;
use types::{ExpressionStatement::ExpressionStatement, FunctionDeclaration::FunctionDeclaration, IfStatement::IfStatement, VariableDeclaration::VariableDeclaration, ReturnStatement::ReturnStatement };


#[derive(Debug, PartialEq)]
pub enum BodyTypes{
    ExpressionStatement(ExpressionStatement),
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    VariableDeclaration(VariableDeclaration),
    ReturnStatement(ReturnStatement),
    ForStatement(),
}


