/// An address in TAC (variable, temporary, or literal)
#[derive(Debug, Clone, PartialEq)]
pub enum Addr {
    Var(String),      // Variable: x, y
    Temp(usize),      // Temporary: t0, t1, t2
    IntLit(i64),      // Integer literal: 42
    BoolLit(bool),    // Boolean literal: true, false
}

impl Addr {
    pub fn display(&self) -> String {
        match self {
            Addr::Var(s) => s.clone(),
            Addr::Temp(n) => format!("t{}", n),
            Addr::IntLit(n) => n.to_string(),
            Addr::BoolLit(b) => b.to_string(),
        }
    }
}

/// Binary operators for TAC
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

/// TAC Instructions
#[derive(Debug, Clone)]
pub enum Instr {
    /// t = y op z
    BinOp {
        dst: Addr,
        op: BinOp,
        left: Addr,
        right: Addr,
    },
    /// t = y (copy)
    Copy {
        dst: Addr,
        src: Addr,
    },
    /// goto L
    Goto(String),
    /// if t goto L
    IfGoto {
        cond: Addr,
        label: String,
    },
    /// L: (label definition)
    Label(String),
    /// return t
    Return(Addr),
    /// halt
    Halt,
}

impl Instr {
    pub fn display(&self) -> String {
        match self {
            Instr::BinOp { dst, op, left, right } => {
                let op_str = match op {
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Mul => "*",
                    BinOp::Div => "/",
                    BinOp::Mod => "%",
                    BinOp::Eq => "==",
                    BinOp::Ne => "!=",
                    BinOp::Lt => "<",
                    BinOp::Gt => ">",
                    BinOp::Le => "<=",
                    BinOp::Ge => ">=",
                    BinOp::And => "&&",
                    BinOp::Or => "||",
                };
                format!(
                    "  {} = {} {} {}",
                    dst.display(),
                    left.display(),
                    op_str,
                    right.display()
                )
            }
            Instr::Copy { dst, src } => format!("  {} = {}", dst.display(), src.display()),
            Instr::Goto(l) => format!("  goto {}", l),
            Instr::IfGoto { cond, label } => format!("  if {} goto {}", cond.display(), label),
            Instr::Label(l) => format!("{}:", l),
            Instr::Return(a) => format!("  return {}", a.display()),
            Instr::Halt => "  halt".to_string(),
        }
    }
}