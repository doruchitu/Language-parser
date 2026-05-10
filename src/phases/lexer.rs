use crate::token::{Token, SpannedToken};

/// Lexer structure for tokenization
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    /// Create a new lexer from source code
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Get current character without consuming
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    /// Get next character (lookahead)
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    /// Consume current character and advance position
    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
            self.advance();
        }
    }

    /// Check if character is valid identifier start
    fn is_ident_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    /// Check if character is valid identifier continuation
    fn is_ident_cont(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Scan an identifier or keyword
    fn scan_ident(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if Self::is_ident_cont(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords
        match s.as_str() {
            "let" => Token::KwLet,
            "fn" => Token::KwFn,
            "if" => Token::KwIf,
            "else" => Token::KwElse,
            "return" => Token::KwReturn,
            "while" => Token::KwWhile,
            "true" => Token::KwTrue,
            "false" => Token::KwFalse,
            _ => Token::Ident(s),
        }
    }

    /// Scan a number (integer or float)
    fn scan_number(&mut self) -> Token {
        let mut s = String::new();
        let mut is_float = false;

        // Handle hexadecimal: 0x...
        if self.peek() == Some('0') && self.peek_next() == Some('x') {
            self.advance(); // consume '0'
            self.advance(); // consume 'x'
            let mut hex = String::new();
            while let Some(c) = self.peek() {
                if c.is_ascii_hexdigit() {
                    hex.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            return match i64::from_str_radix(&hex, 16) {
                Ok(n) => Token::Int(n),
                Err(_) => Token::Error(format!("invalid hex: 0x{}", hex)),
            };
        }

        // Decimal/Float
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float && self.peek_next().map_or(false, |n| n.is_ascii_digit()) {
                is_float = true;
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            s.parse::<f64>()
                .map(Token::Float)
                .unwrap_or_else(|_| Token::Error(format!("invalid float: {}", s)))
        } else {
            s.parse::<i64>()
                .map(Token::Int)
                .unwrap_or_else(|_| Token::Error(format!("invalid int: {}", s)))
        }
    }

    /// Scan a string literal
    fn scan_string(&mut self) -> Token {
        self.advance(); // consume opening '"'
        let mut s = String::new();

        loop {
            match self.peek() {
                None | Some('\n') => {
                    return Token::Error(format!("unterminated string: \"{}\"", s));
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.peek() {
                        Some('n') => { s.push('\n'); self.advance(); }
                        Some('t') => { s.push('\t'); self.advance(); }
                        Some('r') => { s.push('\r'); self.advance(); }
                        Some('"') => { s.push('"'); self.advance(); }
                        Some('\\') => { s.push('\\'); self.advance(); }
                        Some(c) => {
                            s.push('\\');
                            s.push(c);
                            self.advance();
                        }
                        None => return Token::Error("unterminated string".to_string()),
                    }
                }
                Some(c) => {
                    s.push(c);
                    self.advance();
                }
            }
        }

        Token::StringLit(s)
    }

    /// Scan a single-line comment
    fn scan_line_comment(&mut self) -> Token {
        self.advance(); // consume first '/'
        self.advance(); // consume second '/'
        while self.peek().map_or(false, |c| c != '\n') {
            self.advance();
        }
        // Return next token (skip comment)
        self.next_token().token
    }

    /// Scan a block comment
    fn scan_block_comment(&mut self) -> Token {
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        loop {
            match self.peek() {
                None => return Token::Error("unterminated block comment".to_string()),
                Some('*') if self.peek_next() == Some('/') => {
                    self.advance(); // consume '*'
                    self.advance(); // consume '/'
                    break;
                }
                _ => { self.advance(); }
            }
        }
        // Return next token (skip comment)
        self.next_token().token
    }

    /// Get next token
    pub fn next_token(&mut self) -> SpannedToken {
        self.skip_whitespace();

        let line = self.line;
        let col = self.col;

        let tok = match self.peek() {
            None => Token::Eof,

            // Operators and delimiters
            Some('+') => { self.advance(); Token::Plus }
            Some('-') => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            Some('*') => { self.advance(); Token::Star }
            Some('/') => {
                // Check for comments
                if self.peek_next() == Some('/') {
                    return SpannedToken::new(self.scan_line_comment(), line, col);
                } else if self.peek_next() == Some('*') {
                    return SpannedToken::new(self.scan_block_comment(), line, col);
                } else {
                    self.advance();
                    Token::Slash
                }
            }
            Some('%') => { self.advance(); Token::Percent }
            Some('=') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            Some('<') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            Some('&') => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    Token::Error("unexpected '&'".to_string())
                }
            }
            Some('|') => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    Token::Error("unexpected '|'".to_string())
                }
            }
            Some('(') => { self.advance(); Token::LParen }
            Some(')') => { self.advance(); Token::RParen }
            Some('{') => { self.advance(); Token::LBrace }
            Some('}') => { self.advance(); Token::RBrace }
            Some('[') => { self.advance(); Token::LBracket }
            Some(']') => { self.advance(); Token::RBracket }
            Some(';') => { self.advance(); Token::Semicolon }
            Some(':') => { self.advance(); Token::Colon }
            Some(',') => { self.advance(); Token::Comma }
            Some('.') => { self.advance(); Token::Dot }

            // Numbers
            Some(c) if c.is_ascii_digit() => self.scan_number(),

            // Identifiers and keywords
            Some(c) if Self::is_ident_start(c) => self.scan_ident(),

            // Strings
            Some('"') => self.scan_string(),

            // Unknown character
            Some(c) => {
                let bad = c.to_string();
                self.advance();
                Token::Error(format!("invalid character: '{}'", bad))
            }
        };

        SpannedToken::new(tok, line, col)
    }

    /// Tokenize entire input
    pub fn tokenize(source: &str) -> Vec<SpannedToken> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            let st = lexer.next_token();
            let is_eof = st.token == Token::Eof;
            tokens.push(st);
            if is_eof {
                break;
            }
        }

        tokens
    }
}