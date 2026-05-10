use std::collections::HashMap;
use crate::tac::{Addr, BinOp, Instr};

/// TAC Interpreter - executes TAC instructions on abstract machine
pub struct Interpreter {
    memory: HashMap<String, i64>,
    pc: usize,  // Program counter
}

impl Interpreter {
    /// Create new interpreter
    pub fn new() -> Self {
        Interpreter {
            memory: HashMap::new(),
            pc: 0,
        }
    }

    /// Get value of an address
    fn get_addr(&self, addr: &Addr) -> i64 {
        match addr {
            Addr::Var(name) => self.memory.get(name).copied().unwrap_or(0),
            Addr::Temp(n) => self.memory.get(&format!("t{}", n)).copied().unwrap_or(0),
            Addr::IntLit(n) => *n,
            Addr::BoolLit(b) => if *b { 1 } else { 0 },
        }
    }

    /// Set value of an address
    fn set_addr(&mut self, addr: &Addr, value: i64) {
        match addr {
            Addr::Var(name) => {
                self.memory.insert(name.clone(), value);
            }
            Addr::Temp(n) => {
                self.memory.insert(format!("t{}", n), value);
            }
            _ => {} // Cannot set literal values
        }
    }

    /// Evaluate a binary operation
    fn eval_binop(&self, op: &BinOp, left: i64, right: i64) -> i64 {
        match op {
            BinOp::Add => left + right,
            BinOp::Sub => left - right,
            BinOp::Mul => left * right,
            BinOp::Div => {
                if right == 0 {
                    0 // Handle division by zero
                } else {
                    left / right
                }
            }
            BinOp::Mod => {
                if right == 0 {
                    0
                } else {
                    left % right
                }
            }
            BinOp::Eq => if left == right { 1 } else { 0 },
            BinOp::Ne => if left != right { 1 } else { 0 },
            BinOp::Lt => if left < right { 1 } else { 0 },
            BinOp::Gt => if left > right { 1 } else { 0 },
            BinOp::Le => if left <= right { 1 } else { 0 },
            BinOp::Ge => if left >= right { 1 } else { 0 },
            BinOp::And => if left != 0 && right != 0 { 1 } else { 0 },
            BinOp::Or => if left != 0 || right != 0 { 1 } else { 0 },
        }
    }

    /// Run the TAC program and return result
    pub fn run(&mut self, instrs: &[Instr]) -> Option<i64> {
        // Build label map
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (i, instr) in instrs.iter().enumerate() {
            if let Instr::Label(l) = instr {
                labels.insert(l.clone(), i);
            }
        }

        let mut return_value: Option<i64> = None;

        while self.pc < instrs.len() {
            let instr = &instrs[self.pc].clone();

            match instr {
                Instr::BinOp { dst, op, left, right } => {
                    let l_val = self.get_addr(left);
                    let r_val = self.get_addr(right);
                    let result = self.eval_binop(op, l_val, r_val);
                    self.set_addr(dst, result);
                    self.pc += 1;
                }

                Instr::Copy { dst, src } => {
                    let val = self.get_addr(src);
                    self.set_addr(dst, val);
                    self.pc += 1;
                }

                Instr::Goto(label) => {
                    if let Some(&target) = labels.get(label) {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }

                Instr::IfGoto { cond, label } => {
                    let cond_val = self.get_addr(cond);
                    if cond_val != 0 {
                        if let Some(&target) = labels.get(label) {
                            self.pc = target;
                        } else {
                            self.pc += 1;
                        }
                    } else {
                        self.pc += 1;
                    }
                }

                Instr::Label(_) => {
                    self.pc += 1;
                }

                Instr::Return(addr) => {
                    return_value = Some(self.get_addr(addr));
                    break;
                }

                Instr::Halt => {
                    break;
                }
            }
        }

        return_value
    }
}