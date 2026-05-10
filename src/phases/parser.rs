use crate::token::Token;
use crate::ast::{Expr, Stmt, BinOp, Type};
use crate::error::{CompileError, CompileResult, Phase};

/// Recursive descent parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Peek at current token
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    /// Consume and return current token
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// Expect a specific token
    fn expect(&mut self, expected: &Token) -> CompileResult<Token> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(expected) {
            Ok(self.advance())
        } else {
            Err(CompileError {
                phase: Phase::Syntax,
                line: 0,
                col: 0,
                message: format!("Expected {:?}, found {:?}", expected, self.peek()),
            })
        }
    }

    /// Parse program (list of statements)
    pub fn parse(&mut self) -> CompileResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    /// Parse a statement
    fn parse_stmt(&mut self) -> CompileResult<Stmt> {
        match self.peek() {
            Token::KwLet => self.parse_let(),
            Token::KwReturn => self.parse_return(),
            Token::KwWhile => self.parse_while(),
            Token::KwIf => self.parse_if_stmt(),
            Token::LBrace => self.parse_block_stmt(),
            Token::Ident(_) => {
                // Check if it's assignment
                let start_pos = self.pos;
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    _ => unreachable!(),
                };

                if self.peek() == &Token::Assign {
                    self.advance(); // consume '='
                    let value = self.parse_expr()?;
                    self.expect(&Token::Semicolon)?;
                    Ok(Stmt::Assign { name, value })
                } else {
                    // It's an expression statement
                    self.pos = start_pos;
                    let expr = self.parse_expr()?;
                    self.expect(&Token::Semicolon)?;
                    Ok(Stmt::ExprStmt(expr))
                }
            }
            _ => {
                // Expression statement
                let expr = self.parse_expr()?;
                self.expect(&Token::Semicolon)?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    /// Parse let statement
    fn parse_let(&mut self) -> CompileResult<Stmt> {
        self.expect(&Token::KwLet)?;

        let name = match self.advance() {
            Token::Ident(n) => n,
            _ => {
                return Err(CompileError {
                    phase: Phase::Syntax,
                    line: 0,
                    col: 0,
                    message: "Expected identifier after 'let'".to_string(),
                })
            }
        };

        // Optional type annotation
        let ty = if self.peek() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&Token::Assign)?;
        let init = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;

        Ok(Stmt::Let { name, ty, init })
    }

    /// Parse return statement
    fn parse_return(&mut self) -> CompileResult<Stmt> {
        self.expect(&Token::KwReturn)?;
        let expr = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    /// Parse while statement
    fn parse_while(&mut self) -> CompileResult<Stmt> {
        self.expect(&Token::KwWhile)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::LBrace)?;
        let body = self.parse_stmts_until_rbrace()?;
        self.expect(&Token::RBrace)?;
        Ok(Stmt::While { cond, body })
    }

    /// Parse if statement
    fn parse_if_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(&Token::KwIf)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::LBrace)?;
        let then_body = self.parse_stmts_until_rbrace()?;
        self.expect(&Token::RBrace)?;

        let else_body = if self.peek() == &Token::KwElse {
            self.advance();
            self.expect(&Token::LBrace)?;
            let body = self.parse_stmts_until_rbrace()?;
            self.expect(&Token::RBrace)?;
            Some(body)
        } else {
            None
        };

        Ok(Stmt::If { cond, then_body, else_body })
    }

    /// Parse block statement
    fn parse_block_stmt(&mut self) -> CompileResult<Stmt> {
        self.expect(&Token::LBrace)?;
        let stmts = self.parse_stmts_until_rbrace()?;
        self.expect(&Token::RBrace)?;
        Ok(Stmt::Block(stmts))
    }

    /// Parse statements until closing brace
    fn parse_stmts_until_rbrace(&mut self) -> CompileResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::Eof {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    /// Parse type annotation
    fn parse_type(&mut self) -> CompileResult<Type> {
        match self.advance() {
            Token::Ident(s) => {
                match s.as_str() {
                    "i32" | "Int" => Ok(Type::Int),
                    "f64" | "Float" => Ok(Type::Float),
                    "bool" => Ok(Type::Bool),
                    "str" => Ok(Type::Str),
                    _ => Ok(Type::Unknown),
                }
            }
            _ => Err(CompileError {
                phase: Phase::Syntax,
                line: 0,
                col: 0,
                message: "Expected type".to_string(),
            }),
        }
    }

    /// Parse expression (top-level)
    fn parse_expr(&mut self) -> CompileResult<Expr> {
        self.parse_expr_or()
    }

    /// Parse logical OR
    fn parse_expr_or(&mut self) -> CompileResult<Expr> {
        let mut left = self.parse_expr_and()?;

        while self.peek() == &Token::Or {
            self.advance();
            let right = self.parse_expr_and()?;
            left = Expr::BinOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse logical AND
    fn parse_expr_and(&mut self) -> CompileResult<Expr> {
        let mut left = self.parse_expr_comparison()?;

        while self.peek() == &Token::And {
            self.advance();
            let right = self.parse_expr_comparison()?;
            left = Expr::BinOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse comparison operators
    fn parse_expr_comparison(&mut self) -> CompileResult<Expr> {
        let mut left = self.parse_expr_add()?;

        loop {
            let op = match self.peek() {
                Token::Eq => BinOp::Eq,
                Token::NotEq => BinOp::Ne,
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                Token::Le => BinOp::Le,
                Token::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_expr_add()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse addition and subtraction
    fn parse_expr_add(&mut self) -> CompileResult<Expr> {
        let mut left = self.parse_expr_mul()?;

        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_expr_mul()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse multiplication and division
    fn parse_expr_mul(&mut self) -> CompileResult<Expr> {
        let mut left = self.parse_expr_unary()?;

        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_expr_unary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse unary operators
    fn parse_expr_unary(&mut self) -> CompileResult<Expr> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_expr_unary()?;
                Ok(Expr::BinOp {
                    op: BinOp::Eq, // Not is a unary, but we'll handle it in semantics
                    left: Box::new(expr),
                    right: Box::new(Expr::BoolLit(false)),
                })
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_expr_unary()?;
                Ok(Expr::BinOp {
                    op: BinOp::Sub,
                    left: Box::new(Expr::IntLit(0)),
                    right: Box::new(expr),
                })
            }
            _ => self.parse_expr_postfix(),
        }
    }

    /// Parse postfix (function calls)
    fn parse_expr_postfix(&mut self) -> CompileResult<Expr> {
        let mut expr = self.parse_expr_primary()?;

        loop {
            match self.peek() {
                Token::LParen => {
                    // Function call
                    if let Expr::Var(func) = expr {
                        self.advance();
                        let mut args = Vec::new();
                        if self.peek() != &Token::RParen {
                            args.push(self.parse_expr()?);
                            while self.peek() == &Token::Comma {
                                self.advance();
                                args.push(self.parse_expr()?);
                            }
                        }
                        self.expect(&Token::RParen)?;
                        expr = Expr::Call { func, args };
                    } else {
                        return Err(CompileError {
                            phase: Phase::Syntax,
                            line: 0,
                            col: 0,
                            message: "Cannot call non-identifier".to_string(),
                        });
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse primary expression
    fn parse_expr_primary(&mut self) -> CompileResult<Expr> {
        match self.peek().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::IntLit(n))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::FloatLit(f))
            }
            Token::KwTrue => {
                self.advance();
                Ok(Expr::BoolLit(true))
            }
            Token::KwFalse => {
                self.advance();
                Ok(Expr::BoolLit(false))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StrLit(s))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expr::Var(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Token::KwIf => self.parse_if_expr(),
            _ => Err(CompileError {
                phase: Phase::Syntax,
                line: 0,
                col: 0,
                message: format!("Unexpected token: {:?}", self.peek()),
            }),
        }
    }

    /// Parse if expression
    fn parse_if_expr(&mut self) -> CompileResult<Expr> {
        self.expect(&Token::KwIf)?;
        let cond = Box::new(self.parse_expr()?);
        self.expect(&Token::LBrace)?;

        // Parse then body - collect statements and return last expression
        let mut stmts = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::Eof {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(&Token::RBrace)?;

        // For now, convert block to expression by evaluating to unit
        let then_ = Box::new(Expr::IntLit(0)); // Placeholder

        let else_ = if self.peek() == &Token::KwElse {
            self.advance();
            self.expect(&Token::LBrace)?;
            let mut stmts = Vec::new();
            while self.peek() != &Token::RBrace && self.peek() != &Token::Eof {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(&Token::RBrace)?;
            Some(Box::new(Expr::IntLit(0)))
        } else {
            None
        };

        Ok(Expr::If { cond, then_, else_ })
    }
}