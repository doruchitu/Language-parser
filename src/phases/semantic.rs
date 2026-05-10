use crate::ast::{Stmt, Expr, BinOp, Type};
use crate::symbol_table::{Symbol, SymbolTable};
use crate::error::CompileResult;

/// Type checker for semantic analysis
pub struct TypeChecker {
    pub table: SymbolTable,
}

impl TypeChecker {
    /// Create new type checker
    pub fn new() -> Self {
        TypeChecker {
            table: SymbolTable::new(),
        }
    }

    /// Check entire program
    pub fn check_program(&mut self, program: &Vec<Stmt>) -> CompileResult<()> {
        for stmt in program {
            self.check_stmt(stmt, &Type::Unit)?;
        }

        if self.table.has_errors() {
            return Err(crate::error::CompileError {
                phase: crate::error::Phase::Semantic,
                line: 0,
                col: 0,
                message: format!("Semantic errors: {}", self.table.errors.join("; ")),
            });
        }

        Ok(())
    }

    /// Check a statement
    fn check_stmt(&mut self, stmt: &Stmt, _ret_ty: &Type) -> CompileResult<()> {
        match stmt {
            Stmt::Let { name, ty, init } => {
                let inferred = self.check_expr(init)?;

                if let Some(declared_ty) = ty {
                    if &inferred != declared_ty {
                        self.table.error(format!(
                            "Type mismatch for '{}': expected {:?}, found {:?}",
                            name, declared_ty, inferred
                        ));
                    }
                }

                self.table.declare(Symbol {
                    name: name.clone(),
                    ty: inferred,
                    mutable: true,
                    defined: true,
                });

                Ok(())
            }

            Stmt::Assign { name, value } => {
                match self.table.lookup(name) {
                    None => {
                        self.table.error(format!("Undefined variable: '{}'", name));
                        Ok(())
                    }
                    Some(sym) => {
                        let val_ty = self.check_expr(value)?;
                        if val_ty != sym.ty {
                            self.table.error(format!(
                                "Type mismatch in assignment to '{}': {:?} = {:?}",
                                name, sym.ty, val_ty
                            ));
                        }
                        Ok(())
                    }
                }
            }

            Stmt::Return(expr) => {
                let _ty = self.check_expr(expr)?;
                Ok(())
            }

            Stmt::ExprStmt(expr) => {
                let _ty = self.check_expr(expr)?;
                Ok(())
            }

            Stmt::Block(stmts) => {
                self.table.push_scope();
                for s in stmts {
                    self.check_stmt(s, _ret_ty)?;
                }
                self.table.pop_scope();
                Ok(())
            }

            Stmt::While { cond, body } => {
                let cond_ty = self.check_expr(cond)?;
                if cond_ty != Type::Bool && cond_ty != Type::Int {
                    self.table.error(format!(
                        "While condition must be bool, found {:?}",
                        cond_ty
                    ));
                }

                self.table.push_scope();
                for s in body {
                    self.check_stmt(s, _ret_ty)?;
                }
                self.table.pop_scope();
                Ok(())
            }

            Stmt::If { cond, then_body, else_body } => {
                let cond_ty = self.check_expr(cond)?;
                if cond_ty != Type::Bool && cond_ty != Type::Int {
                    self.table.error(format!(
                        "If condition must be bool, found {:?}",
                        cond_ty
                    ));
                }

                self.table.push_scope();
                for s in then_body {
                    self.check_stmt(s, _ret_ty)?;
                }
                self.table.pop_scope();

                if let Some(else_stmts) = else_body {
                    self.table.push_scope();
                    for s in else_stmts {
                        self.check_stmt(s, _ret_ty)?;
                    }
                    self.table.pop_scope();
                }

                Ok(())
            }
        }
    }

    /// Check an expression and return its type
    fn check_expr(&mut self, expr: &Expr) -> CompileResult<Type> {
        match expr {
            Expr::IntLit(_) => Ok(Type::Int),
            Expr::FloatLit(_) => Ok(Type::Float),
            Expr::BoolLit(_) => Ok(Type::Bool),
            Expr::StrLit(_) => Ok(Type::Str),

            Expr::Var(name) => {
                match self.table.lookup(name) {
                    Some(sym) => Ok(sym.ty),
                    None => {
                        self.table.error(format!("Undefined variable: '{}'", name));
                        Ok(Type::Unknown)
                    }
                }
            }

            Expr::BinOp { op, left, right } => {
                let lt = self.check_expr(left)?;
                let rt = self.check_expr(right)?;

                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                        if (lt != Type::Int && lt != Type::Float)
                            || (rt != Type::Int && rt != Type::Float)
                        {
                            self.table.error(format!(
                                "Arithmetic operator requires numbers, got {:?} and {:?}",
                                lt, rt
                            ));
                        }
                        if lt == Type::Float || rt == Type::Float {
                            Ok(Type::Float)
                        } else {
                            Ok(Type::Int)
                        }
                    }

                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                        if lt != rt {
                            self.table.error(format!(
                                "Comparison types don't match: {:?} vs {:?}",
                                lt, rt
                            ));
                        }
                        Ok(Type::Bool)
                    }

                    BinOp::And | BinOp::Or => {
                        if lt != Type::Bool || rt != Type::Bool {
                            self.table.error(format!(
                                "Logical operators require bool, got {:?} and {:?}",
                                lt, rt
                            ));
                        }
                        Ok(Type::Bool)
                    }
                }
            }

            Expr::Call { func, args } => {
                for arg in args {
                    let _ty = self.check_expr(arg)?;
                }
                // For now, assume functions return Int
                Ok(Type::Int)
            }

            Expr::If { cond, then_, else_ } => {
                let cond_ty = self.check_expr(cond)?;
                if cond_ty != Type::Bool && cond_ty != Type::Int {
                    self.table.error(format!(
                        "If condition must be bool, found {:?}",
                        cond_ty
                    ));
                }

                let then_ty = self.check_expr(then_)?;
                if let Some(else_expr) = else_ {
                    let else_ty = self.check_expr(else_expr)?;
                    if then_ty != else_ty {
                        self.table.error(format!(
                            "If/else branches have different types: {:?} vs {:?}",
                            then_ty, else_ty
                        ));
                    }
                }

                Ok(then_ty)
            }
        }
    }
}