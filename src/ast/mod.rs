#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Span {
    pub file: String,
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub line: usize,
    pub column: usize,
}