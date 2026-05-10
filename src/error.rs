use std::fmt;

/// Compilation error with phase information and position tracking
#[derive(Debug, Clone)]
pub struct CompileError {
    pub phase: Phase,
    pub line: usize,
    pub col: usize,
    pub message: String,
}

/// Compilation phases
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Lexical,
    Syntax,
    Semantic,
    CodeGen,
}

/// Result type for compilation operations
pub type CompileResult<T> = Result<T, CompileError>;

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}] {}:{} – {}",
            self.phase, self.line, self.col, self.message
        )
    }
}

impl std::error::Error for CompileError {}