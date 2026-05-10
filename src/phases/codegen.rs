use crate::ast::{Stmt, Expr, BinOp as AstBinOp};
use crate::tac::{Addr, BinOp as TacBinOp, Instr};

/// Code generator - converts AST to TAC
pub struct CodeGen {
    pub instrs: Vec<Instr>,
    temp_count: usize,
    label_count: usize,
}

impl CodeGen {
    /// Create new code generator
    pub fn new() -> Self {
        CodeGen {
            instrs: Vec::new(),
            temp_count: 0,
            label_count: 0,
        }
    }

    /// Allocate a new temporary
    fn new_temp(&mut self) -> Addr {
        let n = self.temp_count;
        self.temp_count += 1;
        Addr::Temp(n)
    }

    /// Allocate a new label
    fn new_label(&mut self) -> String {
        let n = self.label_count;
        self.label_count += 1;
        format!("L{}", n)
    }

    /// Emit an instruction
    fn emit(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    /// Translate AST binary operator to TAC binary operator
    fn translate_binop(op: &AstBinOp) -> TacBinOp {
        match op {
            AstBinOp::Add => TacBinOp::Add,
            AstBinOp::Sub => TacBinOp::Sub,
            AstBinOp::Mul => TacBinOp::Mul,
            AstBinOp::Div => TacBinOp::Div,
            AstBinOp::Mod => TacBinOp::Mod,
            AstBinOp::Eq => TacBinOp::Eq,
            AstBinOp::Ne => TacBinOp::Ne,
            AstBinOp::Lt => TacBinOp::Lt,
            AstBinOp::Gt => TacBinOp::Gt,
            AstBinOp::Le => TacBinOp::Le,
            AstBinOp::Ge => TacBinOp::Ge,
            AstBinOp::And => TacBinOp::And,
            AstBinOp::Or => TacBinOp::Or,
        }
    }

    /// Generate code for a statement
    pub fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty: _, init } => {
                let src = self.gen_expr(init);
                self.emit(Instr::Copy {
                    dst: Addr::Var(name.clone()),
                    src,
                });
            }

            Stmt::Assign { name, value } => {
                let src = self.gen_expr(value);
                self.emit(Instr::Copy {
                    dst: Addr::Var(name.clone()),
                    src,
                });
            }

            Stmt::Return(expr) => {
                let src = self.gen_expr(expr);
                self.emit(Instr::Return(src));
            }

            Stmt::ExprStmt(expr) => {
                let _addr = self.gen_expr(expr);
            }

            Stmt::Block(stmts) => {
                for s in stmts {
                    self.gen_stmt(s);
                }
            }

            Stmt::While { cond, body } => {
                let l_start = self.new_label();
                let l_body = self.new_label();
                let l_end = self.new_label();

                self.emit(Instr::Label(l_start.clone()));
                let t_cond = self.gen_expr(cond);
                self.emit(Instr::IfGoto {
                    cond: t_cond,
                    label: l_body.clone(),
                });
                self.emit(Instr::Goto(l_end.clone()));

                self.emit(Instr::Label(l_body));
                for s in body {
                    self.gen_stmt(s);
                }
                self.emit(Instr::Goto(l_start));

                self.emit(Instr::Label(l_end));
            }

            Stmt::If { cond, then_body, else_body } => {
                let l_then = self.new_label();
                let l_else = self.new_label();
                let l_end = self.new_label();

                let t_cond = self.gen_expr(cond);
                self.emit(Instr::IfGoto {
                    cond: t_cond,
                    label: l_then.clone(),
                });
                self.emit(Instr::Goto(l_else.clone()));

                self.emit(Instr::Label(l_then));
                for s in then_body {
                    self.gen_stmt(s);
                }
                self.emit(Instr::Goto(l_end.clone()));

                self.emit(Instr::Label(l_else));
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.gen_stmt(s);
                    }
                }
                self.emit(Instr::Goto(l_end.clone()));

                self.emit(Instr::Label(l_end));
            }
        }
    }

    /// Generate code for an expression and return address of result
    fn gen_expr(&mut self, expr: &Expr) -> Addr {
        match expr {
            Expr::IntLit(n) => Addr::IntLit(*n),
            Expr::FloatLit(f) => Addr::IntLit(*f as i64), // Simplified: cast to int
            Expr::BoolLit(b) => Addr::BoolLit(*b),
            Expr::StrLit(_s) => Addr::IntLit(0), // Simplified: strings as 0

            Expr::Var(name) => Addr::Var(name.clone()),

            Expr::BinOp { op, left, right } => {
                let l = self.gen_expr(left);
                let r = self.gen_expr(right);
                let dst = self.new_temp();
                self.emit(Instr::BinOp {
                    dst: dst.clone(),
                    op: Self::translate_binop(op),
                    left: l,
                    right: r,
                });
                dst
            }

            Expr::Call { func: _, args } => {
                for _arg in args {
                    // Simplified: don't actually call functions
                }
                self.new_temp()
            }

            Expr::If { cond, then_, else_ } => {
                let l_then = self.new_label();
                let l_else = self.new_label();
                let l_end = self.new_label();
                let result = self.new_temp();

                let t_cond = self.gen_expr(cond);
                self.emit(Instr::IfGoto {
                    cond: t_cond,
                    label: l_then.clone(),
                });
                self.emit(Instr::Goto(l_else.clone()));

                self.emit(Instr::Label(l_then));
                let then_val = self.gen_expr(then_);
                self.emit(Instr::Copy {
                    dst: result.clone(),
                    src: then_val,
                });
                self.emit(Instr::Goto(l_end.clone()));

                self.emit(Instr::Label(l_else));
                if let Some(else_expr) = else_ {
                    let else_val = self.gen_expr(else_expr);
                    self.emit(Instr::Copy {
                        dst: result.clone(),
                        src: else_val,
                    });
                }
                self.emit(Instr::Goto(l_end.clone()));

                self.emit(Instr::Label(l_end));
                result
            }
        }
    }
}