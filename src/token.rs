/// Token type and related structures
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    StringLit(String),
    BoolLit(bool),
    
    // Keywords
    KwLet,
    KwFn,
    KwIf,
    KwElse,
    KwReturn,
    KwWhile,
    KwTrue,
    KwFalse,
    
    // Identifiers
    Ident(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    
    // Comparison
    Eq,      // ==
    NotEq,   // !=
    Lt,      // 
    Gt,      // >
    Le,      // <=
    Ge,      // >=
    
    // Assignment
    Assign,  // =
    
    // Logical
    And,     // &&
    Or,      // ||
    Not,     // !
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    
    // Punctuation
    Semicolon,
    Colon,
    Comma,
    Dot,
    Arrow,   // ->
    
    // Special
    Eof,
    Error(String),
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

impl SpannedToken {
    pub fn new(token: Token, line: usize, col: usize) -> Self {
        SpannedToken { token, line, col }
    }
}