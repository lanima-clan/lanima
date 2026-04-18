use crate::ast::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    pub msg: String,
    pub span: Option<Span>,
}